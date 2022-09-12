use super::{marshal_slice, unmarshal_slice, FixedSize, Marshal, Unmarshal};
use crate::{Error, Result};

#[derive(Clone, Copy, Debug)]
pub enum TpmL<'a, T> {
    Unparsed(&'a [u8]), // len is always a multiple of T::SIZE
    Parsed(&'a [T]),
}

impl<'a, T: Clone + Default + FixedSize + Unmarshal<'a>> TpmL<'a, T> {
    pub fn get(&self, n: usize) -> Result<T> {
        match *self {
            TpmL::Unparsed(data) => {
                let (start, overflow) = n.overflowing_mul(T::SIZE);
                if overflow || start >= data.len() {
                    Err(Error::IndexOutOfBounds)
                } else {
                    T::unmarshal_val(&mut &data[start..])
                }
            }
            TpmL::Parsed(elms) => match elms.get(n) {
                Some(elm) => Ok(elm.clone()),
                None => Err(Error::IndexOutOfBounds),
            },
        }
    }
}

impl<'a, T: FixedSize> TpmL<'a, T> {
    pub fn len(&self) -> usize {
        match self {
            TpmL::Unparsed(buf) => buf.len() / T::SIZE,
            TpmL::Parsed(elms) => elms.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            TpmL::Unparsed(buf) => buf.is_empty(),
            TpmL::Parsed(elms) => elms.is_empty(),
        }
    }
}

impl<'a, T> From<&'a [T]> for TpmL<'a, T> {
    fn from(s: &'a [T]) -> Self {
        Self::Parsed(s)
    }
}

impl<T: FixedSize + Marshal> Marshal for TpmL<'_, T> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        let len: u32 = self.len().try_into()?;
        len.marshal(buf)?;
        match *self {
            TpmL::Unparsed(data) => {
                marshal_slice(data.len(), buf)?.copy_from_slice(data);
            }
            TpmL::Parsed(elms) => {
                for elm in elms {
                    elm.marshal(buf)?;
                }
            }
        }
        Ok(())
    }
}

impl<'a, T: FixedSize> Unmarshal<'a> for TpmL<'a, T> {
    fn unmarshal(&mut self, buf: &mut &'a [u8]) -> Result<()> {
        let len: usize = u32::unmarshal_val(buf)?.try_into()?;
        let byte_len = len.checked_mul(T::SIZE).ok_or(Error::IntegerOverflow)?;
        *self = Self::Unparsed(unmarshal_slice(byte_len, buf)?);
        Ok(())
    }
}
