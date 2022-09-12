use crate::{tpm, tpma, tpms, Handle, Result};

pub trait Auth: core::fmt::Debug {
    fn get_auth(&self) -> tpms::AuthCommand;
    fn set_auth(&self, auth: &tpms::AuthResponse) -> Result<()>;
}

#[derive(Debug, Clone, Copy)]
pub struct AuthHandle<'a> {
    pub handle: Handle,
    pub auth: &'a dyn Auth,
}

#[derive(Debug)]
pub struct PasswordAuth<'a>(&'a [u8]);

const CONTINUE_SESSION: tpma::Session = {
    let mut s = tpma::Session::empty();
    s.continue_session = true;
    s
};

impl Auth for PasswordAuth<'_> {
    fn get_auth(&self) -> tpms::AuthCommand {
        tpms::AuthCommand {
            session_handle: tpm::rh::PASSWORD,
            nonce: &[],
            session_attributes: CONTINUE_SESSION,
            hmac: self.0,
        }
    }

    fn set_auth(&self, auth: &tpms::AuthResponse) -> Result<()> {
        assert!(auth.nonce.is_empty());
        assert_eq!(auth.session_attributes, CONTINUE_SESSION);
        assert!(auth.hmac.is_empty());
        Ok(())
    }
}

/// Convert a handle to an AuthHandle with Password Authorization
impl From<Handle> for AuthHandle<'_> {
    fn from(handle: Handle) -> Self {
        AuthHandle {
            handle,
            auth: &PasswordAuth(&[]),
        }
    }
}

pub trait AuthHandleSlice {
    fn empty() -> Self;
    fn as_slice(&self) -> &[AuthHandle];
}

impl<const N: usize> AuthHandleSlice for [AuthHandle<'_>; N] {
    fn empty() -> Self {
        let ah: AuthHandle = (0 as Handle).into();
        [ah; N]
    }
    fn as_slice(&self) -> &[AuthHandle] {
        self
    }
}

pub trait HandleSlice {
    fn empty() -> Self;
    fn as_slice(&self) -> &[Handle];
    fn as_mut_slice(&mut self) -> &mut [Handle];
}

impl<const N: usize> HandleSlice for [Handle; N] {
    fn empty() -> Self {
        [0; N]
    }
    fn as_slice(&self) -> &[Handle] {
        self
    }
    fn as_mut_slice(&mut self) -> &mut [Handle] {
        self
    }
}
