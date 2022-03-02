//! TODO: Docs about Datatypes and how they work.

use crate::{
    error::{DecodeError, EncodeError},
    Data, Decode, Encode, FixedData,
};
use core::convert::TryFrom;

mod constants;
pub use constants::*;

mod structs;
pub use structs::*;

pub(crate) mod traits;

/// Abstraction over all types in `Raw`
pub trait FromRaw: Data + TryFrom<Self::Raw, Error = DecodeError> {
    type Raw: Data;
    type Prefix: FixedData;
    fn prefix(&self) -> Result<Self::Prefix, EncodeError>;
}

enum Inner<T: FromRaw> {
    Parsed(T),
    Unparsed(T::Raw),
}

/// TODO: Explain why this type exists (for TPM2B_* and enums)
pub struct Raw<T: FromRaw>(Inner<T>);

impl<T: FromRaw> Raw<T> {
    pub fn raw(u: T::Raw) -> Self {
        Self(Inner::Unparsed(u))
    }
    /// TODO: explain why this can't be try_from/try_into (https://github.com/rust-lang/rust/issues/50133)
    pub fn parse(self) -> Result<T, DecodeError> {
        match self.0 {
            Inner::Parsed(t) => Ok(t),
            Inner::Unparsed(u) => T::try_from(u),
        }
    }
}

impl<T: FromRaw> From<T> for Raw<T> {
    fn from(t: T) -> Self {
        Self(Inner::Parsed(t))
    }
}

impl<T: FromRaw> Data for Raw<T> {
    fn data_len(&self) -> usize {
        match &self.0 {
            Inner::Parsed(t) => T::Prefix::LEN + t.data_len(),
            Inner::Unparsed(u) => u.data_len(),
        }
    }
}

impl<T: FromRaw> Encode for Raw<T>
where
    T: Encode,
    T::Raw: Encode,
    T::Prefix: Encode,
{
    fn encode_to_buf(&self, output: &mut &mut [u8]) -> Result<(), EncodeError> {
        match &self.0 {
            Inner::Parsed(t) => {
                t.prefix()?.encode_to_buf(output)?;
                t.encode_to_buf(output)
            }
            Inner::Unparsed(u) => u.encode_to_buf(output),
        }
    }
}

impl<T: FromRaw> Default for Raw<T>
where
    T::Raw: Default,
{
    fn default() -> Self {
        Self::raw(Default::default())
    }
}

impl<'a, T: FromRaw> Decode<'a> for Raw<T>
where
    T: Decode<'a>,
    T::Raw: Decode<'a> + Default,
{
    fn decode_from_buf(&mut self, input: &mut &'a [u8]) -> Result<(), DecodeError> {
        if let Inner::Parsed(_) = self.0 {
            *self = Default::default();
        }
        match &mut self.0 {
            Inner::Parsed(_) => unreachable!(),
            Inner::Unparsed(u) => u.decode_from_buf(input),
        }
    }
}
