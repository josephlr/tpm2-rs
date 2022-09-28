//! TODO: Document this module
#![cfg(feature = "std")]
#![doc(cfg(feature = "std"))]

use crate::{error::DriverError, polyfill::ToUsize, Tpm};
use alloc::{boxed::Box, vec, vec::Vec};
use core::ops::DerefMut;
use std::io::{self, ErrorKind, Read, Write};

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        #[doc(cfg(target_os = "linux"))]
        pub use linux::*;
    } else if #[cfg(windows)] {
        mod windows;
        #[doc(cfg(windows))]
        pub use windows::*;
    } else {

    }
}

struct RwTpm<RW: ?Sized> {
    cmd: Box<[u8]>,
    rsp: Vec<u8>,
    rw: RW,
}

impl<T: Read + Write + ?Sized, RW: DerefMut<Target = T> + ?Sized> Tpm for RwTpm<RW> {
    fn command_buf(&mut self) -> &mut [u8] {
        &mut self.cmd
    }

    fn response_buf(&self) -> &[u8] {
        &self.rsp
    }

    fn execute_command(&mut self, cmd_size: u32) -> Result<u32, DriverError> {
        self.rw.write_all(&self.cmd[..cmd_size.to_usize()])?;
        let rsp_len: u32 = self.rw.read_to_end(&mut self.rsp)?.try_into()?;
        Ok(rsp_len)
    }
}

// TODO: explain why you would want this
pub fn tpm_from_read_write(rw: impl DerefMut<Target = impl Read + Write>) -> impl Tpm {
    RwTpm {
        cmd: vec![0; 4096].into_boxed_slice(),
        rsp: vec![],
        rw,
    }
}

// TODO: Document that this blocks
pub fn get_default_tpm() -> io::Result<impl Tpm> {
    default_impl()
}
