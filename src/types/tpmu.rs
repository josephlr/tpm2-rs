//! `TPMU_*` Untagged Union Types
//!
//! TODO: Explain why [`Name`] is weird

use super::{tpml, tpms, tpmt, Handle};
use crate::{
    error::{MarshalError, UnmarshalError},
    marshal::pop_array,
    Marshal, Unmarshal, UnmarshalFixed,
};

/// TPMU_NAME
#[derive(Clone, Copy, Debug)]
pub enum Name {
    Handle(Handle),
    Digest(tpmt::Hash),
}

impl Marshal for Name {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        match self {
            Name::Handle(h) => h.marshal(buf),
            Name::Digest(d) => d.marshal(buf),
        }
    }
}

impl Marshal for Option<Name> {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        match self {
            Some(n) => n.marshal(buf),
            None => Ok(()),
        }
    }
}

impl Unmarshal<'_> for Option<Name> {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        *self = match buf.len() {
            0 => None,
            4 => {
                let arr: &[u8; 4] = pop_array(buf).unwrap();
                Some(Name::Handle(Handle::unmarshal_fixed(arr)))
            }
            _ => {
                let h = Option::<tpmt::Hash>::unmarshal_val(buf)?.unwrap();
                Some(Name::Digest(h))
            }
        };
        Ok(())
    }
}

/// TPMU_PUBLIC_ID
///
/// Part of the TPMT_PUBLIC structure
#[derive(Clone, Copy, Debug)]
pub enum PublicId<'t> {
    KeyedHash(&'t [u8]),
    SymCipher(&'t [u8]),
    Rsa(&'t [u8]),
    Ecc(tpms::EccPoint<'t>),
    // Derive(tpms::Derive<'t>),
}

#[derive(Clone, Copy, Debug)]
pub enum Capabilities<'t> {
    Algorithms(tpml::AlgProperty<'t>),
    Handles(tpml::Handle<'t>),
    Command(tpml::Cca<'t>),
    PpCommands(tpml::CommandCode<'t>),
    AssignedPcr(tpml::PcrSelection<'t>),
    PcrProperties(tpml::TaggedPcrProperty<'t>),
    AuditCommands(tpml::CommandCode<'t>),
    TpmProperties(tpml::TaggedTpmProperty<'t>),
    EccCurves(tpml::EccCurve<'t>),
    AuthPolicies(tpml::TaggedPolicy<'t>),
}
