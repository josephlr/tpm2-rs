//! Error types for various TPM-related operations

use core::num::{NonZeroU32, TryFromIntError};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Tpm(TpmError),
    Marshal(MarshalError),
    Unmarshal(UnmarshalError),
    Driver(DriverError),
    TooManyAuths(usize),
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct TpmError(pub NonZeroU32);

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum MarshalError {
    BufferOverflow,
    BufferRemaining,
    IntegerOverflow,
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum UnmarshalError {
    BufferOverflow,
    BufferRemaining,
    InvalidValue,
    PcrTooLarge(usize),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DriverError {
    IntegerOverflow,
    #[cfg(feature = "std")]
    #[doc(cfg(feature = "std"))]
    Io(std::io::Error),
}

impl From<TpmError> for Error {
    fn from(e: TpmError) -> Self {
        Self::Tpm(e)
    }
}
impl From<MarshalError> for Error {
    fn from(e: MarshalError) -> Self {
        Self::Marshal(e)
    }
}
impl From<UnmarshalError> for Error {
    fn from(e: UnmarshalError) -> Self {
        Self::Unmarshal(e)
    }
}
impl From<DriverError> for Error {
    fn from(e: DriverError) -> Self {
        Self::Driver(e)
    }
}

impl From<TryFromIntError> for MarshalError {
    fn from(_: TryFromIntError) -> Self {
        Self::IntegerOverflow
    }
}
impl From<TryFromIntError> for DriverError {
    fn from(_: TryFromIntError) -> Self {
        Self::IntegerOverflow
    }
}

#[cfg(feature = "std")]
#[doc(cfg(feature = "std"))]
mod std_impl {
    use super::*;

    // impl std::error::Error for Error {}
    // impl std::error::Error for TpmError {}
    // impl std::error::Error for MarshalError {}
    // impl std::error::Error for UnmarshalError {}
    // impl std::error::Error for AuthError {}
    // impl std::error::Error for DriverError {}

    impl From<std::io::Error> for DriverError {
        fn from(err: std::io::Error) -> Self {
            Self::Io(err)
        }
    }
}
