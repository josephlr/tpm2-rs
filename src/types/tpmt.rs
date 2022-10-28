//! `TPMT_*` Tagged Union Types
//!
//! TODO: [`Public`] is weird.

use super::{tpm, tpmi, tpms};
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

/// TPMT_PUBLIC_PARMS (TPMU_PUBLIC_PARMS)
pub enum PublicParms {
    KeyedHash(tpms::KeyedHashParms),
    SymCipher(tpms::SymCipherParms),
    Rsa(()),
    Ecc(()),
}

impl PublicParms {
    pub const fn alg(&self) -> tpm::Alg {
        match self {
            PublicParms::KeyedHash(_) => tpm::Alg::KeyedHash,
            PublicParms::SymCipher(_) => tpm::Alg::SymCipher,
            PublicParms::Rsa(_) => tpm::Alg::Rsa,
            PublicParms::Ecc(_) => tpm::Alg::Ecc,
        }
    }
}

/// TPMT_KEYEDHASH_SCHEME (TPMU_SCHEME_KEYEDHASH)
#[derive(Clone, Copy, Default, Debug)]
pub enum KeyedHashScheme {
    Hmac(tpms::SchemeHmac),
    Xor(tpms::SchemeXor),
    #[default]
    Null,
}

#[derive(Clone, Copy, Default, Debug)]
pub enum SymDefObject {
    Sym(tpmi::AlgSym, tpm::KeyBits, tpmi::AlgSymMode),
    Xor(tpmi::AlgHash),
    #[default]
    Null,
}

/// TPMT_ASYM_SCHEME (TPMU_ASYM_SCHEME)
///
/// Also effectivly includes the TPMS_{KEY,SIG,ENC} types
#[derive(Clone, Copy, Default, Debug)]
pub enum AsymScheme {
    Ecdh(tpmi::AlgHash),
    Ecmqv(tpmi::AlgHash),
    RsaSsa(tpmi::AlgHash),
    RsaPss(tpmi::AlgHash),
    Ecdsa(tpmi::AlgHash),
    Ecdaa(tpmi::AlgHash, u16),
    Sm2(tpmi::AlgHash),
    EcSchnorr(tpmi::AlgHash),
    RsaEs,
    Oaep(tpmi::AlgHash),
    #[default]
    Null,
}

impl AsymScheme {
    /// The asymetric signing algorithm
    pub const fn alg(&self) -> tpmi::AlgAsymScheme {
        match self {
            AsymScheme::Ecdh(_) => tpm::Alg::Ecdh,
            AsymScheme::Ecmqv(_) => tpm::Alg::Ecmqv,
            AsymScheme::RsaSsa(_) => tpm::Alg::RsaSsa,
            AsymScheme::RsaPss(_) => tpm::Alg::RsaPss,
            AsymScheme::Ecdsa(_) => tpm::Alg::Ecdsa,
            AsymScheme::Ecdaa(_, _) => tpm::Alg::Ecdaa,
            AsymScheme::Sm2(_) => tpm::Alg::Sm2,
            AsymScheme::EcSchnorr(_) => tpm::Alg::EcSchnorr,
            AsymScheme::RsaEs => tpm::Alg::RsaEs,
            AsymScheme::Oaep(_) => tpm::Alg::Oaep,
            AsymScheme::Null => tpm::Alg::Null,
        }
    }

    /// The hash algorithm used in this signing method. Returns `Alg::Null` if
    /// the method doesn't use a hash algorithm.
    pub const fn hash(&self) -> tpmi::AlgHash {
        match *self {
            AsymScheme::Ecdh(h) => h,
            AsymScheme::Ecmqv(h) => h,
            AsymScheme::RsaSsa(h) => h,
            AsymScheme::RsaPss(h) => h,
            AsymScheme::Ecdsa(h) => h,
            AsymScheme::Ecdaa(h, _) => h,
            AsymScheme::Sm2(h) => h,
            AsymScheme::EcSchnorr(h) => h,
            AsymScheme::RsaEs => tpm::Alg::Null,
            AsymScheme::Oaep(h) => h,
            AsymScheme::Null => tpm::Alg::Null,
        }
    }
}
