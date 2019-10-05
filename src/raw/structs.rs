//! Structures defined in the TPM2 Spec
use super::constants::*;
use super::{CommandData, ResponseData, Tpm};
use crate::Result;

// Header for all commands (see v1.55, Part 1, Section 18)
#[derive(CommandData)]
pub(crate) struct CommandHeader {
    pub tag: tag::Command,
    pub size: u32,
    pub code: CommandCode,
}

// Header for all respsonses (see v1.55, Part 1, Section 18)
#[derive(ResponseData)]
pub(crate) struct ResponseHeader {
    pub tag: tag::Command,
    pub size: u32,
    pub code: ResponseCode,
}

// TPMS_CLOCK_INFO (v1.55, Part 2, Section 10.11.1, Table 116)
#[derive(Clone, Copy, Debug, ResponseData)]
pub(crate) struct ClockInfo {
    clock: u64,
    reset_count: u32,
    restart_count: u32,
    safe: bool,
}

// TPMS_TIME_INFO (v1.55, Part 2, Section 10.11.6, Table 117)
#[derive(Clone, Copy, Debug, ResponseData)]
pub struct TimeInfo {
    time: u64,
    clock: ClockInfo,
}
