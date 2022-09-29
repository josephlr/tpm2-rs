use core::mem;

use bitflags::bitflags;

use crate::{MarshalFixed, UnmarshalAny};

bitflags! {
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
    #[derive(Default)]
    #[repr(transparent)]
    pub struct Memory: u32 {
        const SHARED_RAM = 1 << 0;
        const SHARED_NV = 1 << 1;
        const OBJECT_COPIED_TO_RAM = 1 << 2;

        const RESERVED = !(0b111);
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

    impl UnmarshalAny for $T {
        fn unmarshal_fixed(&mut self, arr: &Self::ARRAY) {
            *self = Self::unmarshal_fixed_val(arr);
        }
        fn unmarshal_fixed_val(arr: &Self::ARRAY) -> Self {
            Self::from_bits_truncate(<_ as UnmarshalAny>::unmarshal_fixed_val(arr))
        }
    }
)+ } }

impl_bitflags!(Session Memory);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn all_bits_are_defined() {
        assert_eq!(Session::all().bits(), u8::MAX);
        assert_eq!(Memory::all().bits(), u32::MAX);
    }
}
