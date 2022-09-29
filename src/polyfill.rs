//! Functionality not yet in libcore

use core::mem::size_of;

pub(crate) trait ToUsize {
    fn to_usize(self) -> usize;
}

impl ToUsize for u32 {
    #[inline(always)]
    fn to_usize(self) -> usize {
        assert!(size_of::<usize>() >= size_of::<u32>());
        self as usize
    }
}

pub(crate) trait ToArr<const N: usize> {
    fn to_arr(&mut self) -> &mut [u8; N];
}

impl<const N: usize> ToArr<N> for [u8] {
    #[inline(always)]
    fn to_arr(&mut self) -> &mut [u8; N] {
        self.try_into().unwrap()
    }
}
