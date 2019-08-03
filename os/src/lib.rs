use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use tpm_core::{Result, Tpm};

pub struct OsTpm {
    file: File,
}

impl OsTpm {
    pub fn new() -> Result<Self> {
        Self::at_path("/dev/tpmrm0").or(Self::at_path("/dev/tpm0"))
    }
    pub fn at_path(path: impl AsRef<Path>) -> Result<Self> {
        let file = OpenOptions::new().read(true).write(true).open(path)?;
        Ok(Self { file })
    }
}

impl Tpm for OsTpm {
    fn exec(&mut self, command: &[u8], response: &mut [u8]) -> Result<usize> {
        self.file.write_all(command)?;
        Ok(self.file.read(response)?)
    }
}
