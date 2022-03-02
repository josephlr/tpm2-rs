#![no_std]

use core::ops::DerefMut;

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::boxed::Box;

mod api;
mod global;
mod traits;

use crate::global::Handle;
pub use crate::traits::{
    Clock, Getrandom, Mem, NVData, Platform, Rng, StdClock, StdPlatform, VolatileData,
};

fn true_means_success(b: bool) -> Result<(), ()> {
    match b {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn manufacture(rng: &dyn Rng, nvdata: &mut dyn NVData) -> Result<(), ()> {
    struct NoClock<'a>(&'a dyn Rng);
    impl Platform for NoClock<'_> {
        fn rng(&self) -> &dyn Rng {
            self.0
        }
        fn clock(&self) -> &dyn Clock {
            unreachable!()
        }
    }

    let res = Handle::new(&NoClock(rng)).set_nvdata(nvdata).manufacture();
    true_means_success(res)
}

// impl Driver for Simulator {
//     fn run_command(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize> {
//         let mut resp_ptr = cmd_resp.as_mut_ptr();
//         let mut resp_size = cmd_resp.len() as u32;

//         unsafe { _plat__RunCommand(cmd_len as u32, resp_ptr, &mut resp_size, &mut resp_ptr) };
//         let resp_size = resp_size as usize;

//         if resp_ptr != cmd_resp.as_mut_ptr() {
//             use core::slice::from_raw_parts;
//             let resp_static = unsafe { from_raw_parts(resp_ptr, resp_size) };
//             cmd_resp[..resp_size].copy_from_slice(resp_static);
//         }
//         Ok(resp_size)
//     }
// }

pub struct Simulator<'a, D> {
    handle: Handle<'a>,
    platform: &'a dyn Platform,
    nvdata: D,
}

impl<D: DerefMut> Simulator<'_, D>
where
    D::Target: NVData + Sized,
{
    pub fn new(platform: &dyn Platform, nvdata: D) -> Result<Simulator<D>, ()> {
        let mut s = Simulator {
            handle: Handle::new(platform),
            platform,
            nvdata,
        };
        s.reset()?;
        Ok(s)
    }
    pub fn hcrtm(&mut self, data: &[u8]) -> Result<(), ()> {
        let res = self
            .handle
            .set_nvdata(self.nvdata.deref_mut())
            .send_hcrtm(data);
        true_means_success(res)
    }
    pub fn run_command(&mut self, cmd_resp: &mut [u8], len: &mut usize) -> Result<(), ()> {
        let res = self
            .handle
            .set_nvdata(self.nvdata.deref_mut())
            .run_command(cmd_resp, len);
        true_means_success(res)
    }
    pub fn reset(&mut self) -> Result<(), ()> {
        let res = self.handle.set_nvdata(self.nvdata.deref_mut()).reset();
        true_means_success(res)
    }
    pub fn platfrom(&self) -> &dyn Platform {
        self.platform
    }
    pub fn nvdata(&self) -> &dyn NVData {
        self.nvdata.deref()
    }
    pub fn nvdata_mut(&mut self) -> &mut dyn NVData {
        self.nvdata.deref_mut()
    }
}

#[cfg(feature = "std")]
pub type TestSimulator = Simulator<'static, Box<VolatileData>>;

#[cfg(feature = "std")]
impl TestSimulator {
    pub fn manufacture() -> Result<Self, ()> {
        let platform = &StdPlatform;
        let mut nvdata = Box::new(VolatileData::new());
        manufacture(platform.rng(), nvdata.deref_mut())?;
        Self::new(platform, nvdata)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    use core::ops::Deref;
    use std::eprintln;

    struct LogRng;
    struct LogClock;
    struct LogPlatform;

    struct LogNVData(VolatileData);

    impl LogNVData {
        fn manufacture() -> Self {
            let mut data = Self(VolatileData::new());
            manufacture(&LogRng, &mut data).unwrap();
            data
        }
    }

    impl Rng for LogRng {
        fn get_entropy(&self, buf: &mut [u8]) -> Result<usize, ()> {
            eprintln!("*** get_entropy: {} bytes", buf.len());
            Getrandom.get_entropy(buf)
        }
    }

    impl Clock for LogClock {
        fn real_time(&self) -> core::time::Duration {
            let d = StdClock.real_time();
            eprintln!("*** read_time: {:?}", d);
            d
        }
    }

    impl Platform for LogPlatform {
        fn canceled(&self) -> bool {
            eprintln!("*** checking if canceled");
            false
        }
        fn physical_presence(&self) -> bool {
            eprintln!("*** checking PPI");
            true
        }
        fn locality(&self) -> u8 {
            eprintln!("*** checking Locality");
            0
        }
        fn rng(&self) -> &dyn Rng {
            &LogRng
        }
        fn clock(&self) -> &dyn Clock {
            &LogClock
        }
    }

    impl Deref for LogNVData {
        type Target = Mem;
        fn deref(&self) -> &Self::Target {
            self.0.deref()
        }
    }

    impl DerefMut for LogNVData {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.0.deref_mut()
        }
    }

    impl NVData for LogNVData {
        fn enable(&mut self) -> Result<(), bool> {
            eprintln!("*** enable");
            self.0.enable()
        }
        fn available(&mut self) -> Result<(), bool> {
            eprintln!("*** available");
            self.0.available()
        }
        fn commit(&mut self) -> Result<(), ()> {
            eprintln!("*** commit");
            self.0.commit()
        }
        fn dirtied(&mut self, range: core::ops::Range<usize>) {
            eprintln!("*** dirtied [0x{:X},0x{:X})", range.start, range.end);
            self.0.dirtied(range)
        }
    }

    // use once_cell::sync::Lazy;
    // use std::sync::{Mutex, MutexGuard};
    // static NV: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(VolatileData::new()));

    // fn get_sim() -> Simulator<MutexGuard<'static, VolatileData>> {
    //     let mem = NV.lock().unwrap();
    //     eprintln!("{:?}", &mem[256..512]);
    //     Simulator::new(mem)
    // }

    // #[test]
    // fn first_cmd_not_statup() {
    //     let mut cmd_resp = [0u8; 100];
    //     let mut len = 8;

    //     let mut sim = Simulator::manufacture().unwrap();
    //     sim.hcrtm(b"SUPER SECURE CODE").unwrap();

    //     {
    //         eprintln!("In: {:?}", &cmd_resp[..len]);
    //         sim.run_command(&mut cmd_resp, &mut len).unwrap();
    //         eprintln!("Out: {:?}", &cmd_resp[..len]);
    //     }
    // }

    #[test]
    fn log_cmds() {
        let mut cmd_resp = [0u8; 100];
        let mut len = 8;

        let mut nvdata = LogNVData::manufacture();
        let mut sim = Simulator::new(&LogPlatform, &mut nvdata).unwrap();

        sim.hcrtm(b"SUPER SECURE CODE").unwrap();
        eprintln!("In: {:?}", &cmd_resp[..len]);
        sim.run_command(&mut cmd_resp, &mut len).unwrap();
        eprintln!("Out: {:?}", &cmd_resp[..len]);
    }
}
