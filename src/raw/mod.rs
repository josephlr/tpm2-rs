use core::{convert::TryInto, num::NonZeroU32};

pub use tpm_derive::*;

use crate::{Error, Result};

mod attributes;
pub use attributes::*;

mod constants;
pub use constants::*;

mod structs;
pub use structs::*;

mod unions;
pub use unions::*;

mod traits;
pub use traits::*;

mod util;
use util::*;
pub use util::{Driver, Read, Write, BUFFER_SIZE};

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::boxed::Box;

        pub struct Tpm<D = Box<dyn Driver>> {
            buf: Buf,
            driver: D,
        }

        impl Tpm {
            pub fn get() -> Result<Self> {
                Ok(Self::new(Box::new(crate::os::get_driver()?)))
            }
        }
    } else {
        pub struct Tpm<D> {
            buf: Buf,
            driver: D,
        }
    }
}

impl<D> Tpm<D> {
    pub fn new(driver: D) -> Self {
        Self {
            buf: Buf::new(),
            driver,
        }
    }
}

impl<D: Driver> Tpm<D> {
    fn run<Output: ResponseData>(
        &mut self,
        code: CommandCode,
        input: &(impl CommandData + ?Sized),
    ) -> Result<Output> {
        let tag = tag::Command::NoSessions;
        let mut cmd_hdr = CommandHeader { tag, size: 0, code };
        cmd_hdr.size = (cmd_hdr.data_len() + input.data_len()).try_into().unwrap();

        self.buf.reset();
        cmd_hdr.encode(&mut self.buf)?;
        input.encode(&mut self.buf)?;
        self.buf.run_command(&mut self.driver)?;

        let resp_hdr = ResponseHeader::decode(&mut self.buf)?;
        if let Some(err) = NonZeroU32::new(resp_hdr.code) {
            return Err(Error::Tpm(err));
        }
        assert_eq!(resp_hdr.tag, tag);

        let len = resp_hdr.size as usize - resp_hdr.data_len();
        let mut reader = CheckedReader {
            r: &mut self.buf,
            len,
        };
        let output = Output::decode(&mut reader)?;

        if reader.len != 0 {
            return Err(Error::RemainingOutputData);
        }
        Ok(output)
    }

    pub fn startup(&mut self, su: StartupType) -> Result<()> {
        self.run(CommandCode::Startup, &su)
    }

    pub fn shutdown(&mut self, su: StartupType) -> Result<()> {
        self.run(CommandCode::Shutdown, &su)
    }

    pub fn get_random(&mut self, _bytes: &mut [u8]) -> Result<u16> {
        unimplemented!()
    }

    pub fn stir_random(&mut self, bytes: &[u8]) -> Result<()> {
        self.run(CommandCode::StirRandom, bytes)
    }

    pub fn read_clock(&mut self) -> Result<TimeInfo> {
        self.run(CommandCode::ReadClock, &())
    }

    pub fn get_capability<T>(&mut self, data: &mut Capabilities, property: u32) -> Result<bool> {
        unimplemented!()
    }
}

struct CheckedReader<'a, T: ?Sized> {
    r: &'a mut T,
    len: usize,
}

impl<T: Read + ?Sized> Read for CheckedReader<'_, T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        if buf.len() > self.len {
            return Err(Error::MissingOutputData);
        }
        self.r.read(buf)?;
        self.len -= buf.len();
        Ok(())
    }
}
