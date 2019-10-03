use core::ops::DerefMut;
use std::sync::Mutex;

use crate::buf::BufTpm;
use crate::Result;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        #[path = "linux.rs"] mod imp;
    } else if #[cfg(windows)] {
        #[path = "windows.rs"] mod imp;
    } else {
        compile_error!("Only Linux and Windows have TPM support.");
    }
}

/// OsTpm needs to be documented
pub type OsTpm = BufTpm<imp::OsExec>;

impl OsTpm {
    pub fn get() -> Result<impl DerefMut<Target = Self>> {
        use once_cell::sync::OnceCell;
        static TPM: OnceCell<Mutex<OsTpm>> = OnceCell::new();

        let tpm = TPM.get_or_try_init(|| -> Result<_> {
            let exec = imp::OsExec::new()?;
            Ok(Mutex::new(Self::new(exec)))
        })?;
        Ok(tpm.lock().unwrap())
    }
}
