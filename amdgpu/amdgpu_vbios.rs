use crate::AMDGPU::*;
use crate::*;

use std::mem::{size_of, MaybeUninit};

impl DeviceHandle {
    unsafe fn query_vbios<T>(
        &self,
        fd: ::std::os::raw::c_int,
        info_id: ::std::os::raw::c_uint,
    ) -> Result<T, i32> {
        use bindings::{drmCommandWrite, drm_amdgpu_info, AMDGPU_INFO_VBIOS};

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
        &self,
        fd: ::std::os::raw::c_int,
    ) -> Result<bindings::drm_amdgpu_info_vbios, i32> {
        use bindings::{drm_amdgpu_info_vbios, AMDGPU_INFO_VBIOS_INFO};

        let vbios: drm_amdgpu_info_vbios = Self::query_vbios(self, fd, AMDGPU_INFO_VBIOS_INFO)?;

        return Ok(vbios);
    }

    pub unsafe fn vbios_size(&self, fd: ::std::os::raw::c_int) -> Result<u32, i32> {
        use bindings::AMDGPU_INFO_VBIOS_SIZE;

        let vbios_size: u32 = Self::query_vbios(self, fd, AMDGPU_INFO_VBIOS_SIZE)?;

        return Ok(vbios_size);
    }
}
