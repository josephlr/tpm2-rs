//! Helper traits for implementing TPMs and TPM-compatible data structures
use core::{convert::TryInto, mem};

use alloc::{vec, vec::Vec};

use super::Tpm;
use crate::{Error, Result};
pub use tpm_derive::*;

pub trait TpmData {
    fn data_len(&self) -> usize;
}

/// A trait for objects that can be written to a TPM command buffer
pub trait CommandData: TpmData {
    /// Serialize this object and write it to `writer`.
    fn encode(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()>;
}

/// A trait for creating objects from a TPM response buffer;
pub trait ResponseData: TpmData + Sized {
    fn decode(reader: &mut (impl Tpm + ?Sized)) -> Result<Self>;
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
        fn encode(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()> {
            writer.write(&self.to_be_bytes())
        }
    }
    impl ResponseData for $T {
        fn decode(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
            let mut arr = [0u8; mem::size_of::<Self>()];
            reader.read(&mut arr)?;
            Ok(Self::from_be_bytes(arr))
        }
    }
)+ } }

int_impls! { u8 u16 u32 u64 }

impl CommandData for () {
    fn encode(&self, _: &mut (impl Tpm + ?Sized)) -> Result<()> {
        Ok(())
    }
}
impl ResponseData for () {
    fn decode(_: &mut (impl Tpm + ?Sized)) -> Result<Self> {
        Ok(())
    }
}

impl CommandData for bool {
    fn encode(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()> {
        (*self as u8).encode(writer)
    }
}

impl ResponseData for bool {
    fn decode(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
        match u8::decode(reader)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::InvalidOutputValue),
        }
    }
}

impl<T: Deref<[u8]>> TpmData for T {
    fn data_len(&self) -> usize {
        2 + self.len()
    }
}

impl<T: Deref<[u8]>> CommandData for T {
    fn encode(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()> {
        let size: u16 = self.len().try_into().or(Err(Error::TooBigInputBuffer))?;
        size.encode(writer)?;
        writer.write(&self)
    }
}

impl ResponseData for Vec<u8> {
    fn decode(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
        let size: usize = u16::decode(reader)?.into();
        let mut data = vec![0u8; size];
        reader.read(&mut data)?;
        Ok(data)
    }
}
