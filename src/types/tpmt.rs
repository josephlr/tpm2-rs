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
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Hash {
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
    pub const fn alg(&self) -> tpmi::AlgHash {
        match self {
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

impl Marshal for Option<Hash> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        match self {
            None => tpm::Alg::Null.marshal(buf),
            Some(h) => h.marshal(buf),
        }
    }
}

impl Unmarshal<'_> for Option<Hash> {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        let alg = tpm::Alg::unmarshal_val(buf)?;
        *self = match alg {
            tpm::Alg::Null => None,
            tpm::Alg::Sha1 => Some(Hash::Sha1(*pop_array(buf)?)),
            tpm::Alg::Sha256 => Some(Hash::Sha256(*pop_array(buf)?)),
            tpm::Alg::Sha384 => Some(Hash::Sha384(*pop_array(buf)?)),
            tpm::Alg::Sha512 => Some(Hash::Sha512(*pop_array(buf)?)),
            tpm::Alg::Sm3_256 => Some(Hash::Sm3_256(*pop_array(buf)?)),
            tpm::Alg::Sha3_256 => Some(Hash::Sha3_256(*pop_array(buf)?)),
            tpm::Alg::Sha3_384 => Some(Hash::Sha3_384(*pop_array(buf)?)),
            tpm::Alg::Sha3_512 => Some(Hash::Sha3_512(*pop_array(buf)?)),
            _ => return Err(UnmarshalError::InvalidValue),
        };
        Ok(())
    }
}

/// TPMT_PUBLIC_PARMS (TPMU_PUBLIC_PARMS)
pub enum PublicParms {
    KeyedHash(Option<KeyedHashScheme>),
    SymCipher(SymDefObject),
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
#[derive(Clone, Copy, Debug)]
pub enum KeyedHashScheme {
    Hmac(tpmi::AlgHash),
    Xor(tpms::SchemeXor),
}

#[derive(Clone, Copy, Debug)]
pub struct SymDefObject {
    pub algorithm: tpmi::AlgSymObject,
    pub key_bits: tpm::KeyBits,
    pub mode: tpmi::AlgSymMode,
}

#[derive(Clone, Copy, Debug)]
pub enum SymDef {
    Sym(SymDefObject),
    Xor(tpmi::AlgHash),
}

/// TPMT_ASYM_SCHEME (TPMU_ASYM_SCHEME)
///
/// Also effectivly includes the TPMS_{KEY,SIG,ENC} types
#[derive(Clone, Copy, Debug)]
pub enum AsymScheme {
    RsaSsa(tpmi::AlgHash),
    RsaEs,
    RsaPss(tpmi::AlgHash),
    Oaep(tpmi::AlgHash),
    Ecdsa(tpmi::AlgHash),
    Ecdh(tpmi::AlgHash),
    Ecdaa(tpmi::AlgHash, u16),
    Sm2(tpmi::AlgHash),
    EcSchnorr(tpmi::AlgHash),
    Ecmqv(tpmi::AlgHash),
}

impl AsymScheme {
    /// The asymetric signing algorithm
    pub const fn alg(&self) -> tpmi::AlgAsymScheme {
        match self {
            Self::RsaSsa(_) => tpm::Alg::RsaSsa,
            Self::RsaEs => tpm::Alg::RsaEs,
            Self::RsaPss(_) => tpm::Alg::RsaPss,
            Self::Oaep(_) => tpm::Alg::Oaep,
            Self::Ecdsa(_) => tpm::Alg::Ecdsa,
            Self::Ecdh(_) => tpm::Alg::Ecdh,
            Self::Ecdaa(_, _) => tpm::Alg::Ecdaa,
            Self::Sm2(_) => tpm::Alg::Sm2,
            Self::EcSchnorr(_) => tpm::Alg::EcSchnorr,
            Self::Ecmqv(_) => tpm::Alg::Ecmqv,
        }
    }

    /// The hash algorithm used in this signing method. Returns `Alg::Null` if
    /// the method doesn't use a hash algorithm.
    pub const fn hash(&self) -> tpmi::AlgHash {
        match *self {
            Self::RsaSsa(h) => h,
            Self::RsaEs => tpm::Alg::Null,
            Self::RsaPss(h) => h,
            Self::Oaep(h) => h,
            Self::Ecdsa(h) => h,
            Self::Ecdh(h) => h,
            Self::Ecdaa(h, _) => h,
            Self::Sm2(h) => h,
            Self::EcSchnorr(h) => h,
            Self::Ecmqv(h) => h,
        }
    }
}
