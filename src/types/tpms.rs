use super::{tpma, FixedSize, Marshal, Unmarshal};
use crate::{Handle, Result};

// TPMS_TIME_INFO
#[derive(Clone, Copy, Debug, Default)]
pub struct TimeInfo {
    time: u64,
    clock_info: ClockInfo,
}

// TPMS_CLOCK_INFO
#[derive(Clone, Copy, Debug, Default)]
pub struct ClockInfo {
    clock: u64,
    reset_count: u32,
    restart_count: u32,
    safe: bool,
}

impl Marshal for TimeInfo {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        self.time.marshal(buf)?;
        self.clock_info.marshal(buf)
    }
}
impl Unmarshal<'_> for TimeInfo {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<()> {
        self.time.unmarshal(buf)?;
        self.clock_info.unmarshal(buf)
    }
}

impl FixedSize for TimeInfo {
    const SIZE: usize = <u64 as FixedSize>::SIZE + <ClockInfo as FixedSize>::SIZE;
}

impl Marshal for ClockInfo {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        self.clock.marshal(buf)?;
        self.reset_count.marshal(buf)?;
        self.restart_count.marshal(buf)?;
        self.safe.marshal(buf)
    }
}
impl Unmarshal<'_> for ClockInfo {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<()> {
        self.clock.unmarshal(buf)?;
        self.reset_count.unmarshal(buf)?;
        self.restart_count.unmarshal(buf)?;
        self.safe.unmarshal(buf)
    }
}

impl FixedSize for ClockInfo {
    const SIZE: usize = <u64 as FixedSize>::SIZE
        + <u32 as FixedSize>::SIZE
        + <u32 as FixedSize>::SIZE
        + <bool as FixedSize>::SIZE;
}

// TPMS_AUTH_COMMAND
#[derive(Clone, Copy, Default)]
pub struct AuthCommand<'a> {
    pub session_handle: Handle,
    pub nonce: &'a [u8],
    pub session_attributes: tpma::Session,
    pub hmac: &'a [u8],
}

impl Marshal for AuthCommand<'_> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        self.session_handle.marshal(buf)?;
        self.nonce.marshal(buf)?;
        self.session_attributes.marshal(buf)?;
        self.hmac.marshal(buf)?;
        Ok(())
    }
}

// TPMS_AUTH_RESPONSE
#[derive(Clone, Copy, Default)]
pub struct AuthResponse<'a> {
    pub nonce: &'a [u8],
    pub session_attributes: tpma::Session,
    pub hmac: &'a [u8],
}

impl<'a> Unmarshal<'a> for AuthResponse<'a> {
    fn unmarshal(&mut self, buf: &mut &'a [u8]) -> Result<()> {
        self.nonce.unmarshal(buf)?;
        self.session_attributes.unmarshal(buf)?;
        self.hmac.unmarshal(buf)?;
        Ok(())
    }
}
