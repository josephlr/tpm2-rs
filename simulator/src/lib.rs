#![no_std]
use core::ops::DerefMut;

#[cfg(feature = "std")]
extern crate std;

use tpm::{
    raw::{Driver, StartupType, Tpm, BUFFER_SIZE},
    Result,
};

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

impl Driver for Simulator {
    fn run_command(&mut self, cmd_resp: &mut [u8; BUFFER_SIZE], cmd_len: usize) -> Result<usize> {
        let mut resp_ptr = cmd_resp.as_mut_ptr();
        let mut resp_size = cmd_resp.len() as u32;

        unsafe { _plat__RunCommand(cmd_len as u32, resp_ptr, &mut resp_size, &mut resp_ptr) };
        let resp_size = resp_size as usize;

        if resp_ptr != cmd_resp.as_mut_ptr() {
            use core::slice::from_raw_parts;
            let resp_static = unsafe { from_raw_parts(resp_ptr, resp_size) };
            cmd_resp[..resp_size].copy_from_slice(resp_static);
        }
        Ok(resp_size)
    }
}

pub struct Simulator {
    is_on: bool,
}

impl Simulator {
    const unsafe fn new() -> Self {
        Self { is_on: false }
    }

    #[cfg(feature = "std")]
    pub fn get() -> Result<impl DerefMut<Target = Self>> {
        use std::sync::Mutex;

        use once_cell::sync::Lazy;
        static SIM: Lazy<Mutex<Simulator>> = Lazy::new(|| Mutex::new(unsafe { Simulator::new() }));

        let mut sim = SIM.lock().unwrap();
        sim.manufacture_reset()?;
        Ok(sim)
    }

    #[cfg(feature = "std")]
    pub fn get_tpm() -> Result<Tpm> {
        use std::boxed::Box;
        Ok(Tpm::new(Box::new(Self::get()?)))
    }

    pub fn manufacture_reset(&mut self) -> Result<()> {
        self.off();
        unsafe { _plat__Reset(true) };
        self.on()
    }
    pub fn reset(&mut self) -> Result<()> {
        self.off();
        unsafe { _plat__Reset(false) };
        self.on()
    }

    fn on(&mut self) -> Result<()> {
        let mut tpm = Tpm::<&mut dyn Driver>::new(self);
        tpm.startup(StartupType::Clear)?;
        self.is_on = true;
        Ok(())
    }
    fn off(&mut self) {
        if self.is_on {
            let mut tpm = Tpm::<&mut dyn Driver>::new(self);
            let _ = tpm.shutdown(StartupType::Clear);
            self.is_on = false;
        }
    }
}
