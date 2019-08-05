use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use tpm_core::{Error, Result, Tpm};

pub struct OsTpm {
    file: File,
}

static IN_USE: AtomicBool = AtomicBool::new(false);

fn open_rw(path: impl AsRef<Path>) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path)
}

impl OsTpm {
    pub fn get() -> Result<Self> {
        let file = open_rw("/dev/tpmrm0").or(open_rw("/dev/tpm0"))?;
        if IN_USE.swap(true, Relaxed) {
            return Err(Error::TpmInUse);
        }
        Ok(Self { file })
    }
}

impl Drop for OsTpm {
    fn drop(&mut self) {
        IN_USE.store(false, Relaxed);
    }
}

impl Tpm for OsTpm {
    fn exec(&mut self, command: &[u8], response: &mut [u8]) -> Result<usize> {
        self.file.write_all(command)?;
        Ok(self.file.read(response)?)
    }
}
