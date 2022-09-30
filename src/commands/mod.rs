//! TPM2 [`Command`]s and Responses
//!
//! These structures should be fairly direct translations of the
//! "TPM2_* Command" and "TPM2_* Response" tables in Part 3 of the TPM2 Spec.
//!
//! Of the 117 TPM2 commands, 0 are implemented.
//! If a command is not implemented, there will be a skeleton of code and doc
//! comments which are commented out.
//!
//! TODO:
//!   - Add additional notes about TPM2_HMAC and TPM2_StartHMAC

use core::fmt::Debug;

use crate::{
    error::{MarshalError, UnmarshalError},
    marshal::pop_array_mut,
    polyfill::ToUsize,
    types::{tpm, tpms, Auth, CommandHeader, ResponseHeader},
    Error, Marshal, MarshalFixed, Tpm, Unmarshal,
};

mod sealed {
    use super::*;
    /// The object-safe functionality of a TPM Command
    pub trait CommandData {
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

    /// Helper trait to reduce generated code (can't use AsRef or Borrow here).
    pub trait Inner {
        fn inner(&self) -> &dyn CommandData;
    }
    impl<T: CommandData> Inner for T {
        fn inner(&self) -> &dyn CommandData {
            self
        }
    }

    /// Helper trait for dealing with Auth Arrays
    pub trait AuthArray {
        fn as_slice(&self) -> &[&dyn Auth];
    }
    impl<const N: usize> AuthArray for [&dyn Auth; N] {
        fn as_slice(&self) -> &[&dyn Auth] {
            self
        }
    }
}
pub(crate) use sealed::*;

/// Auth arrays which can add an additional authorization
pub trait AppendAuth<'e>: AuthArray {
    type Output: AuthArray;
    fn append(self, auth: &'e dyn Auth) -> Self::Output;
}
impl<'e> AppendAuth<'e> for [&dyn Auth; 0] {
    type Output = [&'e dyn Auth; 1];
    fn append(self, auth: &'e dyn Auth) -> Self::Output {
        [auth]
    }
}
impl<'e, 'a: 'e> AppendAuth<'e> for [&'a dyn Auth; 1] {
    type Output = [&'e dyn Auth; 2];
    fn append(self, auth: &'e dyn Auth) -> Self::Output {
        let [a1] = self;
        [a1, auth]
    }
}
impl<'e, 'a: 'e> AppendAuth<'e> for [&'a dyn Auth; 2] {
    type Output = [&'e dyn Auth; 3];
    fn append(self, auth: &'e dyn Auth) -> Self::Output {
        let [a1, a2] = self;
        [a1, a2, auth]
    }
}

/// Helper type for implementing [`Command::with_auth`]
#[derive(Debug)]
pub struct WithAuth<'a, C>(C, &'a dyn Auth);
impl<'a, C: Command> Inner for WithAuth<'a, C> {
    fn inner(&self) -> &dyn CommandData {
        self.0.inner()
    }
}
impl<'a, C: Command> Command for WithAuth<'a, C>
where
    C::Auths: AppendAuth<'a>,
{
    const CODE: tpm::CC = C::CODE;
    type Response<'b> = C::Response<'b>;

    type Auths = <C::Auths as AppendAuth<'a>>::Output;
    fn auths(&self) -> Self::Auths {
        self.0.auths().append(self.1)
    }
}

/// Common Trait for all TPM2 Commands
pub trait Command: Inner + Debug + Sized {
    const CODE: tpm::CC;
    type Response<'a>: ResponseData<'a> + Default + Debug;

    type Auths: AuthArray;
    fn auths(&self) -> Self::Auths;

    // TODO: Move to `impl Command` when that's supported.
    fn with_auth<'a>(self, auth: &'a dyn Auth) -> WithAuth<'a, Self>
    where
        WithAuth<'a, Self>: Command,
    {
        WithAuth(self, auth)
    }
}

// This functions is intentionally non-generic to reduce code size.
pub(crate) fn run_command<'a>(
    tpm: &'a mut dyn Tpm,
    auths: &[&dyn Auth],
    cmd: &dyn CommandData,
    rsp: &mut dyn ResponseData<'a>,
    code: tpm::CC,
) -> Result<(), Error> {
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
