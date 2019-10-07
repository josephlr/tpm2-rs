use core::{convert::TryInto, num::NonZeroU32};

use alloc::vec::Vec;

use crate::{Error, Result};

pub mod constants;
use constants::*;

pub mod structs;
use structs::*;

pub mod unions;

mod traits;
pub use traits::*;

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
        run(self, CommandCode::Startup, &su)
    }

    fn shutdown(&mut self, su: StartupType) -> Result<()> {
        run(self, CommandCode::Shutdown, &su)
    }

    fn get_random<'a>(&mut self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        let len = buf.len() as u16;
        run(self, CommandCode::GetRandom, &len)?.parse_ref(buf)
    }

    fn stir_random(&mut self, data: &[u8]) -> Result<()> {
        run(self, CommandCode::StirRandom, data)
    }

    fn read_clock(&mut self) -> Result<TimeInfo> {
        run(self, CommandCode::ReadClock, &())
    }
}

fn run<'a, Output: ResponseData>(
    tpm: &'a mut (impl Tpm + ?Sized),
    code: CommandCode,
    input: &(impl CommandData + ?Sized),
) -> Result<Output> {
    let tag = tag::Command::NoSessions;
    let mut cmd_hdr = CommandHeader { tag, size: 0, code };
    cmd_hdr.size = (cmd_hdr.data_len() + input.data_len()).try_into().unwrap();

    tpm.reset_command()?;
    cmd_hdr.encode(tpm)?;
    input.encode(tpm)?;

    tpm.run_command()?;

    let resp_hdr = ResponseHeader::decode(tpm)?;
    if let Some(err) = NonZeroU32::new(resp_hdr.code) {
        return Err(Error::Tpm(err));
    }
    assert_eq!(resp_hdr.tag, tag);

    let len = resp_hdr.size as usize - resp_hdr.data_len();
    let mut reader = CheckedReader { tpm, len };
    let output = Output::decode(&mut reader)?;

    if reader.len != 0 {
        return Err(Error::RemainingOutputData);
    }
    Ok(output)
}

struct CheckedReader<'a, T: ?Sized> {
    tpm: &'a mut T,
    len: usize,
}

impl<T: Tpm + ?Sized> Tpm for CheckedReader<'_, T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        if buf.len() > self.len {
            return Err(Error::MissingOutputData);
        }
        self.tpm.read(buf)?;
        self.len -= buf.len();
        Ok(())
    }
    fn write(&mut self, _: &[u8]) -> Result<()> {
        unimplemented!()
    }
    fn run_command(&mut self) -> Result<()> {
        unimplemented!()
    }
    fn reset_command(&mut self) -> Result<()> {
        unimplemented!()
    }
}
