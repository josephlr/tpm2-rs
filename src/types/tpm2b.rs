//! `TPM2B_*` Buffer Types
//!
//! TODO: Which buffers should be typed vs. sized vs. plain

use core::marker::PhantomData;

use super::{tpms, tpmt, tpmu};
use crate::{
    error::{MarshalError, UnmarshalError},
    marshal::pop_array_mut,
    Marshal, MarshalFixed, Unmarshal,
};

pub type NameIn<'b> = In<'b, tpmu::Name>;
pub type NameOut<'t> = Out<'t, tpmu::Name>;
pub type PublicIn<'b> = In<'b, tpmt::Public<'b>>;
pub type PublicOut<'t> = Out<'t, tpmt::Public<'t>>;
pub type CreationData<'t> = Out<'t, tpms::CreationData<'t>>;

#[derive(Clone, Copy, Debug)]
pub enum In<'b, T> {
    Raw(&'b [u8]),
    Val(T),
}

impl<T: Default> Default for In<'_, T> {
    fn default() -> Self {
        Self::Val(T::default())
    }
}

impl<T: Marshal> Marshal for In<'_, T> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        match self {
            In::Raw(s) => s.marshal(buf),
            In::Val(t) => {
                let size_buf = pop_array_mut::<2>(buf)?;
                let buf_len = buf.len();
                t.marshal(buf)?;
                let size: u16 = (buf_len - buf.len()).try_into()?;
                size.marshal_fixed(size_buf);
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Out<'t, T: ?Sized>(&'t [u8], PhantomData<&'t T>);

impl<T> Default for Out<'_, T> {
    fn default() -> Self {
        Self(&[], PhantomData)
    }
}

impl<'t, T> Out<'t, T> {
    pub fn bytes(self) -> &'t [u8] {
        self.0
    }
}

impl<'t, T: Unmarshal<'t> + Default> Out<'t, T> {
    pub fn parse(mut self) -> Result<T, UnmarshalError> {
        let t = T::unmarshal_val(&mut self.0)?;
        if self.0.is_empty() {
            Ok(t)
        } else {
            Err(UnmarshalError::BufferRemaining)
        }
    }
}

impl<'t, T> Unmarshal<'t> for Out<'t, T> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.0.unmarshal(buf)
    }
    fn unmarshal_val(buf: &mut &'t [u8]) -> Result<Self, UnmarshalError> {
        Ok(Self(<&'t [u8]>::unmarshal_val(buf)?, PhantomData))
    }
}
