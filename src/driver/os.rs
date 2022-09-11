#![cfg(feature = "std")]

use crate::{Box, Driver, Result};
#[cfg(target_os = "linux")]
use std::fs::{File, OpenOptions};
use std::{
    io::{self, Read, Write},
    sync::{Mutex, MutexGuard},
};

// TODO: explain why you would want this
pub fn from_read_write(rw: impl Read + Write + 'static) -> Box<dyn Driver> {
    Box::new(RwDriver(rw))
}

#[repr(transparent)]
struct RwDriver<T>(T);

impl<T: Read + Write> Driver for RwDriver<T> {
    fn run_command(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize> {
        self.0.write_all(&cmd_resp[..cmd_len])?;
        let resp_len = self.0.read(cmd_resp)?;
        Ok(resp_len)
    }
}

// Linux Implementation

#[cfg(target_os = "linux")]
fn open(path: &str) -> io::Result<RwDriver<File>> {
    let file = OpenOptions::new().read(true).write(true).open(path)?;
    Ok(RwDriver(file))
}

#[cfg(target_os = "linux")]
fn os_default() -> io::Result<RwDriver<File>> {
    open("/dev/tpmrm0").or_else(|e| open("/dev/tpm0").map_err(|_| e))
}

// Windows Implementation

/// TODO: explain that this blocks if called multiple times.
#[cfg(any(target_os = "linux", windows))]
pub fn get_os_default() -> Result<Box<dyn Driver>> {
    static LOCK: Mutex<()> = Mutex::new(());

    let _guard = LOCK.lock()?;
    let driver = os_default()?;
    Ok(Box::new(LockedDriver { _guard, driver }))
}

struct LockedDriver<D: ?Sized> {
    _guard: MutexGuard<'static, ()>,
    driver: D,
}

impl<D: Driver + ?Sized> Driver for LockedDriver<D> {
    fn run_command(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize> {
        self.driver.run_command(cmd_resp, cmd_len)
    }
}
