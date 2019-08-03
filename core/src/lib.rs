//! A TPM2 TSS. Add more docs and doc-tests.

// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
// #![doc(test(attr(allow(unused_variables), deny(warnings))))]

#![no_std]
use core::num::NonZeroU32;

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod constants;
pub mod data;
mod error;
pub use error::{Error, Result};
pub mod structs;
pub mod unions;
pub mod util;

use constants::StartupType;
use data::{Buffer, DataIn, DataOut};
use structs::TimeInfo;

pub trait Tpm {
    fn exec(&mut self, command: &[u8], response: &mut [u8]) -> Result<usize>;

    fn startup(&mut self, su: StartupType) -> Result<()> {
        run_command(self, 0x144, su)
    }

    fn shutdown(&mut self, su: StartupType) -> Result<()> {
        run_command(self, 0x145, su)
    }

    fn get_random(&mut self, num_bytes: u16) -> Result<Buffer> {
        run_command(self, 0x17B, num_bytes)
    }

    fn read_clock(&mut self) -> Result<TimeInfo> {
        run_command(self, 0x181, ())
    }
}

fn run_command<T, I, O>(tpm: &mut T, cmd: u32, input: I) -> Result<O>
where
    T: Tpm + ?Sized,
    I: DataIn,
    O: DataOut,
{
    let mut in_buffer = [0u8; 4096];
    let mut out_buffer = [0u8; 4096];

    let (header, data) = in_buffer.split_at_mut(10);
    let unused = input.into_bytes(data)?;

    let tag = 0x8001;
    let size = 4096 - unused.len();
    CommandHeader {
        tag,
        size: size as u32,
        cmd,
    }
    .into_bytes(header)?;

    let outlen = tpm.exec(&in_buffer[..size], &mut out_buffer)?;
    let mut output = &out_buffer[..outlen];

    let header = ResponseHeader::from_bytes(&mut output)?;
    if header.tag != tag {
        return Err(Error::TagMismatch);
    }
    if header.size as usize != outlen {
        return Err(Error::OutLenMismatch);
    }
    if let Some(err) = NonZeroU32::new(header.resp) {
        return Err(Error::Tpm(err));
    }

    let o = O::from_bytes(&mut output)?;
    if !output.is_empty() {
        return Err(Error::RemainingData);
    }
    Ok(o)
}

struct CommandHeader {
    tag: u16,
    size: u32,
    cmd: u32,
}

impl DataIn for CommandHeader {
    fn into_bytes<'a>(&self, mut bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
        bytes = self.tag.into_bytes(bytes)?;
        bytes = self.size.into_bytes(bytes)?;
        bytes = self.cmd.into_bytes(bytes)?;
        Ok(bytes)
    }
}

struct ResponseHeader {
    tag: u16,
    size: u32,
    resp: u32,
}

impl DataOut for ResponseHeader {
    fn from_bytes(bytes: &mut &[u8]) -> Result<Self> {
        Ok(Self {
            tag: DataOut::from_bytes(bytes)?,
            size: DataOut::from_bytes(bytes)?,
            resp: DataOut::from_bytes(bytes)?,
        })
    }
}
