use core::fmt::Debug;

use crate::{
    error::{MarshalError, UnmarshalError},
    marshal::{pop_slice, pop_slice_mut},
    Marshal, Unmarshal,
};

pub trait Buffer: Marshal + Default + Debug {}

impl Marshal for &[u8] {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        let len: u16 = self.len().try_into()?;
        len.marshal(buf)?;
        pop_slice_mut(self.len(), buf)?.copy_from_slice(self);
        Ok(())
    }
}

impl<'a> Unmarshal<'a> for &'a [u8] {
    fn unmarshal(&mut self, buf: &mut &'a [u8]) -> Result<(), UnmarshalError> {
        let len: usize = u16::unmarshal_val(buf)?.into();
        *self = pop_slice(len, buf)?;
        Ok(())
    }
}

impl<'a> Buffer for &'a [u8] {}

#[cfg(feature = "alloc")]
mod vec_buffer {
    use alloc::vec::Vec;

    use super::*;

    impl Marshal for Vec<u8> {
        fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
            self.as_slice().marshal(buf)
        }
    }

    impl Unmarshal<'_> for Vec<u8> {
        fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
            let len: usize = u16::unmarshal_val(buf)?.into();
            let data = pop_slice(len, buf)?;
            self.clear();
            self.extend_from_slice(data);
            Ok(())
        }
    }

    impl Buffer for Vec<u8> {}
}
