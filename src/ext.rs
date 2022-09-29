use crate::{commands::GetRandom, Error, Run, Tpm};

// Helper trait for higher-level TPM functionality.
pub trait TpmExt: Tpm {
    fn getrandom(&mut self, buf: &mut [u8]) -> Result<(), Error>;
}

impl<T: Run + ?Sized> TpmExt for T {
    fn getrandom(&mut self, mut buf: &mut [u8]) -> Result<(), Error> {
        while !buf.is_empty() {
            let bytes_requested = buf.len().try_into().unwrap_or(u16::MAX);
            let rsp = self.run(&GetRandom { bytes_requested })?;

            let bytes_recieved = rsp.random_bytes.len();
            buf[..bytes_recieved].copy_from_slice(rsp.random_bytes);
            buf = &mut buf[bytes_recieved..];
        }
        Ok(())
    }
}
