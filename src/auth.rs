use crate::{error::AuthError, tpm, tpma, tpms, Handle};

pub trait Auth: core::fmt::Debug {
    fn get_auth(&self) -> tpms::AuthCommand;
    fn set_auth(&self, auth: &tpms::AuthResponse) -> Result<(), AuthError>;
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

    fn set_auth(&self, auth: &tpms::AuthResponse) -> Result<(), AuthError> {
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

pub trait AuthSlice {
    fn empty() -> Self;
    fn as_slice(&self) -> &[&dyn Auth];
}

impl<const N: usize> AuthSlice for [&dyn Auth; N] {
    fn empty() -> Self {
        [&PasswordAuth(&[]); N]
    }
    fn as_slice(&self) -> &[&dyn Auth] {
        self
    }
}
