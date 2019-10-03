use crate::{Error, Result};
use core::num::NonZeroU32;

pub mod constants;
pub mod data;
pub mod structs;
pub mod unions;
pub mod util;

use constants::{CommandCode, StartupType};
use data::{Buffer, DataIn, DataOut};
use structs::{CommandHeader, ResponseHeader, TimeInfo};

pub trait Tpm {
    /// Attempts to write all of `buf` into the writer. This differs from
    /// [`std::io::Write::write`] (which returns the number of bytes written),
    /// and is more similar to [`std::io::Write::write_all`].
    fn write(&mut self, buf: &[u8]) -> Result<()>;

    /// Fills all of `buf` or fails. This differs from [`std::io::Read::read`]
    /// (which returns the number of bytes read), and is more similar to
    /// [`std::io::Read::read_exact`].
    fn read(&mut self, buf: &mut [u8]) -> Result<()>;

    /// Executes previously written command data.
    fn run_command(&mut self) -> Result<()>;
    /// Resets any date written by [`write`].
    fn reset_command(&mut self) -> Result<()>;

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
