use super::{marshal_slice, unmarshal_slice, Marshal, Unmarshal};
use crate::Result;
use core::fmt::Debug;

pub trait Buffer: Marshal + Default + Debug {}

impl Marshal for &[u8] {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        let len: u16 = self.len().try_into()?;
        len.marshal(buf)?;
        marshal_slice(self.len(), buf)?.copy_from_slice(self);
        Ok(())
    }
}

impl<'a> Unmarshal<'a> for &'a [u8] {
    fn unmarshal(&mut self, buf: &mut &'a [u8]) -> Result<()> {
        let len: usize = u16::unmarshal_val(buf)?.into();
        *self = unmarshal_slice(len, buf)?;
        Ok(())
    }
}

impl<'a> Buffer for &'a [u8] {}

#[cfg(feature = "alloc")]
mod vec_buffer {
    use super::*;
    use alloc::vec::Vec;

    impl Marshal for Vec<u8> {
        fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
            self.as_slice().marshal(buf)
        }
    }

    impl Unmarshal<'_> for Vec<u8> {
        fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<()> {
            let len: usize = u16::unmarshal_val(buf)?.into();
            let data = unmarshal_slice(len, buf)?;
            self.clear();
            self.extend_from_slice(data);
            Ok(())
        }
    }

    impl Buffer for Vec<u8> {}
}
