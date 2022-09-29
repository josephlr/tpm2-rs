//! TPM2 TSS written in pure Rust
#![no_std]
#![feature(split_array, doc_cfg)]

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod marshal;
mod polyfill;
mod traits;

pub mod commands;
pub mod error;
pub mod os;
pub mod types;

pub use commands::Command;
pub use error::Error;
pub use marshal::{Marshal, MarshalFixed, Unmarshal, UnmarshalAny};
pub use traits::{Tpm, TpmExt, TpmRaw};

#[cfg(test)]
mod test {
    use std::vec::Vec;

    use crate::{commands::*, types::tpm, *};

    #[test]
    fn can_exec() {
        #[allow(dead_code)]
        fn take_tpm(tpm: &mut dyn Tpm) -> Result<Vec<u8>, Error> {
            tpm.run(&Startup {
                startup_type: tpm::SU::Clear,
            })?;

            let rsp = tpm.run(&GetRandom {
                bytes_requested: 12,
            })?;
            let b = Vec::from(rsp.random_bytes);

            tpm.run(&Shutdown {
                shutdown_type: tpm::SU::Clear,
            })?;
            Ok(b)
        }
    }
}
