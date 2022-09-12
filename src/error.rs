use core::fmt;
use core::num::{NonZeroU32, TryFromIntError};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Tpm(NonZeroU32),
    MarshalBufferOverflow,
    MarshalBufferRemaining,
    UnmarshalBufferOverflow,
    UnmarshalBufferRemaining,
    UnmarshalInvalidValue,
    IntegerOverflow,
    IndexOutOfBounds,
    PoisonError,
    #[cfg(feature = "std")]
    #[doc(cfg(feature = "std"))]
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Tpm(code) => code.fmt(f),
            #[cfg(feature = "std")]
            Error::Io(err) => err.fmt(f),
            _ => fmt::Debug::fmt(self, f),
        }
    }
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Error::IntegerOverflow
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

#[cfg(feature = "std")]
impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Self::PoisonError
    }
}

pub type Result<T> = core::result::Result<T, Error>;
