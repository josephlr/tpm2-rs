//! TODO: Describe why Drivers need to exist (and how it relates to TCTI)

use crate::Result;

/// TODO Document driver
pub trait Driver {
    /// Uses the same buffer for the command and the response.
    fn run_command(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize>;
}

mod os;
pub use os::*;
