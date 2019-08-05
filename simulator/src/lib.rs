#![no_std]
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use tpm_core::constants::StartupType;
use tpm_core::{Error, Result, Tpm};

extern "C" {
    // Platform Commands
    fn _plat__RunCommand(
        requestSize: u32,
        request: *const u8,
        responseSize: *mut u32,
        response: *mut *mut u8,
    );
    fn _plat__Signal_Reset() -> libc::c_int;
    fn _plat__Signal_PowerOn() -> libc::c_int;
    fn _plat__Signal_PowerOff();
    fn _plat__SetNvAvail();
    fn _plat__ClearNvAvail();
    fn _plat__Signal_PhysicalPresenceOn();
    fn _plat__Signal_PhysicalPresenceOff();

    // TPM Manufacturing Commands
    fn TPM_Manufacture(firstTime: libc::c_int) -> libc::c_int;
    fn TPM_TearDown() -> libc::c_int;
}

static IN_USE: AtomicBool = AtomicBool::new(false);
static WAS_RESET: AtomicBool = AtomicBool::new(false);

pub struct Simulator;

impl Simulator {
    pub fn get() -> Result<Self> {
        if IN_USE.swap(true, Relaxed) {
            return Err(Error::TpmInUse);
        }
        let mut s = Self;
        unsafe { s.on(true) }?;
        Ok(s)
    }
}

impl Simulator {
    pub fn reset(&mut self) -> Result<()> {
        unsafe {
            self.off()?;
            self.on(false)
        }
    }
    pub fn manufacture_reset(&mut self) -> Result<()> {
        unsafe {
            self.off()?;
            self.on(true)
        }
    }
    unsafe fn on(&mut self, manufacture_reset: bool) -> Result<()> {
        // TODO: Should we be ignoring return codes here?
        _plat__Signal_PowerOn();
        _plat__Signal_Reset();
        _plat__SetNvAvail();
        _plat__Signal_PhysicalPresenceOn();

        if manufacture_reset {
            if WAS_RESET.swap(true, Relaxed) {
                TPM_TearDown();
                TPM_Manufacture(0);
            } else {
                TPM_Manufacture(1);
            }
        }

        self.startup(StartupType::Clear)
    }
    unsafe fn off(&mut self) -> Result<()> {
        self.shutdown(StartupType::Clear)?;
        _plat__Signal_PhysicalPresenceOff();
        _plat__ClearNvAvail();
        _plat__Signal_PowerOff();
        Ok(())
    }
}

impl Drop for Simulator {
    fn drop(&mut self) {
        IN_USE.store(false, Relaxed);
        unsafe { self.off() }.expect("failed to shutdown simulator");
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
