use core::ops::DerefMut;

use crate::{Error, Result};

pub trait Write {
    /// Write command data for future execution. May be called multiple times.
    /// Attempts to write all of `buf` into the writer. This differs from
    /// [`std::io::Write::write`] (which returns the number of bytes written),
    /// and is more similar to [`std::io::Write::write_all`].
    fn write(&mut self, buf: &[u8]) -> Result<()>;
}

pub trait Read {
    /// Read response data from a completed command. May be called multiple times.
    /// Fills all of `buf` or fails. This differs from [`std::io::Read::read`]
    /// (which returns the number of bytes read), and is more similar to
    /// [`std::io::Read::read_exact`].
    fn read(&mut self, buf: &mut [u8]) -> Result<()>;
}

pub trait Driver: Read + Write {
    /// Execute a command consisting of perviously written data.
    /// Executes previously written command data.
    fn run_command(&mut self) -> Result<()>;
    /// Clear any currently pending writes or running commands.
    /// Resets any data written by [`write`].
    fn reset_command(&mut self) -> Result<()>;
}

pub trait Exec {
    /// Uses the same buffer for the command and the response.
    fn exec(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize>;
}

// Maximum size of a command or response buffer.
const BUFFER_SIZE: usize = 4096;

pub struct BufDriver<E> {
    buf: [u8; BUFFER_SIZE],
    cmd_end: usize,
    resp_start: usize,
    resp_end: usize,
    exec: E,
}

impl<E> BufDriver<E> {
    pub const fn new(exec: E) -> Self {
        Self {
            buf: [0u8; BUFFER_SIZE],
            cmd_end: 0,
            resp_start: 0,
            resp_end: 0,
            exec,
        }
    }
}

impl<E> Write for BufDriver<E> {
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        let next_end = self.cmd_end + buf.len();
        if next_end > self.buf.len() {
            return Err(Error::TooMuchInputData);
        }
        self.buf[self.cmd_end..next_end].copy_from_slice(buf);
        self.cmd_end = next_end;
        Ok(())
    }
}

impl<E> Read for BufDriver<E> {
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        let next_start = self.resp_start + buf.len();
        if next_start > self.resp_end {
            return Err(Error::MissingOutputData);
        }
        buf.copy_from_slice(&self.buf[self.resp_start..next_start]);
        self.resp_start = next_start;
        Ok(())
    }
}

impl<E: Exec> Driver for BufDriver<E> {
    fn run_command(&mut self) -> Result<()> {
        self.resp_end = self.exec.exec(&mut self.buf, self.cmd_end)?;
        self.cmd_end = 0;
        self.resp_start = 0;
        Ok(())
    }
    fn reset_command(&mut self) -> Result<()> {
        self.cmd_end = 0;
        self.resp_start = 0;
        self.resp_end = 0;
        Ok(())
    }
}

impl<D: Write + ?Sized, T: DerefMut<Target = D>> Write for T {
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        (**self).write(buf)
    }
}

impl<D: Read + ?Sized, T: DerefMut<Target = D>> Read for T {
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        (**self).read(buf)
    }
}

impl<D: Driver + ?Sized, T: DerefMut<Target = D>> Driver for T {
    fn run_command(&mut self) -> Result<()> {
        (**self).run_command()
    }
    fn reset_command(&mut self) -> Result<()> {
        (**self).reset_command()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_safety() {
        let _: Option<&dyn Write> = None;
        let _: Option<&dyn Read> = None;
        let _: Option<&dyn Driver> = None;
        let _: Option<&dyn Exec> = None;
    }
}
