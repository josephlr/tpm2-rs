//! Constants (i.e. C-style enums) defined in the TPM2 Spec
use core::mem::size_of;

use super::{CommandData, ResponseData, TpmData};
use crate::{
    driver::{Read, Write},
    Error, Result,
};

/// TPM_GENERATED_VALUE (v1.55, Part 2, Section 6.2, Table 7)
static TPM_MAGIC: &[u8; 4] = b"\xffTPM";

/// TPM_ALG_ID (v1.55, Part 2, Section 6.3, Tables 8 and 9)
pub(crate) mod alg {
    /// TPMI_ALG_HASH (v1.55, Part 2, Section 9.25, Table 63)
    /// TPMI_ALG_MAC_SCHEME (v1.55, Part 2, Section 9.34, Table 72)
    #[derive(PartialEq, Eq, Clone, Copy)]
    #[repr(u16)]
    pub enum Hash {
        SHA1 = 0x0004,
        SHA256 = 0x000B,
        SHA384 = 0x000C,
        SHA512 = 0x000D,
        SM3_256 = 0x0012,
        SHA3_256 = 0x0027,
        SHA3_384 = 0x0028,
        SHA3_512 = 0x0029,
    }

    impl Hash {
        /// The maximum hash size of the supported hashes
        pub const MAX_SIZE: usize = Hash::SHA512.size();
        /// The hash algorithm's digest size
        pub const fn size(self) -> usize {
            //
            const fn eq(lhs: Hash, rhs: Hash) -> usize {
                (lhs as u16 == rhs as u16) as usize
            }
            //
            let s = 0
                + eq(self, Hash::SHA1) * 20
                + eq(self, Hash::SHA256) * 32
                + eq(self, Hash::SHA384) * 48
                + eq(self, Hash::SHA512) * 64
                + eq(self, Hash::SM3_256) * 32
                + eq(self, Hash::SHA3_256) * 32
                + eq(self, Hash::SHA3_384) * 48
                + eq(self, Hash::SHA3_512) * 64;
            // This checks that a hash case was not ommitted.
            let _ = 1 / s;
            s
        }
    }

    // TPMI_ALG_ASYM (v1.55, Part 2, Section 9.26, Table 64)
    pub enum Asym {
        RSA = 0x0001,
        ECC = 0x0023,
    }

    // TPMI_ALG_SYM_OBJECT (v1.55, Part 2, Section 9.28, Table 66)
    // Right now we also use this object to represent TPMI_ALG_SYM
    // (v1.55, Part 2, Section 9.27, Table 65). May change later.
    pub enum Sym {
        TDES = 0x0003,
        AES = 0x0006,
        SM4 = 0x0013,
        Camellia = 0x0026,
    }

    // TPMI_ALG_SYM_MODE (v1.55, Part 2, Section 9.29, Table 67)
    // TPMI_ALG_CIPHER_MODE (v1.55, Part 2, Section 9.35, Table 73)
    pub enum SymMode {
        CTR = 0x0040,
        OFB = 0x0041,
        CBC = 0x0042,
        CFB = 0x0043,
        ECB = 0x0044,
    }

    // TPMI_ALG_KDF (v1.55, Part 2, Section 9.30, Table 68)
    #[allow(non_camel_case_types)]
    pub enum Kdf {
        MGF1 = 0x0007,
        KDF1_SP800_56A = 0x0020,
        KDF2 = 0x0021,
        KDF1_SP800_108 = 0x0022,
    }

    // TPMI_ALG_KDF (v1.55, Part 2, Section 9.31, Table 69)
    pub enum SigScheme {
        HMAC = 0x0005,
        RSASSA = 0x0014,
        RSAPSS = 0x0016,
        ECDSA = 0x0018,
        ECDAA = 0x001A,
        SM2 = 0x001B,
        ECSchnorr = 0x001C,
    }

    // TPMI_ALG_KDF (v1.55, Part 2, Section 9.32, Table 70)
    pub enum KeyExchange {
        ECDH = 0x0019,
        SM2 = 0x001B,
        ECMQV = 0x001D,
    }

    // TPMI_ALG_KEYEDHASH_SCHEME (v1.55, Part 2, Section 11.1.19, Table 149)
    pub enum KeyedHashScheme {
        HMAC = 0x0005,
        XOR = 0x000A,
    }

    // TPMI_ALG_ASYM_SCHEME (v1.55, Part 2, Section 11.2.3.4, Table 163)
    pub enum AsymScheme {
        RSASSA = 0x0014,
        RSAES = 0x0015,
        RSAPSS = 0x0016,
        OAEP = 0x0017,
        ECDSA = 0x0018,
        ECDH = 0x0019,
        ECDAA = 0x001A,
        SM2 = 0x001B,
        ECSchnorr = 0x001C,
        ECMQV = 0x001D,
    }

    // TPMI_ALG_RSA_SCHEME (v1.55, Part 2, Section 11.2.4.1, Table 166)
    pub enum RsaScheme {
        RSASSA = 0x0014,
        RSAES = 0x0015,
        RSAPSS = 0x0016,
        OAEP = 0x0017,
    }

    // TPMI_ALG_RSA_DECRYPT (v1.55, Part 2, Section 11.2.4.4, Table 169)
    pub enum RsaDecrypt {
        RSAES = 0x0015,
        OAEP = 0x0017,
    }

    // TPMI_ALG_ECC_SCHEME (v1.55, Part 2, Section 11.2.5.4, Table 176)
    pub enum EccScheme {
        ECDSA = 0x0018,
        ECDH = 0x0019,
        ECDAA = 0x001A,
        SM2 = 0x001B,
        ECSchnorr = 0x001C,
        ECMQV = 0x001D,
    }

    // TPMI_ALG_PUBLIC (v1.55, Part 2, Section 12.2.2, Table 188)
    pub enum Public {
        RSA = 0x0001,
        KeyedHash = 0x0008,
        ECC = 0x0023,
        SymCipher = 0x0025,
    }
}

// TPM_ECC_CURVE (v1.55, Part 2, Section 6.4, Table 10)
#[allow(non_camel_case_types)]
pub enum ECCCurve {
    NIST_P192 = 0x0001,
    NIST_P224 = 0x0002,
    NIST_P256 = 0x0003,
    NIST_P384 = 0x0004,
    NIST_P521 = 0x0005,
    BN_P256 = 0x0010, // curve to support ECDAA
    BN_P638 = 0x0011, // curve to support ECDAA
    SM2_P256 = 0x0020,
}

// TPM_CC (v1.55, Part 2, Section 6.5, Tables 11 and 12)
#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum CommandCode {
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

pub type ResponseCode = u32;

// TPM_ST (v1.55, Part 2, Section 6.9, Table 19)
pub(crate) mod tag {
    // TPMI_ST_COMMAND_TAG (v1.55, Part 2, Section 9.33, Table 71)
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    #[repr(u16)]
    pub(crate) enum Command {
        NoSessions = 0x8001,
        Sessions = 0x8002,
    }

    // TPMI_ST_ATTEST (v1.55, Part 2, Section 10.12.7, Table 126)
    #[allow(non_camel_case_types)]
    #[derive(PartialEq, Eq, Copy, Clone)]
    #[repr(u16)]
    pub(crate) enum Attest {
        NV = 0x8014,
        CommandAudit = 0x8015,
        SessionAudit = 0x8016,
        Certify = 0x8017,
        Quote = 0x8018,
        Time = 0x8019,
        Creation = 0x801A,
        // 0x801B skipped as it was perviously assigned to TPM_ST_ATTEST_NV.
        NV_Digest = 0x801C,
    }

    // Ticket types (see v1.55, Part 2, Section 10.7)
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub(crate) enum Ticket {
        Creation = 0x8021,
        Verified = 0x8022,
        AuthSecret = 0x8023,
        HashCheck = 0x8024,
        AuthSigned = 0x8025,
    }
}

// TPM_SU (v1.55, Part 2, Section 6.10, Table 20)
#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(u16)]
pub enum StartupType {
    Clear = 0x0000,
    State = 0x0001,
}

/// TPM_CAP
pub enum Cap {}

/// TPM_HANDLE
pub struct Handle(u32);

/// TPM_RH
impl Handle {
    pub const OWNER: Self = Self(0x40000001);
    pub const NULL: Self = Self(0x40000007);
    pub const UNASSIGNED: Self = Self(0x40000008);
    pub const PW: Self = Self(0x40000009);
    pub const LOCKOUT: Self = Self(0x4000000A);
    pub const ENDORSEMENT: Self = Self(0x4000000B);
    pub const PLATFORM: Self = Self(0x4000000C);
    pub const PLATFORM_NV: Self = Self(0x4000000D);
}

//

// // GENERATED CODE BELOW

// impl TpmData for tag::Attest {
//     fn data_len(&self) -> usize {
//         size_of::<Self>()
//     }
// }

// impl CommandData for tag::Attest {
//     fn encode(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()> {
//         (*self as u16).encode(writer)
//     }
// }

impl TpmData for StartupType {
    fn data_len(&self) -> usize {
        size_of::<Self>()
    }
}

impl CommandData for StartupType {
    fn encode(&self, writer: &mut dyn Write) -> Result<()> {
        (*self as u16).encode(writer)
    }
}

impl TpmData for CommandCode {
    fn data_len(&self) -> usize {
        size_of::<Self>()
    }
}

impl CommandData for CommandCode {
    fn encode(&self, writer: &mut dyn Write) -> Result<()> {
        (*self as u32).encode(writer)
    }
}

impl TpmData for tag::Command {
    fn data_len(&self) -> usize {
        size_of::<Self>()
    }
}

impl CommandData for tag::Command {
    fn encode(&self, writer: &mut dyn Write) -> Result<()> {
        (*self as u16).encode(writer)
    }
}

impl ResponseData for tag::Command {
    fn decode(reader: &mut dyn Read) -> Result<Self> {
        match u16::decode(reader)? {
            0x8001 => Ok(tag::Command::NoSessions),
            0x8002 => Ok(tag::Command::Sessions),
            _ => Err(Error::InvalidOutputValue),
        }
    }
}
