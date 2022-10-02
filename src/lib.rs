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

pub use error::Error;
pub use marshal::{Marshal, MarshalFixed, Unmarshal, UnmarshalAny};
pub use traits::{Tpm, TpmExt, TpmRaw};

#[cfg(test)]
mod test {
    use std::vec::Vec;

    use crate::{
        commands::*,
        types::{tpm, Auth, PasswordAuth},
        *,
    };

    #[test]
    fn can_exec() {
        #[allow(dead_code)]
        fn take_tpm(tpm: &mut dyn Tpm) -> Result<Vec<u8>, Error> {
            tpm.run(Startup {
                startup_type: tpm::SU::Clear,
            })?;

            let rsp = tpm.run(GetRandom {
                bytes_requested: 12,
            })?;
            let b = Vec::from(rsp.random_bytes);

            tpm.run(Shutdown {
                shutdown_type: tpm::SU::Clear,
            })?;
            Ok(b)
        }
    }

    #[test]
    #[allow(clippy::needless_borrow)]
    fn with_auth() {
        fn check_run<C: Command + Auths<N>, const N: usize>(_: C) {}

        let password = [1, 2, 3, 4, 5];
        let auth1 = PasswordAuth(&password);

        static PASS_AUTH: PasswordAuth = PasswordAuth(&[42; 5]);
        let auth2: &'static dyn Auth = &PASS_AUTH;

        let c0 = GetRandom {
            bytes_requested: 12,
        };
        check_run(c0);
        check_run(&c0);

        let c1 = c0.with_auth(&auth1);
        check_run(c1);
        check_run(&c1);

        let c2 = c1.with_auth(auth2);
        check_run(c2);
        check_run(&c2);

        let c3 = c2.with_auth(&auth1);
        check_run(c3);
        check_run(&c3);

        let _c4 = c3.with_auth(auth2);
        // Does not compile
        // check_run(_c4);
    }
}
