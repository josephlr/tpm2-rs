//! `TPMT_*` Tagged Union Types
//!
//! TODO: [`Public`] is weird.

use super::tpm;
use crate::{
    error::{MarshalError, UnmarshalError},
    marshal::{pop_array, pop_slice_mut},
    Marshal, Unmarshal,
};

#[derive(Clone, Copy, Default, Debug)]
pub enum Hash {
    #[default]
    Null,
    SHA1([u8; 20]),
    SHA256([u8; 32]),
    // TODO implement the other hash algos
}

impl Hash {
    pub const fn alg(&self) -> tpm::Alg {
        match self {
            Self::Null => tpm::Alg::Null,
            Self::SHA1(_) => tpm::Alg::SHA1,
            Self::SHA256(_) => tpm::Alg::SHA256,
        }
    }
    pub const fn digest(&self) -> &[u8] {
        match self {
            Self::Null => &[],
            Self::SHA1(d) => d,
            Self::SHA256(d) => d,
        }
    }
}

impl Marshal for Hash {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.alg().marshal(buf)?;
        let d = self.digest();
        pop_slice_mut(d.len(), buf)?.copy_from_slice(d);
        Ok(())
    }
}

impl Unmarshal<'_> for Hash {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        *self = Self::unmarshal_val(buf)?;
        Ok(())
    }
    fn unmarshal_val(buf: &mut &[u8]) -> Result<Self, UnmarshalError> {
        match tpm::Alg::unmarshal_val(buf)? {
            tpm::Alg::Null => Ok(Self::Null),
            tpm::Alg::SHA1 => Ok(Self::SHA1(*pop_array(buf)?)),
            tpm::Alg::SHA256 => Ok(Self::SHA256(*pop_array(buf)?)),
            _ => Err(UnmarshalError::InvalidValue),
        }
    }
}
