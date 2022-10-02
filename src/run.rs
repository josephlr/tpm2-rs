//! Main TPM2 device traits

use crate::{
    commands::{run_command, Auths, Command},
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
