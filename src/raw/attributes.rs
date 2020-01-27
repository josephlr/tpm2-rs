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
    /// TPMA_ACT
    pub struct ACTAttr: u32 {
        const SIGNALED          = 1 << 0;
        const PRESERVE_SIGNALED = 1 << 1;
    }
}
