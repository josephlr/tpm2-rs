use super::{marshal_array, unmarshal_slice, FixedSize, Marshal, Unmarshal};
use crate::{tpm, tpma, Error, Handle, Result};

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

const SIZE_OF_SELECT: usize = 3;
pub const NUM_PCRS: usize = 8 * SIZE_OF_SELECT;

// TODO: Do we want to support anything other than 24 PCRs?
pub type PcrSelect = [bool; NUM_PCRS];

impl Marshal for PcrSelect {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        (SIZE_OF_SELECT as u8).marshal(buf)?;
        let sel = marshal_array::<SIZE_OF_SELECT>(buf)?;

        *sel = [0; SIZE_OF_SELECT];
        for (i, &bit) in self.iter().enumerate() {
            if !bit {
                continue;
            }
            let byte_idx = i / 8;
            let bit_idx = i % 8;
            sel[byte_idx] |= 1 << bit_idx;
        }
        Ok(())
    }
}

impl Unmarshal<'_> for PcrSelect {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<()> {
        let size: usize = u8::unmarshal_val(buf)?.into();
        let sel = unmarshal_slice(size, buf)?;

        *self = [false; NUM_PCRS];
        for (byte_idx, &byte) in sel.iter().enumerate() {
            for bit_idx in 0..8 {
                let pcr_num = 8 * byte_idx + bit_idx;
                if byte & (1 << bit_idx) == 0 {
                    continue;
                }
                if pcr_num > NUM_PCRS {
                    return Err(Error::PcrTooLarge(pcr_num));
                }
                self[pcr_num] = true;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct PcrSelection {
    pub hash: tpm::Alg,
    pub select: PcrSelect,
}

impl Marshal for PcrSelection {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        self.hash.marshal(buf)?;
        self.select.marshal(buf)
    }
}

impl Unmarshal<'_> for PcrSelection {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<()> {
        self.hash.unmarshal(buf)?;
        self.select.unmarshal(buf)
    }
}
