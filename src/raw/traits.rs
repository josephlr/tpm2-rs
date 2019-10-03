//! Helper traits for implementing TPMs and TPM-compatible data structures
use core::{convert::TryInto, mem};

use super::Tpm;
use crate::{Error, Result};

/// A trait for objects that can be written to a TPM command buffer
pub trait WriteData {
    /// The number of bytes written with a call to [`WriteData::write_data`].
    fn data_len(&self) -> usize {
        mem::size_of_val(self)
    }
    /// Serialize this object and write it to `writer`.
    fn write_data(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()>;
}

/// A trait for objects that can be read from a TPM response buffer
pub trait ReadDataMut {
    /// Deserialize this object by reading bytes from `reader`.
    fn read_data_mut(&mut self, reader: &mut (impl Tpm + ?Sized)) -> Result<()>;
}

/// A trait for creating objects from a TPM response buffer;
pub trait ReadData: Sized {
    fn read_data(reader: &mut (impl Tpm + ?Sized)) -> Result<Self>;
}

impl<T: ReadData> ReadDataMut for T {
    fn read_data_mut(&mut self, reader: &mut (impl Tpm + ?Sized)) -> Result<()> {
        *self = Self::read_data(reader)?;
        Ok(())
    }
}

// Rust integral types don't have a command trait, so we have to use a macro.
macro_rules! int_impls { ($($T: ty)+) => { $(
    impl WriteData for $T {
        fn write_data(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()> {
            writer.write(&self.to_be_bytes())
        }
    }

    impl ReadData for $T {
        fn read_data(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
            let mut arr = [0u8; mem::size_of::<Self>()];
            reader.read(&mut arr)?;
            Ok(Self::from_be_bytes(arr))
        }
    }
)+ } }

int_impls! { u8 u16 u32 u64 }

impl WriteData for () {
    fn write_data(&self, _: &mut (impl Tpm + ?Sized)) -> Result<()> {
        Ok(())
    }
}

impl ReadData for () {
    fn read_data(_: &mut (impl Tpm + ?Sized)) -> Result<Self> {
        Ok(())
    }
}

impl WriteData for bool {
    fn write_data(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()> {
        (*self as u8).write_data(writer)
    }
}

impl ReadData for bool {
    fn read_data(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
        match u8::read_data(reader)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::InvalidOutputValue),
        }
    }
}

impl WriteData for [u8] {
    fn data_len(&self) -> usize {
        2 + self.len()
    }
    fn write_data(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()> {
        let size: u16 = self.len().try_into().or(Err(Error::TooBigInputBuffer))?;
        size.write_data(writer)?;
        writer.write(&self)
    }
}

impl ReadDataMut for &mut [u8] {
    fn read_data_mut(&mut self, reader: &mut (impl Tpm + ?Sized)) -> Result<()> {
        let size: usize = u16::read_data(reader)?.into();
        // We have to resize the buffer without creating lifetime issues.
        match mem::replace(self, &mut []).get_mut(..size) {
            Some(buf) => *self = buf,
            None => return Err(Error::TooSmallOutputBuffer),
        };
        reader.read(self)
    }
}
