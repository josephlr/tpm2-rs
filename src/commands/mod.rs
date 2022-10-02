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
    marshal::{pop_array_mut, CommandData, ResponseData},
    polyfill::ToUsize,
    types::{tpm, tpms, Auth, CommandHeader, ResponseHeader},
    Error, Marshal, MarshalFixed, Tpm, Unmarshal,
};

/// Common Trait for all TPM2 Commands
pub trait Command: CommandData + Copy + Debug {
    const CODE: tpm::CC;
    type Response<'t>: ResponseData<'t> + Default + Copy + Debug;

    /// This helper function isn't necessary for correctness, but exists to
    /// reduce the number of vtables we use. If we have a type of `&C`, `&&C`,
    /// [`WithAuth<'a, C>`] or `&WithAuth<'a, &C>`, we can instead use the
    /// vtable for `C`. See the difference in code generation:
    /// - [without the helper](https://godbolt.org/z/3Yv9TYT18)
    /// - [with the helper](https://godbolt.org/z/793r1ccjc)
    /// Note the difference in the number of vtables emitted.
    ///
    /// It should always be the case that `c.data().marshal_*()` and
    /// `c.marshal_*() do the exact same thing.
    fn data(&self) -> &dyn CommandData {
        self
    }

    fn with_auth(self, auth: &'_ dyn Auth) -> WithAuth<'_, Self> {
        WithAuth(self, auth)
    }
}

pub trait Auths<const N: usize> {
    fn auths(&self) -> [&dyn Auth; N] {
        [Default::default(); N]
    }
}

impl<C: CommandData> CommandData for &C {
    fn marshal_handles(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        (*self).marshal_handles(buf)
    }
    fn marshal_params(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        (*self).marshal_params(buf)
    }
}
impl<C: Command> Command for &C {
    const CODE: tpm::CC = C::CODE;
    type Response<'t> = C::Response<'t>;
    #[inline]
    fn data(&self) -> &dyn CommandData {
        (*self).data()
    }
}
impl<const N: usize, C: Auths<N>> Auths<N> for &C {
    #[inline]
    fn auths(&self) -> [&dyn Auth; N] {
        (*self).auths()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WithAuth<'a, C>(C, &'a dyn Auth);

impl<C: CommandData> CommandData for WithAuth<'_, C> {
    #[inline]
    fn marshal_handles(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.0.marshal_handles(buf)
    }
    #[inline]
    fn marshal_params(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.0.marshal_params(buf)
    }
}
impl<C: Command> Command for WithAuth<'_, C> {
    const CODE: tpm::CC = C::CODE;
    type Response<'t> = C::Response<'t>;
    #[inline]
    fn data(&self) -> &dyn CommandData {
        self.0.data()
    }
}

impl<C: Auths<0>> Auths<1> for WithAuth<'_, C> {
    #[inline]
    fn auths(&self) -> [&dyn Auth; 1] {
        [self.1]
    }
}
impl<C: Auths<1>> Auths<2> for WithAuth<'_, C> {
    #[inline]
    fn auths(&self) -> [&dyn Auth; 2] {
        let [a1] = self.0.auths();
        [a1, self.1]
    }
}
impl<C: Auths<2>> Auths<3> for WithAuth<'_, C> {
    #[inline]
    fn auths(&self) -> [&dyn Auth; 3] {
        let [a1, a2] = self.0.auths();
        [a1, a2, self.1]
    }
}

// This function is intentionally non-generic to reduce code size.
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
