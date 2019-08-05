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

use constants::{CommandCode, StartupType};
use data::{Buffer, DataIn, DataOut};
use structs::{CommandHeader, ResponseHeader, TimeInfo};

pub trait Tpm {
    fn exec(&mut self, command: &[u8], response: &mut [u8]) -> Result<usize>;

    fn startup(&mut self, su: StartupType) -> Result<()> {
        self.run_command(CommandCode::Startup, &su)
    }

    fn shutdown(&mut self, su: StartupType) -> Result<()> {
        self.run_command(CommandCode::Shutdown, &su)
    }

    fn get_random(&mut self, num_bytes: u16) -> Result<Buffer> {
        self.run_command(CommandCode::GetRandom, &num_bytes)
    }

    fn stir_random(&mut self, data: &[u8]) -> Result<()> {
        self.run_command(CommandCode::StirRandom, data)
    }

    fn read_clock(&mut self) -> Result<TimeInfo> {
        self.run_command(CommandCode::ReadClock, &())
    }
}

trait TpmImpl: Tpm {
    fn run_command<O: DataOut>(
        &mut self,
        cmd: CommandCode,
        input: &(impl DataIn + ?Sized),
    ) -> Result<O> {
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

        let outlen = self.exec(&in_buffer[..size], &mut out_buffer)?;
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
}

impl<T: Tpm + ?Sized> TpmImpl for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_safety() {
        let _: Option<&dyn Tpm> = None;
    }
}
