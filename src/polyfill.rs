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
