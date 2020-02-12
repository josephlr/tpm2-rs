//! A TPM2 TSS. Add more docs and doc-tests.

// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
// #![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![allow(dead_code)]
// #![feature(const_if_match)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod error;
pub use error::{Error, Result};

pub mod raw;

#[cfg(feature = "std")]
mod os;
