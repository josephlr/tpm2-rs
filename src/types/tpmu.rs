//! `TPMU_*` Untagged Union Types
//!
//! TODO: Explain why [`Name`] is weird

use super::{tpmt, Handle};
use crate::{
    error::{MarshalError, UnmarshalError},
    marshal::pop_array,
    Marshal, Unmarshal, UnmarshalFixed,
};

/// TPMU_NAME
#[derive(Clone, Copy, Default, Debug)]
pub enum Name {
    #[default]
    None,
    Handle(Handle),
    Digest(tpmt::Hash),
}

impl Marshal for Name {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        match self {
            Name::None => Ok(()),
            Name::Handle(h) => h.marshal(buf),
            Name::Digest(d) => d.marshal(buf),
        }
    }
}

impl Unmarshal<'_> for Name {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        *self = Self::unmarshal_val(buf)?;
        Ok(())
    }

    fn unmarshal_val(buf: &mut &[u8]) -> Result<Self, UnmarshalError> {
        Ok(match buf.len() {
            0 => Self::None,
            4 => {
                let arr: &[u8; 4] = pop_array(buf).unwrap();
                Self::Handle(Handle::unmarshal_fixed(arr))
            }
            _ => Self::Digest(tpmt::Hash::unmarshal_val(buf)?),
        })
    }
}
