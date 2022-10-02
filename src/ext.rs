use crate::{commands::GetRandom, Error, TpmRun};

/// Trait extending [`Tpm`](crate::Tpm) for running higher-level TPM workflows.
///
/// These methods almost always issues multiple TPM commands under the hood.
pub trait TpmExt: TpmRun {
    fn getrandom(&mut self, mut buf: &mut [u8]) -> Result<(), Error> {
        while !buf.is_empty() {
            let bytes_requested = buf.len().try_into().unwrap_or(u16::MAX);
            let rsp = self.run(GetRandom { bytes_requested })?;

            let bytes_received = rsp.random_bytes.len();
            buf[..bytes_received].copy_from_slice(rsp.random_bytes);
            buf = &mut buf[bytes_received..];
        }
        Ok(())
    }
}

impl<T: TpmRun + ?Sized> TpmExt for T {}
