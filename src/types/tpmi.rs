//! `TPMI_*` Interface Types
//!
//! Most of these types are found in Part 2, Section 9 (Interface Types).
//! TODO: Should we have these types at all?

use super::tpm;

/// TPMI_ALG_HASH
pub type AlgHash = tpm::Alg;
/// TPMI_ALG_KDF
pub type AlgKdf = tpm::Alg;
/// TPMI_ALG_SYM (TPMI_ALG_SYM_OBJECT), expect not xor or null
pub type AlgSym = tpm::Alg;
/// TPMI_ALG_SYM_MODE
pub type AlgSymMode = tpm::Alg;
