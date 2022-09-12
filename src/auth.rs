use crate::{tpms, Handle, Result};

pub trait Auth: core::fmt::Debug {
    fn get_auth(&self) -> tpms::AuthCommand;
    fn set_auth(&self, auth: &tpms::AuthResponse) -> Result<()>;
}

#[derive(Debug, Clone, Copy)]
pub struct AuthHandle<'a> {
    pub handle: Handle,
    pub auth: &'a dyn Auth,
}

pub trait AuthHandleSlice {
    fn empty() -> Self;
    fn as_slice(&self) -> &[AuthHandle];
}

impl<const N: usize> AuthHandleSlice for [AuthHandle<'_>; N] {
    fn empty() -> Self {
        todo!()
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
