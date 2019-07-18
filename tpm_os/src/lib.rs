use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use tpm_core::{Exec, Result, Runner, Tpm};

struct OsExec {
    file: File,
}

impl Exec for OsExec {
    fn execute(&mut self, input: &[u8], output: &mut [u8]) -> Result<usize> {
        self.file.write(input)?;
        Ok(self.file.read(output)?)
    }
}

pub struct OsTpm(Runner<OsExec, Vec<u8>>);

impl OsTpm {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = OpenOptions::new().read(true).write(true).open(path)?;
        Ok(Self(Runner::new(OsExec { file })))
    }
}

impl Tpm for OsTpm {
    #[inline]
    fn buffer(&mut self) -> &mut [u8] {
        self.0.buffer()
    }
    #[inline]
    fn run(&mut self, len: usize) -> Result<&[u8]> {
        self.0.run(len)
    }
}
