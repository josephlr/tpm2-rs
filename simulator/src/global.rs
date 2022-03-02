use core::{
    cell::UnsafeCell,
    convert::TryInto,
    marker::PhantomData,
    ptr::NonNull,
    slice::from_raw_parts,
    sync::atomic::{AtomicU8, Ordering},
};

use crate::{
    api,
    traits::{NVData, Platform},
};

struct SyncCell<T>(UnsafeCell<Option<T>>);

impl<T> SyncCell<T> {
    const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }
}

unsafe impl<T> Sync for SyncCell<T> {}

static STATE: AtomicU8 = AtomicU8::new(0u8);
static PLATFORM: SyncCell<NonNull<dyn Platform>> = SyncCell::new();
static NVDATA: SyncCell<NonNull<dyn NVData>> = SyncCell::new();

pub struct Handle<'b>(PhantomData<&'b dyn Platform>);

pub struct Guard<'a, 'b: 'a>(&'a Handle<'b>, PhantomData<&'a mut dyn NVData>);

impl<'b> Handle<'b> {
    pub fn new(p: &'b dyn Platform) -> Self {
        let old = STATE.compare_and_swap(0, 1, Ordering::SeqCst);
        if old != 0 {
            panic!("Multiple simulators cannot be used at the same time.")
        }
        unsafe { *PLATFORM.0.get() = NonNull::new(p as *const _ as *mut _) };
        Self(PhantomData)
    }
    pub fn set_nvdata<'a>(&'a mut self, n: &'a mut dyn NVData) -> Guard<'a, 'b> {
        debug_assert!(STATE.load(Ordering::SeqCst) == 1);
        STATE.store(2, Ordering::SeqCst);
        unsafe { *NVDATA.0.get() = NonNull::new(n) };
        Guard(self, PhantomData)
    }
}

impl Drop for Handle<'_> {
    fn drop(&mut self) {
        debug_assert!(STATE.load(Ordering::SeqCst) == 1);
        unsafe { *PLATFORM.0.get() = None };
        STATE.store(0, Ordering::SeqCst);
    }
}

impl Guard<'_, '_> {
    pub fn manufacture(&mut self) -> bool {
        unsafe { api::_plat__Manufacture() }
    }
    pub fn reset(&mut self) -> bool {
        unsafe { api::_plat__Reset() }
    }
    pub fn send_hcrtm(&mut self, data: &[u8]) -> bool {
        let size = data.len().try_into().unwrap();
        unsafe { api::_plat__SendHCRTM(size, data.as_ptr()) }
    }
    pub fn run_command(&mut self, buf: &mut [u8], size: &mut usize) -> bool {
        assert!(*size <= buf.len());
        let mut buf_ptr = buf.as_mut_ptr();
        let cmd_size: u32 = (*size).try_into().unwrap();
        let mut resp_size: u32 = buf.len().try_into().unwrap();
        let ret = unsafe {
            api::_plat__RunCommand(cmd_size, buf_ptr, &mut resp_size, &mut buf_ptr)
        };
        *size = resp_size.try_into().unwrap();
        

        if buf_ptr != buf.as_mut_ptr() {
            let resp = unsafe { from_raw_parts(buf_ptr, *size) };
            buf[..*size].copy_from_slice(resp);
        }
        ret
    }
    
    
}

impl Drop for Guard<'_, '_> {
    fn drop(&mut self) {
        unsafe { *NVDATA.0.get() = None };
        debug_assert!(STATE.load(Ordering::SeqCst) == 2);
        STATE.store(1, Ordering::SeqCst);
    }
}

// SAFETY: This should only be called when a Handle is active.
pub unsafe fn platform<'a>() -> &'a dyn Platform {
    let nn = (*PLATFORM.0.get()).expect("Platform not set");
    &*nn.as_ptr()
}

// SAFETY: This should only be called when a Guard is active. The NVData must
// not be aliased, and 'a must have a shorter lifetime than the Guard.
pub unsafe fn nvdata<'a>() -> &'a mut dyn NVData {
    let nn = (*NVDATA.0.get()).expect("NVData not set");
    &mut *nn.as_ptr()
}
