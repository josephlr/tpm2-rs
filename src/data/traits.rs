use crate::error::{DecodeError, EncodeError};
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use core::{convert::TryInto, mem};

/// A trait for objects that have a constant encode/decode length
pub trait FixedData {
    const LEN: usize;
}

/// A trait for all TPM data objects
pub trait Data {
    fn data_len(&self) -> usize;
}
impl<T: FixedData> Data for T {
    fn data_len(&self) -> usize {
        Self::LEN
    }
}

/// A trait for objects that can be written to a TPM command buffer
pub trait Encode: Data {
    fn encode_to_buf(&self, output: &mut &mut [u8]) -> Result<(), EncodeError>;
    #[cfg(feature = "alloc")]
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        let mut v = vec![0u8; self.data_len()];
        let mut s = v.as_mut_slice();
        self.encode_to_buf(&mut s)?;
        assert_eq!(s.len(), 0);
        Ok(v)
    }
}

/// A trait for objects that can be read to a TPM response buffer
pub trait Decode<'a>: Data {
    fn decode_from_buf(&mut self, input: &mut &'a [u8]) -> Result<(), DecodeError>;
    fn decode(&mut self, mut input: &'a [u8]) -> Result<(), DecodeError> {
        self.decode_from_buf(&mut input)?;
        if input.len() > 0 {
            return Err(DecodeError::InputTooLong);
        }
        Ok(())
    }
}

// No ownership issues, returns value directly
pub trait DecodeVal: Data + Sized {
    fn decode_val(input: &mut &[u8]) -> Result<Self, DecodeError>;
}

impl<T: DecodeVal> Decode<'_> for T {
    fn decode_from_buf(&mut self, input: &mut &[u8]) -> Result<(), DecodeError> {
        *self = Self::decode_val(input)?;
        Ok(())
    }
}

fn encode_buf(buf: &[u8], output: &mut &mut [u8]) -> Result<(), EncodeError> {
    if buf.len() > output.len() {
        return Err(EncodeError::OutputTooShort);
    }
    let (prefix, suffix) = mem::replace(output, &mut []).split_at_mut(buf.len());
    prefix.copy_from_slice(buf);
    *output = suffix;
    Ok(())
}

fn decode_buf<'a>(len: usize, input: &mut &'a [u8]) -> Result<&'a [u8], DecodeError> {
    if len > input.len() {
        return Err(DecodeError::InputTooShort);
    }
    let (prefix, suffix) = mem::replace(input, &[]).split_at(len);
    *input = suffix;
    Ok(prefix)
}

macro_rules! data_impls { ($($T: ty)+) => { $(
    impl FixedData for $T {
        const LEN: usize = mem::size_of::<Self>();
    }
)+ } }

data_impls! { () bool u8 u16 u32 u64 }

// Rust integral types don't have a common trait, so we have to use a macro.
macro_rules! int_impls { ($($T: ty)+) => { $(
    impl Encode for $T {
        fn encode_to_buf(&self, output: &mut &mut [u8]) -> Result<(), EncodeError> {
            encode_buf(&self.to_be_bytes(), output)
        }
    }
    impl DecodeVal for $T {
        fn decode_val(input: &mut &[u8]) -> Result<Self, DecodeError> {
            let arr: [u8; Self::LEN] = decode_buf(Self::LEN, input)?.try_into().unwrap();
            Ok(Self::from_be_bytes(arr))
        }
    }

)+ } }

int_impls! { u8 u16 u32 u64 }

impl Encode for () {
    fn encode_to_buf(&self, _: &mut &mut [u8]) -> Result<(), EncodeError> {
        Ok(())
    }
}
impl DecodeVal for () {
    fn decode_val(_: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(())
    }
}

impl Encode for bool {
    fn encode_to_buf(&self, output: &mut &mut [u8]) -> Result<(), EncodeError> {
        (*self as u8).encode_to_buf(output)
    }
}
impl DecodeVal for bool {
    fn decode_val(input: &mut &[u8]) -> Result<Self, DecodeError> {
        match u8::decode_val(input)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::InvalidInput),
        }
    }
}

impl Data for &[u8] {
    fn data_len(&self) -> usize {
        2 + self.len()
    }
}
impl Encode for &[u8] {
    fn encode_to_buf(&self, cmd: &mut &mut [u8]) -> Result<(), EncodeError> {
        let size: u16 = self.len().try_into()?;
        size.encode_to_buf(cmd)?;
        encode_buf(self, cmd)
    }
}
impl<'a> Decode<'a> for &'a [u8] {
    fn decode_from_buf(&mut self, input: &mut &'a [u8]) -> Result<(), DecodeError> {
        let len: usize = u16::decode_val(input)?.into();
        *self = decode_buf(len, input)?;
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl Data for Vec<u8> {
    fn data_len(&self) -> usize {
        self.as_slice().data_len()
    }
}
#[cfg(feature = "alloc")]
impl Encode for Vec<u8> {
    fn encode_to_buf(&self, output: &mut &mut [u8]) -> Result<(), EncodeError> {
        self.as_slice().encode_to_buf(output)
    }
}
#[cfg(feature = "alloc")]
impl Decode<'_> for Vec<u8> {
    fn decode_from_buf(&mut self, input: &mut &[u8]) -> Result<(), DecodeError> {
        let len: usize = u16::decode_val(input)?.into();
        let buf = decode_buf(len, input)?;
        self.resize(len, 0);
        self.copy_from_slice(buf);
        Ok(())
    }
}
