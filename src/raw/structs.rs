//! Structures defined in the TPM2 Spec
use super::constants::*;
use super::{ReadData, Tpm, WriteData};
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

impl ReadData for ClockInfo {
    fn read_data(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
        Ok(Self {
            clock: u64::read_data(reader)?,
            reset_count: u32::read_data(reader)?,
            restart_count: u32::read_data(reader)?,
            safe: bool::read_data(reader)?,
        })
    }
}

impl ReadData for TimeInfo {
    fn read_data(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
        Ok(Self {
            time: u64::read_data(reader)?,
            clock: ClockInfo::read_data(reader)?,
        })
    }
}

impl WriteData for CommandHeader {
    fn data_len(&self) -> usize {
        0 + self.tag.data_len() + self.size.data_len() + self.code.data_len()
    }
    fn write_data(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()> {
        self.tag.write_data(writer)?;
        self.size.write_data(writer)?;
        self.code.write_data(writer)?;
        Ok(())
    }
}

impl ReadData for ResponseHeader {
    fn read_data(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
        Ok(Self {
            tag: tag::Command::read_data(reader)?,
            size: u32::read_data(reader)?,
            code: ResponseCode::read_data(reader)?,
        })
    }
}
