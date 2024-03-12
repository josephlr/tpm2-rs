//! `TPMA_*` Attribute Types
//!
//! Most of these types are found int Part 2, Section 8 (Attributes Structures).

use core::mem;

use bitflags::bitflags;

use crate::{MarshalFixed, UnmarshalFixed};

bitflags! {
    /// TPMA_SESSION
    #[derive(Default)]
    #[repr(transparent)]
    pub struct Session: u8 {
        const CONTINUE_SESSION = 1 << 0;
        const AUDIT_EXCLUSIVE = 1 << 1;
        const AUDIT_RESET = 1 << 2;
        const DECRYPT = 1 << 5;
        const ENCRYPT = 1 << 6;
        const AUDIT = 1 << 7;

        const RESERVED = !(0b11100111);
    }
}

bitflags! {
    /// TPMA_MEMORY
    #[derive(Default)]
    #[repr(transparent)]
    pub struct Memory: u32 {
        const SHARED_RAM = 1 << 0;
        const SHARED_NV = 1 << 1;
        const OBJECT_COPIED_TO_RAM = 1 << 2;

        const RESERVED = !(0b111);
    }
}

bitflags! {
    /// TPMA_MEMORY
    #[derive(Default)]
    #[repr(transparent)]
    pub struct Object: u32 {
        const FIXED_TPM = 1 << 1;
        const ST_CLEAR = 1 << 2;
        const FIXED_PARENT = 1 << 4;
        const SENSITIVE_DATA_ORIGIN = 1 << 5;
        const USER_WITH_AUTH = 1 << 6;
        const ADMIN_WITH_POLICY = 1 << 7;
        const NO_DA = 1 << 10;
        const ENCRYPTED_DUPLICATION = 1 << 11;
        const RESTRICTED = 1 << 16;
        const DECRYPT = 1 << 17;
        const SIGN = 1 << 18;
        const ENCRYPT = 1 << 18;
        const X509_SIGN = 1 << 19;

        const RESERVED = !(0b11110000110011110110);
    }
}

bitflags! {
    /// TPMA_ALGORITHM
    #[derive(Default)]
    #[repr(transparent)]
    pub struct Algorithm: u32 {
        const ASYMMETRIC = 1 << 0;
        const SYMMETRIC = 1 << 1;
        const HASH = 1 << 2;
        const OBJECT = 1 << 3;
        const SIGNING = 1 << 8;
        const ENCRYPTING = 1 << 9;
        const METHOD = 1 << 10;

        const RESERVED = !(0b11100001111);
    }
}

bitflags! {
    /// TPMA_CC
    #[derive(Default)]
    #[repr(transparent)]
    pub struct Cc: u32 {
        const COMMANDIDX = 0xff;
        const NV = 1 << 22;
        const EXTENSIVE = 1 << 23;
        const FLUSHED = 1 << 24;
        const CHANDLES = 7 << 25;
        const RHANDLE = 1 << 28;
        const V = 1 << 19;
    }
}

macro_rules! impl_bitflags { ($($T: ty)+) => { $(
    impl MarshalFixed for $T {
        const SIZE: usize = mem::size_of::<Self>();
        type ARRAY = [u8; Self::SIZE];
        fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
            self.bits().marshal_fixed(arr)
        }
    }

    impl UnmarshalFixed for $T {
        fn unmarshal_fixed(arr: &Self::ARRAY) -> Self {
            Self::from_bits_truncate(<_ as UnmarshalFixed>::unmarshal_fixed(arr))
        }
    }
)+ } }

impl_bitflags!(Session Memory Object Algorithm Cc);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn all_bits_are_defined() {
        assert_eq!(Session::all().bits(), u8::MAX);
        assert_eq!(Memory::all().bits(), u32::MAX);
        assert_eq!(Object::all().bits(), u32::MAX);
        assert_eq!(Algorithm::all().bits(), u32::MAX);
    }
}
