//! A TPM2 TSS. Add more docs and doc-tests.

// #![warn(missing_docs)]
// #![warn(missing_debug_implementations)]
#![no_std]

use core::{convert::TryInto, num::NonZeroU32};
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod command;
pub use command::Command;

pub mod data;
pub use data::traits::*;

pub mod driver;
use driver::{Driver, DynDriver, MAX_CMD_SIZE};

pub mod error;
pub use error::Error;
use error::{DecodeError, EncodeError};

pub type Result<T> = core::result::Result<T, Error>;

cfg_if::cfg_if! {
    if #[cfg(feature = "alloc")] {
        type Buffer = alloc::boxed::Box<[u8; MAX_CMD_SIZE]>;
        fn new_buffer() -> Buffer {
            // TODO: switch to box syntax (or otherwise avoid stack alloc)
            Buffer::new([0; MAX_CMD_SIZE])
        }
    } else {
        type Buffer = [u8; MAX_CMD_SIZE];
        fn new_buffer() -> Buffer {
            [0; MAX_CMD_SIZE]
        }
    }
}

/// TODO: this is the TPM
pub struct Tpm<D = DynDriver> {
    buffer: Buffer,
    driver: D,
}

impl Tpm {
    /// TODO: note that get blocks and is only available on std
    #[cfg(feature = "std")]
    pub fn get() -> Result<Self> {
        Ok(Self::new_unboxed(DynDriver::get()?))
    }
    /// TODO: note that this needs alloc
    #[cfg(feature = "alloc")]
    pub fn new(driver: impl Driver + 'static) -> Self {
        Self::new_unboxed(DynDriver::from_driver(driver))
    }
}

impl<D: Driver> Tpm<D> {
    /// Only really needed if no-alloc
    pub fn new_unboxed(driver: D) -> Self {
        Self {
            buffer: new_buffer(),
            driver,
        }
    }
    /// Run a TPM command
    #[inline]
    pub fn run<'a, C: Command<'a>>(&'a mut self, command: &C) -> Result<C::Response> {
        let mut response = Default::default();
        run_impl(
            &mut self.driver,
            &mut self.buffer,
            C::CODE,
            command,
            &mut response,
        )?;
        Ok(response)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
enum Tag {
    NoSessions = 0x8001,
    Sessions = 0x8002,
}

struct Header {
    pub tag: Tag,
    pub size: u32,
    pub code: u32,
}

// Force monomorphisation
fn run_impl<'a>(
    driver: &mut dyn Driver,
    buffer: &'a mut [u8; MAX_CMD_SIZE],
    code: command::Code,
    command: &dyn Encode,
    response: &mut dyn Decode<'a>,
) -> Result<()> {
    let tag = Tag::NoSessions; // TODO, handle other tags
    let cmd_size = Header::LEN + command.data_len();
    let mut header = Header {
        tag,
        size: cmd_size.try_into()?,
        code: code.into(),
    };

    let mut cmd_buf: &mut [u8] = buffer;
    header.encode_to_buf(&mut cmd_buf)?;
    command.encode_to_buf(&mut cmd_buf)?;
    assert_eq!(MAX_CMD_SIZE, cmd_size + cmd_buf.len());

    let resp_size = driver.run_command(buffer, cmd_size)?;
    let mut resp_buf: &[u8] = &buffer[..resp_size];

    header.decode_from_buf(&mut resp_buf)?;
    assert_eq!(header.size, resp_size.try_into()?);
    if let Some(err) = NonZeroU32::new(header.code) {
        return Err(Error::Command(err.into()));
    }
    assert_eq!(header.tag, tag);

    response.decode_from_buf(&mut resp_buf)?;
    if !resp_buf.is_empty() {
        return Err(Error::Decode(DecodeError::InputTooLong));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tpm = Tpm::get().unwrap();

        let resp = tpm
            .run(&command::GetRandom {
                bytes_requested: 1000,
            })
            .unwrap();
        let b1: alloc::vec::Vec<u8> = resp.random_bytes.into();
        let resp = tpm
            .run(&command::GetRandom {
                bytes_requested: 1000,
            })
            .unwrap();
        let b2 = resp.random_bytes;
        panic!("Got {:?}, {:?}", b1, b2);
    }
}

// ** BEGIN GENERATED CODE **

impl From<Tag> for u16 {
    fn from(t: Tag) -> Self {
        t as Self
    }
}
impl FixedData for Tag {
    const LEN: usize = u16::LEN;
}
impl Encode for Tag {
    fn encode_to_buf(&self, output: &mut &mut [u8]) -> core::result::Result<(), EncodeError> {
        u16::from(*self).encode_to_buf(output)
    }
}
impl DecodeVal for Tag {
    fn decode_val(input: &mut &[u8]) -> core::result::Result<Self, DecodeError> {
        match u16::decode_val(input)? {
            0x8001 => Ok(Self::NoSessions),
            0x8002 => Ok(Self::Sessions),
            _ => Err(DecodeError::InvalidInput),
        }
    }
}

impl FixedData for Header {
    const LEN: usize = Tag::LEN + u32::LEN + u32::LEN;
}
impl Encode for Header {
    fn encode_to_buf(&self, output: &mut &mut [u8]) -> core::result::Result<(), EncodeError> {
        self.tag.encode_to_buf(output)?;
        self.size.encode_to_buf(output)?;
        self.code.encode_to_buf(output)
    }
}
impl Decode<'_> for Header {
    fn decode_from_buf(
        &mut self,
        input: &mut &[u8],
    ) -> core::result::Result<(), error::DecodeError> {
        self.tag.decode_from_buf(input)?;
        self.size.decode_from_buf(input)?;
        self.code.decode_from_buf(input)
    }
}
