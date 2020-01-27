use std::sync::Mutex;

use once_cell::sync::OnceCell;

use crate::{
    driver::{BufDriver, Driver},
    Result,
};

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        #[path = "linux.rs"] mod imp;
    } else if #[cfg(windows)] {
        #[path = "windows.rs"] mod imp;
    } else {
        compile_error!("Only Linux and Windows have TPM support.");
    }
}

pub fn get_driver() -> Result<impl Driver> {
    static DRIVER: OnceCell<Mutex<BufDriver<imp::OsExec>>> = OnceCell::new();

    let driver = DRIVER.get_or_try_init(|| -> Result<_> {
        let exec = imp::OsExec::new()?;
        Ok(Mutex::new(BufDriver::new(exec)))
    })?;
    Ok(driver.lock().unwrap())
}
