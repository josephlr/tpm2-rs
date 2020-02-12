//! Helper traits for implementing TPMs and TPM-compatible data structures
use core::{
    convert::TryInto,
    mem::{replace, size_of},
};

use crate::{Error, Result};

pub trait TpmData {
    fn data_len(&self) -> usize;
}

/// A trait for objects that can be written to a TPM command buffer
pub trait CommandData: TpmData {
    /// Serialize this object and write it to `writer`.
    fn encode(&self, cmd: &mut &mut [u8]) -> Result<()>;
}

/// A trait for creating objects from a TPM response buffer;
pub trait ResponseData: TpmData + Sized {
    fn decode(resp: &mut &[u8]) -> Result<Self>;
}

/// A helper trait to make impls nicer
pub trait ResponseRef: TpmData {
    fn decode_ref(&mut self, resp: &mut &[u8]) -> Result<()>;
}

impl<T: ResponseData> ResponseRef for T {
    fn decode_ref(&mut self, resp: &mut &[u8]) -> Result<()> {
        *self = Self::decode(resp)?;
        Ok(())
    }
}

macro_rules! data_impls { ($($T: ty)+) => { $(
    impl TpmData for $T {
        fn data_len(&self) -> usize {
            size_of::<Self>()
        }
    }
)+ } }

data_impls! { () bool u8 u16 u32 u64 }

// Rust integral types don't have a common trait, so we have to use a macro.
macro_rules! int_impls { ($($T: ty)+) => { $(
    impl CommandData for $T {
        fn encode(&self, cmd: &mut &mut [u8]) -> Result<()> {
            self.to_be_bytes().encode(cmd)
        }
    }
    impl ResponseData for $T {
        fn decode(resp: &mut &[u8]) -> Result<Self> {
            let mut arr = [0u8; size_of::<Self>()];
            arr.decode_ref(resp)?;
            Ok(Self::from_be_bytes(arr))
        }
    }
)+ } }

int_impls! { u8 u16 u32 u64 }

impl CommandData for () {
    fn encode(&self, _: &mut &mut [u8]) -> Result<()> {
        Ok(())
    }
}

impl ResponseData for () {
    fn decode(_: &mut &[u8]) -> Result<Self> {
        Ok(())
    }
}

impl CommandData for bool {
    fn encode(&self, cmd: &mut &mut [u8]) -> Result<()> {
        (*self as u8).encode(cmd)
    }
}

impl ResponseData for bool {
    fn decode(resp: &mut &[u8]) -> Result<Self> {
        match u8::decode(resp)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::InvalidOutputValue),
        }
    }
}

impl TpmData for [u8] {
    fn data_len(&self) -> usize {
        self.len()
    }
}

impl CommandData for [u8] {
    fn encode(&self, cmd: &mut &mut [u8]) -> Result<()> {
        if self.len() > cmd.len() {
            return Err(Error::TooMuchInputData);
        }
        cmd[..self.len()].copy_from_slice(self);
        *cmd = &mut replace(cmd, &mut [])[self.len()..];
        Ok(())
    }
}

impl ResponseRef for [u8] {
    fn decode_ref(&mut self, resp: &mut &[u8]) -> Result<()> {
        if self.len() > resp.len() {
            return Err(Error::MissingOutputData);
        }
        self.copy_from_slice(&resp[..self.len()]);
        *resp = &replace(resp, &[])[self.len()..];
        Ok(())
    }
}

pub(crate) struct InBuf<'a>(pub &'a [u8]);

impl TpmData for InBuf<'_> {
    fn data_len(&self) -> usize {
        2 + self.0.len()
    }
}

impl CommandData for InBuf<'_> {
    fn encode(&self, cmd: &mut &mut [u8]) -> Result<()> {
        let size: u16 = self.0.len().try_into().or(Err(Error::TooBigInputBuffer))?;
        size.encode(cmd)?;
        self.0.encode(cmd)
    }
}

pub(crate) struct OutBuf<'a>(pub &'a mut [u8]);

impl TpmData for OutBuf<'_> {
    fn data_len(&self) -> usize {
        2 + self.0.len()
    }
}

impl ResponseRef for OutBuf<'_> {
    fn decode_ref(&mut self, resp: &mut &[u8]) -> Result<()> {
        let size = u16::decode(resp)? as usize;
        if self.0.len() > size {
            return Err(Error::TooSmallOutputBuffer);
        }
        // Shrink the slice to the correct size
        self.0 = &mut replace(&mut self.0, &mut [])[..size];
        // Now we will read the right amount of data
        self.0.decode_ref(resp)
    }
}
