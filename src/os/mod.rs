//! TODO: Document this module
#![cfg(feature = "std")]
#![doc(cfg(feature = "std"))]

use std::{io, prelude::v1::*};

use crate::{error::DriverError, polyfill::ToUsize, Tpm};

// Keep in sync with default_tpm cfg
cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        use linux::*;
    } else if #[cfg(windows)] {
        mod windows;
        use windows::*;
    }
}

struct RwTpm<RW> {
    cmd: Box<[u8]>,
    rsp: Vec<u8>,
    rw: RW,
}

impl<RW: io::Read + io::Write> Tpm for RwTpm<RW> {
    fn command_buf(&mut self) -> &mut [u8] {
        &mut self.cmd
    }

    fn response_buf(&self) -> &[u8] {
        &self.rsp
    }

    fn execute_command(&mut self, cmd_size: u32) -> Result<(), DriverError> {
        self.rw.write_all(&self.cmd[..cmd_size.to_usize()])?;
        self.rsp.clear();
        self.rw.read_to_end(&mut self.rsp)?;
        Ok(())
    }
}

// TODO: explain why you would want this
pub fn tpm_from_read_write(rw: impl io::Read + io::Write) -> impl Tpm {
    RwTpm {
        cmd: vec![0; 4096].into_boxed_slice(),
        rsp: vec![],
        rw,
    }
}

/// TODO: Document this for Linux and Windows
// Keep in sync with cfg_if
#[cfg(any(target_os = "linux", windows))]
#[doc(cfg(any(target_os = "linux", windows)))]
pub fn default_tpm() -> io::Result<impl Tpm> {
    default_impl()
}
