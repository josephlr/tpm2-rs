use super::{pop_slice_mut, tpms, Marshal, Unmarshal};
use crate::{
    error::{MarshalError, UnmarshalError},
    polyfill::ToUsize,
};
use core::mem;

pub type Digest<'a> = TpmL<'a, &'a [u8]>;
pub type PcrSelection<'a> = TpmL<'a, tpms::PcrSelection>;

#[derive(Clone, Copy, Debug)]
pub enum TpmL<'a, T> {
    Raw(u32, &'a [u8]), // This data has been validated, just not stored
    Slice(&'a [T]),     // len fits in u32 (checked when constructing)
    Value(Option<T>),
}

impl<'a, T> TpmL<'a, T> {
    #[inline]
    pub fn len(&self) -> u32 {
        match self {
            Self::Raw(len, _) => *len,
            Self::Slice(s) => s.len() as u32,
            Self::Value(Some(_)) => 1,
            Self::Value(None) => 0,
        }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
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
        Self::Value(None)
    }
}

pub struct Iter<'a, T>(TpmL<'a, T>);

// TODO: We could implement additional Iterator methods more efficiently
impl<'a, T: Unmarshal<'a> + Default + Copy> Iterator for Iter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match &mut self.0 {
            TpmL::Raw(count, data) => {
                *count = count.checked_sub(1)?;
                Some(T::unmarshal_val(data).unwrap())
            }
            TpmL::Slice(s) => {
                let v: &T;
                (v, *s) = s.split_first()?;
                Some(*v)
            }
            TpmL::Value(s) => mem::take(s),
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    fn count(self) -> usize {
        self.len()
    }
}

impl<'a, T: Unmarshal<'a> + Default + Copy> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.0.len().to_usize()
    }
}

impl<T> From<T> for TpmL<'_, T> {
    fn from(v: T) -> Self {
        Self::Value(Some(v))
    }
}
impl<T> From<[T; 0]> for TpmL<'_, T> {
    fn from(_: [T; 0]) -> Self {
        Self::Value(None)
    }
}
impl<T> From<[T; 1]> for TpmL<'_, T> {
    fn from(a: [T; 1]) -> Self {
        let [v] = a;
        Self::Value(Some(v))
    }
}
impl<'a, T> From<&'a [T]> for TpmL<'a, T> {
    fn from(s: &'a [T]) -> Self {
        assert!(s.len() <= u32::MAX.to_usize());
        Self::Slice(s)
    }
}
impl<'a, T, const N: usize> From<&'a [T; N]> for TpmL<'a, T> {
    fn from(s: &'a [T; N]) -> Self {
        Self::Slice(s)
    }
}

impl<T: Marshal> Marshal for TpmL<'_, T> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.len().marshal(buf)?;
        match self {
            TpmL::Raw(_, data) => {
                pop_slice_mut(data.len(), buf)?.copy_from_slice(data);
            }
            TpmL::Slice(s) => {
                for v in *s {
                    v.marshal(buf)?;
                }
            }
            TpmL::Value(Some(v)) => v.marshal(buf)?,
            TpmL::Value(None) => {}
        }
        Ok(())
    }
}

impl<'a, T: Unmarshal<'a> + Default> Unmarshal<'a> for TpmL<'a, T> {
    fn unmarshal(&mut self, buf: &mut &'a [u8]) -> Result<(), UnmarshalError> {
        let count = u32::unmarshal_val(buf)?;

        let orig: &'a [u8] = buf;
        let mut tmp = T::default();
        for _ in 0..count {
            tmp.unmarshal(buf)?
        }
        let data_len = orig.len() - buf.len();

        *self = Self::Raw(count, &orig[..data_len]);
        Ok(())
    }
}
