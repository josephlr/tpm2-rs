//! C interface to the TPM2 Simulator
use core::{
    convert::TryInto,
    ffi::c_void,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use crate::global::{nvdata, platform};

// External Simulator Entrypoints (Simulator.h)
extern "C" {
    pub fn _plat__Manufacture() -> bool;
    pub fn _plat__Reset() -> bool;
    pub fn _plat__SendHCRTM(size: u32, data: *const u8) -> bool;
    pub fn _plat__RunCommand(
        requestSize: u32,
        request: *const u8,
        responseSize: *mut u32,
        response: *mut *mut u8,
    ) -> bool;
    
}

// C ints aren't in core, but it doesn't matter.
type Int = i32;
type Uint = u32;

// Functions that must be implemented for the Simulator (tpm/Platform_fp.h)
// Note that we don't need to implement some of the Platform functions as they
// are implemented in the Google sample code.

#[no_mangle]
unsafe extern "C" fn _plat__IsCanceled() -> Int {
    platform().canceled() as Int
}

#[no_mangle]
unsafe extern "C" fn _plat__RealTimeNs() -> u64 {
    platform().clock().real_time().as_nanos() as u64
}

#[no_mangle]
unsafe extern "C" fn _plat__GetEntropy(entropy: *mut u8, amount: u32) -> i32 {
    let buf = from_raw_parts_mut(entropy, amount as usize);
    match platform().rng().get_entropy(buf) {
        Ok(len) => len.try_into().unwrap(),
        Err(()) => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn _plat__LocalityGet() -> u8 {
    platform().locality()
}

#[no_mangle]
unsafe extern "C" fn _plat__NVEnable(_: *mut c_void) -> Int {
    match nvdata().enable() {
        Ok(()) => 0,
        Err(true) => 1,
        Err(false) => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn _plat__IsNvAvailable() -> Int {
    match nvdata().available() {
        Ok(()) => 0,
        Err(true) => 1,
        Err(false) => 2,
    }
}

#[no_mangle]
unsafe extern "C" fn _plat__NvMemoryRead(offset: Uint, size: Uint, data: *mut c_void) {
    let buf = from_raw_parts_mut(data as *mut u8, size as usize);
    nvdata().read(offset as usize, buf)
}

#[no_mangle]
unsafe extern "C" fn _plat__NvIsDifferent(offset: Uint, size: Uint, data: *const c_void) -> Int {
    let buf = from_raw_parts(data as *const u8, size as usize);
    nvdata().is_different(offset as usize, buf) as Int
}

#[no_mangle]
unsafe extern "C" fn _plat__NvMemoryWrite(offset: Uint, size: Uint, data: *const c_void) -> bool {
    let buf = from_raw_parts(data as *const u8, size as usize);
    nvdata().write(offset as usize, buf);
    true // Bounds checked by NVData::write
}

#[no_mangle]
unsafe extern "C" fn _plat__NvMemoryClear(start: Uint, size: Uint) {
    let start = start as usize;
    let end = start + size as usize;
    nvdata().clear(start..end);
}

#[no_mangle]
unsafe extern "C" fn _plat__NvMemoryMove(source: Uint, dest: Uint, size: Uint) {
    let start = source as usize;
    let end = start + size as usize;
    nvdata().shift(start..end, dest as usize);
}

#[no_mangle]
unsafe extern "C" fn _plat__NvCommit() -> Int {
    match nvdata().commit() {
        Ok(()) => 0,
        Err(()) => 1,
    }
}

#[no_mangle]
unsafe extern "C" fn _plat__PhysicalPresenceAsserted() -> Int {
    platform().physical_presence() as Int
}
