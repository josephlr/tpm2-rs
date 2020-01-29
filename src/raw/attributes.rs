use bitflags::bitflags;

bitflags! {
    /// TPMA_ALGORITHM
    pub struct AlgAttr: u32 {
        const ASYMMETRIC = 1 << 0;
        const SYMMETRIC =  1 << 1;
        const HASH =       1 << 2;
        const OBJECT =     1 << 3;
        const SIGNING =    1 << 8;
        const ENCRYPTING = 1 << 9;
        const METHOD =     1 << 10;
    }
}

bitflags! {
    /// TPMA_OBJTECT
    pub struct ObjectAttr: u32 {
        const FIXED_TPM             = 1 << 1;
        const ST_CLEAR              = 1 << 2;
        const FIXED_PARENT          = 1 << 4;
        const SENSITIVE_DATA_ORIGIN = 1 << 5;
        const USER_WITH_AUTH        = 1 << 6;
        const ADMIN_WITH_POLICY     = 1 << 7;
        const NO_DA                 = 1 << 10;
        const ENCRYPTED_DUPLICATION = 1 << 11;
        const RESTRICTED            = 1 << 16;
        const DECRYPT               = 1 << 17;
        const SIGN                  = 1 << 18;
        const ENCRYPT               = 1 << 18;
        const X509_SIGN             = 1 << 19;
    }
}

bitflags! {
    /// TPMA_SESSION
    pub struct SessionAttr: u8 {
        const CONTINUE        = 1 << 0;
        const AUDIT_EXCLUSIVE = 1 << 1;
        const AUDIT_RESET     = 1 << 2;
        const DECRYPT         = 1 << 5;
        const ENCRYPT         = 1 << 6;
        const AUDIT           = 1 << 7;
    }
}

bitflags! {
    /// TPMA_LOCALITY
    pub struct LocalityAttr: u8 {
        const ZERO  = 1 << 0;
        const ONCE  = 1 << 1;
        const TWO   = 1 << 2;
        const THREE = 1 << 3;
        const FOUR  = 1 << 4;
    }
}

bitflags! {
    /// TPMA_PERMANENT
    pub struct PermanentAttr: u32 {
        const OWNER_AUTH       = 1 << 0;
        const ENDORSEMENT_AUTH = 1 << 1;
        const LOCKOUT_AUTH     = 1 << 2;
        const DISABLE_CLEAR    = 1 << 8;
        const IN_LOCKOUT       = 1 << 9;
        const GENERATED_EPS    = 1 << 10;
    }
}

bitflags! {
    /// TPMA_STARTUP_CLEAR
    pub struct StartupClearAttr: u32 {
        const PH_ENABLE    = 1 << 0;
        const SH_ENABLE    = 1 << 1;
        const EH_ENABLE    = 1 << 2;
        const PH_ENABLE_NV = 1 << 3;
        const ORDERLY      = 1 << 31;
    }
}

bitflags! {
    /// TPMA_MEMORY
    pub struct MemoryAttr: u32 {
        const SHARED_RAM     = 1 << 0;
        const SHARED_NV      = 1 << 1;
        const COPIED_TO_RAM  = 1 << 2;
    }
}

bitflags! {
    /// TPMA_ACT
    pub struct ACTAttr: u32 {
        const SIGNALED          = 1 << 0;
        const PRESERVE_SIGNALED = 1 << 1;
    }
}

bitflags! {
    /// TPMA_NV TPM_NT
    pub struct NVAttr: u32 {
        const PLATFORM_WRITE  = 1 << 0;
        const OWNER_WRITE     = 1 << 1;
        const AUTH_WRITE      = 1 << 2;
        const POLICY_WRITE    = 1 << 3;

        // TPM_NT types
        const TYPE_COUNTER    = 0x1 << 4;
        const TYPE_BITS       = 0x2 << 4;
        const TYPE_EXTEND     = 0x4 << 4;
        const TYPE_PIN_FAIL   = 0x8 << 4;
        const TYPE_PIN_PASS   = 0x9 << 4;

        const POLICY_DELETE   = 1 << 10;
        const WRITE_LOCKED    = 1 << 11;
        const WRITE_ALL       = 1 << 12;
        const WRITE_DEFINE    = 1 << 13;
        const WRITE_STCLEAR   = 1 << 14;
        const GLOBAL_LOCK     = 1 << 15;
        const PLATFORM_READ   = 1 << 16;
        const OWNER_READ      = 1 << 17;
        const AUTH_READ       = 1 << 18;
        const POLICY_READ     = 1 << 19;
        const NO_DA           = 1 << 25;
        const ORDERLY         = 1 << 26;
        const CLEAR_STCLEAR   = 1 << 27;
        const READ_LOCKED     = 1 << 28;
        const WRITTEN         = 1 << 29;
        const PLATFORM_CREATE = 1 << 30;
        const READ_STCLEAR    = 1 << 31;
    }
}
