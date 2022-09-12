use super::*;
use std::{
    fs::{File, OpenOptions},
    io::{self, ErrorKind},
};

fn open(path: &str) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path)
}

pub fn default_impl() -> io::Result<impl Tpm> {
    let file = match open("/dev/tpmrm0") {
        Ok(f) => Ok(f),
        Err(e) if e.kind() == ErrorKind::NotFound => open("/dev/tpm0"),
        Err(e) => Err(e),
    }?;
    Ok(tpm_from_read_write(file))
}
