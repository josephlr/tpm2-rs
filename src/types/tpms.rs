//! `TPMS_*` Structure Types

use super::{tpm, tpma, tpmi, tpml, tpmt, tpmu, Handle};
use crate::{
    error::{MarshalError, UnmarshalError},
    marshal::{pop_array_mut, pop_slice},
    polyfill::ToMutArr,
    Marshal, MarshalFixed, Unmarshal,
};

/// TPMS_TIME_INFO
#[derive(Clone, Copy, Debug, Default)]
pub struct TimeInfo {
    time: u64,
    clock_info: ClockInfo,
}
impl MarshalFixed for TimeInfo {
    const SIZE: usize = <u64 as MarshalFixed>::SIZE + <ClockInfo as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        self.time.marshal_fixed(arr[0..8].to_mut_arr());
        self.clock_info.marshal_fixed(arr[8..].to_mut_arr());
    }
}
impl Unmarshal<'_> for TimeInfo {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        self.time.unmarshal(buf)?;
        self.clock_info.unmarshal(buf)
    }
}

/// TPMS_CLOCK_INFO
#[derive(Clone, Copy, Debug, Default)]
pub struct ClockInfo {
    clock: u64,
    reset_count: u32,
    restart_count: u32,
    safe: bool,
}

impl MarshalFixed for ClockInfo {
    const SIZE: usize = <u64 as MarshalFixed>::SIZE
        + <u32 as MarshalFixed>::SIZE
        + <u32 as MarshalFixed>::SIZE
        + <bool as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];

    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        self.clock.marshal_fixed(arr[0..8].to_mut_arr());
        self.reset_count.marshal_fixed(arr[8..12].to_mut_arr());
        self.restart_count.marshal_fixed(arr[12..16].to_mut_arr());
        self.safe.marshal_fixed(arr[16..].to_mut_arr());
    }
}
impl Unmarshal<'_> for ClockInfo {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        self.clock.unmarshal(buf)?;
        self.reset_count.unmarshal(buf)?;
        self.restart_count.unmarshal(buf)?;
        self.safe.unmarshal(buf)
    }
}

/// TPMS_AUTH_COMMAND
#[derive(Clone, Copy, Default)]
pub struct AuthCommand<'a> {
    pub session_handle: Handle,
    pub nonce: &'a [u8],
    pub session_attributes: tpma::Session,
    pub hmac: &'a [u8],
}

impl Marshal for AuthCommand<'_> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.session_handle.marshal(buf)?;
        self.nonce.marshal(buf)?;
        self.session_attributes.marshal(buf)?;
        self.hmac.marshal(buf)?;
        Ok(())
    }
}

/// TPMS_AUTH_RESPONSE
#[derive(Clone, Copy, Default)]
pub struct AuthResponse<'a> {
    pub nonce: &'a [u8],
    pub session_attributes: tpma::Session,
    pub hmac: &'a [u8],
}

impl<'a> Unmarshal<'a> for AuthResponse<'a> {
    fn unmarshal(&mut self, buf: &mut &'a [u8]) -> Result<(), UnmarshalError> {
        self.nonce.unmarshal(buf)?;
        self.session_attributes.unmarshal(buf)?;
        self.hmac.unmarshal(buf)?;
        Ok(())
    }
}

const SIZE_OF_SELECT: usize = 3;
pub const NUM_PCRS: usize = 8 * SIZE_OF_SELECT;

/// TPMS_PCR_SELECT
///
/// TODO: Do we want to support anything other than 24 PCRs?
pub type PcrSelect = [bool; NUM_PCRS];

impl Marshal for PcrSelect {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        (SIZE_OF_SELECT as u8).marshal(buf)?;
        let sel = pop_array_mut(buf)?;

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
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        let size: usize = u8::unmarshal_val(buf)?.into();
        let sel = pop_slice(size, buf)?;

        *self = [false; NUM_PCRS];
        for (byte_idx, &byte) in sel.iter().enumerate() {
            for bit_idx in 0..8 {
                let pcr_num = 8 * byte_idx + bit_idx;
                if byte & (1 << bit_idx) == 0 {
                    continue;
                }
                if pcr_num > NUM_PCRS {
                    return Err(UnmarshalError::PcrTooLarge(pcr_num));
                }
                self[pcr_num] = true;
            }
        }

        Ok(())
    }
}

/// TPMS_TAGGED_PCR_SELECT
#[derive(Clone, Copy, Default, Debug)]
pub struct TaggedPcrSelect {
    pub tag: tpm::PtPcr,
    pub select: PcrSelect,
}
impl Marshal for TaggedPcrSelect {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.tag.marshal(buf)?;
        self.select.marshal(buf)
    }
}
impl Unmarshal<'_> for TaggedPcrSelect {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        self.tag.unmarshal(buf)?;
        self.select.unmarshal(buf)
    }
}

/// TPMS_PCR_SELECTION
#[derive(Clone, Copy, Default, Debug)]
pub struct PcrSelection {
    pub hash: tpmi::AlgHash,
    pub select: PcrSelect,
}

impl Marshal for PcrSelection {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.hash.marshal(buf)?;
        self.select.marshal(buf)
    }
}

impl Unmarshal<'_> for PcrSelection {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        self.hash.unmarshal(buf)?;
        self.select.unmarshal(buf)
    }
}

/// TPMS_SCHEME_HASH (not used)
// pub type SchemeHash = tpmi::AlgHash;
/// TPMS_SCHEME_HMAC (not used)
// pub type SchemeHmac = SchemeHash;
/// TPMS_KEYEDHASH_PARMS (not used)
// pub type KeyedHashParms = Option<tpmt::KeyedHashScheme>;
/// TPMS_SYMCIPHER_PARMS (not used)
// pub type SymCipherParms = tpmt::SymDefObject;

/// TPMS_SCHEME_XOR
#[derive(Clone, Copy, Default, Debug)]
pub struct SchemeXor {
    pub hash: tpmi::AlgHash,
    pub kdf: tpmi::AlgKdf,
}

impl Marshal for SchemeXor {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.hash.marshal(buf)?;
        self.kdf.marshal(buf)
    }
}

impl Unmarshal<'_> for SchemeXor {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        self.hash.unmarshal(buf)?;
        self.kdf.unmarshal(buf)
    }
}

/// TPMS_ASYM_PARMS
#[derive(Clone, Copy, Default, Debug)]
pub struct AsymParms {
    pub symmetric: Option<tpmt::SymDefObject>,
    pub scheme: Option<tpmt::AsymScheme>,
}

/// TPMS_RSA_PARMS
#[derive(Clone, Copy, Debug)]
pub struct RsaParms {
    pub symmetric: Option<tpmt::SymDefObject>,
    pub scheme: Option<tpmt::AsymScheme>,
    pub key_bits: tpmi::RsaKeyBits,
    pub exponent: u32,
}

/// TPMS_ECC_PARMS
#[derive(Clone, Copy, Debug)]
pub struct EccParms {
    pub symmetric: Option<tpmt::SymDefObject>,
    pub scheme: Option<tpmt::AsymScheme>,
    pub curve_id: tpm::EccCurve,
    pub kdf: Option<tpmt::KdfScheme>,
}

/// TPMS_ECC_POINT
#[derive(Clone, Copy, Debug)]
pub struct EccPoint<'t> {
    pub x: &'t [u8],
    pub y: &'t [u8],
}

/// TPMS_CAPABILITY_DATA
#[derive(Clone, Copy, Debug)]
pub struct CapabilityData<'t> {
    pub capability: tpm::TpmCap,
    pub data: tpmu::Capabilities<'t>,
}

impl Default for CapabilityData<'_> {
    fn default() -> Self {
        Self {
            capability: tpm::TpmCap::default(),
            data: tpmu::Capabilities::Algorithms(tpml::AlgProperty::default()),
        }
    }
}

impl<'t> Unmarshal<'t> for CapabilityData<'t> {
    fn unmarshal(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
        self.capability.unmarshal(buf)?;
        self.data = match self.capability {
            tpm::TpmCap::Algs => {
                let mut prop = tpml::AlgProperty::default();
                prop.unmarshal(buf)?;
                tpmu::Capabilities::Algorithms(prop)
            }
            tpm::TpmCap::Handles => {
                let mut h = tpml::Handle::default();
                h.unmarshal(buf)?;
                tpmu::Capabilities::Handles(h)
            }
            tpm::TpmCap::Commands => {
                let mut c = tpml::Cca::default();
                c.unmarshal(buf)?;
                tpmu::Capabilities::Command(c)
            }
            tpm::TpmCap::PpCommands => {
                let mut c = tpml::CommandCode::default();
                c.unmarshal(buf)?;
                tpmu::Capabilities::PpCommands(c)
            }
            tpm::TpmCap::AuditCommands => {
                let mut c = tpml::CommandCode::default();
                c.unmarshal(buf)?;
                tpmu::Capabilities::AuditCommands(c)
            }
            tpm::TpmCap::Pcrs => {
                let mut p = tpml::PcrSelection::default();
                p.unmarshal(buf)?;
                tpmu::Capabilities::AssignedPcr(p)
            }
            tpm::TpmCap::TpmProperties => {
                let mut p = tpml::TaggedTpmProperty::default();
                p.unmarshal(buf)?;
                tpmu::Capabilities::TpmProperties(p)
            }
            tpm::TpmCap::PcrProperties => {
                let mut p = tpml::TaggedPcrProperty::default();
                p.unmarshal(buf)?;
                tpmu::Capabilities::PcrProperties(p)
            }
            tpm::TpmCap::EccCurves => {
                let mut c = tpml::EccCurve::default();
                c.unmarshal(buf)?;
                tpmu::Capabilities::EccCurves(c)
            }
            tpm::TpmCap::AuthPolicies => {
                let mut c = tpml::TaggedPolicy::default();
                c.unmarshal(buf)?;
                tpmu::Capabilities::AuthPolicies(c)
            }
            tpm::TpmCap::VendorProperty => todo!(),
        };

        Ok(())
    }
}

/// TPMS_ALG_PROPERTY
#[derive(Clone, Copy, Debug, Default)]
pub struct AlgProperty {
    pub alg: tpm::Alg,
    pub alg_properties: tpma::Algorithm,
}
impl Marshal for AlgProperty {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.alg.marshal(buf)?;
        self.alg_properties.marshal(buf)
    }
}
impl Unmarshal<'_> for AlgProperty {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        self.alg.unmarshal(buf)?;
        self.alg_properties.unmarshal(buf)
    }
}

/// TPMS_TAGGED_PROPERTY
#[derive(Clone, Copy, Debug, Default)]
pub struct TpmProperty {
    pub property: tpm::Pt,
    pub value: u32,
}
impl Marshal for TpmProperty {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.property.marshal(buf)?;
        self.value.marshal(buf)
    }
}
impl Unmarshal<'_> for TpmProperty {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        self.property.unmarshal(buf)?;
        self.value.unmarshal(buf)
    }
}

/// TPMS_TAGGED_POLICY
#[derive(Clone, Copy, Debug, Default)]
pub struct TaggedPolicy {
    pub handle: Handle,
    pub hash: Option<tpmt::Hash>,
}
impl Marshal for TaggedPolicy {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.handle.marshal(buf)?;
        self.hash.marshal(buf)
    }
}
impl Unmarshal<'_> for TaggedPolicy {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        self.handle.unmarshal(buf)?;
        self.hash.unmarshal(buf)
    }
}
