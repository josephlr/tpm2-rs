//! TODO: Docs about erros

use core::{
    fmt::Display,
    num::{NonZeroU32, TryFromIntError},
};
#[cfg(feature = "std")]
use std::boxed::Box;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Command(CommandError),
    Encode(EncodeError), // mu layer ?
    Decode(DecodeError), // mu layer ?
    #[cfg(feature = "std")]
    Os(Box<dyn std::error::Error>),
}

#[derive(Debug)]
pub struct CommandError(NonZeroU32);

impl From<NonZeroU32> for CommandError {
    fn from(c: NonZeroU32) -> Self {
        CommandError(c)
    }
}


#[derive(Debug)]
#[non_exhaustive]
pub enum EncodeError {
    OutputTooShort,
    IntegerOverflow,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DecodeError {
    InputTooLong,
    InputTooShort,
    InvalidInput,
}

impl From<CommandError> for Error {
    fn from(err: CommandError) -> Self {
        Error::Command(err)
    }
}

impl From<EncodeError> for Error {
    fn from(err: EncodeError) -> Self {
        Error::Encode(err)
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Self {
        Error::Decode(err)
    }
}

impl From<TryFromIntError> for EncodeError {
    fn from(_: TryFromIntError) -> Self {
        EncodeError::IntegerOverflow
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Self::Encode(EncodeError::from(e))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TODO: impl Display")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
