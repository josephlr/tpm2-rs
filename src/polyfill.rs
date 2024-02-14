//! Functionality not yet in libcore

use core::mem::size_of;
const _: () = assert!(size_of::<usize>() >= 4, "usize must be at least 32 bits");

pub(crate) trait ToArr<const N: usize> {
    fn to_arr(&mut self) -> &mut [u8; N];
}

impl<const N: usize> ToArr<N> for [u8] {
    #[inline(always)]
    fn to_arr(&mut self) -> &mut [u8; N] {
        self.try_into().unwrap()
    }
}
