#![allow(dead_code)]

use crate::Result;
use std::{boxed::Box, convert::TryInto, ffi, mem, ptr};
use tbs_bindings::{
    Windows::Win32::System::TpmBaseServices::{
        Tbsi_Context_Create, Tbsi_GetDeviceInfo, Tbsip_Context_Close, Tbsip_Submit_Command,
        TBS_COMMAND_LOCALITY_ZERO, TBS_COMMAND_PRIORITY_NORMAL, TBS_CONTEXT_PARAMS,
        TBS_CONTEXT_PARAMS2, TBS_CONTEXT_PARAMS2_0, TBS_CONTEXT_VERSION_TWO, TPM_DEVICE_INFO,
        TPM_VERSION_20,
    },
    HRESULT,
};

impl From<tbs_bindings::Error> for crate::Error {
    fn from(error: tbs_bindings::Error) -> Self {
        crate::Error::Os(Box::new(error))
    }
}

pub struct DriverImp(*mut ffi::c_void);

// TODO: Figure out where the Windows docs say the context can be sent.
unsafe impl Send for DriverImp {}

impl Drop for DriverImp {
    fn drop(&mut self) {
        unsafe { Tbsip_Context_Close(self.0) };
    }
}

fn get_info() -> Result<TPM_DEVICE_INFO> {
    let mut info = TPM_DEVICE_INFO::default();
    // Size will not overflow a u32
    let size: u32 = mem::size_of_val(&info).try_into().unwrap();
    let ptr = &mut info as *mut _ as *mut ffi::c_void;
    // SAFETY: ptr points to a TPM_DEVICE_INFO
    let ret = unsafe { Tbsi_GetDeviceInfo(size, ptr) };
    HRESULT::from_win32(ret).ok()?;
    Ok(info)
}

// The Windows bindings don't include bitfields, so we manually define them.
const PARAMS2_REQUEST_RAW: u32 = 1 << 0;
const PARAMS2_INCLUDE_TPM_12: u32 = 1 << 1;
const PARAMS2_INCLUDE_TPM_20: u32 = 1 << 2;

impl DriverImp {
    pub fn new() -> Result<Self> {
        // We only want a TPM2.0
        let mut params = TBS_CONTEXT_PARAMS2 {
            version: TBS_CONTEXT_VERSION_TWO,
            Anonymous: TBS_CONTEXT_PARAMS2_0 {
                asUINT32: PARAMS2_INCLUDE_TPM_20,
            },
        };
        // The Windows API takes the old PARAMS struct, but we need to pass the new one.
        let params_ptr = &mut params as *mut _ as *mut TBS_CONTEXT_PARAMS;
        let mut d = Self(ptr::null_mut());
        // SAFETY: both parameters point to valid structures
        let ret = unsafe { Tbsi_Context_Create(params_ptr, &mut d.0) };
        HRESULT::from_win32(ret).ok()?;
        // We need to make sure we have a TPM 2.0
        if get_info()?.tpmVersion == TPM_VERSION_20 {
            Ok(d)
        } else {
            Err(crate::Error::Os("Could not find TPM 2.0 device".into()))
        }
    }

    pub fn run_driver(&mut self, cmd_resp: &mut [u8], cmd_len: usize) -> Result<usize> {
        let cmd_len: u32 = cmd_len.try_into().unwrap();
        let mut resp_len: u32 = 0;
        let ret = unsafe {
            Tbsip_Submit_Command(
                self.0,
                TBS_COMMAND_LOCALITY_ZERO,
                TBS_COMMAND_PRIORITY_NORMAL,
                cmd_resp.as_mut_ptr(),
                cmd_len,
                cmd_resp.as_mut_ptr(),
                &mut resp_len,
            )
        };
        HRESULT::from_win32(ret).ok()?;
        Ok(resp_len.try_into().unwrap())
    }
}
