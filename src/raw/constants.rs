//! Constants (i.e. C-style enums) defined in the TPM2 Spec
use super::{ReadData, Tpm, WriteData};
use crate::{Error, Result};

use super::data::DataIn;
use crate::Result;

// TPM_CC constants
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types, dead_code)]
pub(crate) enum CommandCode {
    NV_UndefineSpaceSpecial = 0x0000011F,
    EvictControl = 0x00000120,
    HierarchyControl = 0x00000121,
    NV_UndefineSpace = 0x00000122,
    ChangeEPS = 0x00000124,
    ChangePPS = 0x00000125,
    Clear = 0x00000126,
    ClearControl = 0x00000127,
    ClockSet = 0x00000128,
    HierarchyChangeAuth = 0x00000129,
    NV_DefineSpace = 0x0000012A,
    PCR_Allocate = 0x0000012B,
    PCR_SetAuthPolicy = 0x0000012C,
    PP_Commands = 0x0000012D,
    SetPrimaryPolicy = 0x0000012E,
    FieldUpgradeStart = 0x0000012F,
    ClockRateAdjust = 0x00000130,
    CreatePrimary = 0x00000131,
    NV_GlobalWriteLock = 0x00000132,
    GetCommandAuditDigest = 0x00000133,
    NV_Increment = 0x00000134,
    NV_SetBits = 0x00000135,
    NV_Extend = 0x00000136,
    NV_Write = 0x00000137,
    NV_WriteLock = 0x00000138,
    DictionaryAttackLockReset = 0x00000139,
    DictionaryAttackParameters = 0x0000013A,
    NV_ChangeAuth = 0x0000013B,
    PCR_Event = 0x0000013C,
    PCR_Reset = 0x0000013D,
    SequenceComplete = 0x0000013E,
    SetAlgorithmSet = 0x0000013F,
    SetCommandCodeAuditStatus = 0x00000140,
    FieldUpgradeData = 0x00000141,
    IncrementalSelfTest = 0x00000142,
    SelfTest = 0x00000143,
    Startup = 0x00000144,
    Shutdown = 0x00000145,
    StirRandom = 0x00000146,
    ActivateCredential = 0x00000147,
    Certify = 0x00000148,
    PolicyNV = 0x00000149,
    CertifyCreation = 0x0000014A,
    Duplicate = 0x0000014B,
    GetTime = 0x0000014C,
    GetSessionAuditDigest = 0x0000014D,
    NV_Read = 0x0000014E,
    NV_ReadLock = 0x0000014F,
    ObjectChangeAuth = 0x00000150,
    PolicySecret = 0x00000151,
    Rewrap = 0x00000152,
    Create = 0x00000153,
    ECDH_ZGen = 0x00000154,
    MAC = 0x00000155,
    Import = 0x00000156,
    Load = 0x00000157,
    Quote = 0x00000158,
    RSA_Decrypt = 0x00000159,
    MAC_Start = 0x0000015B,
    SequenceUpdate = 0x0000015C,
    Sign = 0x0000015D,
    Unseal = 0x0000015E,
    PolicySigned = 0x00000160,
    ContextLoad = 0x00000161,
    ContextSave = 0x00000162,
    ECDH_KeyGen = 0x00000163,
    EncryptDecrypt = 0x00000164,
    FlushContext = 0x00000165,
    LoadExternal = 0x00000167,
    MakeCredential = 0x00000168,
    NV_ReadPublic = 0x00000169,
    PolicyAuthorize = 0x0000016A,
    PolicyAuthValue = 0x0000016B,
    PolicyCommandCode = 0x0000016C,
    PolicyCounterTimer = 0x0000016D,
    PolicyCpHash = 0x0000016E,
    PolicyLocality = 0x0000016F,
    PolicyNameHash = 0x00000170,
    PolicyOR = 0x00000171,
    PolicyTicket = 0x00000172,
    ReadPublic = 0x00000173,
    RSA_Encrypt = 0x00000174,
    StartAuthSession = 0x00000176,
    VerifySignature = 0x00000177,
    ECC_Parameters = 0x00000178,
    FirmwareRead = 0x00000179,
    GetCapability = 0x0000017A,
    GetRandom = 0x0000017B,
    GetTestResult = 0x0000017C,
    Hash = 0x0000017D,
    PCR_Read = 0x0000017E,
    PolicyPCR = 0x0000017F,
    PolicyRestart = 0x00000180,
    ReadClock = 0x00000181,
    PCR_Extend = 0x00000182,
    PCR_SetAuthValue = 0x00000183,
    NV_Certify = 0x00000184,
    EventSequenceComplete = 0x00000185,
    HashSequenceStart = 0x00000186,
    PolicyPhysicalPresence = 0x00000187,
    PolicyDuplicationSelect = 0x00000188,
    PolicyGetDigest = 0x00000189,
    TestParms = 0x0000018A,
    Commit = 0x0000018B,
    PolicyPassword = 0x0000018C,
    ZGen_2Phase = 0x0000018D,
    EC_Ephemeral = 0x0000018E,
    PolicyNvWritten = 0x0000018F,
    PolicyTemplate = 0x00000190,
    CreateLoaded = 0x00000191,
    PolicyAuthorizeNV = 0x00000192,
    EncryptDecrypt2 = 0x00000193,
    AC_GetCapability = 0x00000194,
    AC_Send = 0x00000195,
    Policy_AC_SendSelect = 0x00000196,
    CertifyX509 = 0x00000197,
}

impl DataIn for CommandCode {
    fn into_bytes<'a>(&self, bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
        (*self as u32).into_bytes(bytes)
    }
}

// TPMI_ALG_HASH
#[repr(u16)]
#[derive(Debug)]
pub enum AlgHash {
    SHA1 = 0x0004,
    SHA256 = 0x000B,
    SHA384 = 0x000C,
    SHA512 = 0x000D,
    SM3_256 = 0x0012,
    SHA3_256 = 0x0027,
    SHA3_384 = 0x0028,
    SHA3_512 = 0x0029,
}

// TPM_SU
#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum StartupType {
    Clear = 0x0000,
    State = 0x0001,
}

impl DataIn for StartupType {
    fn into_bytes<'a>(&self, bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
        (*self as u16).into_bytes(bytes)
    }
}
