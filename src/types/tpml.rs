//! `TPML_*` List Types
//!
//! Most of these types are found in Part 2, Section 10.9 (Lists).

use core::marker::PhantomData;

use crate::{
    error::{MarshalError, UnmarshalError},
    types,
    types::{tpm, tpma, tpms, tpmt},
    Marshal, Unmarshal,
};

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
        *self
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
        *self
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
        self.count as usize
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

/// TPML_ALG_PROPERTY
#[derive(Clone, Copy, Debug, Default)]
pub struct AlgProperty<'t> {
    pub handles: Out<'t, tpms::AlgProperty>,
}
impl<'t> Unmarshal<'t> for AlgProperty<'t> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.handles.unmarshal(buf)
    }
}

/// TPML_HANDLE
#[derive(Clone, Copy, Debug, Default)]
pub struct Handle<'t> {
    pub handles: Out<'t, types::Handle>,
}
impl<'t> Unmarshal<'t> for Handle<'t> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.handles.unmarshal(buf)
    }
}

/// TPML_CCA
#[derive(Clone, Copy, Debug, Default)]
pub struct Cca<'t> {
    pub attributes: Out<'t, tpma::Cc>,
}
impl<'t> Unmarshal<'t> for Cca<'t> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.attributes.unmarshal(buf)
    }
}

/// TPML_PCR_SELECTION
#[derive(Clone, Copy, Debug, Default)]
pub struct PcrSelection<'t> {
    pub selection: Out<'t, tpms::PcrSelection>,
}
impl<'t> Unmarshal<'t> for PcrSelection<'t> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.selection.unmarshal(buf)
    }
}

/// TPML_TAGGED_PCR_PROPERTY
#[derive(Clone, Copy, Debug, Default)]
pub struct TaggedPcrProperty<'t> {
    pub selection: Out<'t, tpms::TaggedPcrSelect>,
}
impl<'t> Unmarshal<'t> for TaggedPcrProperty<'t> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.selection.unmarshal(buf)
    }
}

/// TPML_CC
#[derive(Clone, Copy, Debug, Default)]
pub struct CommandCode<'t> {
    pub codes: Out<'t, tpm::CC>,
}
impl<'t> Unmarshal<'t> for CommandCode<'t> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.codes.unmarshal(buf)
    }
}

/// TPML_TAGGED_TPM_PROPERTY
#[derive(Clone, Copy, Debug, Default)]
pub struct TaggedTpmProperty<'t> {
    pub selection: Out<'t, tpms::TpmProperty>,
}
impl<'t> Unmarshal<'t> for TaggedTpmProperty<'t> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.selection.unmarshal(buf)
    }
}

/// TPML_ECC_CURVE
#[derive(Clone, Copy, Debug, Default)]
pub struct EccCurve<'t> {
    pub curves: Out<'t, tpm::EccCurve>,
}
impl<'t> Unmarshal<'t> for EccCurve<'t> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.curves.unmarshal(buf)
    }
}

/// TPML_TAGGED_POLICY
#[derive(Clone, Copy, Debug, Default)]
pub struct TaggedPolicy<'t> {
    pub policies: Out<'t, tpms::TaggedPolicy>,
}
impl<'t> Unmarshal<'t> for TaggedPolicy<'t> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.policies.unmarshal(buf)
    }
}

/// TPML_DIGEST_VALUES
#[derive(Clone, Copy, Debug, Default)]
pub struct DigestValues<'t> {
    pub digest: In<'t, tpmt::Hash>,
}
impl<'t> Marshal for DigestValues<'t> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.digest.marshal(buf)
    }
}
