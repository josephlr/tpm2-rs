//! `TPML_*` List Types
//!
//! Most of these types are found in Part 2, Section 10.9 (Lists).

use core::marker::PhantomData;

use super::tpms;
use crate::{
    error::{MarshalError, UnmarshalError},
    polyfill::ToUsize,
    Marshal, Unmarshal,
};

pub type PcrSelectionIn<'b> = In<'b, tpms::PcrSelection>;
pub type PcrSelectionOut<'t> = Out<'t, tpms::PcrSelection>;
pub type DigestIn<'b> = In<'b, &'b [u8]>;
pub type DigestOut<'t> = Out<'t, &'t [u8]>;

/// Generic type for TPM inputs (just contains a slice)
#[derive(Debug)]
pub struct In<'b, T>(pub &'b [T]);

impl<T> In<'_, T> {
    pub const fn len(&self) -> usize {
        self.0.len()
    }
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> Clone for In<'_, T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<T> Copy for In<'_, T> {}
impl<T> Default for In<'_, T> {
    fn default() -> Self {
        Self(&[])
    }
}

impl<'b, T> From<&'b [T]> for In<'b, T> {
    fn from(s: &'b [T]) -> Self {
        Self(s)
    }
}
impl<'b, T, const N: usize> From<&'b [T; N]> for In<'b, T> {
    fn from(s: &'b [T; N]) -> Self {
        Self(s)
    }
}
impl<'b, T> From<In<'b, T>> for &'b [T] {
    fn from(s: In<'b, T>) -> Self {
        s.0
    }
}

impl<T: Marshal> Marshal for In<'_, T> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        u32::try_from(self.0.len())?.marshal(buf)?;
        for v in self.0 {
            v.marshal(buf)?;
        }
        Ok(())
    }
}

/// Generic type for TPM outputs (iterator over T values)
#[derive(Debug)]
pub struct Out<'t, T> {
    count: u32,
    buf: &'t [u8], // This data has been validated, just not copied
    phantom: PhantomData<&'t [T]>,
}
impl<T> Clone for Out<'_, T> {
    fn clone(&self) -> Self {
        Self {
            count: self.count,
            buf: self.buf,
            phantom: self.phantom,
        }
    }
}
impl<T> Copy for Out<'_, T> {}
impl<T> Default for Out<'_, T> {
    fn default() -> Self {
        Self {
            count: 0,
            buf: &[],
            phantom: PhantomData,
        }
    }
}

impl<T> Out<'_, T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.count.to_usize()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

// TODO: We could implement additional Iterator methods more efficiently
impl<'t, T: Unmarshal<'t> + Default> Iterator for Out<'t, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        let v = T::unmarshal_val(&mut self.buf).unwrap();
        Some(v)
    }
}

impl<'t, T: Unmarshal<'t> + Default> ExactSizeIterator for Out<'t, T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<'t, T: Unmarshal<'t> + Default> Unmarshal<'t> for Out<'t, T> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.count = u32::unmarshal_val(buf)?;
        let orig: &'t [u8] = buf;

        // Validate but don't store the values
        let mut v = T::default();
        for _ in 0..self.count {
            v.unmarshal(buf)?;
        }
        // Store the buffer we just read
        let data_len = orig.len() - buf.len();
        self.buf = &orig[..data_len];
        Ok(())
    }
}
