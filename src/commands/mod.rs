use crate::{
    auth::{AuthHandle, AuthHandleSlice, HandleSlice},
    run_impl,
    types::{tpm::CC, Buffer, Marshal, Unmarshal},
    Auth, Handle, Result, Tpm,
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

    fn run<'a>(&self, tpm: &'a mut dyn Tpm) -> Result<Self::Response<&'a [u8]>>
    where
        Self::Response<&'a [u8]>: Unmarshal<'a>,
    {
        self.run_with_auths(tpm, &[])
    }

    #[inline]
    fn run_with_auths<'a>(
        &self,
        tpm: &'a mut dyn Tpm,
        auths: &[&dyn Auth],
    ) -> Result<Self::Response<&'a [u8]>>
    where
        Self::Response<&'a [u8]>: Unmarshal<'a>,
    {
        let mut rsp: Self::Response<&'a [u8]> = Default::default();
        let mut rsp_handles = <Self::Response<&'a [u8]> as Response>::Handles::empty();
        run_impl(
            tpm,
            Self::CODE,
            self.auth_handles().as_slice(),
            self.handles().as_slice(),
            auths,
            self,
            rsp_handles.as_mut_slice(),
            &mut rsp,
        )?;
        rsp.set_handles(rsp_handles);
        Ok(rsp)
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
