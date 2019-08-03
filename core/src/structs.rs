//! Structures defined in the TPM2 Spec

use crate::constants;
use crate::data::DataOut;
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

// TPML_PCR_SELECTION
#[derive(Debug)]
pub struct PcrSelection {
    hash: Option<constants::AlgHash>,
}
