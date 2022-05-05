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
use bindings::{
    AMDGPU_INFO_DEV_INFO,
    AMDGPU_INFO_MEMORY,
    AMDGPU_INFO_VRAM_USAGE,
    AMDGPU_INFO_GDS_CONFIG,
};

pub trait HANDLE {
    fn init(fd: ::std::os::raw::c_int) -> Result<Self, i32> where Self: Sized;
    fn get_marketing_name(self) -> Result<String, std::str::Utf8Error>;
    fn query_gpu_info(self) -> Result<amdgpu_gpu_info, i32>;

    fn device_info(self) -> Result<drm_amdgpu_info_device, i32>;
    fn memory_info(self) -> Result<drm_amdgpu_memory_info, i32>;
    fn vram_usage_info(self) -> Result<u64, i32>;
    fn gds_info(self) -> Result<drm_amdgpu_info_gds, i32>;

    #[doc(hidden)]
    fn query<T>(self, info_id: ::std::os::raw::c_uint) -> Result<T, i32>;

    #[doc(hidden)]
    unsafe fn query_vbios<T>(
        self,
        fd: ::std::os::raw::c_int,
        info_id: ::std::os::raw::c_uint
    ) -> Result<T, i32>;
    unsafe fn vbios_info(
        self,
        fd: ::std::os::raw::c_int,
    ) -> Result<bindings::drm_amdgpu_info_vbios, i32>;
    unsafe fn vbios_size(
        self,
        fd: ::std::os::raw::c_int,
    ) -> Result<u32, i32>;
}

pub type DEVICE = bindings::amdgpu_device;
pub type DEVICE_HANDLE = bindings::amdgpu_device_handle;

impl HANDLE for DEVICE_HANDLE {
    fn init(fd: ::std::os::raw::c_int) -> Result<Self, i32> {
        unsafe {
            let mut amdgpu_dev: MaybeUninit<amdgpu_device_handle> = MaybeUninit::uninit();
            let mut _major: MaybeUninit<u32> = MaybeUninit::zeroed();
            let mut _minor: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = amdgpu_device_initialize(
                fd,
                _major.as_mut_ptr(),
                _minor.as_mut_ptr(),
                amdgpu_dev.as_mut_ptr(),
            );

            query_error!(r);

            let [_major, _minor] = [_major, _minor].map(
                |v| v.assume_init()
            );

            return Ok(amdgpu_dev.assume_init());
        }
    }
    fn get_marketing_name(self) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let c_str = CStr::from_ptr(bindings::amdgpu_get_marketing_name(self));

            match c_str.to_str() {
                Ok(v) => Ok(v.to_string()),
                Err(e) => Err(e),
            }
        }
    }
    fn query_gpu_info(self) -> Result<amdgpu_gpu_info, i32> {
        unsafe {
            let mut gpu_info: MaybeUninit<amdgpu_gpu_info> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_gpu_info(
                self,
                gpu_info.as_mut_ptr()
            );

            query_error!(r);

            return Ok(gpu_info.assume_init());
        }
    }
    fn query<T>(self, info_id: ::std::os::raw::c_uint) -> Result<T, i32> {
        unsafe {
            let mut device_info: MaybeUninit<T> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_info(
                self,
                info_id,
                size_of::<T>() as u32,
                device_info.as_mut_ptr() as *mut ::std::os::raw::c_void,
            );

            query_error!(r);

            return Ok(device_info.assume_init());
        }
    }
    fn device_info(self) -> Result<drm_amdgpu_info_device, i32> {
        Self::query(self, AMDGPU_INFO_DEV_INFO)
    }
    fn memory_info(self) -> Result<drm_amdgpu_memory_info, i32> {
        Self::query(self, AMDGPU_INFO_MEMORY)
    }
    fn vram_usage_info(self) -> Result<u64, i32> {
        Self::query(self, AMDGPU_INFO_VRAM_USAGE)
    }
    fn gds_info(self) -> Result<drm_amdgpu_info_gds, i32> {
        Self::query(self, AMDGPU_INFO_GDS_CONFIG)
    }
    unsafe fn query_vbios<T>(
        self,
        fd: ::std::os::raw::c_int,
        info_id: ::std::os::raw::c_uint,
    ) -> Result<T, i32> {
        use bindings::{
            drmCommandWrite,
            drm_amdgpu_info,
            AMDGPU_INFO_VBIOS,
        };

        let mut vbios: MaybeUninit<T> = MaybeUninit::uninit();

        // std::ptr::write_bytes(device_info.as_mut_ptr(), 0x0, 1);
        let mut device_info: drm_amdgpu_info = std::mem::zeroed();

        device_info.return_pointer = vbios.as_mut_ptr() as u64;
        device_info.return_size = size_of::<T>() as u32;
        device_info.query = AMDGPU_INFO_VBIOS;

        device_info.__bindgen_anon_1.vbios_info.type_ = info_id;

        // println!("vbios type: {}", device_info.__bindgen_anon_1.vbios_info.type_);

        let mut device_info = MaybeUninit::new(device_info);

        let r = drmCommandWrite(
            fd,
            bindings::DRM_AMDGPU_INFO as u64,
            device_info.as_mut_ptr() as *mut ::std::os::raw::c_void,
            size_of::<drm_amdgpu_info> as u64, 
        );

        query_error!(r);

        let _ = device_info.assume_init();
        let vbios = vbios.assume_init();

        return Ok(vbios);
    }
    unsafe fn vbios_info(
        self,
        fd: ::std::os::raw::c_int,
    ) -> Result<bindings::drm_amdgpu_info_vbios, i32> {
        use bindings::{
            drm_amdgpu_info_vbios,
            AMDGPU_INFO_VBIOS_INFO,
        };

        let vbios: drm_amdgpu_info_vbios = Self::query_vbios(
            self,
            fd,
            AMDGPU_INFO_VBIOS_INFO
        )?;

        return Ok(vbios);
    }
    unsafe fn vbios_size(
        self,
        fd: ::std::os::raw::c_int,
    ) -> Result<u32, i32> {
        use bindings::{
            AMDGPU_INFO_VBIOS_SIZE,
        };

        let vbios_size: u32 = Self::query_vbios(
            self,
            fd,
            AMDGPU_INFO_VBIOS_SIZE
        )?;

        return Ok(vbios_size);
    }
}
