//! Structures defined in the TPM2 Spec
use super::constants::*;
use super::{CommandData, ResponseData, Tpm};
use crate::Result;

// Header for all commands (see v1.55, Part 1, Section 18)
pub(crate) struct CommandHeader {
    pub tag: tag::Command,
    pub size: u32,
    pub code: CommandCode,
}

// Header for all respsonses (see v1.55, Part 1, Section 18)
pub(crate) struct ResponseHeader {
    pub tag: tag::Command,
    pub size: u32,
    pub code: ResponseCode,
}

// TPMS_CLOCK_INFO (v1.55, Part 2, Section 10.11.1, Table 116)
#[derive(Clone, Copy, Debug)]
pub(crate) struct ClockInfo {
    clock: u64,
    reset_count: u32,
    restart_count: u32,
    safe: bool,
}

// TPMS_TIME_INFO (v1.55, Part 2, Section 10.11.6, Table 117)
#[derive(Clone, Copy, Debug)]
pub struct TimeInfo {
    time: u64,
    clock: ClockInfo,
}

// GENERATED CODE BELOW

impl ResponseData for ClockInfo {
    fn decode(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
        Ok(Self {
            clock: u64::decode(reader)?,
            reset_count: u32::decode(reader)?,
            restart_count: u32::decode(reader)?,
            safe: bool::decode(reader)?,
        })
    }
}

impl ResponseData for TimeInfo {
    fn decode(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
        Ok(Self {
            time: u64::decode(reader)?,
            clock: ClockInfo::decode(reader)?,
        })
    }
}

impl CommandData for CommandHeader {
    fn encoded_len(&self) -> usize {
        0 + self.tag.encoded_len() + self.size.encoded_len() + self.code.encoded_len()
    }
    fn encode(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()> {
        self.tag.encode(writer)?;
        self.size.encode(writer)?;
        self.code.encode(writer)?;
        Ok(())
    }
}

impl ResponseData for ResponseHeader {
    fn decode(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
        Ok(Self {
            tag: tag::Command::decode(reader)?,
            size: u32::decode(reader)?,
            code: ResponseCode::decode(reader)?,
        })
    }
}
