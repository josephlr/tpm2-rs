use core::{
    ops::{Deref, DerefMut, Range},
    time::Duration,
};

// pub struct Act<F: ?Sized> {
//     state: u8,
//     on_signaled: F,
// }

// impl<F: Sized> Act<F> {
//     pub const fn new(on_signaled: F) -> Self {
//         Self {
//             state: 0,
//             on_signaled,
//         }
//     }
// }

// impl<F: ?Sized + Fn()> Act<F> {
//     pub fn call(&self) {
//         (self.on_signaled)()
//     }
// }

pub trait Rng {
    fn get_entropy(&self, buf: &mut [u8]) -> Result<usize, ()>;
}

#[cfg(feature = "getrandom")]
pub struct Getrandom;
#[cfg(feature = "getrandom")]
impl Rng for Getrandom {
    fn get_entropy(&self, buf: &mut [u8]) -> Result<usize, ()> {
        match getrandom::getrandom(buf) {
            Ok(()) => Ok(buf.len()),
            Err(_) => Err(()),
        }
    }
}

pub trait Clock {
    fn real_time(&self) -> Duration;
}

#[cfg(feature = "std")]
pub struct StdClock;
#[cfg(feature = "std")]
impl Clock for StdClock {
    fn real_time(&self) -> Duration {
        std::time::UNIX_EPOCH.elapsed().unwrap()
    }
}

pub trait Platform {
    fn canceled(&self) -> bool {
        false
    }
    fn physical_presence(&self) -> bool {
        true
    }
    fn locality(&self) -> u8 {
        0
    }

    fn rng(&self) -> &dyn Rng;
    fn clock(&self) -> &dyn Clock;
}

#[cfg(feature = "std")]
pub struct StdPlatform;
#[cfg(feature = "std")]
impl Platform for StdPlatform {
    fn rng(&self) -> &dyn Rng {
        &Getrandom
    }
    fn clock(&self) -> &dyn Clock {
        &StdClock
    }
}

pub const NV_MEMORY_SIZE: usize = 16384;
pub type Mem = [u8; NV_MEMORY_SIZE];

pub trait NVData: 'static + DerefMut<Target = Mem> {
    fn enable(&mut self) -> Result<(), bool>;
    fn available(&mut self) -> Result<(), bool>;
    fn commit(&mut self) -> Result<(), ()>;
    fn dirtied(&mut self, range: Range<usize>);
}

pub struct VolatileData(Mem);

impl VolatileData {
    pub const fn new() -> Self {
        Self([0u8; NV_MEMORY_SIZE])
    }
}

impl Deref for VolatileData {
    type Target = Mem;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VolatileData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl NVData for VolatileData {
    fn enable(&mut self) -> Result<(), bool> {
        Ok(())
    }
    fn available(&mut self) -> Result<(), bool> {
        Ok(())
    }
    fn commit(&mut self) -> Result<(), ()> {
        Ok(())
    }
    fn dirtied(&mut self, _: Range<usize>) {}
}

impl dyn NVData {
    pub fn read(&mut self, offset: usize, data: &mut [u8]) {
        let range = offset..offset + data.len();
        data.copy_from_slice(&self[range])
    }
    pub fn is_different(&mut self, offset: usize, data: &[u8]) -> bool {
        let range = offset..offset + data.len();
        &self[range] != data
    }
    pub fn write(&mut self, offset: usize, data: &[u8]) {
        let range = offset..offset + data.len();
        self[range.clone()].copy_from_slice(data);
        self.dirtied(range);
    }
    pub fn clear(&mut self, range: Range<usize>) {
        for b in &mut self[range.clone()] {
            *b = 0xff;
        }
        self.dirtied(range);
    }
    pub fn shift(&mut self, src: Range<usize>, dst: usize) {
        self.copy_within(src.clone(), dst);
        self.dirtied(dst..dst + src.len());
    }
}
