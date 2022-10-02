//! Main TPM2 device traits

use crate::{
    commands::{run_command, Auths, Command, GetRandom},
    error::DriverError,
    Error,
};

/// A TPM2 Device
pub trait Tpm {
    fn command_buf(&mut self) -> &mut [u8];
    fn response_buf(&self) -> &[u8];
    fn execute_command(&mut self, cmd_size: u32) -> Result<(), DriverError>;
}

/// Trait extending [`Tpm`] for running raw commands.
pub trait TpmRun: Tpm {
    fn run<C: Command + Auths<N>, const N: usize>(
        &mut self,
        cmd: C,
    ) -> Result<C::Response<'_>, Error>;
}

impl<T: Tpm> TpmRun for T {
    #[inline]
    fn run<C: Command + Auths<N>, const N: usize>(
        &mut self,
        cmd: C,
    ) -> Result<C::Response<'_>, Error> {
        <dyn Tpm>::run(self, cmd)
    }
}
impl TpmRun for dyn Tpm + '_ {
    #[inline]
    fn run<C: Command + Auths<N>, const N: usize>(
        &mut self,
        cmd: C,
    ) -> Result<C::Response<'_>, Error> {
        let mut rsp = C::Response::default();
        run_command(self, &cmd.auths(), cmd.data(), &mut rsp, C::CODE)?;
        Ok(rsp)
    }
}

/// Trait extending [`Tpm`] for running higher-level TPM workflows.
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
