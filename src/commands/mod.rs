use crate::{
    error::{MarshalError, UnmarshalError},
    marshal::pop_array_mut,
    polyfill::ToUsize,
    types::{tpm, tpms, Auth, CommandHeader, ResponseHeader},
    Error, Fixed, Marshal, Tpm, Unmarshal,
};
use core::fmt::Debug;

/// The maximum number of authorizations for a Command
const MAX_NUM_AUTHS: usize = 3;

mod sealed {
    use super::*;
    /// The object-safe functionality of a TPM Command
    pub trait CommandData {
        fn get_auths(&self, _: &mut [&dyn Auth; MAX_NUM_AUTHS]) -> usize {
            0
        }
        fn marshal_handles(&self, _: &mut &mut [u8]) -> Result<(), MarshalError> {
            Ok(())
        }
        fn marshal_params(&self, _: &mut &mut [u8]) -> Result<(), MarshalError> {
            Ok(())
        }
    }

    /// The object-safe functionality of a TPM Response
    pub trait ResponseData<'a> {
        fn unmarshal_handles(&mut self, _: &mut &[u8]) -> Result<(), UnmarshalError> {
            Ok(())
        }
        fn unmarshal_params(&mut self, _: &mut &'a [u8]) -> Result<(), UnmarshalError> {
            Ok(())
        }
    }
    impl ResponseData<'_> for () {}
}
use sealed::*;

/// Common Trait for all TPM2 Commands
pub trait Command: CommandData + Default + Debug {
    const CODE: tpm::CC;
    type Response<'a>: ResponseData<'a> + Default + Debug;
}

pub(crate) fn run_command<'a>(
    tpm: &'a mut dyn Tpm,
    code: tpm::CC,
    extra_auths: &[&dyn Auth],
    cmd: &dyn CommandData,
    rsp: &mut dyn ResponseData<'a>,
) -> Result<(), Error> {
    // Merge Auths into a single slice
    let mut auths: [&dyn Auth; 3] = Default::default();
    let num_cmd_auths = cmd.get_auths(&mut auths);
    let num_auths = num_cmd_auths + extra_auths.len();
    if num_auths > 3 {
        return Err(Error::TooManyAuths(num_auths));
    }
    auths[num_cmd_auths..num_auths].copy_from_slice(extra_auths);
    let auths = &auths[..num_auths];

    //// Marshal Command
    let mut cmd_buf = tpm.command_buf();
    let buf_len = cmd_buf.len();
    // Marshal the header at the end
    let header_buf: &mut [u8; CommandHeader::SIZE] = pop_array_mut(&mut cmd_buf)?;

    // Marshal Handles
    cmd.marshal_handles(&mut cmd_buf)?;

    // Marshal Authorization Area
    if !auths.is_empty() {
        // Marshal auth size at the end
        let auth_size_buf: &mut [u8; 4] = pop_array_mut(&mut cmd_buf)?;
        let cmd_buf_len = cmd_buf.len();

        for auth in auths {
            auth.get_auth().marshal(&mut cmd_buf)?;
        }

        let auth_size: u32 = (cmd_buf_len - cmd_buf.len()).try_into().unwrap();
        auth_size.marshal_fixed(auth_size_buf);
    }

    // Marshal Parameters
    cmd.marshal_params(&mut cmd_buf)?;

    // Marshal Header
    let cmd_header = CommandHeader {
        tag: if auths.is_empty() {
            tpm::ST::NoSessions
        } else {
            tpm::ST::Sessions
        },
        size: (buf_len - cmd_buf.len()).try_into().unwrap(),
        code,
    };
    cmd_header.marshal_fixed(header_buf);

    //// Execute the command
    tpm.execute_command(cmd_header.size)?;

    //// Unmarshal Response
    let mut rsp_buf: &'a [u8] = tpm.response_buf();
    let rsp_len = rsp_buf.len();
    let rsp_header = ResponseHeader::unmarshal_val(&mut rsp_buf)?;

    // Check for errors
    assert!(rsp_header.size.to_usize() == rsp_len);
    if let Some(tpm_err) = rsp_header.code {
        return Err(Error::Tpm(tpm_err));
    }
    assert!(rsp_header.tag == cmd_header.tag);

    // Unmarshal Handles
    rsp.unmarshal_handles(&mut rsp_buf)?;

    // Unmarshal Authorization Area
    if !auths.is_empty() {
        let param_size = u32::unmarshal_val(&mut rsp_buf)?;
        let mut auth_buf: &[u8];
        (rsp_buf, auth_buf) = rsp_buf.split_at(param_size.try_into().unwrap());

        let mut auth_rsp = tpms::AuthResponse::default();
        for auth in auths {
            auth_rsp.unmarshal(&mut auth_buf)?;
            auth.set_auth(&auth_rsp)?;
        }
        assert!(auth_buf.is_empty());
    }

    // Unmarshal Parameters
    rsp.unmarshal_params(&mut rsp_buf)?;
    if !rsp_buf.is_empty() {
        return Err(Error::Unmarshal(UnmarshalError::BufferRemaining));
    }
    Ok(())
}

mod structs;
pub use structs::*;
