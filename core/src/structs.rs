//! Structures defined in the TPM2 Spec

use crate::constants::{AlgHash, CommandCode};
use crate::data::{DataIn, DataOut};
use crate::Result;

// TPMS_CLOCK_INFO
#[derive(Debug)]
pub struct ClockInfo {
    pub clock: u64,
    pub reset_count: u32,
    pub restart_count: u32,
    pub safe: bool,
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
    pub time: u64,
    pub clock: ClockInfo,
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
    hash: Option<AlgHash>,
}

pub(crate) struct CommandHeader {
    pub tag: u16,
    pub size: u32,
    pub cmd: CommandCode,
}

impl DataIn for CommandHeader {
    fn into_bytes<'a>(&self, mut bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
        bytes = self.tag.into_bytes(bytes)?;
        bytes = self.size.into_bytes(bytes)?;
        bytes = self.cmd.into_bytes(bytes)?;
        Ok(bytes)
    }
}

pub(crate) struct ResponseHeader {
    pub tag: u16,
    pub size: u32,
    pub resp: u32,
}

impl DataOut for ResponseHeader {
    fn from_bytes(bytes: &mut &[u8]) -> Result<Self> {
        Ok(Self {
            tag: DataOut::from_bytes(bytes)?,
            size: DataOut::from_bytes(bytes)?,
            resp: DataOut::from_bytes(bytes)?,
        })
    }
}
