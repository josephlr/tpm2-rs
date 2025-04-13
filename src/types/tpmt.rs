//! `TPMT_*` Tagged Union Types
//!
//! TODO: [`Public`] is weird.

use super::{tpm, tpma, tpmi, tpms, tpmu};
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
    fn unmarshal_with_alg(alg: tpmi::AlgHash, buf: &mut &[u8]) -> Result<Self, UnmarshalError> {
        let v = match alg {
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
        Ok(v)
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self::Sha256([0; 32])
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

impl Unmarshal<'_> for Hash {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        let alg = tpm::Alg::unmarshal_val(buf)?;
        *self = Hash::unmarshal_with_alg(alg, buf)?;
        Ok(())
    }
}

impl Unmarshal<'_> for Option<Hash> {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        let alg = tpm::Alg::unmarshal_val(buf)?;
        *self = if alg == tpm::Alg::Null {
            None
        } else {
            Some(Hash::unmarshal_with_alg(alg, buf)?)
        };
        Ok(())
    }
}

/// TPMT_PUBLIC_PARMS (TPMU_PUBLIC_PARMS)
#[derive(Clone, Copy, Debug)]
pub enum PublicParms {
    KeyedHash(Option<KeyedHashScheme>),
    SymCipher(SymDefObject),
    Rsa(tpms::RsaParms),
    Ecc(tpms::EccParms),
}

impl PublicParms {
    pub const fn alg(&self) -> tpmi::AlgPublic {
        match self {
            PublicParms::KeyedHash(_) => tpm::Alg::KeyedHash,
            PublicParms::SymCipher(_) => tpm::Alg::SymCipher,
            PublicParms::Rsa(_) => tpm::Alg::Rsa,
            PublicParms::Ecc(_) => tpm::Alg::Ecc,
        }
    }
    /// Returns common asymmetric scheme details if using RSA or ECC
    pub const fn asym(&self) -> Option<tpms::AsymParms> {
        match self {
            PublicParms::KeyedHash(_) => None,
            PublicParms::SymCipher(_) => None,
            PublicParms::Rsa(p) => Some(tpms::AsymParms {
                symmetric: p.symmetric,
                scheme: p.scheme,
            }),
            PublicParms::Ecc(p) => Some(tpms::AsymParms {
                symmetric: p.symmetric,
                scheme: p.scheme,
            }),
        }
    }
    fn unmarshal_with_alg(alg: tpmi::AlgPublic, buf: &mut &[u8]) -> Result<Self, UnmarshalError> {
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

impl Default for PublicParms {
    fn default() -> Self {
        Self::KeyedHash(None)
    }
}

/// TPMT_KEYEDHASH_SCHEME (TPMU_SCHEME_KEYEDHASH)
#[derive(Clone, Copy, Debug)]
pub enum KeyedHashScheme {
    Hmac(tpmi::AlgHash),
    Xor(tpms::SchemeXor),
}

impl KeyedHashScheme {
    pub const fn alg(&self) -> tpmi::AlgKeyedHashScheme {
        match self {
            KeyedHashScheme::Hmac(_) => tpm::Alg::Hmac,
            KeyedHashScheme::Xor(_) => tpm::Alg::Xor,
        }
    }
}

impl Unmarshal<'_> for Option<KeyedHashScheme> {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        *self = Self::unmarshal_val(buf)?;
        Ok(())
    }

    fn unmarshal_val(buf: &mut &[u8]) -> Result<Self, UnmarshalError> {
        match tpmi::AlgKeyedHashScheme::unmarshal_val(buf)? {
            tpm::Alg::Null => Ok(None),
            tpm::Alg::Hmac => Ok(Some(KeyedHashScheme::Hmac(Unmarshal::unmarshal_val(buf)?))),
            tpm::Alg::Xor => Ok(Some(KeyedHashScheme::Xor(Unmarshal::unmarshal_val(buf)?))),
            _ => Err(UnmarshalError::InvalidValue),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SymDefObject {
    pub algorithm: tpmi::AlgSymObject,
    pub key_bits: tpm::KeyBits,
    pub mode: tpmi::AlgSymMode,
}

impl Unmarshal<'_> for SymDefObject {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        *self = Self::unmarshal_val(buf)?;
        Ok(())
    }
    fn unmarshal_val(buf: &mut &[u8]) -> Result<Self, UnmarshalError> {
        match Option::<SymDef>::unmarshal_val(buf)? {
            Some(SymDef::Sym(s)) => Ok(s),
            _ => Err(UnmarshalError::InvalidValue),
        }
    }
}

impl Unmarshal<'_> for Option<SymDefObject> {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        *self = Self::unmarshal_val(buf)?;
        Ok(())
    }
    fn unmarshal_val(buf: &mut &[u8]) -> Result<Self, UnmarshalError> {
        match Option::<SymDef>::unmarshal_val(buf)? {
            Some(SymDef::Sym(s)) => Ok(Some(s)),
            Some(SymDef::Xor(_)) => Err(UnmarshalError::InvalidValue),
            None => Ok(None),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SymDef {
    Sym(SymDefObject),
    Xor(tpmi::AlgHash),
}

impl Unmarshal<'_> for Option<SymDef> {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        *self = Self::unmarshal_val(buf)?;
        Ok(())
    }
    fn unmarshal_val(buf: &mut &[u8]) -> Result<Self, UnmarshalError> {
        Ok(match tpmi::AlgSym::unmarshal_val(buf)? {
            tpm::Alg::Null => None,
            tpm::Alg::Xor => Some(SymDef::Xor(Unmarshal::unmarshal_val(buf)?)),
            a => Some(SymDef::Sym(SymDefObject {
                algorithm: a,
                key_bits: Unmarshal::unmarshal_val(buf)?,
                mode: Unmarshal::unmarshal_val(buf)?,
            })),
        })
    }
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

impl Unmarshal<'_> for Option<AsymScheme> {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        *self = Self::unmarshal_val(buf)?;
        Ok(())
    }

    fn unmarshal_val(buf: &mut &[u8]) -> Result<Self, UnmarshalError> {
        Ok(match tpmi::AlgAsymScheme::unmarshal_val(buf)? {
            tpm::Alg::Null => None,
            tpm::Alg::RsaEs => Some(AsymScheme::RsaEs),
            alg => {
                let h = tpmi::AlgHash::unmarshal_val(buf)?;
                Some(match alg {
                    tpm::Alg::RsaSsa => AsymScheme::RsaSsa(h),
                    tpm::Alg::RsaPss => AsymScheme::RsaPss(h),
                    tpm::Alg::Oaep => AsymScheme::Oaep(h),
                    tpm::Alg::Ecdsa => AsymScheme::Ecdsa(h),
                    tpm::Alg::Ecdh => AsymScheme::Ecdh(h),
                    tpm::Alg::Ecdaa => AsymScheme::Ecdaa(h, u16::unmarshal_val(buf)?),
                    tpm::Alg::Sm2 => AsymScheme::Sm2(h),
                    tpm::Alg::EcSchnorr => AsymScheme::EcSchnorr(h),
                    tpm::Alg::Ecmqv => AsymScheme::Ecmqv(h),
                    _ => return Err(UnmarshalError::InvalidValue),
                })
            }
        })
    }
}

/// TPMT_KDF_SCHEME (TPMU_KDF_SCHEME)
#[derive(Clone, Copy, Debug, Default)]
pub struct KdfScheme {
    pub scheme: tpmi::AlgKdf,
    pub hash: tpmi::AlgHash,
}

impl Unmarshal<'_> for Option<KdfScheme> {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        *self = Self::unmarshal_val(buf)?;
        Ok(())
    }

    fn unmarshal_val(buf: &mut &[u8]) -> Result<Self, UnmarshalError> {
        Ok(match tpmi::AlgKdf::unmarshal_val(buf)? {
            tpm::Alg::Null => None,
            scheme => {
                let hash = tpmi::AlgHash::unmarshal_val(buf)?;
                Some(KdfScheme { scheme, hash })
            }
        })
    }
}

/// TPMT_PUBLIC
///
/// The algorithm is encoded via the parameters field.
#[derive(Clone, Copy, Debug, Default)]
pub struct Public<'t> {
    pub name_alg: Option<tpmi::AlgHash>,
    pub object_attributes: tpma::Object,
    pub auth_policy: &'t [u8],
    pub parameters: PublicParms,
    pub unique: tpmu::PublicId<'t>,
}

impl Public<'_> {
    /// The Object's type
    pub const fn alg(&self) -> tpmi::AlgPublic {
        self.parameters.alg()
    }
}

impl<'t> Unmarshal<'t> for Public<'t> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        let alg = tpmi::AlgPublic::unmarshal_val(buf)?;
        self.name_alg.unmarshal(buf)?;
        self.object_attributes.unmarshal(buf)?;
        self.auth_policy.unmarshal(buf)?;
        self.parameters = PublicParms::unmarshal_with_alg(alg, buf)?;
        self.unique = tpmu::PublicId::unmarshal_with_alg(alg, buf)?;
        Ok(())
    }
}

/// TPMT_TK_CREATION
#[derive(Clone, Copy, Debug, Default)]
pub struct TkCreation<'a> {
    pub hierarchy: tpmi::RhHierarchy,
    pub digest: &'a [u8],
}
