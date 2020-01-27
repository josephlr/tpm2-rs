use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use crate::driver::Exec;
use crate::Result;

pub struct OsExec(File);

impl OsExec {
    pub fn new() -> Result<Self> {
        let open_rw = |path| OpenOptions::new().read(true).write(true).open(path);
        let file = open_rw("/dev/tpmrm0").or(open_rw("/dev/tpm0"))?;
        Ok(Self(file))
    }
}

impl Exec for OsExec {
    fn exec(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize> {
        self.0.write_all(&mut cmd_resp[..cmd_len])?;
        let resp_len = self.0.read(cmd_resp)?;
        Ok(resp_len)
    }
}
