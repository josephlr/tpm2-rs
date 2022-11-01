//! `TPMI_*` Interface Types
//!
//! Most of these types are found in Part 2, Section 9 (Interface Types).
//! TODO: Should we have these types at all?

use super::tpm;

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
