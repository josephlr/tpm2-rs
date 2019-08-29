#![no_std]
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use tpm_core::constants::StartupType;
use tpm_core::{Error, Result, Tpm};

extern "C" {
    // Simulator Commands
    fn _TPM_Init();
    fn ExecuteCommand(
        requestSize: u32,
        request: *const u8,
        responseSize: *mut u32,
        response: *mut *mut u8,
    );

    // TPM Manufacturing Commands
    fn TPM_Manufacture(firstTime: libc::c_int) -> libc::c_int;
    fn TPM_TearDown() -> libc::c_int;
}

#[no_mangle]
extern "C" fn _plat__IsCanceled() -> libc::c_int {
    0
}
#[no_mangle]
extern "C" fn _plat__LocalityGet() -> u8 {
    0
}

static IN_USE: AtomicBool = AtomicBool::new(false);

pub struct Simulator(bool);

impl Simulator {
    pub fn get() -> Result<Self> {
        if IN_USE.swap(true, Relaxed) {
            return Err(Error::TpmInUse);
        }
        let mut s = Self(false);
        unsafe { s.on()? };
        Ok(s)
    }
}

impl Simulator {
    pub fn reset(&mut self) -> Result<()> {
        unsafe {
            self.off()?;
            self.on()
        }
    }
    pub fn manufacture_reset(&mut self) -> Result<()> {
        unsafe {
            self.off()?;
            TPM_TearDown();
            self.on()
        }
    }
    unsafe fn on(&mut self) -> Result<()> {
        TPM_Manufacture(0);
        _TPM_Init();
        self.startup(StartupType::Clear)?;
        self.0 = true;
        Ok(())
    }
    unsafe fn off(&mut self) -> Result<()> {
        self.shutdown(StartupType::Clear)?;
        self.0 = false;
        Ok(())
    }
}

impl Drop for Simulator {
    fn drop(&mut self) {
        if self.0 {
            let _ = unsafe { self.off() };
        }
        IN_USE.store(false, Relaxed);
    }
}

impl Tpm for Simulator {
    fn exec(&mut self, command: &[u8], response: &mut [u8]) -> Result<usize> {
        if !self.0 {
            return Err(Error::SimulatorOff);
        }

        let mut response_size = response.len() as u32;
        let mut response_ptr = response.as_mut_ptr();
        unsafe {
            ExecuteCommand(
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
