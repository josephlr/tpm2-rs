//! Helper traits for implementing TPMs and TPM-compatible data structures
use core::{convert::TryInto, mem};

use super::*;
use crate::{Error, Result};

pub trait TpmData {
    fn data_len(&self) -> usize;
}

/// A trait for objects that can be written to a TPM command buffer
pub trait CommandData: TpmData {
    /// Serialize this object and write it to `writer`.
    fn encode(&self, writer: &mut dyn Write) -> Result<()>;
}

/// A trait for creating objects from a TPM response buffer;
pub trait ResponseData: TpmData + Sized {
    fn decode(reader: &mut dyn Read) -> Result<Self>;
}

macro_rules! data_impls { ($($T: ty)+) => { $(
    impl TpmData for $T {
        fn data_len(&self) -> usize {
            mem::size_of::<Self>()
        }
    }
)+ } }

data_impls! { () bool u8 u16 u32 u64 }

// Rust integral types don't have a common trait, so we have to use a macro.
macro_rules! int_impls { ($($T: ty)+) => { $(
    impl CommandData for $T {
        fn encode(&self, writer: &mut dyn Write) -> Result<()> {
            writer.write(&self.to_be_bytes())
        }
    }
    impl ResponseData for $T {
        fn decode(reader: &mut dyn Read) -> Result<Self> {
            let mut arr = [0u8; mem::size_of::<Self>()];
            reader.read(&mut arr)?;
            Ok(Self::from_be_bytes(arr))
        }
    }
)+ } }

int_impls! { u8 u16 u32 u64 }

impl CommandData for () {
    fn encode(&self, _: &mut dyn Write) -> Result<()> {
        Ok(())
    }
}
impl ResponseData for () {
    fn decode(_: &mut dyn Read) -> Result<Self> {
        Ok(())
    }
}

impl CommandData for bool {
    fn encode(&self, writer: &mut dyn Write) -> Result<()> {
        (*self as u8).encode(writer)
    }
}

impl ResponseData for bool {
    fn decode(reader: &mut dyn Read) -> Result<Self> {
        match u8::decode(reader)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::InvalidOutputValue),
        }
    }
}

// We can't just add a impl for Deref<Target=[u8]> as that would lead to
// conflicting implmentations if (in an unlikly future) u64: Deref<Target=[u8]>.
macro_rules! buf_impls { ($($T: ty)+) => { $(
    impl TpmData for $T {
        fn data_len(&self) -> usize {
            2 + self.len()
        }
    }
    impl CommandData for $T {
        fn encode(&self, writer: &mut dyn Write) -> Result<()> {
            let size: u16 = self.len().try_into().or(Err(Error::TooBigInputBuffer))?;
            size.encode(writer)?;
            writer.write(&self)
        }
    }
)+ } }

buf_impls! { [u8] }
