use crate::raw::Tpm;
use crate::{Error, Result};

// Maximum size of a command or response buffer.
const BUFFER_SIZE: usize = 4096;

pub trait Exec {
    fn exec(&mut self, cmd_len: usize, cmd_resp: &mut [u8]) -> Result<usize>;
}

pub struct BufTpm<E> {
    buf: [u8; BUFFER_SIZE],
    cmd_end: usize,
    resp_start: usize,
    resp_end: usize,
    exec: E,
}

impl<E> BufTpm<E> {
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

impl<E: Exec> Tpm for BufTpm<E> {
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        let next_end = self.cmd_end + buf.len();
        if next_end > self.buf.len() {
            return Err(Error::TooMuchInputData);
        }
        self.buf[self.cmd_end..next_end].copy_from_slice(buf);
        self.cmd_end = next_end;
        Ok(())
    }
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        let next_start = self.resp_start + buf.len();
        if next_start > self.resp_end {
            return Err(Error::MissingOutputData);
        }
        buf.copy_from_slice(&self.buf[self.resp_start..next_start]);
        self.resp_start = next_start;
        Ok(())
    }
    fn run_command(&mut self) -> Result<()> {
        self.resp_end = self.exec.exec(self.cmd_end, &mut self.buf)?;
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
