// Traits and types for dealing with TPM data
use core::convert::TryInto;
use core::mem;

use crate::{Error, Result};

pub trait DataIn {
    fn into_bytes<'a>(&self, bytes: &'a mut [u8]) -> Result<&'a mut [u8]>;
}

pub trait DataOut: Sized {
    fn from_bytes(bytes: &mut &[u8]) -> Result<Self>;
}

fn check_size_in(size: usize, bytes: &mut [u8]) -> Result<(&mut [u8], &mut [u8])> {
    if bytes.len() < size {
        return Err(Error::BufTooShort);
    }
    Ok(bytes.split_at_mut(size))
}

fn check_size_out<'a>(size: usize, bytes: &mut &'a [u8]) -> Result<&'a [u8]> {
    if bytes.len() < size {
        return Err(Error::BufTooShort);
    }
    let (data, rest) = bytes.split_at(size);
    *bytes = rest;
    Ok(data)
}

impl DataIn for () {
    #[inline]
    fn into_bytes<'a>(&self, bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
        Ok(bytes)
    }
}

impl DataOut for () {
    #[inline]
    fn from_bytes(_: &mut &[u8]) -> Result<Self> {
        Ok(())
    }
}

impl DataIn for bool {
    #[inline]
    fn into_bytes<'a>(&self, bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
        (*self as u8).into_bytes(bytes)
    }
}

impl DataOut for bool {
    #[inline]
    fn from_bytes(bytes: &mut &[u8]) -> Result<Self> {
        match u8::from_bytes(bytes)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::InvalidValue),
        }
    }
}

macro_rules! int_impls { ($($T: ty)+) => { $(
    impl DataOut for $T {
        #[inline]
        fn from_bytes(bytes: &mut &[u8]) -> Result<Self> {
            let data = check_size_out(mem::size_of::<Self>(), bytes)?;
            let arr = data.try_into().unwrap();
            Ok(Self::from_be_bytes(arr))
        }
    }

    impl DataIn for $T {
        #[inline]
        fn into_bytes<'a>(&self, bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
            let (data, rest) = check_size_in(mem::size_of::<Self>(), bytes)?;
            let arr: &mut [u8; mem::size_of::<Self>()] = data.try_into().unwrap();
            *arr = self.to_be_bytes();
            Ok(rest)
        }
    }
)+ } }

int_impls! { u8 u16 u32 u64 }

impl DataIn for [u8] {
    fn into_bytes<'a>(&self, mut bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
        bytes = (self.len() as u16).into_bytes(bytes)?;
        let (data, rest) = check_size_in(self.len(), bytes)?;
        data.copy_from_slice(self);
        Ok(rest)
    }
}

#[cfg(not(feature = "alloc"))]
const MAX_BUFFER: usize = 64;

#[cfg(feature = "alloc")]
pub struct Buffer(alloc::vec::Vec<u8>);

#[cfg(not(feature = "alloc"))]
pub struct Buffer {
    size: u16,
    data: [u8; MAX_BUFFER],
}

impl Buffer {
    #[cfg(feature = "alloc")]
    fn new(data: &[u8]) -> Result<Self> {
        Ok(Buffer(data.to_vec()))
    }
    #[cfg(not(feature = "alloc"))]
    fn new(data: &[u8]) -> Result<Self> {
        if data.len() > MAX_BUFFER {
            return Err(Error::WouldAllocate);
        }
        let mut b = [0u8; MAX_BUFFER];
        b[..data.len()].copy_from_slice(data);
        Ok(Buffer {
            size: data.len() as u16,
            data: b,
        })
    }

    #[cfg(feature = "alloc")]
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
    #[cfg(not(feature = "alloc"))]
    pub fn bytes(&self) -> &[u8] {
        &self.data[..self.size as usize]
    }
}

impl DataOut for Buffer {
    fn from_bytes(bytes: &mut &[u8]) -> Result<Self> {
        let size = u16::from_bytes(bytes)? as usize;
        if size > bytes.len() {
            return Err(Error::BufTooShort);
        }
        let (data, rest) = bytes.split_at(size);
        *bytes = rest;
        Buffer::new(data)
    }
}
