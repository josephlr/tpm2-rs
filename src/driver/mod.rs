//! Explain driver architecture

use crate::Result;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Maximum size of a TPM command or response buffer.
pub const MAX_CMD_SIZE: usize = 4096;

/// Interface to a TPM implementation
pub trait Driver {
    /// Uses the same buffer for the command and the response.
    fn run_command(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize>;
}

impl<D: Driver + ?Sized> Driver for &'_ mut D {
    fn run_command(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize> {
        (**self).run_command(cmd_resp, cmd_len)
    }
}

/// Docs about the DynDriver
#[cfg(feature = "alloc")]
pub struct DynDriver(Box<dyn Driver>);
#[cfg(not(feature = "alloc"))]
pub struct DynDriver(&'static mut dyn Driver);

impl Driver for DynDriver {
    #[inline]
    fn run_command(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize> {
        self.0.run_command(cmd_resp, cmd_len)
    }
}

impl DynDriver {
    #[cfg(feature = "alloc")]
    pub fn from_driver(driver: impl Driver + 'static) -> Self {
        Self(Box::new(driver))
    }
    
    pub fn from_driver_ref(driver: &'static mut dyn Driver) -> Self {
        #[cfg(feature = "alloc")]
        let inner = Box::new(driver);
        #[cfg(not(feature = "alloc"))]
        let inner = driver;
        Self(inner)
    }
}

#[cfg(feature = "std")]
mod os;
