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

pub unsafe fn vbios_info_query<T>(
    fd: ::std::os::raw::c_int,
    _dev: amdgpu_device_handle,
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

pub unsafe fn vbios_info(
    fd: ::std::os::raw::c_int,
    dev: amdgpu_device_handle,
) -> Result<bindings::drm_amdgpu_info_vbios, i32> {
    use bindings::{
        drm_amdgpu_info_vbios,
        AMDGPU_INFO_VBIOS_INFO,
    };

    let vbios: drm_amdgpu_info_vbios = vbios_info_query(
        fd,
        dev,
        AMDGPU_INFO_VBIOS_INFO
    )?;

    // println!("{:?}", vbios);
    println!("name: {}", String::from_utf8(vbios.name.to_vec()).unwrap());
    println!("pn: {}", String::from_utf8(vbios.vbios_pn.to_vec()).unwrap());
    println!("ver: {}", String::from_utf8(vbios.vbios_ver_str.to_vec()).unwrap());
    println!("date: {}", String::from_utf8(vbios.date.to_vec()).unwrap());

    return Ok(vbios);
}

pub unsafe fn vbios_size(
    fd: ::std::os::raw::c_int,
    dev: amdgpu_device_handle,
) -> Result<u32, i32> {
    use bindings::{
        // drm_amdgpu_info_vbios,
        AMDGPU_INFO_VBIOS_SIZE,
    };

    let vbios_size: u32 = vbios_info_query(
        fd,
        dev,
        AMDGPU_INFO_VBIOS_SIZE
    )?;

    println!("vbios size: {vbios_size}");

    return Ok(vbios_size);
}
