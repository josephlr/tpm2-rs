use super::*;
use crate::{
    types::{tpm, tpml, tpms},
    Marshal, Unmarshal,
};

/// TPM2_Startup Command
///
/// This command (and its response) are defined in the
/// TPM2 Library Specification - v1.59 - Part 3 - Section 9.3
#[derive(Default, Debug)]
pub struct Startup {
    pub startup_type: tpm::SU,
}
impl CommandData for Startup {
    fn marshal_params(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.startup_type.marshal(buf)
    }
}
impl Command for Startup {
    const CODE: tpm::CC = tpm::CC::Startup;
    type Response<'a> = ();
}

/// TPM2_Shutdown Command
///
/// This command (and its response) are defined in the
/// TPM2 Library Specification - v1.59 - Part 3 - Section 9.4
#[derive(Default, Debug)]
pub struct Shutdown {
    pub shutdown_type: tpm::SU,
}
impl CommandData for Shutdown {
    fn marshal_params(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.shutdown_type.marshal(buf)
    }
}
impl Command for Shutdown {
    const CODE: tpm::CC = tpm::CC::Shutdown;
    type Response<'a> = ();
}

// /// TPM2_SelfTest Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 10.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct SelfTest {
//     pub todo: (),
// }
// /// TPM2_SelfTest Response
// ///
// /// See [SelfTest] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct SelfTestResponse {
//     pub todo: (),
// }

// /// TPM2_IncrementalSelfTest Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 10.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct IncrementalSelfTest {
//     pub todo: (),
// }
// /// TPM2_IncrementalSelfTest Response
// ///
// /// See [IncrementalSelfTest] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct IncrementalSelfTestResponse {
//     pub todo: (),
// }

// /// TPM2_GetTestResult Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 10.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct GetTestResult {
//     pub todo: (),
// }
// /// TPM2_GetTestResult Response
// ///
// /// See [GetTestResult] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct GetTestResultResponse {
//     pub todo: (),
// }

// /// TPM2_StartAuthSession Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 11.1
// #[derive(CommandData, Command, Default, Debug)]
// pub struct StartAuthSession {
//     pub todo: (),
// }
// /// TPM2_StartAuthSession Response
// ///
// /// See [StartAuthSession] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct StartAuthSessionResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyRestart Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 11.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyRestart {
//     pub todo: (),
// }
// /// TPM2_PolicyRestart Response
// ///
// /// See [PolicyRestart] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyRestartResponse {
//     pub todo: (),
// }

// /// TPM2_Create Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 12.1
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Create {
//     pub todo: (),
// }
// /// TPM2_Create Response
// ///
// /// See [Create] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct CreateResponse {
//     pub todo: (),
// }

// /// TPM2_Load Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 12.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Load {
//     pub todo: (),
// }
// /// TPM2_Load Response
// ///
// /// See [Load] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct LoadResponse {
//     pub todo: (),
// }

// /// TPM2_LoadExternal Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 12.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct LoadExternal {
//     pub todo: (),
// }
// /// TPM2_LoadExternal Response
// ///
// /// See [LoadExternal] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct LoadExternalResponse {
//     pub todo: (),
// }

// /// TPM2_ReadPublic Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 12.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ReadPublic {
//     pub todo: (),
// }
// /// TPM2_ReadPublic Response
// ///
// /// See [ReadPublic] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ReadPublicResponse {
//     pub todo: (),
// }

// /// TPM2_ActivateCredential Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 12.5
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ActivateCredential {
//     pub todo: (),
// }
// /// TPM2_ActivateCredential Response
// ///
// /// See [ActivateCredential] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ActivateCredentialResponse {
//     pub todo: (),
// }

// /// TPM2_MakeCredential Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 12.6
// #[derive(CommandData, Command, Default, Debug)]
// pub struct MakeCredential {
//     pub todo: (),
// }
// /// TPM2_MakeCredential Response
// ///
// /// See [MakeCredential] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct MakeCredentialResponse {
//     pub todo: (),
// }

// /// TPM2_Unseal Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 12.7
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Unseal {
//     pub todo: (),
// }
// /// TPM2_Unseal Response
// ///
// /// See [Unseal] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct UnsealResponse {
//     pub todo: (),
// }

// /// TPM2_ObjectChangeAuth Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 12.8
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ObjectChangeAuth {
//     pub todo: (),
// }
// /// TPM2_ObjectChangeAuth Response
// ///
// /// See [ObjectChangeAuth] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ObjectChangeAuthResponse {
//     pub todo: (),
// }

// /// TPM2_CreateLoaded Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 12.9
// #[derive(CommandData, Command, Default, Debug)]
// pub struct CreateLoaded {
//     pub todo: (),
// }
// /// TPM2_CreateLoaded Response
// ///
// /// See [CreateLoaded] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct CreateLoadedResponse {
//     pub todo: (),
// }

// /// TPM2_Duplicate Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 13.1
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Duplicate {
//     pub todo: (),
// }
// /// TPM2_Duplicate Response
// ///
// /// See [Duplicate] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct DuplicateResponse {
//     pub todo: (),
// }

// /// TPM2_Rewrap Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 13.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Rewrap {
//     pub todo: (),
// }
// /// TPM2_Rewrap Response
// ///
// /// See [Rewrap] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct RewrapResponse {
//     pub todo: (),
// }

// /// TPM2_Import Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 13.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Import {
//     pub todo: (),
// }
// /// TPM2_Import Response
// ///
// /// See [Import] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ImportResponse {
//     pub todo: (),
// }

// /// TPM2_RSA_Encrypt Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 14.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct RsaEncrypt {
//     pub todo: (),
// }
// /// TPM2_RSA_Encrypt Response
// ///
// /// See [RsaEncrypt] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct RsaEncryptResponse {
//     pub todo: (),
// }

// /// TPM2_RSA_Decrypt Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 14.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct RsaDecrypt {
//     pub todo: (),
// }
// /// TPM2_RSA_Decrypt Response
// ///
// /// See [RsaDecrypt] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct RsaDecryptResponse {
//     pub todo: (),
// }

// /// TPM2_ECDH_KeyGen Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 14.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct EcdhKeyGen {
//     pub todo: (),
// }
// /// TPM2_ECDH_KeyGen Response
// ///
// /// See [EcdhKeyGen] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct EcdhKeyGenResponse {
//     pub todo: (),
// }

// /// TPM2_ECDH_ZGen Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 14.5
// #[derive(CommandData, Command, Default, Debug)]
// pub struct EcdhZGen {
//     pub todo: (),
// }
// /// TPM2_ECDH_ZGen Response
// ///
// /// See [EcdhZGen] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct EcdhZGenResponse {
//     pub todo: (),
// }

// /// TPM2_ECC_Parameters Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 14.6
// #[derive(CommandData, Command, Default, Debug)]
// pub struct EccParameters {
//     pub todo: (),
// }
// /// TPM2_ECC_Parameters Response
// ///
// /// See [EccParameters] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct EccParametersResponse {
//     pub todo: (),
// }

// /// TPM2_ZGen_2Phase Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 14.7
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ZGen2Phase {
//     pub todo: (),
// }
// /// TPM2_ZGen_2Phase Response
// ///
// /// See [ZGen2Phase] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ZGen2PhaseResponse {
//     pub todo: (),
// }

// /// TPM2_EncryptDecrypt Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 15.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct EncryptDecrypt {
//     pub todo: (),
// }
// /// TPM2_EncryptDecrypt Response
// ///
// /// See [EncryptDecrypt] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct EncryptDecryptResponse {
//     pub todo: (),
// }

// /// TPM2_EncryptDecrypt2 Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 15.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct EncryptDecrypt2 {
//     pub todo: (),
// }
// /// TPM2_EncryptDecrypt2 Response
// ///
// /// See [EncryptDecrypt2] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct EncryptDecrypt2Response {
//     pub todo: (),
// }

// /// TPM2_Hash Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 15.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Hash {
//     pub todo: (),
// }
// /// TPM2_Hash Response
// ///
// /// See [Hash] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct HashResponse {
//     pub todo: (),
// }

// /// TPM2_MAC Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 15.6
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Mac {
//     pub todo: (),
// }
// /// TPM2_MAC Response
// ///
// /// See [Mac] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct MacResponse {
//     pub todo: (),
// }

/// TPM2_GetRandom Command
///
/// This command (and its response) are defined in the
/// TPM2 Library Specification - v1.59 - Part 3 - Section 16.1
#[derive(Default, Debug)]
pub struct GetRandom {
    pub bytes_requested: u16,
}
/// TPM2_GetRandom Response
///
/// See [GetRandom] for more information.
#[derive(Default, Debug)]
pub struct GetRandomResponse<'a> {
    pub random_bytes: &'a [u8],
}
impl CommandData for GetRandom {
    fn marshal_params(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.bytes_requested.marshal(buf)
    }
}
impl Command for GetRandom {
    const CODE: tpm::CC = tpm::CC::GetRandom;
    type Response<'a> = GetRandomResponse<'a>;
}
impl<'a> ResponseData<'a> for GetRandomResponse<'a> {
    fn unmarshal_params(&mut self, buf: &mut &'a [u8]) -> Result<(), UnmarshalError> {
        self.random_bytes.unmarshal(buf)
    }
}

// /// TPM2_StirRandom Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 16.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct StirRandom {
//     pub todo: (),
// }
// /// TPM2_StirRandom Response
// ///
// /// See [StirRandom] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct StirRandomResponse {
//     pub todo: (),
// }

// /// TPM2_MAC_Start Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 17.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct MacStart {
//     pub todo: (),
// }
// /// TPM2_MAC_Start Response
// ///
// /// See [MacStart] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct MacStartResponse {
//     pub todo: (),
// }

// /// TPM2_HashSequenceStart Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 17.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct HashSequenceStart {
//     pub todo: (),
// }
// /// TPM2_HashSequenceStart Response
// ///
// /// See [HashSequenceStart] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct HashSequenceStartResponse {
//     pub todo: (),
// }

// /// TPM2_SequenceUpdate Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 17.5
// #[derive(CommandData, Command, Default, Debug)]
// pub struct SequenceUpdate {
//     pub todo: (),
// }
// /// TPM2_SequenceUpdate Response
// ///
// /// See [SequenceUpdate] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct SequenceUpdateResponse {
//     pub todo: (),
// }

// /// TPM2_SequenceComplete Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 17.6
// #[derive(CommandData, Command, Default, Debug)]
// pub struct SequenceComplete {
//     pub todo: (),
// }
// /// TPM2_SequenceComplete Response
// ///
// /// See [SequenceComplete] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct SequenceCompleteResponse {
//     pub todo: (),
// }

// /// TPM2_EventSequenceComplete Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 17.7
// #[derive(CommandData, Command, Default, Debug)]
// pub struct EventSequenceComplete {
//     pub todo: (),
// }
// /// TPM2_EventSequenceComplete Response
// ///
// /// See [EventSequenceComplete] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct EventSequenceCompleteResponse {
//     pub todo: (),
// }

// /// TPM2_Certify Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 18.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Certify {
//     pub todo: (),
// }
// /// TPM2_Certify Response
// ///
// /// See [Certify] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct CertifyResponse {
//     pub todo: (),
// }

// /// TPM2_CertifyCreation Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 18.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct CertifyCreation {
//     pub todo: (),
// }
// /// TPM2_CertifyCreation Response
// ///
// /// See [CertifyCreation] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct CertifyCreationResponse {
//     pub todo: (),
// }

// /// TPM2_Quote Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 18.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Quote {
//     pub todo: (),
// }
// /// TPM2_Quote Response
// ///
// /// See [Quote] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct QuoteResponse {
//     pub todo: (),
// }

// /// TPM2_GetSessionAuditDigest Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 18.5
// #[derive(CommandData, Command, Default, Debug)]
// pub struct GetSessionAuditDigest {
//     pub todo: (),
// }
// /// TPM2_GetSessionAuditDigest Response
// ///
// /// See [GetSessionAuditDigest] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct GetSessionAuditDigestResponse {
//     pub todo: (),
// }

// /// TPM2_GetCommandAuditDigest Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 18.6
// #[derive(CommandData, Command, Default, Debug)]
// pub struct GetCommandAuditDigest {
//     pub todo: (),
// }
// /// TPM2_GetCommandAuditDigest Response
// ///
// /// See [GetCommandAuditDigest] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct GetCommandAuditDigestResponse {
//     pub todo: (),
// }

// /// TPM2_GetTime Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 18.7
// #[derive(CommandData, Command, Default, Debug)]
// pub struct GetTime {
//     pub todo: (),
// }
// /// TPM2_GetTime Response
// ///
// /// See [GetTime] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct GetTimeResponse {
//     pub todo: (),
// }

// /// TPM2_CertifyX509 Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 18.8
// #[derive(CommandData, Command, Default, Debug)]
// pub struct CertifyX509 {
//     pub todo: (),
// }
// /// TPM2_CertifyX509 Response
// ///
// /// See [CertifyX509] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct CertifyX509Response {
//     pub todo: (),
// }

// /// TPM2_Commit Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 19.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Commit {
//     pub todo: (),
// }
// /// TPM2_Commit Response
// ///
// /// See [Commit] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct CommitResponse {
//     pub todo: (),
// }

// /// TPM2_EC_Ephemeral Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 19.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct EcEphemeral {
//     pub todo: (),
// }
// /// TPM2_EC_Ephemeral Response
// ///
// /// See [EcEphemeral] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct EcEphemeralResponse {
//     pub todo: (),
// }

// /// TPM2_VerifySignature Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 20.1
// #[derive(CommandData, Command, Default, Debug)]
// pub struct VerifySignature {
//     pub todo: (),
// }
// /// TPM2_VerifySignature Response
// ///
// /// See [VerifySignature] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct VerifySignatureResponse {
//     pub todo: (),
// }

// /// TPM2_Sign Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 20.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Sign {
//     pub todo: (),
// }
// /// TPM2_Sign Response
// ///
// /// See [Sign] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct SignResponse {
//     pub todo: (),
// }

// /// TPM2_SetCommandCodeAuditStatus Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 21.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct SetCommandCodeAuditStatus {
//     pub todo: (),
// }
// /// TPM2_SetCommandCodeAuditStatus Response
// ///
// /// See [SetCommandCodeAuditStatus] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct SetCommandCodeAuditStatusResponse {
//     pub todo: (),
// }

// /// TPM2_PCR_Extend Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 22.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PcrExtend {
//     pub todo: (),
// }
// /// TPM2_PCR_Extend Response
// ///
// /// See [PcrExtend] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PcrExtendResponse {
//     pub todo: (),
// }

// /// TPM2_PCR_Event Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 22.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PcrEvent {
//     pub todo: (),
// }
// /// TPM2_PCR_Event Response
// ///
// /// See [PcrEvent] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PcrEventResponse {
//     pub todo: (),
// }

/// TPM2_PCR_Read Command
///
/// This command (and its response) are defined in the
/// TPM2 Library Specification - v1.59 - Part 3 - Section 22.4
#[derive(Default, Debug)]
pub struct PcrRead<'a> {
    pub pcr_selection: tpml::PcrSelection<'a>,
}
impl CommandData for PcrRead<'_> {
    fn marshal_params(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        self.pcr_selection.marshal(buf)
    }
}
impl Command for PcrRead<'_> {
    const CODE: tpm::CC = tpm::CC::PcrRead;
    type Response<'a> = PcrReadResponse<'a>;
}

/// TPM2_PCR_Read Response
///
/// See [PcrRead] for more information.
#[derive(Default, Debug)]
pub struct PcrReadResponse<'a> {
    pub pcr_update_counter: u32,
    pub pcr_selection: tpml::PcrSelection<'a>,
    pub pcr_values: tpml::Digest<'a>,
}
impl<'a> ResponseData<'a> for PcrReadResponse<'a> {
    fn unmarshal_params(&mut self, buf: &mut &'a [u8]) -> Result<(), UnmarshalError> {
        self.pcr_update_counter.unmarshal(buf)?;
        self.pcr_selection.unmarshal(buf)?;
        self.pcr_values.unmarshal(buf)
    }
}

// /// TPM2_PCR_Allocate Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 22.5
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PcrAllocate {
//     pub todo: (),
// }
// /// TPM2_PCR_Allocate Response
// ///
// /// See [PcrAllocate] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PcrAllocateResponse {
//     pub todo: (),
// }

// /// TPM2_PCR_SetAuthPolicy Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 22.6
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PcrSetAuthPolicy {
//     pub todo: (),
// }
// /// TPM2_PCR_SetAuthPolicy Response
// ///
// /// See [PcrSetAuthPolicy] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PcrSetAuthPolicyResponse {
//     pub todo: (),
// }

// /// TPM2_PCR_SetAuthValue Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 22.7
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PcrSetAuthValue {
//     pub todo: (),
// }
// /// TPM2_PCR_SetAuthValue Response
// ///
// /// See [PcrSetAuthValue] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PcrSetAuthValueResponse {
//     pub todo: (),
// }

// /// TPM2_PCR_Reset Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 22.8
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PcrReset {
//     pub todo: (),
// }
// /// TPM2_PCR_Reset Response
// ///
// /// See [PcrReset] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PcrResetResponse {
//     pub todo: (),
// }

// /// TPM2_PolicySigned Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicySigned {
//     pub todo: (),
// }
// /// TPM2_PolicySigned Response
// ///
// /// See [PolicySigned] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicySignedResponse {
//     pub todo: (),
// }

// /// TPM2_PolicySecret Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicySecret {
//     pub todo: (),
// }
// /// TPM2_PolicySecret Response
// ///
// /// See [PolicySecret] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicySecretResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyTicket Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.5
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyTicket {
//     pub todo: (),
// }
// /// TPM2_PolicyTicket Response
// ///
// /// See [PolicyTicket] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyTicketResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyOR Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.6
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyOR {
//     pub todo: (),
// }
// /// TPM2_PolicyOR Response
// ///
// /// See [PolicyOR] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyORResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyPCR Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.7
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyPcr {
//     pub todo: (),
// }
// /// TPM2_PolicyPCR Response
// ///
// /// See [PolicyPcr] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyPcrResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyLocality Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.8
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyLocality {
//     pub todo: (),
// }
// /// TPM2_PolicyLocality Response
// ///
// /// See [PolicyLocality] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyLocalityResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyNV Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.9
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyNv {
//     pub todo: (),
// }
// /// TPM2_PolicyNV Response
// ///
// /// See [PolicyNv] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyNvResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyCounterTimer Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.10
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyCounterTimer {
//     pub todo: (),
// }
// /// TPM2_PolicyCounterTimer Response
// ///
// /// See [PolicyCounterTimer] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyCounterTimerResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyCommandCode Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.11
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyCommandCode {
//     pub todo: (),
// }
// /// TPM2_PolicyCommandCode Response
// ///
// /// See [PolicyCommandCode] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyCommandCodeResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyPhysicalPresence Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.12
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyPhysicalPresence {
//     pub todo: (),
// }
// /// TPM2_PolicyPhysicalPresence Response
// ///
// /// See [PolicyPhysicalPresence] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyPhysicalPresenceResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyCpHash Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.13
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyCpHash {
//     pub todo: (),
// }
// /// TPM2_PolicyCpHash Response
// ///
// /// See [PolicyCpHash] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyCpHashResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyNameHash Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.14
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyNameHash {
//     pub todo: (),
// }
// /// TPM2_PolicyNameHash Response
// ///
// /// See [PolicyNameHash] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyNameHashResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyDuplicationSelect Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.15
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyDuplicationSelect {
//     pub todo: (),
// }
// /// TPM2_PolicyDuplicationSelect Response
// ///
// /// See [PolicyDuplicationSelect] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyDuplicationSelectResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyAuthorize Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.16
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyAuthorize {
//     pub todo: (),
// }
// /// TPM2_PolicyAuthorize Response
// ///
// /// See [PolicyAuthorize] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyAuthorizeResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyAuthValue Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.17
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyAuthValue {
//     pub todo: (),
// }
// /// TPM2_PolicyAuthValue Response
// ///
// /// See [PolicyAuthValue] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyAuthValueResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyPassword Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.18
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyPassword {
//     pub todo: (),
// }
// /// TPM2_PolicyPassword Response
// ///
// /// See [PolicyPassword] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyPasswordResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyGetDigest Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.19
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyGetDigest {
//     pub todo: (),
// }
// /// TPM2_PolicyGetDigest Response
// ///
// /// See [PolicyGetDigest] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyGetDigestResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyNvWritten Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.20
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyNvWritten {
//     pub todo: (),
// }
// /// TPM2_PolicyNvWritten Response
// ///
// /// See [PolicyNvWritten] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyNvWrittenResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyTemplate Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.21
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyTemplate {
//     pub todo: (),
// }
// /// TPM2_PolicyTemplate Response
// ///
// /// See [PolicyTemplate] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyTemplateResponse {
//     pub todo: (),
// }

// /// TPM2_PolicyAuthorizeNV Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 23.22
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyAuthorizeNv {
//     pub todo: (),
// }
// /// TPM2_PolicyAuthorizeNV Response
// ///
// /// See [PolicyAuthorizeNv] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyAuthorizeNvResponse {
//     pub todo: (),
// }

// /// TPM2_CreatePrimary Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 24.1
// #[derive(CommandData, Command, Default, Debug)]
// pub struct CreatePrimary {
//     pub todo: (),
// }
// /// TPM2_CreatePrimary Response
// ///
// /// See [CreatePrimary] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct CreatePrimaryResponse {
//     pub todo: (),
// }

// /// TPM2_HierarchyControl Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 24.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct HierarchyControl {
//     pub todo: (),
// }
// /// TPM2_HierarchyControl Response
// ///
// /// See [HierarchyControl] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct HierarchyControlResponse {
//     pub todo: (),
// }

// /// TPM2_SetPrimaryPolicy Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 24.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct SetPrimaryPolicy {
//     pub todo: (),
// }
// /// TPM2_SetPrimaryPolicy Response
// ///
// /// See [SetPrimaryPolicy] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct SetPrimaryPolicyResponse {
//     pub todo: (),
// }

// /// TPM2_ChangePPS Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 24.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ChangePps {
//     pub todo: (),
// }
// /// TPM2_ChangePPS Response
// ///
// /// See [ChangePps] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ChangePpsResponse {
//     pub todo: (),
// }

// /// TPM2_ChangeEPS Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 24.5
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ChangeEps {
//     pub todo: (),
// }
// /// TPM2_ChangeEPS Response
// ///
// /// See [ChangeEps] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ChangeEpsResponse {
//     pub todo: (),
// }

// /// TPM2_Clear Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 24.6
// #[derive(CommandData, Command, Default, Debug)]
// pub struct Clear {
//     pub todo: (),
// }
// /// TPM2_Clear Response
// ///
// /// See [Clear] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ClearResponse {
//     pub todo: (),
// }

// /// TPM2_ClearControl Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 24.7
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ClearControl {
//     pub todo: (),
// }
// /// TPM2_ClearControl Response
// ///
// /// See [ClearControl] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ClearControlResponse {
//     pub todo: (),
// }

// /// TPM2_HierarchyChangeAuth Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 24.8
// #[derive(CommandData, Command, Default, Debug)]
// pub struct HierarchyChangeAuth {
//     pub todo: (),
// }
// /// TPM2_HierarchyChangeAuth Response
// ///
// /// See [HierarchyChangeAuth] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct HierarchyChangeAuthResponse {
//     pub todo: (),
// }

// /// TPM2_DictionaryAttackLockReset Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 25.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct DictionaryAttackLockReset {
//     pub todo: (),
// }
// /// TPM2_DictionaryAttackLockReset Response
// ///
// /// See [DictionaryAttackLockReset] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct DictionaryAttackLockResetResponse {
//     pub todo: (),
// }

// /// TPM2_DictionaryAttackParameters Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 25.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct DictionaryAttackParameters {
//     pub todo: (),
// }
// /// TPM2_DictionaryAttackParameters Response
// ///
// /// See [DictionaryAttackParameters] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct DictionaryAttackParametersResponse {
//     pub todo: (),
// }

// /// TPM2_PP_Commands Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 26.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PpCommands {
//     pub todo: (),
// }
// /// TPM2_PP_Commands Response
// ///
// /// See [PpCommands] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PpCommandsResponse {
//     pub todo: (),
// }

// /// TPM2_SetAlgorithmSet Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 26.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct SetAlgorithmSet {
//     pub todo: (),
// }
// /// TPM2_SetAlgorithmSet Response
// ///
// /// See [SetAlgorithmSet] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct SetAlgorithmSetResponse {
//     pub todo: (),
// }

// /// TPM2_FieldUpgradeStart Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 27.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct FieldUpgradeStart {
//     pub todo: (),
// }
// /// TPM2_FieldUpgradeStart Response
// ///
// /// See [FieldUpgradeStart] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct FieldUpgradeStartResponse {
//     pub todo: (),
// }

// /// TPM2_FieldUpgradeData Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 27.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct FieldUpgradeData {
//     pub todo: (),
// }
// /// TPM2_FieldUpgradeData Response
// ///
// /// See [FieldUpgradeData] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct FieldUpgradeDataResponse {
//     pub todo: (),
// }

// /// TPM2_FirmwareRead Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 27.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct FirmwareRead {
//     pub todo: (),
// }
// /// TPM2_FirmwareRead Response
// ///
// /// See [FirmwareRead] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct FirmwareReadResponse {
//     pub todo: (),
// }

// /// TPM2_ContextSave Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 28.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ContextSave {
//     pub todo: (),
// }
// /// TPM2_ContextSave Response
// ///
// /// See [ContextSave] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ContextSaveResponse {
//     pub todo: (),
// }

// /// TPM2_ContextLoad Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 28.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ContextLoad {
//     pub todo: (),
// }
// /// TPM2_ContextLoad Response
// ///
// /// See [ContextLoad] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ContextLoadResponse {
//     pub todo: (),
// }

// /// TPM2_FlushContext Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 28.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct FlushContext {
//     pub todo: (),
// }
// /// TPM2_FlushContext Response
// ///
// /// See [FlushContext] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct FlushContextResponse {
//     pub todo: (),
// }

// /// TPM2_EvictControl Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 28.5
// #[derive(CommandData, Command, Default, Debug)]
// pub struct EvictControl {
//     pub todo: (),
// }
// /// TPM2_EvictControl Response
// ///
// /// See [EvictControl] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct EvictControlResponse {
//     pub todo: (),
// }

/// TPM2_ReadClock Command
///
/// This command (and its response) are defined in the
/// TPM2 Library Specification - v1.59 - Part 3 - Section 29.1
#[derive(Default, Debug)]
pub struct ReadClock {}

impl CommandData for ReadClock {}
impl Command for ReadClock {
    const CODE: tpm::CC = tpm::CC::ReadClock;
    type Response<'a> = ReadClockResponse;
}

/// TPM2_ReadClock Response
///
/// See [ReadClock] for more information.
#[derive(Default, Debug)]
pub struct ReadClockResponse {
    pub current_time: tpms::TimeInfo,
}
impl ResponseData<'_> for ReadClockResponse {
    fn unmarshal_params(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        self.current_time.unmarshal(buf)
    }
}

// /// TPM2_ClockSet Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 29.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ClockSet {
//     pub todo: (),
// }
// /// TPM2_ClockSet Response
// ///
// /// See [ClockSet] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ClockSetResponse {
//     pub todo: (),
// }

// /// TPM2_ClockRateAdjust Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 29.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ClockRateAdjust {
//     pub todo: (),
// }
// /// TPM2_ClockRateAdjust Response
// ///
// /// See [ClockRateAdjust] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ClockRateAdjustResponse {
//     pub todo: (),
// }

// /// TPM2_GetCapability Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 30.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct GetCapability {
//     pub todo: (),
// }
// /// TPM2_GetCapability Response
// ///
// /// See [GetCapability] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct GetCapabilityResponse {
//     pub todo: (),
// }

// /// TPM2_TestParms Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 30.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct TestParms {
//     pub todo: (),
// }
// /// TPM2_TestParms Response
// ///
// /// See [TestParms] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct TestParmsResponse {
//     pub todo: (),
// }

// /// TPM2_NV_DefineSpace Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvDefineSpace {
//     pub todo: (),
// }
// /// TPM2_NV_DefineSpace Response
// ///
// /// See [NvDefineSpace] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvDefineSpaceResponse {
//     pub todo: (),
// }

// /// TPM2_NV_UndefineSpace Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvUndefineSpace {
//     pub todo: (),
// }
// /// TPM2_NV_UndefineSpace Response
// ///
// /// See [NvUndefineSpace] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvUndefineSpaceResponse {
//     pub todo: (),
// }

// /// TPM2_NV_UndefineSpaceSpecial Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.5
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvUndefineSpaceSpecial {
//     pub todo: (),
// }
// /// TPM2_NV_UndefineSpaceSpecial Response
// ///
// /// See [NvUndefineSpaceSpecial] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvUndefineSpaceSpecialResponse {
//     pub todo: (),
// }

// /// TPM2_NV_ReadPublic Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.6
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvReadPublic {
//     pub todo: (),
// }
// /// TPM2_NV_ReadPublic Response
// ///
// /// See [NvReadPublic] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvReadPublicResponse {
//     pub todo: (),
// }

// /// TPM2_NV_Write Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.7
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvWrite {
//     pub todo: (),
// }
// /// TPM2_NV_Write Response
// ///
// /// See [NvWrite] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvWriteResponse {
//     pub todo: (),
// }

// /// TPM2_NV_Increment Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.8
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvIncrement {
//     pub todo: (),
// }
// /// TPM2_NV_Increment Response
// ///
// /// See [NvIncrement] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvIncrementResponse {
//     pub todo: (),
// }

// /// TPM2_NV_Extend Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.9
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvExtend {
//     pub todo: (),
// }
// /// TPM2_NV_Extend Response
// ///
// /// See [NvExtend] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvExtendResponse {
//     pub todo: (),
// }

// /// TPM2_NV_SetBits Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.10
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvSetBits {
//     pub todo: (),
// }
// /// TPM2_NV_SetBits Response
// ///
// /// See [NvSetBits] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvSetBitsResponse {
//     pub todo: (),
// }

// /// TPM2_NV_WriteLock Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.11
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvWriteLock {
//     pub todo: (),
// }
// /// TPM2_NV_WriteLock Response
// ///
// /// See [NvWriteLock] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvWriteLockResponse {
//     pub todo: (),
// }

// /// TPM2_NV_GlobalWriteLock Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.12
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvGlobalWriteLock {
//     pub todo: (),
// }
// /// TPM2_NV_GlobalWriteLock Response
// ///
// /// See [NvGlobalWriteLock] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvGlobalWriteLockResponse {
//     pub todo: (),
// }

// /// TPM2_NV_Read Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.13
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvRead {
//     pub todo: (),
// }
// /// TPM2_NV_Read Response
// ///
// /// See [NvRead] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvReadResponse {
//     pub todo: (),
// }

// /// TPM2_NV_ReadLock Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.14
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvReadLock {
//     pub todo: (),
// }
// /// TPM2_NV_ReadLock Response
// ///
// /// See [NvReadLock] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvReadLockResponse {
//     pub todo: (),
// }

// /// TPM2_NV_ChangeAuth Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.15
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvChangeAuth {
//     pub todo: (),
// }
// /// TPM2_NV_ChangeAuth Response
// ///
// /// See [NvChangeAuth] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvChangeAuthResponse {
//     pub todo: (),
// }

// /// TPM2_NV_Certify Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 31.16
// #[derive(CommandData, Command, Default, Debug)]
// pub struct NvCertify {
//     pub todo: (),
// }
// /// TPM2_NV_Certify Response
// ///
// /// See [NvCertify] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct NvCertifyResponse {
//     pub todo: (),
// }

// /// TPM2_AC_GetCapability Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 32.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct AcGetCapability {
//     pub todo: (),
// }
// /// TPM2_AC_GetCapability Response
// ///
// /// See [AcGetCapability] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct AcGetCapabilityResponse {
//     pub todo: (),
// }

// /// TPM2_AC_Send Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 32.3
// #[derive(CommandData, Command, Default, Debug)]
// pub struct AcSend {
//     pub todo: (),
// }
// /// TPM2_AC_Send Response
// ///
// /// See [AcSend] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct AcSendResponse {
//     pub todo: (),
// }

// /// TPM2_Policy_AC_SendSelect Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 32.4
// #[derive(CommandData, Command, Default, Debug)]
// pub struct PolicyAcSendSelect {
//     pub todo: (),
// }
// /// TPM2_Policy_AC_SendSelect Response
// ///
// /// See [PolicyAcSendSelect] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct PolicyAcSendSelectResponse {
//     pub todo: (),
// }

// /// TPM2_ACT_SetTimeout Command
// ///
// /// This command (and its response) are defined in the
// /// TPM2 Library Specification - v1.59 - Part 3 - Section 33.2
// #[derive(CommandData, Command, Default, Debug)]
// pub struct ActSetTimeout {
//     pub todo: (),
// }
// /// TPM2_ACT_SetTimeout Response
// ///
// /// See [ActSetTimeout] for more information.
// #[derive(ResponseData, Default, Debug)]
// pub struct ActSetTimeoutResponse {
//     pub todo: (),
// }
