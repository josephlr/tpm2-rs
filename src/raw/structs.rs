//! Structures defined in the TPM2 Spec
use super::*;

// Header for all commands (see v1.55, Part 1, Section 18)
#[derive(TpmData, CommandData)]
pub(crate) struct CommandHeader {
    pub tag: tag::Command,
    pub size: u32,
    pub code: CommandCode,
}

// Header for all respsonses (see v1.55, Part 1, Section 18)
#[derive(TpmData, ResponseData)]
pub(crate) struct ResponseHeader {
    pub tag: tag::Command,
    pub size: u32,
    pub code: ResponseCode,
}

// TPMS_CLOCK_INFO (v1.55, Part 2, Section 10.11.1, Table 116)
#[derive(Clone, Copy, Debug, TpmData, ResponseData)]
pub(crate) struct ClockInfo {
    clock: u64,
    reset_count: u32,
    restart_count: u32,
    safe: bool,
}

// TPMS_TIME_INFO (v1.55, Part 2, Section 10.11.6, Table 117)
#[derive(Clone, Copy, Debug, TpmData, ResponseData)]
pub struct TimeInfo {
    time: u64,
    clock: ClockInfo,
}

/// TPMS_ALGORITHM_DESCRIPTION
/// TPMS_ALG_PROPERTY
pub struct AlgProp {
    alg: u16,
    attr: AttrAlg,
}

/// TPMA_CC TPM_CC?
pub struct CommandProp {
    todo: u32,
}

/// TPMS_PCR_SELECTION
pub struct PCRSelection {}

/// TPMS_TAGGED_PROPERTY
pub struct TPMProp {}

/// TPMS_TAGGED_PCR_SELECT
pub struct PCRProp {}

/// TPMS_TAGGED_POLICY
pub struct PolicyProp {
    handle: Handle,
    hash: Hash,
}

/// TPMS_ACT_DATA
pub struct ACTData {
    handle: Handle,
    timeout: u32,
    attr: AttrACT,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_data_len() {
        let tag = tag::Command::NoSessions;
        let size = 0;
        let code = CommandCode::Startup;

        let ch = CommandHeader { tag, size, code };
        assert_eq!(ch.data_len(), 10);

        let rh = ResponseHeader { tag, size, code: 0 };
        assert_eq!(rh.data_len(), 10);
    }
}
