use super::{Driver, DynDriver};
use crate::Result;
use cfg_if::cfg_if;
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};

cfg_if! {
    if #[cfg(target_os = "linux")] {
        #[path = "linux.rs"] mod imp;
    } else if #[cfg(windows)] {
        #[path = "windows.rs"] mod imp;
    } else {
        #[path = "other.rs"] mod imp;
    }
}
use imp::DriverImp;

// We use a private wrapper type to avoid leeking the impl below.
struct Priv<'a>(MutexGuard<'a, DriverImp>);
impl Driver for Priv<'_> {
    #[inline]
    fn run_command(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize> {
        Ok(self.0.run_driver(cmd_resp, cmd_len)?)
    }
}

// TODO: Explain singleton
static DRIVER: OnceCell<Mutex<DriverImp>> = OnceCell::new();
impl DynDriver {
    #[inline]
    pub fn get() -> Result<Self> {
        let guard = DRIVER
            .get_or_try_init(|| -> Result<_> { Ok(Mutex::new(DriverImp::new()?)) })?
            .lock()
            .unwrap();
        Ok(Self::from_driver(Priv(guard)))
    }
}
