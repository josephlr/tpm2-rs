//! A TPM2 TSS. Add more docs and doc-tests.

// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
// #![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![allow(dead_code)]
#![no_std]

extern crate alloc;
cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        extern crate std;
        mod os;
        pub use os::OsTpm;
    }
}

mod error;
pub use error::{Error, Result};

pub mod buf;
pub mod raw;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_safety() {
        let _: Option<&dyn raw::Tpm> = None;
    }
}
