//! A TPM2 TSS. Add more docs and doc-tests.

// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
// #![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![no_std]

pub mod raw;
pub use raw::Tpm;
mod error;
pub use error::{Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

// The os module will be empty depending on the platform/features.
mod os;
pub use os::*;
