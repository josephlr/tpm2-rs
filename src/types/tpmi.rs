//! `TPMI_*` Interface Types
//!
//! Most of these types are found in Part 2, Section 9 (Interface Types).
//! TODO: Should we have these types at all?

use super::tpm;
use crate::marshal::{MarshalFixed, UnmarshalFixed};

/// TPMI_ALG_HASH
pub type AlgHash = tpm::Alg;
/// TPMI_ALG_KDF
pub type AlgKdf = tpm::Alg;
/// TPMI_ALG_SYM
pub type AlgSym = tpm::Alg;
// TPMI_ALG_SYM_OBJECT
pub type AlgSymObject = tpm::Alg;
/// TPMI_ALG_SYM_MODE
pub type AlgSymMode = tpm::Alg;
/// TPMI_ALG_ASYM_SCHEME
pub type AlgAsymScheme = tpm::Alg;
/// TPMI_ALG_PUBLIC
pub type AlgPublic = tpm::Alg;

/// TPMI_RSA_KEY_BITS
pub type RsaKeyBits = u16;

/// TPMI_RH_ENABLES
pub type RhEnables = u32;

pub fn hash_length(hash: AlgHash) -> usize {
    match hash {
        AlgHash::Sha1 => 20,
        AlgHash::Sha256 => 32,
        AlgHash::Sha384 => 48,
        AlgHash::Sha512 => 64,
        AlgHash::Sm3_256 => 32,
        AlgHash::Sha3_256 => 32,
        AlgHash::Sha3_384 => 48,
        AlgHash::Sha3_512 => 64,
        _ => panic!("Not a hash algorithm!"),
    }
}

/// TPMI_YES_NO
#[repr(u8)]
#[derive(Debug, Copy, Clone, Default)]
pub enum YesNo {
    #[default]
    No = 0,
    Yes = 1,
}

impl MarshalFixed for YesNo {
    const SIZE: usize = <u8 as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        (*self as u8).marshal_fixed(arr)
    }
}

impl UnmarshalFixed for YesNo {
    fn unmarshal_fixed(arr: &Self::ARRAY) -> Self {
        match arr[0] {
            0 => YesNo::No,
            1 => YesNo::Yes,
            _ => unreachable!(),
        }
    }
}
