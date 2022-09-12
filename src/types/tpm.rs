//! Constants (TPM_* types)
//!
//! This module ... TODO
use super::{FixedSize, Marshal, Unmarshal};
use crate::{Error, Result};

/// TPM_CC values
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CC {
    NvUndefineSpaceSpecial = 0x0000011f,
    EvictControl = 0x00000120,
    HierarchyControl = 0x00000121,
    NvUndefineSpace = 0x00000122,
    ChangeEps = 0x00000124,
    ChangePps = 0x00000125,
    Clear = 0x00000126,
    ClearControl = 0x00000127,
    ClockSet = 0x00000128,
    HierarchyChangeAuth = 0x00000129,
    NvDefineSpace = 0x0000012a,
    PcrAllocate = 0x0000012b,
    PcrSetAuthPolicy = 0x0000012c,
    PpCommands = 0x0000012d,
    SetPrimaryPolicy = 0x0000012e,
    FieldUpgradeStart = 0x0000012f,
    ClockRateAdjust = 0x00000130,
    CreatePrimary = 0x00000131,
    NvGlobalWriteLock = 0x00000132,
    GetCommandAuditDigest = 0x00000133,
    NvIncrement = 0x00000134,
    NvSetBits = 0x00000135,
    NvExtend = 0x00000136,
    NvWrite = 0x00000137,
    NvWriteLock = 0x00000138,
    DictionaryAttackLockReset = 0x00000139,
    DictionaryAttackParameters = 0x0000013a,
    NvChangeAuth = 0x0000013b,
    PcrEvent = 0x0000013c,
    PcrReset = 0x0000013d,
    SequenceComplete = 0x0000013e,
    SetAlgorithmSet = 0x0000013f,
    SetCommandCodeAuditStatus = 0x00000140,
    FieldUpgradeData = 0x00000141,
    IncrementalSelfTest = 0x00000142,
    SelfTest = 0x00000143,
    Startup = 0x00000144,
    Shutdown = 0x00000145,
    StirRandom = 0x00000146,
    ActivateCredential = 0x00000147,
    Certify = 0x00000148,
    PolicyNv = 0x00000149,
    CertifyCreation = 0x0000014a,
    Duplicate = 0x0000014b,
    GetTime = 0x0000014c,
    GetSessionAuditDigest = 0x0000014d,
    NvRead = 0x0000014e,
    NvReadLock = 0x0000014f,
    ObjectChangeAuth = 0x00000150,
    PolicySecret = 0x00000151,
    Rewrap = 0x00000152,
    Create = 0x00000153,
    EcdhZGen = 0x00000154,
    Mac = 0x00000155,
    Import = 0x00000156,
    Load = 0x00000157,
    Quote = 0x00000158,
    RsaDecrypt = 0x00000159,
    MacStart = 0x0000015b,
    SequenceUpdate = 0x0000015c,
    Sign = 0x0000015d,
    Unseal = 0x0000015e,
    PolicySigned = 0x00000160,
    ContextLoad = 0x00000161,
    ContextSave = 0x00000162,
    EcdhKeyGen = 0x00000163,
    EncryptDecrypt = 0x00000164,
    FlushContext = 0x00000165,
    LoadExternal = 0x00000167,
    MakeCredential = 0x00000168,
    NvReadPublic = 0x00000169,
    PolicyAuthorize = 0x0000016a,
    PolicyAuthValue = 0x0000016b,
    PolicyCommandCode = 0x0000016c,
    PolicyCounterTimer = 0x0000016d,
    PolicyCpHash = 0x0000016e,
    PolicyLocality = 0x0000016f,
    PolicyNameHash = 0x00000170,
    PolicyOR = 0x00000171,
    PolicyTicket = 0x00000172,
    ReadPublic = 0x00000173,
    RsaEncrypt = 0x00000174,
    StartAuthSession = 0x00000176,
    VerifySignature = 0x00000177,
    EccParameters = 0x00000178,
    FirmwareRead = 0x00000179,
    GetCapability = 0x0000017a,
    GetRandom = 0x0000017b,
    GetTestResult = 0x0000017c,
    Hash = 0x0000017d,
    PcrRead = 0x0000017e,
    PolicyPcr = 0x0000017f,
    PolicyRestart = 0x00000180,
    ReadClock = 0x00000181,
    PcrExtend = 0x00000182,
    PcrSetAuthValue = 0x00000183,
    NvCertify = 0x00000184,
    EventSequenceComplete = 0x00000185,
    HashSequenceStart = 0x00000186,
    PolicyPhysicalPresence = 0x00000187,
    PolicyDuplicationSelect = 0x00000188,
    PolicyGetDigest = 0x00000189,
    TestParms = 0x0000018a,
    Commit = 0x0000018b,
    PolicyPassword = 0x0000018c,
    ZGen2Phase = 0x0000018d,
    EcEphemeral = 0x0000018e,
    PolicyNvWritten = 0x0000018f,
    PolicyTemplate = 0x00000190,
    CreateLoaded = 0x00000191,
    PolicyAuthorizeNv = 0x00000192,
    EncryptDecrypt2 = 0x00000193,
    AcGetCapability = 0x00000194,
    AcSend = 0x00000195,
    PolicyAcSendSelect = 0x00000196,
    CertifyX509 = 0x00000197,
    ActSetTimeout = 0x00000198,
}

impl Marshal for CC {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        (*self as u32).marshal(buf)
    }
}
impl FixedSize for CC {
    const SIZE: usize = <u32 as FixedSize>::SIZE;
}

// TPM_SU values
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[repr(u16)]
pub enum SU {
    #[default]
    Clear = 0x0000,
    State = 0x0001,
}
impl Marshal for SU {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        (*self as u16).marshal(buf)
    }
}
