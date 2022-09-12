use crate::{
    auth::AuthSlice,
    run_impl,
    types::{tpm::CC, Marshal, Unmarshal},
    Auth, Result, Tpm,
};
use core::fmt::Debug;

pub trait CommandData {
    fn marshal_handles(&self, _: &mut &mut [u8]) -> Result<()> {
        Ok(())
    }
    fn marshal_params(&self, _: &mut &mut [u8]) -> Result<()>;
}

pub trait ResponseData<'a> {
    fn unmarshal_handles(&mut self, _: &mut &[u8]) -> Result<()> {
        Ok(())
    }
    fn unmarshal_params(&mut self, _: &mut &'a [u8]) -> Result<()>;
}

impl ResponseData<'_> for () {
    fn unmarshal_params(&mut self, _: &mut &[u8]) -> Result<()> {
        Ok(())
    }
}

pub trait Command: CommandData + Default + Debug {
    const CODE: CC;
    type Response<'a>: ResponseData<'a> + Default + Debug;

    type Auths: AuthSlice;
    fn auths(&self) -> Self::Auths {
        Self::Auths::empty()
    }

    fn run<'a>(&self, tpm: &'a mut dyn Tpm) -> Result<Self::Response<'a>> {
        self.run_with_auths(tpm, &[])
    }

    #[inline]
    fn run_with_auths<'a>(
        &self,
        tpm: &'a mut dyn Tpm,
        auths: &[&dyn Auth],
    ) -> Result<Self::Response<'a>> {
        let mut rsp: Self::Response<'a> = Default::default();
        run_impl(
            tpm,
            Self::CODE,
            self.auths().as_slice(),
            auths,
            self,
            &mut rsp,
        )?;
        Ok(rsp)
    }
}

mod structs;
pub use structs::*;
