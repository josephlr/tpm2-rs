#![no_std]
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use tpm::raw::constants::StartupType;
use tpm::{Error, Result, raw::Tpm};

// External Simulator API from Samples/Google/Platform.h
extern "C" {
    fn _plat__Reset(forceManufacture: bool);
    fn _plat__RunCommand(
        requestSize: u32,
        request: *const u8,
        responseSize: *mut u32,
        response: *mut *mut u8,
    );
}

static IN_USE: AtomicBool = AtomicBool::new(false);

pub struct Simulator(bool);

impl Simulator {
    pub fn get() -> Result<Self> {
        if IN_USE.swap(true, Relaxed) {
            return Err(Error::TpmInUse);
        }
        let mut s = Self(false);
        unsafe { _plat__Reset(true) };
        s.on()?;
        Ok(s)
    }
    pub fn reset(&mut self) -> Result<()> {
        self.off()?;
        unsafe { _plat__Reset(false) };
        self.on()
    }
    pub fn manufacture_reset(&mut self) -> Result<()> {
        self.off()?;
        unsafe { _plat__Reset(true) };
        self.on()
    }
    fn on(&mut self) -> Result<()> {
        self.startup(StartupType::Clear)?;
        self.0 = true;
        Ok(())
    }
    fn off(&mut self) -> Result<()> {
        self.shutdown(StartupType::Clear)?;
        self.0 = false;
        Ok(())
    }
}

impl Drop for Simulator {
    fn drop(&mut self) {
        if self.0 {
            let _ = self.off();
        }
        IN_USE.store(false, Relaxed);
    }
}

impl Tpm for Simulator {
    fn exec(&mut self, command: &[u8], response: &mut [u8]) -> Result<usize> {
        let mut response_size = response.len() as u32;
        let mut response_ptr = response.as_mut_ptr();
        unsafe {
            _plat__RunCommand(
                command.len() as u32,
                command.as_ptr(),
                &mut response_size,
                &mut response_ptr,
            )
        };
        if response_ptr != response.as_mut_ptr() {
            use core::slice::from_raw_parts;
            let response_out = unsafe { from_raw_parts(response_ptr, response_size as usize) };
            if response_out.len() > response.len() {
                return Err(Error::ResponseBuffer);
            }
            response[..response_out.len()].copy_from_slice(response_out);
        }
        Ok(response_size as usize)
    }
}
