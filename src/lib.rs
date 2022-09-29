#![no_std]
#![feature(split_array, doc_cfg, once_cell)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use core::fmt::Debug;

mod auth;
pub use auth::*;
pub mod commands;
pub use error::Error;
pub mod error;
pub mod os;
mod polyfill;
pub mod types;

use commands::{CommandData, ResponseData};
use error::{DriverError, UnmarshalError};
use polyfill::ToUsize;
use types::*;

pub type Handle = u32;

/// A TPM2 Device
pub trait Tpm {
    fn command_buf(&mut self) -> &mut [u8];
    fn response_buf(&self) -> &[u8];
    fn execute_command(&mut self, cmd_size: u32) -> Result<(), DriverError>;
}

/// A TPM2 Command
pub trait Command: CommandData + Default + Debug {
    const CODE: tpm::CC;
    type Response<'a>: ResponseData<'a> + Default + Debug;

    type Auths: AuthSlice;
    fn auths(&self) -> Self::Auths {
        Self::Auths::empty()
    }
}

// Helper trait for running raw commands directly
pub trait Run: Tpm {
    #[inline]
    fn run<'a, C: Command>(&'a mut self, cmd: &C) -> Result<C::Response<'a>, Error> {
        self.run_with_auths(cmd, &[])
    }

    #[inline]
    fn run_with_auths<'a, C: Command>(
        &'a mut self,
        cmd: &C,
        auths: &[&dyn Auth],
    ) -> Result<C::Response<'a>, Error> {
        let mut rsp: C::Response<'a> = Default::default();
        run_impl(
            self.as_dyn(),
            C::CODE,
            cmd.auths().as_slice(),
            auths,
            cmd,
            &mut rsp,
        )?;
        Ok(rsp)
    }

    fn as_dyn(&mut self) -> &mut dyn Tpm;
}

impl<T: Tpm> Run for T {
    #[inline]
    fn as_dyn(&mut self) -> &mut dyn Tpm {
        self
    }
}
impl Run for dyn Tpm + '_ {
    #[inline]
    fn as_dyn(&mut self) -> &mut dyn Tpm {
        self
    }
}

fn run_impl<'a>(
    tpm: &'a mut dyn Tpm,
    code: tpm::CC,
    cmd_auths: &[&dyn Auth],
    extra_auths: &[&dyn Auth],
    cmd: &dyn CommandData,
    rsp: &mut dyn ResponseData<'a>,
) -> Result<(), Error> {
    let has_auths = !cmd_auths.is_empty() || !extra_auths.is_empty();

    //// Marshal Command
    let mut cmd_buf = tpm.command_buf();
    let buf_len = cmd_buf.len();
    // Marshal the header at the end
    let header_buf: &mut [u8; CommandHeader::SIZE] = pop_array_mut(&mut cmd_buf)?;

    // Marshal Handles
    cmd.marshal_handles(&mut cmd_buf)?;

    // Marshal Authorization Area
    if has_auths {
        assert!(cmd_auths.len() + extra_auths.len() <= 3);
        // Marshal auth size at the end
        let auth_size_buf: &mut [u8; 4] = pop_array_mut(&mut cmd_buf)?;
        let cmd_buf_len = cmd_buf.len();

        for auth in cmd_auths {
            auth.get_auth().marshal(&mut cmd_buf)?;
        }
        for auth in extra_auths {
            auth.get_auth().marshal(&mut cmd_buf)?;
        }

        let auth_size: u32 = (cmd_buf_len - cmd_buf.len()).try_into().unwrap();
        auth_size.marshal_fixed(auth_size_buf);
    }

    // Marshal Parameters
    cmd.marshal_params(&mut cmd_buf)?;

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
    if has_auths {
        let param_size = u32::unmarshal_val(&mut rsp_buf)?;
        let mut auth_buf: &[u8];
        (rsp_buf, auth_buf) = rsp_buf.split_at(param_size.try_into().unwrap());

        let mut auth_rsp = tpms::AuthResponse::default();
        for auth in cmd_auths {
            auth_rsp.unmarshal(&mut auth_buf)?;
            auth.set_auth(&auth_rsp)?;
        }
        for auth in extra_auths {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::*;
    use std::vec::Vec;

    #[test]
    fn can_exec() {
        #[allow(dead_code)]
        fn take_tpm(tpm: &mut dyn Tpm) -> Result<Vec<u8>, Error> {
            tpm.run(&Startup {
                startup_type: tpm::SU::Clear,
            })?;

            let rsp = tpm.run(&GetRandom {
                bytes_requested: 12,
            })?;
            let b = Vec::from(rsp.random_bytes);

            tpm.run(&Shutdown {
                shutdown_type: tpm::SU::Clear,
            })?;
            Ok(b)
        }
    }
}
