use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use crate::{
    raw::{Driver, BUFFER_SIZE},
    Result,
};

pub struct OsDriver(File);

impl OsDriver {
    pub fn new() -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tpm0")?;
        Ok(Self(file))
    }
}

impl Driver for OsDriver {
    fn run_command(&mut self, cmd_resp: &mut [u8; BUFFER_SIZE], cmd_len: usize) -> Result<usize> {
        self.0.write_all(&mut cmd_resp[..cmd_len])?;
        let resp_len = self.0.read(cmd_resp)?;
        Ok(resp_len)
    }
}
