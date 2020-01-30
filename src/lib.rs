//! A TPM2 TSS. Add more docs and doc-tests.

// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
// #![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![allow(dead_code)]
// #![feature(const_if_match)]
#![no_std]

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        extern crate std;
        mod os;
    }
}

mod error;
pub use error::{Error, Result};

pub mod raw;
