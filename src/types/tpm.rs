//! `TPM_*` Constants and Enumerated Types
//!
//! TODO explain submodule vs Enum
//! TODO explain sections (6, 7, 13, 16)
//! TODO Move Handle here?

use core::num::NonZeroU32;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{
    error::{TpmError, UnmarshalError},
    polyfill::*,
    MarshalFixed, Unmarshal, UnmarshalFixed,
};

/// TPM_RH constants
pub mod rh {
    use crate::types::Handle;
    pub const PASSWORD: Handle = 0x40000009;
}

// 5.3 Miscellaneous Types
/// TPM_KEY_BITS
pub type KeyBits = u16;

/// TPM_CC values
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, FromPrimitive)]
#[non_exhaustive]
#[repr(u32)]
pub enum CC {
    #[default]
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

impl MarshalFixed for CC {
    const SIZE: usize = <u32 as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        (*self as u32).marshal_fixed(arr)
    }
}

impl UnmarshalFixed for CC {
    fn unmarshal_fixed(arr: &Self::ARRAY) -> Self {
        let raw_u32: u32 = u32::unmarshal_fixed(arr[0..4].to_arr());
        let val: CC = FromPrimitive::from_u32(raw_u32).unwrap();
        val
    }
}

/// TPM_SU values
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[non_exhaustive]
#[repr(u16)]
pub enum SU {
    #[default]
    Clear = 0x0000,
    State = 0x0001,
}
impl MarshalFixed for SU {
    const SIZE: usize = <u16 as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        (*self as u16).marshal_fixed(arr)
    }
}

/// TPM_RC
pub type RC = Option<TpmError>;

impl MarshalFixed for RC {
    const SIZE: usize = <u32 as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        let v = match self {
            Some(TpmError(n)) => n.get(),
            None => 0,
        };
        v.marshal_fixed(arr)
    }
}

impl UnmarshalFixed for RC {
    fn unmarshal_fixed(arr: &Self::ARRAY) -> Self {
        NonZeroU32::new(u32::unmarshal_fixed(arr)).map(TpmError)
    }
}

/// TPM_ST values
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[non_exhaustive]
#[repr(u16)]
pub enum ST {
    #[default]
    NoSessions = 0x8001,
    Sessions = 0x8002,
}
impl MarshalFixed for ST {
    const SIZE: usize = <u16 as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        (*self as u16).marshal_fixed(arr)
    }
}
impl Unmarshal<'_> for ST {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        *self = match u16::unmarshal_val(buf)? {
            0x8001 => Self::NoSessions,
            0x8002 => Self::Sessions,
            _ => return Err(UnmarshalError::InvalidValue),
        };
        Ok(())
    }
}

/// TPM_ALG_ID
///
/// TODO: This isn't all the ALG_IDs
#[derive(Clone, Copy, Default, Debug, FromPrimitive, Eq, PartialEq)]
#[non_exhaustive]
#[repr(u16)]
pub enum Alg {
    Error = 0x0000,
    Rsa = 0x0001,
    Tdes = 0x0003,
    Sha1 = 0x0004,
    Hmac = 0x0005,
    Aes = 0x0006,
    Mgf1 = 0x0007,
    KeyedHash = 0x0008,
    Xor = 0x000A,
    Sha256 = 0x000B,
    Sha384 = 0x000C,
    Sha512 = 0x000D,
    #[default]
    Null = 0x0010,
    Sm3_256 = 0x0012,
    Sm4 = 0x0013,
    RsaSsa = 0x0014,
    RsaEs = 0x0015,
    RsaPss = 0x0016,
    Oaep = 0x0017,
    Ecdsa = 0x0018,
    Ecdh = 0x0019,
    Ecdaa = 0x001A,
    Sm2 = 0x001B,
    EcSchnorr = 0x001C,
    Ecmqv = 0x001D,
    Kdf1Sp800_56A = 0x0020,
    Kdf2 = 0x0021,
    Kdf1Sp800_108 = 0x0022,
    Ecc = 0x0023,
    SymCipher = 0x0025,
    Camellia = 0x0026,
    Sha3_256 = 0x0027,
    Sha3_384 = 0x0028,
    Sha3_512 = 0x0029,
    Ctr = 0x0040,
    Ofb = 0x0041,
    Cbc = 0x0042,
    Cfb = 0x0043,
    Ecb = 0x0044,
}
impl MarshalFixed for Alg {
    const SIZE: usize = <u16 as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        (*self as u16).marshal_fixed(arr)
    }
}
impl UnmarshalFixed for Alg {
    fn unmarshal_fixed(arr: &Self::ARRAY) -> Self {
        let raw_u16: u16 = u16::unmarshal_fixed(arr[0..2].to_arr());
        let val: Alg = FromPrimitive::from_u16(raw_u16).unwrap();
        val
    }
}

/// TPM_ECC_CURVE (TPMI_ECC_CURVE)
///
/// List of reistered curve identifiers
#[derive(Clone, Copy, Debug, Default, FromPrimitive)]
#[non_exhaustive]
#[repr(u16)]
pub enum EccCurve {
    #[default]
    NistP192 = 0x0001,
    NistP224 = 0x0002,
    NistP256 = 0x0003,
    NistP384 = 0x0004,
    NistP521 = 0x0005,
    BnP256 = 0x0010,
    BnP638 = 0x0011,
    Sm2P256 = 0x0020,
}
impl MarshalFixed for EccCurve {
    const SIZE: usize = <u16 as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        (*self as u16).marshal_fixed(arr)
    }
}
impl UnmarshalFixed for EccCurve {
    fn unmarshal_fixed(arr: &Self::ARRAY) -> Self {
        let raw_u16: u16 = u16::unmarshal_fixed(arr[0..2].to_arr());
        let val: EccCurve = FromPrimitive::from_u16(raw_u16).unwrap();
        val
    }
}

/// TPM_CAP (Capabilities)
///
/// Used in GetCapability calls to select type of value to be returned
#[derive(Clone, Copy, Debug, Default, FromPrimitive)]
#[non_exhaustive]
#[repr(u32)]
pub enum TpmCap {
    #[default]
    Algs = 0x0000,
    Handles = 0x0001,
    Commands = 0x0002,
    PpCommands = 0x0003,
    AuditCommands = 0x0004,
    Pcrs = 0x0005,
    TpmProperties = 0x0006,
    PcrProperties = 0x0007,
    EccCurves = 0x0008,
    AuthPolicies = 0x0009,
    VendorProperty = 0x100,
}

impl MarshalFixed for TpmCap {
    const SIZE: usize = <u32 as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        (*self as u32).marshal_fixed(arr)
    }
}

impl UnmarshalFixed for TpmCap {
    fn unmarshal_fixed(arr: &Self::ARRAY) -> Self {
        let raw = u32::unmarshal_fixed(arr);
        let val: TpmCap = FromPrimitive::from_u32(raw).unwrap();
        val
    }
}

#[derive(Clone, Copy, Debug, Default, FromPrimitive)]
#[non_exhaustive]
#[repr(u32)]
pub enum PtPcr {
    #[default]
    Save = 0x0,
    ExtendL0 = 0x1,
    ResetL0 = 0x2,
    ExtendL1 = 0x3,
    ResetL1 = 0x4,
    ExtendL2 = 0x5,
    ResetL2 = 0x6,
    ExtendL3 = 0x7,
    ResetL3 = 0x8,
    ExtendL4 = 0x9,
    ResetL4 = 0xa,
    NoIncrement = 0x11,
    DrtmReset = 0x12,
    Policy = 0x13,
    Auth = 0x14,
}
impl MarshalFixed for PtPcr {
    const SIZE: usize = <u32 as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        (*self as u32).marshal_fixed(arr)
    }
}
impl UnmarshalFixed for PtPcr {
    fn unmarshal_fixed(arr: &Self::ARRAY) -> Self {
        let raw_u32: u32 = u32::unmarshal_fixed(arr[0..4].to_arr());
        let val: PtPcr = FromPrimitive::from_u32(raw_u32).unwrap();
        val
    }
}

/// TPM_PT
#[derive(Clone, Copy, Debug, Default, FromPrimitive)]
#[non_exhaustive]
#[repr(u32)]
pub enum Pt {
    #[default]
    None = 0x0,
    FamilyIndicator = 0x100,
    Level = 0x101,
    Revision = 0x102,
    DayOfYear = 0x103,
    Year = 0x104,
    Manufacturer = 0x105,
    VendorString1 = 0x106,
    VendorString2 = 0x107,
    VendorString3 = 0x108,
    VendorString4 = 0x109,
    VendorTpmType = 0x10a,
    FirmwareVersion1 = 0x10b,
    FirmwareVersion2 = 0x10c,
    InputBuffer = 0x10d,
    HrTransientMin = 0x10e,
    HrPersistentMin = 0x10f,
    HrLoadedMin = 0x110,
    ActiveSessionsMax = 0x111,
    PcrCount = 0x112,
    PcrSelectMin = 0x113,
    ContextGapMax = 0x114,
    NvCountersMax = 0x116, // note: 0x115 is skipped
    NvIndexMax = 0x117,
    Memory = 0x118,
    ClockUpdate = 0x119,
    ContextHash = 0x11a,
    ContextSym = 0x11b,
    ContextSymSize = 0x11c,
    OrderlyCount = 0x11d,
    MaxCommandSize = 0x11e,
    MaxResponseSize = 0x11f,
    MaxDigest = 0x120,
    MaxObjectContext = 0x121,
    MaxSessionContext = 0x122,
    PsFamilyIndicator = 0x123,
    PsLevel = 0x124,
    PsRevision = 0x125,
    PsDayOfYear = 0x126,
    PsYear = 0x127,
    SplitMax = 0x128,
    TotalCommands = 0x129,
    LibraryCommands = 0x12a,
    VendorCommands = 0x12b,
    NvBufferMax = 0x12c,
    Modes = 0x12d,
    MaxCapBuffer = 0x12e,
    Permanent = 0x200,
    StartupClear = 0x201,
    HrNvIndex = 0x202,
    HrLoaded = 0x203,
    HrLoadedAvail = 0x204,
    HrActive = 0x205,
    HrActiveAvail = 0x206,
    HrTransientAvail = 0x207,
    HrPersistent = 0x208,
    HrPersistentAvail = 0x209,
    NvCounters = 0x20a,
    NvCountersAvail = 0x20b,
}
impl MarshalFixed for Pt {
    const SIZE: usize = <u32 as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        (*self as u32).marshal_fixed(arr)
    }
}
impl UnmarshalFixed for Pt {
    fn unmarshal_fixed(arr: &Self::ARRAY) -> Self {
        let raw_u32: u32 = u32::unmarshal_fixed(arr[0..4].to_arr());
        match FromPrimitive::from_u32(raw_u32) {
            Some(val) => val,
            None => panic!("{:x} is not a TpmPt", raw_u32),
        }
    }
}
