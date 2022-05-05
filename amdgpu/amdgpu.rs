use crate::*;

use std::mem::{MaybeUninit, size_of};
use std::ffi::CStr;
use bindings::{
    amdgpu_device_handle,
    amdgpu_device_initialize,
    amdgpu_gpu_info,
    drm_amdgpu_info_device,
    drm_amdgpu_memory_info,
    drm_amdgpu_info_gds,
};

#[macro_export]
macro_rules! query_error {
    ($r: expr) => {
        if $r != 0 {
            return Err($r);
        }
    };
}

pub fn device_initialize(
    fd: ::std::os::raw::c_int,
) -> Result<amdgpu_device_handle, i32> {
    unsafe {
        let mut amdgpu_dev: MaybeUninit<amdgpu_device_handle> = MaybeUninit::uninit();
        let mut major: MaybeUninit<u32> = MaybeUninit::zeroed();
        let mut minor: MaybeUninit<u32> = MaybeUninit::zeroed();

        let r = amdgpu_device_initialize(
            fd,
            major.as_mut_ptr(),
            minor.as_mut_ptr(),
            amdgpu_dev.as_mut_ptr(),
        );

        query_error!(r);

        let [_major, _minor] = [major, minor].map(
            |v| v.assume_init()
        );

        return Ok(amdgpu_dev.assume_init());
    }
}

pub fn query_gpu_info(
    dev: amdgpu_device_handle,
) -> Result<amdgpu_gpu_info, i32> {
    unsafe {
        let mut gpu_info: MaybeUninit<amdgpu_gpu_info> = MaybeUninit::zeroed();

        let r = bindings::amdgpu_query_gpu_info(
            dev,
            gpu_info.as_mut_ptr()
        );

        query_error!(r);

        return Ok(gpu_info.assume_init());
    }
}

pub fn get_marketing_name(
    dev: amdgpu_device_handle
) -> Result<String, std::str::Utf8Error> {
    unsafe {
        let c_str = CStr::from_ptr(bindings::amdgpu_get_marketing_name(dev));

        match c_str.to_str() {
            Ok(v) => Ok(v.to_string()),
            Err(e) => Err(e),
        }
    }
}

use bindings::{
    AMDGPU_INFO_DEV_INFO,
    AMDGPU_INFO_MEMORY,
    AMDGPU_INFO_VRAM_USAGE,
    AMDGPU_INFO_GDS_CONFIG,
};

pub struct INFO;

impl INFO {
    fn query<T>(
        dev: amdgpu_device_handle,
        info_id: ::std::os::raw::c_uint,
    ) -> Result<T, i32> {
        unsafe {
            let mut device_info: MaybeUninit<T> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_info(
                dev,
                info_id,
                size_of::<T>() as u32,
                device_info.as_mut_ptr() as *mut ::std::os::raw::c_void,
            );

            query_error!(r);

            return Ok(device_info.assume_init());
        }
    }
    pub fn device_info(dev: amdgpu_device_handle) -> Result<drm_amdgpu_info_device, i32> {
        Self::query(dev, AMDGPU_INFO_DEV_INFO)
    }
    pub fn memory_info(dev: amdgpu_device_handle) -> Result<drm_amdgpu_memory_info, i32> {
        Self::query(dev, AMDGPU_INFO_MEMORY)
    }
    pub fn vram_usage_info(dev: amdgpu_device_handle) -> Result<u64, i32> {
        Self::query(dev, AMDGPU_INFO_VRAM_USAGE)
    }
    pub fn gds_info(dev: amdgpu_device_handle) -> Result<drm_amdgpu_info_gds, i32> {
        Self::query(dev, AMDGPU_INFO_GDS_CONFIG)
    }
}
