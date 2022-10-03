//! `TPMT_*` Tagged Union Types
//!
//! TODO: [`Public`] is weird.

use super::tpm;
use crate::{
    error::{MarshalError, UnmarshalError},
    marshal::{pop_array, pop_slice_mut},
    Marshal, Unmarshal,
};

/// TPMT_HA (TPMU_HA)
#[derive(Clone, Copy, Default, Debug)]
#[non_exhaustive]
pub enum Hash {
    #[default]
    Null,
    Sha1([u8; 20]),
    Sha256([u8; 32]),
    Sha384([u8; 48]),
    Sha512([u8; 64]),
    Sm3_256([u8; 32]),
    Sha3_256([u8; 32]),
    Sha3_384([u8; 48]),
    Sha3_512([u8; 64]),
}

impl Hash {
    pub const fn alg(&self) -> tpm::Alg {
        match self {
            Self::Null => tpm::Alg::Null,
            Self::Sha1(_) => tpm::Alg::Sha1,
            Self::Sha256(_) => tpm::Alg::Sha256,
            Self::Sha384(_) => tpm::Alg::Sha384,
            Self::Sha512(_) => tpm::Alg::Sha512,
            Self::Sm3_256(_) => tpm::Alg::Sm3_256,
            Self::Sha3_256(_) => tpm::Alg::Sha3_256,
            Self::Sha3_384(_) => tpm::Alg::Sha3_384,
            Self::Sha3_512(_) => tpm::Alg::Sha3_512,
        }
    }
    pub const fn digest(&self) -> &[u8] {
        match self {
            Self::Null => &[],
            Self::Sha1(d) => d,
            Self::Sha256(d) => d,
            Self::Sha384(d) => d,
            Self::Sha512(d) => d,
            Self::Sm3_256(d) => d,
            Self::Sha3_256(d) => d,
            Self::Sha3_384(d) => d,
            Self::Sha3_512(d) => d,
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
        *self = match tpm::Alg::unmarshal_val(buf)? {
            tpm::Alg::Null => Self::Null,
            tpm::Alg::Sha1 => Self::Sha1(*pop_array(buf)?),
            tpm::Alg::Sha256 => Self::Sha256(*pop_array(buf)?),
            tpm::Alg::Sha384 => Self::Sha384(*pop_array(buf)?),
            tpm::Alg::Sha512 => Self::Sha512(*pop_array(buf)?),
            tpm::Alg::Sm3_256 => Self::Sm3_256(*pop_array(buf)?),
            tpm::Alg::Sha3_256 => Self::Sha3_256(*pop_array(buf)?),
            tpm::Alg::Sha3_384 => Self::Sha3_384(*pop_array(buf)?),
            tpm::Alg::Sha3_512 => Self::Sha3_512(*pop_array(buf)?),
            _ => return Err(UnmarshalError::InvalidValue),
        };
        Ok(())
    }
}
