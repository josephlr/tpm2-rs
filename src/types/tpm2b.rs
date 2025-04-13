//! `TPM2B_*` Buffer Types
//!
//! TODO: Which buffers should be typed vs. sized vs. plain
use core::fmt::Debug;

use super::{tpmi, tpms, tpmt, Handle};
use crate::{
    error::{MarshalError, UnmarshalError},
    marshal::pop_array_mut,
    Marshal, MarshalFixed, Unmarshal, UnmarshalFixed,
};

/// TPM2B_NAME (TPMU_NAME)
#[derive(Clone, Copy, Debug)]
pub enum Name {
    Handle(Handle),
    Digest(tpmt::Hash),
}

impl Default for Name {
    fn default() -> Self {
        Self::Handle(0)
    }
}

impl Name {
    const fn len(&self) -> usize {
        match self {
            Name::Handle(_) => Handle::SIZE,
            Name::Digest(d) => tpmi::AlgHash::SIZE + d.digest().len(),
        }
    }
}

impl Marshal for Name {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        // self.len() will always fit in a u16
        u16::try_from(self.len()).unwrap().marshal(buf)?;
        match self {
            Name::Handle(h) => h.marshal(buf),
            Name::Digest(d) => d.marshal(buf),
        }
    }
}

impl Marshal for Option<Name> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        match self {
            Some(n) => n.marshal(buf),
            None => 0u16.marshal(buf),
        }
    }
}

impl Unmarshal<'_> for Option<Name> {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        let mut raw: &[u8] = Unmarshal::unmarshal_val(buf)?;
        *self = if raw.is_empty() {
            None
        } else if let Ok(arr) = raw.try_into() {
            let h = Handle::unmarshal_fixed(arr);
            Some(Name::Handle(h))
        } else {
            let d = tpmt::Hash::unmarshal_val(&mut raw)?;
            if !raw.is_empty() {
                return Err(UnmarshalError::BufferRemaining);
            }
            Some(Name::Digest(d))
        };
        Ok(())
    }
}

pub type PublicIn<'b> = dyn In<tpmt::Public<'b>>;
pub type PublicOut<'t> = Out<tpmt::Public<'t>>;
pub type CreationData<'t> = Out<tpms::CreationData<'t>>;

pub trait In<T>: Debug {
    fn marshal_impl(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError>;
}

impl<T: Marshal + Debug> In<T> for T {
    fn marshal_impl(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        let size_buf = pop_array_mut::<2>(buf)?;
        let buf_len = buf.len();
        self.marshal(buf)?;
        let size: u16 = (buf_len - buf.len()).try_into()?;
        size.marshal_fixed(size_buf);
        Ok(())
    }
}
impl<T> In<T> for [u8] {
    fn marshal_impl(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.marshal(buf)
    }
}

impl<T: Marshal> Marshal for dyn In<T> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.marshal_impl(buf)
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct Out<T: ?Sized>(pub T);

impl<'t, T: Unmarshal<'t> + ?Sized> Unmarshal<'t> for Out<T> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        let mut raw: &'t [u8] = Unmarshal::unmarshal_val(buf)?;
        self.0.unmarshal(&mut raw)?;
        if !raw.is_empty() {
            return Err(UnmarshalError::BufferRemaining);
        }
        Ok(())
    }
}
