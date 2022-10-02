//! TPM2 [`Command`]s and Responses
//!
//! These structures should be fairly direct translations of the
//! "TPM2_* Command" and "TPM2_* Response" tables in Part 3 of the TPM2 Spec.
//!
//! Of the 117 TPM2 commands, 0 are implemented.
//! If a command is not implemented, there will be a skeleton of code and doc
//! comments which are commented out.
//!
//! TODO:
//!   - Add additional notes about TPM2_HMAC and TPM2_StartHMAC

mod structs;
pub use structs::*;
