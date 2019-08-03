//! Constants (i.e. C-style enums) defined in the TPM2 Spec

use crate::data::DataIn;
use crate::Result;

// TPMI_ALG_HASH (see table )
#[repr(u16)]
#[derive(Debug)]
pub enum AlgHash {
    SHA1 = 0x0004,
    SHA256 = 0x000B,
    SHA384 = 0x000C,
    SHA512 = 0x000D,
    SM3_256 = 0x0012,
    SHA3_256 = 0x0027,
    SHA3_384 = 0x0028,
    SHA3_512 = 0x0029,
}

// TPM_SU
#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum StartupType {
    Clear = 0x0000,
    State = 0x0001,
}

impl DataIn for StartupType {
    fn into_bytes<'a>(&self, bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
        (*self as u16).into_bytes(bytes)
    }
}
