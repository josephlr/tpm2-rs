// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
// #![doc(test(attr(allow(unused_variables), deny(warnings))))]

#![no_std]
use core::num::NonZeroU32;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod data;
mod error;
pub use error::{Error, Result};
pub mod structs;

use data::{Buffer, DataIn, DataOut};
use structs::{StartupType, TimeInfo};

pub trait Exec {
    fn execute(&mut self, input: &[u8], output: &mut [u8]) -> Result<usize>;
}

pub struct Runner<T, B> {
    exec: T,
    inbuf: B,
    outbuf: B,
}

impl<T, B> Runner<T, B> {
    pub fn with_buffers(exec: T, inbuf: B, outbuf: B) -> Self {
        Runner {
            exec,
            inbuf,
            outbuf,
        }
    }
}

#[cfg(feature = "alloc")]
impl<T> Runner<T, alloc::vec::Vec<u8>> {
    pub fn new(exec: T) -> Self {
        Runner {
            exec,
            inbuf: alloc::vec![0; 4096],
            outbuf: alloc::vec![0; 4096],
        }
    }
}

impl<T: Exec, B: AsRef<[u8]> + AsMut<[u8]>> Tpm for Runner<T, B> {
    fn buffer(&mut self) -> &mut [u8] {
        self.inbuf.as_mut()
    }
    fn run(&mut self, input_len: usize) -> Result<&[u8]> {
        let input = &self.inbuf.as_ref()[..input_len];
        let outlen = self.exec.execute(input, self.outbuf.as_mut())?;
        Ok(&self.outbuf.as_ref()[..outlen])
    }
}

pub trait Tpm {
    fn buffer(&mut self) -> &mut [u8];
    fn run(&mut self, len: usize) -> Result<&[u8]>;

    fn startup(&mut self, su: StartupType) -> Result<()> {
        run_command(self, 0x144, su)
    }

    fn shutdown(&mut self, su: StartupType) -> Result<()> {
        run_command(self, 0x145, su)
    }

    fn get_random(&mut self, num_bytes: u16) -> Result<Buffer> {
        run_command(self, 0x17B, num_bytes)
    }

    fn read_clock(&mut self) -> Result<TimeInfo> {
        run_command(self, 0x181, ())
    }
}

fn run_command<T, I, O>(tpm: &mut T, cmd: u32, input: I) -> Result<O>
where
    T: Tpm + ?Sized,
    I: DataIn,
    O: DataOut,
{
    let tag: u16 = 0x8001;

    let buffer = tpm.buffer();
    let buflen = buffer.len();

    let (header, data) = buffer.split_at_mut(10);
    let unused = input.into_bytes(data)?;

    let size = (buflen - unused.len()) as u32;
    CommandHeader { tag, size, cmd }.into_bytes(header)?;

    let mut output = tpm.run(size as usize)?;
    let outlen = output.len();

    let header = ResponseHeader::from_bytes(&mut output)?;
    if header.tag != tag {
        return Err(Error::TagMismatch);
    }
    if header.size != (outlen as u32) {
        return Err(Error::OutLenMismatch);
    }
    if let Some(err) = NonZeroU32::new(header.resp) {
        return Err(Error::Tpm(err));
    }

    let o = O::from_bytes(&mut output)?;
    if !output.is_empty() {
        return Err(Error::RemainingData);
    }
    Ok(o)
}

struct CommandHeader {
    tag: u16,
    size: u32,
    cmd: u32,
}

impl DataIn for CommandHeader {
    fn into_bytes<'a>(&self, mut bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
        bytes = self.tag.into_bytes(bytes)?;
        bytes = self.size.into_bytes(bytes)?;
        bytes = self.cmd.into_bytes(bytes)?;
        Ok(bytes)
    }
}

struct ResponseHeader {
    tag: u16,
    size: u32,
    resp: u32,
}

impl DataOut for ResponseHeader {
    fn from_bytes(bytes: &mut &[u8]) -> Result<Self> {
        Ok(Self {
            tag: DataOut::from_bytes(bytes)?,
            size: DataOut::from_bytes(bytes)?,
            resp: DataOut::from_bytes(bytes)?,
        })
    }
}
