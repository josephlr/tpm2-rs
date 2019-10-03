//! Helper traits for implementing TPMs and TPM-compatible data structures
use core::{convert::TryInto, mem};

use super::Tpm;
use crate::{Error, Result};

/// A trait for objects that can be written to a TPM command buffer
pub trait CommandData {
    /// The number of bytes written with a call to [`CommandData::encode`].
    fn encoded_len(&self) -> usize {
        mem::size_of_val(self)
    }
    /// Serialize this object and write it to `writer`.
    fn encode(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()>;
}

/// A trait for objects that can be read from a TPM response buffer
pub trait ResponseDataRef {
    /// Deserialize this object by reading bytes from `reader`.
    fn decode_ref(&mut self, reader: &mut (impl Tpm + ?Sized)) -> Result<()>;
}

/// A trait for creating objects from a TPM response buffer;
pub trait ResponseData: Sized {
    fn decode(reader: &mut (impl Tpm + ?Sized)) -> Result<Self>;
}

impl<T: ResponseData> ResponseDataRef for T {
    fn decode_ref(&mut self, reader: &mut (impl Tpm + ?Sized)) -> Result<()> {
        *self = Self::decode(reader)?;
        Ok(())
    }
}

// Rust integral types don't have a command trait, so we have to use a macro.
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

impl CommandData for [u8] {
    fn encoded_len(&self) -> usize {
        2 + self.len()
    }
    fn encode(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()> {
        let size: u16 = self.len().try_into().or(Err(Error::TooBigInputBuffer))?;
        size.encode(writer)?;
        writer.write(&self)
    }
}

impl ResponseDataRef for &mut [u8] {
    fn decode_ref(&mut self, reader: &mut (impl Tpm + ?Sized)) -> Result<()> {
        let size: usize = u16::decode(reader)?.into();
        // We have to resize the buffer without creating lifetime issues.
        match mem::replace(self, &mut []).get_mut(..size) {
            Some(buf) => *self = buf,
            None => return Err(Error::TooSmallOutputBuffer),
        };
        reader.read(self)
    }
}
