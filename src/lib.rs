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
pub trait Tpm {
    fn command_buf(&mut self) -> &mut [u8];
    fn response_buf(&self) -> &[u8];
    fn execute_command(&mut self, cmd_size: u32) -> Result<u32>;
}

impl dyn Tpm + '_ {
    #[inline]
    pub fn run<'a, C: Command>(&'a mut self, cmd: C) -> Result<C::Response<&'a [u8]>>
    where
        C::Response<&'a [u8]>: Unmarshal<'a>,
    {
        self.run_with_auths(cmd, &[])
    }

    pub fn run_with_auths<'a, C: Command>(
        &'a mut self,
        cmd: C,
        auths: &[&dyn Auth],
    ) -> Result<C::Response<&'a [u8]>>
    where
        C::Response<&'a [u8]>: Unmarshal<'a>,
    {
        let mut rsp: C::Response<&'a [u8]> = Default::default();
        let mut rsp_handles = <C::Response<&'a [u8]> as Response>::Handles::empty();
        run_impl(
            self,
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

fn run_impl<'a>(
    tpm: &'a mut dyn Tpm,
    code: tpm::CC,
    auth_handles: &[AuthHandle],
    cmd_handles: &[Handle],
    extra_auths: &[&dyn Auth],
    cmd_params: &dyn Marshal,
    rsp_handles: &mut [Handle],
    rsp_params: &mut dyn Unmarshal<'a>,
) -> Result<()> {
    let has_auths = !auth_handles.is_empty() || !extra_auths.is_empty();

    //// Marshal Command
    let mut cmd_buf = tpm.command_buf();
    let buf_len = cmd_buf.len();
    // Marshal the header at the end
    let header_buf: &mut [u8];
    (header_buf, cmd_buf) = cmd_buf.split_at_mut(CommandHeader::SIZE);

    // Marshal Handles
    for auth_handle in auth_handles {
        auth_handle.handle.marshal(&mut cmd_buf)?;
    }
    for handle in cmd_handles {
        handle.marshal(&mut cmd_buf)?;
    }

    // Marshal Authorization Area
    if has_auths {
        assert!(auth_handles.len() + extra_auths.len() <= 3);
        // Marshal auth size at the end
        let auth_size_buf: &mut [u8];
        (auth_size_buf, cmd_buf) = cmd_buf.split_at_mut(u32::SIZE);
        let cmd_buf_len = cmd_buf.len();

        for auth_handle in auth_handles {
            auth_handle.auth.get_auth().marshal(&mut cmd_buf)?;
        }
        for auth in extra_auths {
            auth.get_auth().marshal(&mut cmd_buf)?;
        }

        let auth_size: u32 = (cmd_buf_len - cmd_buf.len()).try_into().unwrap();
        auth_size.marshal_exact(auth_size_buf)?;
    }

    // Marshal Parameters
    cmd_params.marshal(&mut cmd_buf)?;

    // Marshal Header
    let cmd_header = CommandHeader {
        tag: if has_auths {
            tpm::ST::Sessions
        } else {
            tpm::ST::NoSessions
        },
        size: (buf_len - cmd_buf.len()).try_into().unwrap(),
        code,
    };
    cmd_header.marshal_exact(header_buf)?;

    //// Execute the command
    let rsp_len = tpm.execute_command(cmd_header.size)?;

    //// Unmarshal Response
    let mut rsp_buf: &'a [u8] = &tpm.response_buf()[..rsp_len.to_usize()];
    let rsp_header = ResponseHeader::unmarshal_val(&mut rsp_buf)?;

    // Check for errors
    assert!(rsp_header.size == rsp_len);
    if let Some(err_code) = NonZeroU32::new(rsp_header.code.0) {
        return Err(Error::Tpm(err_code));
    }
    assert!(rsp_header.tag == cmd_header.tag);

    // Unmarshal Handles
    for handle in rsp_handles {
        handle.unmarshal(&mut rsp_buf)?;
    }

    // Unmarshal Authorization Area
    if has_auths {
        let param_size = u32::unmarshal_val(&mut rsp_buf)?;
        let mut auth_buf: &[u8];
        (rsp_buf, auth_buf) = rsp_buf.split_at(param_size.try_into().unwrap());

        let mut auth_rsp = tpms::AuthResponse::default();
        for auth_handle in auth_handles {
            auth_rsp.unmarshal(&mut auth_buf)?;
            auth_handle.auth.set_auth(&auth_rsp)?;
        }
        for auth in extra_auths {
            auth_rsp.unmarshal(&mut auth_buf)?;
            auth.set_auth(&auth_rsp)?;
        }
        assert!(auth_buf.is_empty());
    }

    // Unmarshal Parameters
    rsp_params.unmarshal_exact(rsp_buf)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::*;
    use std::vec::Vec;

    #[test]
    fn can_exec() {
        #[allow(dead_code)]
        fn take_tpm(tpm: &mut dyn Tpm) -> Result<Vec<u8>> {
            tpm.run(Startup {
                startup_type: tpm::SU::Clear,
            })?;

            let rsp = tpm.run(GetRandom {
                bytes_requested: 12,
            })?;
            let b = Vec::from(rsp.random_bytes);

            tpm.run(Shutdown {
                shutdown_type: tpm::SU::Clear,
            })?;
            Ok(b)
        }
    }
}
