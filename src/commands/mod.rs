use crate::{
    auth::{AuthHandle, AuthHandleSlice, HandleSlice},
    types::{tpm::CC, Buffer, Marshal, Unmarshal},
    Handle, Result,
};
use core::fmt::Debug;

pub trait Command: Marshal + Default + Debug {
    const CODE: CC;
    type Response<B: Buffer>: Response;

    type AuthHandles: AuthHandleSlice;
    fn auth_handles(&self) -> Self::AuthHandles {
        Self::AuthHandles::empty()
    }
    type Handles: HandleSlice;
    fn handles(&self) -> Self::Handles {
        Self::Handles::empty()
    }
}

pub trait Response: Default + Debug {
    type Handles: HandleSlice;
    fn set_handles(&mut self, _: Self::Handles) {}
}

impl Response for () {
    type Handles = [Handle; 0];
}

mod structs;
pub use structs::*;
