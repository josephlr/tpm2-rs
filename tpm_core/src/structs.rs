use crate::data::{DataIn, DataOut};
use crate::Result;

// TPMS_CLOCK_INFO
#[derive(Debug)]
pub struct ClockInfo {
    clock: u64,
    reset_count: u32,
    restart_count: u32,
    safe: bool,
}

impl DataOut for ClockInfo {
    fn from_bytes(bytes: &mut &[u8]) -> Result<Self> {
        Ok(Self {
            clock: DataOut::from_bytes(bytes)?,
            reset_count: DataOut::from_bytes(bytes)?,
            restart_count: DataOut::from_bytes(bytes)?,
            safe: DataOut::from_bytes(bytes)?,
        })
    }
}

// TPMS_TIME_INFO
#[derive(Debug)]
pub struct TimeInfo {
    time: u64,
    clock: ClockInfo,
}

impl DataOut for TimeInfo {
    fn from_bytes(bytes: &mut &[u8]) -> Result<Self> {
        Ok(Self {
            time: DataOut::from_bytes(bytes)?,
            clock: DataOut::from_bytes(bytes)?,
        })
    }
}

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

// TPML_PCR_SELECTION
#[derive(Debug)]
pub struct PcrSelection {
    hash: Option<AlgHash>,
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
