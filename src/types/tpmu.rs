//! `TPMU_*` Untagged Union Types
//!
//! TODO: Explain why [`Name`] is weird

use super::{tpm, tpmi, tpms};
use crate::{error::UnmarshalError, Unmarshal};

/// TPMU_PUBLIC_ID
///
/// Part of the TPMT_PUBLIC structure
#[derive(Clone, Copy, Debug)]
pub enum PublicId<'t> {
    KeyedHash(&'t [u8]),
    SymCipher(&'t [u8]),
    Rsa(&'t [u8]),
    Ecc(tpms::EccPoint<'t>),
    // Derive(tpms::Derive<'t>),
}

impl<'t> PublicId<'t> {
    pub(crate) fn unmarshal_with_alg(
        alg: tpmi::AlgPublic,
        buf: &mut &'t [u8],
    ) -> Result<Self, UnmarshalError> {
        let v = match alg {
            tpm::Alg::KeyedHash => Self::KeyedHash(Unmarshal::unmarshal_val(buf)?),
            tpm::Alg::SymCipher => Self::SymCipher(Unmarshal::unmarshal_val(buf)?),
            tpm::Alg::Rsa => Self::Rsa(Unmarshal::unmarshal_val(buf)?),
            tpm::Alg::Ecc => Self::Ecc(Unmarshal::unmarshal_val(buf)?),
            _ => return Err(UnmarshalError::InvalidValue),
        };
        Ok(v)
    }
}

impl Default for PublicId<'_> {
    fn default() -> Self {
        Self::KeyedHash(&[])
    }
}
