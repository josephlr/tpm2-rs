//! Types and helper functions for Marshalling and Unmarshalling

use core::mem;

use crate::error::{MarshalError, UnmarshalError};

pub trait Marshal {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError>;
}

pub trait Unmarshal<'a> {
    fn unmarshal(&mut self, buf: &mut &'a [u8]) -> Result<(), UnmarshalError>;
    fn unmarshal_val(buf: &mut &'a [u8]) -> Result<Self, UnmarshalError>
    where
        Self: Default,
    {
        let mut val = Self::default();
        val.unmarshal(buf)?;
        Ok(val)
    }
}

pub trait MarshalFixed {
    const SIZE: usize;
    // Ideally this would just be [u8; Self::SIZE], but that's not stable yet.
    type ARRAY: AsMut<[u8]> + AsRef<[u8]> + Default;
    fn marshal_fixed(&self, arr: &mut Self::ARRAY);
}

pub trait UnmarshalAny: MarshalFixed {
    fn unmarshal_fixed(arr: &Self::ARRAY) -> Self;
}

mod sealed {
    use super::*;
    /// The object-safe supertrait of [`Command`]
    pub trait CommandData {
        fn marshal_handles(&self, _: &mut &mut [u8]) -> Result<(), MarshalError> {
            Ok(())
        }
        fn marshal_params(&self, _: &mut &mut [u8]) -> Result<(), MarshalError> {
            Ok(())
        }
    }

    /// The object-safe supertrait of [`Response`](Command::Response)
    pub trait ResponseData<'b> {
        fn unmarshal_handles(&mut self, _: &mut &'b [u8]) -> Result<(), UnmarshalError> {
            Ok(())
        }
        fn unmarshal_params(&mut self, _: &mut &'b [u8]) -> Result<(), UnmarshalError> {
            Ok(())
        }
    }
    impl ResponseData<'_> for () {}
}
pub(crate) use sealed::*;

#[inline]
pub(crate) fn pop_array<'a, const N: usize>(
    buf: &mut &'a [u8],
) -> Result<&'a [u8; N], UnmarshalError> {
    if buf.len() < N {
        return Err(UnmarshalError::BufferOverflow);
    }
    let (arr, suffix) = buf.split_array_ref();
    *buf = suffix;
    Ok(arr)
}

#[inline]
pub(crate) fn pop_array_mut<'a, const N: usize>(
    buf: &mut &'a mut [u8],
) -> Result<&'a mut [u8; N], MarshalError> {
    if buf.len() < N {
        return Err(MarshalError::BufferOverflow);
    }
    let old = mem::take(buf);
    let (arr, suffix) = old.split_array_mut();
    *buf = suffix;
    Ok(arr)
}

#[inline]
pub(crate) fn pop_slice<'a>(n: usize, buf: &mut &'a [u8]) -> Result<&'a [u8], UnmarshalError> {
    if buf.len() < n {
        return Err(UnmarshalError::BufferOverflow);
    }
    let tmp = *buf;
    let (prefix, suffix) = tmp.split_at(n);
    *buf = suffix;
    Ok(prefix)
}

#[inline]
pub(crate) fn pop_slice_mut<'a>(
    n: usize,
    buf: &mut &'a mut [u8],
) -> Result<&'a mut [u8], MarshalError> {
    if buf.len() < n {
        return Err(MarshalError::BufferOverflow);
    }
    let tmp = mem::take(buf);
    let (prefix, suffix) = tmp.split_at_mut(n);
    *buf = suffix;
    Ok(prefix)
}

impl<const N: usize, T: MarshalFixed<ARRAY = [u8; N]>> Marshal for T {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
        let arr = pop_array_mut(buf)?;
        self.marshal_fixed(arr);
        Ok(())
    }
}

impl<const N: usize, T: UnmarshalAny<ARRAY = [u8; N]>> Unmarshal<'_> for T {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        let arr = pop_array(buf)?;
        *self = Self::unmarshal_fixed(arr);
        Ok(())
    }
    fn unmarshal_val(buf: &mut &[u8]) -> Result<Self, UnmarshalError> {
        let arr = pop_array(buf)?;
        Ok(Self::unmarshal_fixed(arr))
    }
}

impl MarshalFixed for () {
    const SIZE: usize = 0;
    type ARRAY = [u8; 0];
    fn marshal_fixed(&self, _: &mut Self::ARRAY) {}
}
impl UnmarshalAny for () {
    fn unmarshal_fixed(_: &Self::ARRAY) -> Self {}
}

macro_rules! impl_ints { ($($T: ty)+) => { $(
    impl MarshalFixed for $T {
        const SIZE: usize = mem::size_of::<Self>();
        type ARRAY = [u8; Self::SIZE];
        fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
            *arr = self.to_be_bytes();
        }
    }
    impl UnmarshalAny for $T {
        fn unmarshal_fixed(arr: &Self::ARRAY) -> Self {
            Self::from_be_bytes(*arr)
        }
    }
)+ } }

impl_ints!(u8 u16 u32 u64);

impl MarshalFixed for bool {
    const SIZE: usize = <u8 as MarshalFixed>::SIZE;
    type ARRAY = [u8; Self::SIZE];
    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        u8::from(*self).marshal_fixed(arr)
    }
}

impl Unmarshal<'_> for bool {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        *self = match u8::unmarshal_val(buf)? {
            0 => false,
            1 => true,
            _ => return Err(UnmarshalError::InvalidValue),
        };
        Ok(())
    }
}

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

#[cfg(feature = "alloc")]
#[doc(cfg(feature = "alloc"))]
mod impl_vec {
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

        let _: &dyn CommandData;
        let _: &dyn ResponseData;
        let _: &dyn ResponseData<'static>;

        trait Bar: for<'a> ResponseData<'static> {}
        let _: &dyn Bar;
    }
}
