use super::{tpm, tpma, tpms, Handle};
use crate::Error;

pub trait Auth: core::fmt::Debug {
    fn get_auth(&self) -> tpms::AuthCommand;
    fn set_auth(&self, auth: &tpms::AuthResponse) -> Result<(), Error>;
}

#[derive(Debug, Clone, Copy)]
pub struct AuthHandle<'a> {
    pub handle: Handle,
    pub auth: &'a dyn Auth,
}

#[derive(Debug)]
pub struct PasswordAuth<'a>(pub &'a [u8]);

impl Auth for PasswordAuth<'_> {
    fn get_auth(&self) -> tpms::AuthCommand {
        tpms::AuthCommand {
            session_handle: tpm::rh::PASSWORD,
            nonce: &[],
            session_attributes: tpma::Session::CONTINUE_SESSION,
            hmac: self.0,
        }
    }

    fn set_auth(&self, auth: &tpms::AuthResponse) -> Result<(), Error> {
        assert!(auth.nonce.is_empty());
        assert_eq!(auth.session_attributes, tpma::Session::CONTINUE_SESSION);
        assert!(auth.hmac.is_empty());
        Ok(())
    }
}

/// The default Auth (i.e. no Auth) is an empty Password Auth.
impl Default for &dyn Auth {
    fn default() -> Self {
        &PasswordAuth(&[])
    }
}

/// Convert a handle to an AuthHandle with Password Authorization
impl From<Handle> for AuthHandle<'_> {
    fn from(handle: Handle) -> Self {
        AuthHandle {
            handle,
            auth: Default::default(),
        }
    }
}
