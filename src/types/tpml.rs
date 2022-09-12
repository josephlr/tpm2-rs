use super::{marshal_slice, Marshal, Unmarshal};
use crate::{tpms, Result, ToUsize};
use core::{convert::TryInto, slice};

pub type Digest<'a> = TpmL<'a, &'a [u8]>;
pub type PcrSelection<'a> = TpmL<'a, tpms::PcrSelection>;

#[derive(Clone, Copy, Debug)]
pub enum TpmL<'a, T> {
    Raw(u32, &'a [u8]), // This data has been validated, just not stored
    Parsed(&'a [T]),
}

impl<'a, T> TpmL<'a, T> {
    #[inline]
    pub fn len(self) -> usize {
        match self {
            TpmL::Raw(len, _) => len.to_usize(),
            TpmL::Parsed(s) => s.len(),
        }
    }
    #[inline]
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
    #[inline]
    pub fn iter(self) -> Iter<'a, T> {
        Iter(self)
    }
}

impl<'a, T: Unmarshal<'a> + Default + Copy> IntoIterator for TpmL<'a, T> {
    type Item = T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self)
    }
}

impl<T> Default for TpmL<'_, T> {
    fn default() -> Self {
        Self::Parsed(&[])
    }
}

pub struct Iter<'a, T>(TpmL<'a, T>);

// TODO: Some of these can be more efficient
impl<'a, T: Unmarshal<'a> + Default + Copy> Iterator for Iter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match &mut self.0 {
            TpmL::Raw(count, data) => {
                *count = count.checked_sub(1)?;
                Some(T::unmarshal_val(data).unwrap())
            }
            TpmL::Parsed(s) => {
                let v: &T;
                (v, *s) = s.split_first()?;
                Some(v.clone())
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (len, Some(len))
    }
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }
}

impl<'a, T: Unmarshal<'a> + Default + Copy> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> From<&'a T> for TpmL<'a, T> {
    fn from(v: &'a T) -> Self {
        Self::Parsed(slice::from_ref(v))
    }
}
impl<'a, T> From<&'a [T]> for TpmL<'a, T> {
    fn from(s: &'a [T]) -> Self {
        Self::Parsed(s)
    }
}
impl<'a, T, const N: usize> From<&'a [T; N]> for TpmL<'a, T> {
    fn from(s: &'a [T; N]) -> Self {
        Self::Parsed(s)
    }
}

impl<T: Marshal> Marshal for TpmL<'_, T> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        match *self {
            TpmL::Raw(count, data) => {
                count.marshal(buf)?;
                marshal_slice(data.len(), buf)?.copy_from_slice(data);
            }
            TpmL::Parsed(s) => {
                let count: u32 = s.len().try_into()?;
                count.marshal(buf)?;
                for v in s {
                    v.marshal(buf)?;
                }
            }
        }
        Ok(())
    }
}

impl<'a, T: Unmarshal<'a> + Default> Unmarshal<'a> for TpmL<'a, T> {
    fn unmarshal(&mut self, buf: &mut &'a [u8]) -> Result<()> {
        let count = u32::unmarshal_val(buf)?;

        let orig: &[u8] = *buf;
        let mut tmp = T::default();
        for _ in 0..count {
            tmp.unmarshal(buf)?
        }
        let data_len = orig.len() - buf.len();

        *self = Self::Raw(count, &orig[..data_len]);
        Ok(())
    }
}
