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
        run(self, CommandCode::Startup, &su)?.parse()
    }

    fn shutdown(&mut self, su: StartupType) -> Result<()> {
        run(self, CommandCode::Shutdown, &su)?.parse()
    }

    fn get_random<'a>(&mut self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        let len = buf.len() as u16;
        run(self, CommandCode::GetRandom, &len)?.parse_ref(buf)
    }

    fn stir_random(&mut self, data: &[u8]) -> Result<()> {
        run(self, CommandCode::StirRandom, data)?.parse()
    }

    fn read_clock(&mut self) -> Result<TimeInfo> {
        run(self, CommandCode::ReadClock, &())?.parse()
    }
}

fn run<'a, T: Tpm + ?Sized>(
    tpm: &'a mut T,
    code: CommandCode,
    input: &(impl CommandData + ?Sized),
) -> Result<Response<'a, T>> {
    let tag = tag::Command::NoSessions;
    let size = (10 + input.encoded_len()).try_into().unwrap();
    let cmd_header = CommandHeader { tag, size, code };

    tpm.reset_command()?;
    cmd_header.encode(tpm)?;
    input.encode(tpm)?;

    tpm.run_command()?;

    let resp_header = ResponseHeader::decode(tpm)?;
    if let Some(err) = NonZeroU32::new(resp_header.code) {
        return Err(Error::Tpm(err));
    }
    assert_eq!(resp_header.tag, tag);

    Ok(Response {
        tpm,
        bytes_read: 10,
        bytes_expected: resp_header.size as usize,
    })
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
