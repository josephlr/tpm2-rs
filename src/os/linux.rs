use super::*;
use std::fs::{File, OpenOptions};

fn open(path: &str) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path)
}
fn open_tpm() -> io::Result<File> {
    let f = match open("/dev/tpmrm0") {
        Ok(f) => Ok(f),
        Err(e) if e.kind() == ErrorKind::NotFound => open("/dev/tpm0"),
        Err(e) => Err(e),
    }?;
    Ok(f)
}

pub(crate) fn default_impl() -> io::Result<impl Tpm> {
    Ok(tpm_from_read_write(open_tpm()?))
}
