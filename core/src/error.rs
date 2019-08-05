use core::fmt;
use core::num::NonZeroU32;

#[derive(Debug)]
pub enum Error {
    BufTooShort,
    InvalidValue,
    TagMismatch,
    OutLenMismatch,
    RemainingData,
    WouldAllocate,
    ResponseBuffer,
    TpmInUse,
    Tpm(NonZeroU32),
    #[cfg(feature = "std")]
    Io(std::io::Error),
    #[doc(hidden)]
    __Nonexhaustive, // TODO: replace with #[non_exhaustive]
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

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
