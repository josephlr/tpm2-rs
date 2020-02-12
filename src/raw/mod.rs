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
pub use util::{Driver, BUFFER_SIZE};

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::boxed::Box;

        pub struct Tpm<D = Box<dyn Driver>> {
            buf: [u8; BUFFER_SIZE],
            driver: D,
        }

        impl Tpm {
            pub fn get() -> Result<Self> {
                Ok(Self::new(Box::new(crate::os::get_driver()?)))
            }
        }
    } else {
        pub struct Tpm<D> {
            buf: [u8; BUFFER_SIZE],
            driver: D,
        }
    }
}

impl<D> Tpm<D> {
    pub fn new(driver: D) -> Self {
        Self {
            buf: [0; BUFFER_SIZE],
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
        let cmd_len = cmd_hdr.data_len() + input.data_len();
        cmd_hdr.size = cmd_len.try_into().unwrap();

        let mut cmd_buf: &mut [u8] = &mut self.buf;
        cmd_hdr.encode(&mut cmd_buf)?;
        input.encode(&mut cmd_buf)?;
        assert_eq!(cmd_len + cmd_buf.len(), self.buf.len());

        let resp_len = self.driver.run_command(&mut self.buf, cmd_len)?;
        let mut resp: &[u8] = &self.buf[..resp_len];

        let resp_hdr = ResponseHeader::decode(&mut resp)?;
        if let Some(err) = NonZeroU32::new(resp_hdr.code) {
            return Err(Error::Tpm(err));
        }
        assert_eq!(resp_hdr.tag, tag);
        assert_eq!(resp_hdr.size as usize, resp_len);

        let output = Output::decode(&mut resp)?;
        if resp.len() > 0 {
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
        self.run(CommandCode::StirRandom, &InBuf(bytes))
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
