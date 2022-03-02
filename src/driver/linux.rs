use std::{
    boxed::Box,
    error, fmt,
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    result::Result,
};

pub struct DriverImp(File, &'static str);

#[derive(Debug)]
pub struct DeviceError {
    op: &'static str,
    path: &'static str,
    err: io::Error,
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}: {}", self.op, self.path, self.err)
    }
}

impl error::Error for DeviceError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.err)
    }
}

impl From<DeviceError> for crate::Error {
    fn from(e: DeviceError) -> Self {
        crate::Error::Os(Box::new(e))
    }
}

fn open_file(path: &'static str) -> Result<DriverImp, DeviceError> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .map(|file| DriverImp(file, path))
        .map_err(|err| DeviceError {
            op: "opening",
            path,
            err,
        })
}

impl DriverImp {
    pub fn new() -> Result<Self, DeviceError> {
        // As users should be using the resource manager, only fall back to the
        // legacy device if the resource manager does not exist.
        open_file("/dev/tpmrm0").or_else(|e| {
            if e.err.kind() == io::ErrorKind::NotFound {
                open_file("/dev/tpm0")
            } else {
                Err(e)
            }
        })
    }

    pub fn run_driver(
        &mut self,
        cmd_resp: &mut [u8],
        cmd_len: usize,
    ) -> Result<usize, DeviceError> {
        let cmd = &cmd_resp[..cmd_len];
        self.0.write_all(cmd).map_err(|err| DeviceError {
            op: "writing command to",
            path: self.1,
            err,
        })?;
        let resp_len = self.0.read(cmd_resp).map_err(|err| DeviceError {
            op: "reading response from",
            path: self.1,
            err,
        })?;
        Ok(resp_len)
    }
}
