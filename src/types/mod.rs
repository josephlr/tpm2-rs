use core::mem;

use crate::{Error, Result};

mod buffer;
pub use buffer::*;

pub mod tpm;
pub mod tpma;
pub mod tpml;
pub mod tpms;
pub mod tpmt;
pub mod tpmu;

pub trait Marshal {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()>;
}

pub trait Unmarshal<'a> {
    fn unmarshal(&mut self, buf: &mut &'a [u8]) -> Result<()>;
    fn unmarshal_val(buf: &mut &'a [u8]) -> Result<Self>
    where
        Self: Default,
    {
        let mut val = Self::default();
        val.unmarshal(buf)?;
        Ok(val)
    }
}

pub trait FixedSize {
    const SIZE: usize;
}

fn unmarshal_array<'a, const N: usize>(buf: &mut &'a [u8]) -> Result<&'a [u8; N]> {
    if buf.len() < N {
        return Err(Error::UnmarshalBufferOverflow);
    }
    let tmp = *buf;
    let (prefix, suffix) = tmp.split_array_ref();
    *buf = suffix;
    Ok(prefix)
}

fn marshal_array<'a, const N: usize>(buf: &mut &'a mut [u8]) -> Result<&'a mut [u8; N]> {
    if buf.len() < N {
        return Err(Error::MarshalBufferOverflow);
    }
    let tmp = mem::take(buf);
    let (prefix, suffix) = tmp.split_array_mut();
    *buf = suffix;
    Ok(prefix)
}

fn unmarshal_slice<'a>(n: usize, buf: &mut &'a [u8]) -> Result<&'a [u8]> {
    if buf.len() < n {
        return Err(Error::UnmarshalBufferOverflow);
    }
    let tmp = *buf;
    let (prefix, suffix) = tmp.split_at(n);
    *buf = suffix;
    Ok(prefix)
}

fn marshal_slice<'a>(n: usize, buf: &mut &'a mut [u8]) -> Result<&'a mut [u8]> {
    if buf.len() < n {
        return Err(Error::MarshalBufferOverflow);
    }
    let tmp = mem::take(buf);
    let (prefix, suffix) = tmp.split_at_mut(n);
    *buf = suffix;
    Ok(prefix)
}

impl Marshal for () {
    fn marshal(&self, _: &mut &mut [u8]) -> Result<()> {
        Ok(())
    }
}

impl Unmarshal<'_> for () {
    fn unmarshal(&mut self, _: &mut &[u8]) -> Result<()> {
        Ok(())
    }
}

macro_rules! int_impls { ($($T: ty)+) => { $(
    impl Marshal for $T {
        fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
            *marshal_array(buf)? = self.to_be_bytes();
            Ok(())
        }
    }

    impl Unmarshal<'_> for $T {
        fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<()> {
            *self = Self::from_be_bytes(*unmarshal_array(buf)?);
            Ok(())
        }
        fn unmarshal_val(buf: &mut &[u8]) -> Result<Self> {
            Ok(Self::from_be_bytes(*unmarshal_array(buf)?))
        }
    }

    impl FixedSize for $T {
        const SIZE: usize = mem::size_of::<Self>();
    }
)+ } }

int_impls!(u8 u16 u32 u64);

impl Marshal for bool {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        u8::from(*self).marshal(buf)
    }
}

impl Unmarshal<'_> for bool {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<()> {
        *self = match u8::unmarshal_val(buf)? {
            0 => false,
            1 => true,
            _ => return Err(Error::UnmarshalInvalidValue),
        };
        Ok(())
    }
}

impl FixedSize for bool {
    const SIZE: usize = 1;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn object_safety() {
        let _: &dyn Marshal;
        let _: &dyn Unmarshal;
        let _: &dyn Unmarshal<'static>;

        trait Foo: for<'a> Unmarshal<'a> {}
        let _: &dyn Foo;

        // let _: &dyn FixedSize;
    }
}
