//! Main TPM2 device traits

use core::fmt::Debug;

use crate::{
    commands::{run_command, Auths},
    error::{DriverError, MarshalError},
    marshal::{CommandData, ResponseData},
    types::{tpm, Auth},
    Error,
};

/// Common Trait for all TPM2 Commands
pub trait Command: CommandData + Copy + Debug {
    const CODE: tpm::CC;
    type Response<'t>: ResponseData<'t> + Default + Copy + Debug;

    /// This helper function isn't necessary for correctness, but exists to
    /// reduce the number of vtables we use. If we have a type of `&C`, `&&C`,
    /// [`WithAuth<'a, C>`] or `&WithAuth<'a, &C>`, we can instead use the
    /// vtable for `C`. See the difference in code generation:
    /// - [without the helper](https://godbolt.org/z/3Yv9TYT18)
    /// - [with the helper](https://godbolt.org/z/793r1ccjc)
    /// Note the difference in the number of vtables emitted.
    ///
    /// It should always be the case that `c.data().marshal_*()` and
    /// `c.marshal_*() do the exact same thing.
    fn data(&self) -> &dyn CommandData {
        self
    }

    fn with_auth(self, auth: &'_ dyn Auth) -> WithAuth<'_, Self> {
        WithAuth(self, auth)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WithAuth<'a, C>(C, &'a dyn Auth);

impl<C: CommandData> CommandData for WithAuth<'_, C> {
    #[inline]
    fn marshal_handles(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.0.marshal_handles(buf)
    }
    #[inline]
    fn marshal_params(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.0.marshal_params(buf)
    }
}
impl<C: Command> Command for WithAuth<'_, C> {
    const CODE: tpm::CC = C::CODE;
    type Response<'t> = C::Response<'t>;
    #[inline]
    fn data(&self) -> &dyn CommandData {
        self.0.data()
    }
}
impl<C: Auths<0>> Auths<1> for WithAuth<'_, C> {
    #[inline]
    fn auths(&self) -> [&dyn Auth; 1] {
        [self.1]
    }
}
impl<C: Auths<1>> Auths<2> for WithAuth<'_, C> {
    #[inline]
    fn auths(&self) -> [&dyn Auth; 2] {
        let [a1] = self.0.auths();
        [a1, self.1]
    }
}
impl<C: Auths<2>> Auths<3> for WithAuth<'_, C> {
    #[inline]
    fn auths(&self) -> [&dyn Auth; 3] {
        let [a1, a2] = self.0.auths();
        [a1, a2, self.1]
    }
}

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
