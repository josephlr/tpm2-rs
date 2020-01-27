#![no_std]
use core::ops::DerefMut;

use tpm::buf::{BufTpm, Exec};
use tpm::raw::{StartupType, Tpm};
use tpm::Result;

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

struct SimulatorExec;

impl Exec for SimulatorExec {
    fn exec(&mut self, cmd_len: usize, cmd_resp: &mut [u8]) -> Result<usize> {
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
    tpm: BufTpm<SimulatorExec>,
    is_on: bool,
}

impl Simulator {
    const fn new() -> Self {
        Self {
            tpm: BufTpm::new(SimulatorExec),
            is_on: false,
        }
    }

    #[cfg(feature = "std")]
    pub fn get() -> Result<impl DerefMut<Target = Self>> {
        extern crate std;
        use std::sync::Mutex;

        use once_cell::sync::Lazy;
        static TPM: Lazy<Mutex<Simulator>> = Lazy::new(|| Mutex::new(Simulator::new()));

        let mut tpm = TPM.lock().unwrap();
        tpm.manufacture_reset()?;
        Ok(tpm)
    }
    #[cfg(not(feature = "std"))]
    pub unsafe fn get() -> Result<impl DerefMut<Target = Self>> {
        static mut TPM: Simulator = Simulator::new();

        TPM.manufacture_reset()?;
        Ok(&mut TPM)
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
        self.tpm.startup(StartupType::Clear)?;
        self.is_on = true;
        Ok(())
    }
    fn off(&mut self) {
        if self.is_on {
            let _ = self.tpm.shutdown(StartupType::Clear);
            self.is_on = false;
        }
    }
}

impl Tpm for Simulator {
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.tpm.write(buf)
    }
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        self.tpm.read(buf)
    }
    fn run_command(&mut self) -> Result<()> {
        self.tpm.run_command()
    }
    fn reset_command(&mut self) -> Result<()> {
        self.tpm.reset_command()
    }
}
