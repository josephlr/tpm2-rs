use super::*;
use std::{
    fs::{File, OpenOptions},
    sync::{Mutex, OnceLock},
};

fn open(path: &str) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path)
}
fn init_tpm_file() -> io::Result<Mutex<File>> {
    let f = match open("/dev/tpmrm0") {
        Ok(f) => Ok(f),
        Err(e) if e.kind() == ErrorKind::NotFound => open("/dev/tpm0"),
        Err(e) => Err(e),
    }?;
    Ok(Mutex::new(f))
}

static FILE: OnceLock<Mutex<File>> = OnceLock::new();

pub fn default_impl() -> io::Result<impl Tpm> {
    let file = FILE.get_or_try_init(init_tpm_file)?;
    Ok(tpm_from_read_write(file.lock().unwrap()))
}
