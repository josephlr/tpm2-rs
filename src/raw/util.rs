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

pub trait Driver {
    /// Uses the same buffer for the command and the response.
    fn run_command(&mut self, cmd_resp: &mut [u8; BUFFER_SIZE], cmd_len: usize) -> Result<usize>;
}

/// Maximum size of a TPM command or response buffer.
pub const BUFFER_SIZE: usize = 4096;

pub(crate) struct Buf {
    cmd_resp: [u8; BUFFER_SIZE],
    cmd_end: usize,
    resp_start: usize,
    resp_end: usize,
}

impl Buf {
    pub const fn new() -> Self {
        Self {
            cmd_resp: [0u8; BUFFER_SIZE],
            cmd_end: 0,
            resp_start: 0,
            resp_end: 0,
        }
    }
    pub fn reset(&mut self) {
        self.cmd_end = 0;
        self.resp_start = 0;
        self.resp_end = 0;
    }
    pub fn run_command(&mut self, driver: &mut impl Driver) -> Result<()> {
        self.resp_end = driver.run_command(&mut self.cmd_resp, self.cmd_end)?;
        self.cmd_end = 0;
        self.resp_start = 0;
        Ok(())
    }
}

impl Write for Buf {
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        let next_end = self.cmd_end + buf.len();
        if next_end > self.cmd_resp.len() {
            return Err(Error::TooMuchInputData);
        }
        self.cmd_resp[self.cmd_end..next_end].copy_from_slice(buf);
        self.cmd_end = next_end;
        Ok(())
    }
}

impl Read for Buf {
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        let next_start = self.resp_start + buf.len();
        if next_start > self.resp_end {
            return Err(Error::MissingOutputData);
        }
        buf.copy_from_slice(&self.cmd_resp[self.resp_start..next_start]);
        self.resp_start = next_start;
        Ok(())
    }
}

impl<D: Driver + ?Sized, T: DerefMut<Target = D>> Driver for T {
    fn run_command(&mut self, cmd_resp: &mut [u8; BUFFER_SIZE], cmd_len: usize) -> Result<usize> {
        (**self).run_command(cmd_resp, cmd_len)
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
    }
}
