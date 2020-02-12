use core::ops::DerefMut;

use crate::Result;

pub trait Driver {
    /// Uses the same buffer for the command and the response.
    fn run_command(&mut self, cmd_resp: &mut [u8; BUFFER_SIZE], cmd_len: usize) -> Result<usize>;
}

/// Maximum size of a TPM command or response buffer.
pub const BUFFER_SIZE: usize = 4096;

impl<D: Driver + ?Sized, T: DerefMut<Target = D>> Driver for T {
    fn run_command(&mut self, cmd_resp: &mut [u8; BUFFER_SIZE], cmd_len: usize) -> Result<usize> {
        (**self).run_command(cmd_resp, cmd_len)
    }
}
