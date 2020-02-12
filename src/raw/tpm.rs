use core::ops::DerefMut;

use crate::Result;

pub trait Driver {
    /// Uses the same buffer for the command and the response.
    fn run_command(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize>;
}

/// Maximum size of a TPM command or response buffer.
const BUFFER_SIZE: usize = 4096;

impl<D: Driver + ?Sized, T: DerefMut<Target = D>> Driver for T {
    fn run_command(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize> {
        self.deref_mut().run_command(cmd_resp, cmd_len)
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "alloc")] {
        use alloc::boxed::Box;
        pub struct Tpm<D = Box<dyn Driver>> {
            pub(crate) buf: Box<[u8]>,
            pub(crate) driver: D,
        }
    } else {
        pub struct Tpm<D> {
            pub(crate) buf: [u8; BUFFER_SIZE],
            pub(crate) driver: D,
        }
    }
}

impl<D> Tpm<D> {
    pub fn new(driver: D) -> Self {
        Self {
            #[cfg(feature = "alloc")]
            buf: alloc::vec![0; BUFFER_SIZE].into_boxed_slice(),
            #[cfg(not(feature = "alloc"))]
            buf: [0; BUFFER_SIZE],
            driver,
        }
    }
}

#[cfg(feature = "std")]
impl Tpm {
    pub fn get() -> Result<Self> {
        Ok(Self::new(Box::new(crate::os::get_driver()?)))
    }
}
