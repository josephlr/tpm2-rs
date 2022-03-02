use crate::{Error, Result};

pub struct DriverImp;

impl DriverImp {
    pub fn new() -> Result<Self> {
        Err(Error::Os("TPM is not supported on this platform".into()))
    }

    pub fn run_driver(&mut self, _: &mut [u8], _: usize) -> Result<usize> {
        unreachable!()
    }
}
