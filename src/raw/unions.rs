//! Tagged unions defined in the TPM2 Spec
use super::*;

/// TPMS_CAPABILITY_DATA/TPMU_CAPABILITY_DATA
pub enum Capabilities<'a> {
    Algorithms(&'a mut [AlgProp]),
    Handles(&'a mut [Handle]),
    Commands(&'a mut [CommandProp]),
    PPCommands(&'a mut [CommandCode]),
    AuditCommands(&'a mut [CommandCode]),
    PCRs(&'a mut [PCRSelection]),
    TPMProps(&'a mut [TPMProp]),
    PCRProps(&'a mut [PCRProp]),
    ECCCurves(&'a mut [ECCCurve]),
    AuthPolicies(&'a mut [PolicyProp]),
    ACT(&'a mut [ACTData]),
}

use alg::Hash::*;
/// TPMT_HA/TPMU_HA
pub enum Hash {
    SHA1([u8; SHA1.size()]),
    SHA256([u8; SHA256.size()]),
    SHA384([u8; SHA384.size()]),
    SHA512([u8; SHA512.size()]),
    SM3_256([u8; SM3_256.size()]),
    SHA3_256([u8; SHA3_256.size()]),
    SHA3_384([u8; SHA3_384.size()]),
    SHA3_512([u8; SHA3_512.size()]),
}

/// TPM2B_NAME/TPMU_NAME
pub enum Name {
    None,
    Handle(Handle),
    Digest(Hash),
}
