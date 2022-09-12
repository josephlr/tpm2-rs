#![no_std]
#![feature(split_array, generic_associated_types, doc_cfg)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use core::num::NonZeroU32;

mod auth;
mod error;
mod polyfill;

pub use auth::*;
pub use error::*;
pub mod commands;
pub mod os;
pub mod types;

use commands::{Command, Response};
use polyfill::*;
use types::*;

pub type Handle = u32;

/// The main interface to a TPM2 device.
pub struct Tpm {
    buf: Box<[u8]>,
    driver: Box<dyn Driver>,
}

impl<'a> Tpm {
    #[inline]
    pub fn exec<C: Command>(&'a mut self, cmd: C) -> Result<C::Response<&'a [u8]>>
    where
        C::Response<&'a [u8]>: Unmarshal<'a>,
    {
        self.exec_with_auths(cmd, &[])
    }

    #[inline]
    pub fn exec_with_auths<C: Command>(
        &'a mut self,
        cmd: C,
        auths: &[&dyn Auth],
    ) -> Result<C::Response<&'a [u8]>>
    where
        C::Response<&'a [u8]>: Unmarshal<'a>,
    {
        let mut rsp: C::Response<&'a [u8]> = Default::default();
        let mut rsp_handles = <C::Response<&'a [u8]> as Response>::Handles::empty();
        exec_impl(
            self.driver.as_mut(),
            &mut self.buf,
            C::CODE,
            cmd.auth_handles().as_slice(),
            cmd.handles().as_slice(),
            auths,
            &cmd,
            rsp_handles.as_mut_slice(),
            &mut rsp,
        )?;
        rsp.set_handles(rsp_handles);
        Ok(rsp)
    }
}

#[allow(unused_variables)]
fn exec_impl(
    driver: &mut dyn Driver,
    buf: &mut [u8],
    code: tpm::CC,
    auth_handles: &[AuthHandle],
    handles: &[Handle],
    extra_auths: &[&dyn Auth],
    params: &dyn Marshal,
    rsp_handles: &mut [Handle],
    rsp_params: &mut dyn Unmarshal,
) -> Result<()> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::*;
    use std::vec::Vec;

    #[test]
    fn can_exec() {
        #[allow(dead_code)]
        fn take_tpm(tpm: &mut Tpm) -> Result<Vec<u8>> {
            tpm.exec(Startup {
                startup_type: tpm::SU::Clear,
            })?;

            let rsp = tpm.exec(GetRandom {
                bytes_requested: 12,
            })?;
            let b = Vec::from(rsp.random_bytes);

            tpm.exec(Shutdown {
                shutdown_type: tpm::SU::Clear,
            })?;
            Ok(b)
        }
    }
}
