use crate::AMDGPU::DEVICE_HANDLE;
use crate::*;

use bindings::{
    amdgpu_device_handle,
    // amdgpu_device_initialize,
    amdgpu_gds_resource_info,
    amdgpu_gpu_info,
    // amdgpu_heap_info,
    drm_amdgpu_info_device,
    drm_amdgpu_info_gds,
    drm_amdgpu_memory_info,
};
use bindings::{
    AMDGPU_INFO_DEV_INFO, AMDGPU_INFO_GDS_CONFIG, AMDGPU_INFO_MEMORY, AMDGPU_INFO_VRAM_USAGE,
};
use std::ffi::CStr;
use std::mem::{size_of, MaybeUninit};

pub struct DeviceHandle(pub(crate) DEVICE_HANDLE);

impl DeviceHandle {
    pub fn init(fd: i32) -> Result<Self, i32> {
        unsafe {
            let mut amdgpu_dev: MaybeUninit<amdgpu_device_handle> = MaybeUninit::uninit();
            let mut _major: MaybeUninit<u32> = MaybeUninit::zeroed();
            let mut _minor: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_device_initialize(
                fd,
                _major.as_mut_ptr(),
                _minor.as_mut_ptr(),
                amdgpu_dev.as_mut_ptr(),
            );

            query_error!(r);

            /*
            let [_major, _minor] = [_major, _minor].map(
                |v| v.assume_init()
            );
            */

            return Ok(Self(amdgpu_dev.assume_init()));
        }
    }

    pub fn deinit(&self) -> Result<i32, i32> {
        let r = unsafe { bindings::amdgpu_device_deinitialize(self.0) };

        query_error!(r);

        return Ok(r);
    }

    pub fn get_fd(&self) -> i32 {
        unsafe { bindings::amdgpu_device_get_fd(self.0) }
    }

    pub fn get_marketing_name(&self) -> Result<String, std::str::Utf8Error> {
        let c_str = unsafe {
            let mark_name = bindings::amdgpu_get_marketing_name(self.0);

            if mark_name.is_null() {
                eprintln!("libdrm_amdgpu_sys: ASIC not found in amdgpu.ids");
                return Ok("".to_string());
            }

            CStr::from_ptr(mark_name)
        };

        Ok(c_str.to_str()?.to_string())
    }

    pub fn query_gpu_info(&self) -> Result<amdgpu_gpu_info, i32> {
        unsafe {
            let mut gpu_info: MaybeUninit<amdgpu_gpu_info> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_gpu_info(self.0, gpu_info.as_mut_ptr());

            query_error!(r);

            return Ok(gpu_info.assume_init());
        }
    }

    pub fn query_gds_info(&self) -> Result<amdgpu_gds_resource_info, i32> {
        unsafe {
            let mut gds_info: MaybeUninit<amdgpu_gds_resource_info> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_gds_info(self.0, gds_info.as_mut_ptr());

            query_error!(r);

            return Ok(gds_info.assume_init());
        }
    }

    pub fn query_sw_info(&self, info: amdgpu_sw_info) -> Result<u32, i32> {
        unsafe {
            let mut val: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_sw_info(
                self.0,
                info as u32,
                val.as_mut_ptr() as *mut ::std::os::raw::c_void,
            );

            query_error!(r);

            return Ok(val.assume_init());
        }
    }

    /*
    fn query_heap_info(self) -> Result<amdgpu_heap_info, i32> {
        unsafe {
            let mut heap_info: MaybeUninit<amdgpu_heap_info> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_heap_info(
                self,
                heap_info.as_mut_ptr(),
            );

            query_error!(r);

            return Ok(heap_info.assume_init());
        }
    }
    */

    fn query<T>(&self, info_id: ::std::os::raw::c_uint) -> Result<T, i32> {
        unsafe {
            let mut device_info: MaybeUninit<T> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_info(
                self.0,
                info_id,
                size_of::<T>() as u32,
                device_info.as_mut_ptr() as *mut ::std::os::raw::c_void,
            );

            query_error!(r);

            return Ok(device_info.assume_init());
        }
    }

    pub fn device_info(&self) -> Result<drm_amdgpu_info_device, i32> {
        Self::query(self, AMDGPU_INFO_DEV_INFO)
    }

    pub fn memory_info(&self) -> Result<drm_amdgpu_memory_info, i32> {
        Self::query(self, AMDGPU_INFO_MEMORY)
    }

    pub fn vram_usage_info(&self) -> Result<u64, i32> {
        Self::query(self, AMDGPU_INFO_VRAM_USAGE)
    }

    pub fn gds_info(&self) -> Result<drm_amdgpu_info_gds, i32> {
        Self::query(self, AMDGPU_INFO_GDS_CONFIG)
    }
}

#[repr(u32)]
pub enum amdgpu_sw_info {
    address32_hi = 0,
}
