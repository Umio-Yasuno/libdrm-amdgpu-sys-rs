use crate::AMDGPU::*;
use crate::*;

use core::mem::{size_of, MaybeUninit};

impl DeviceHandle {
    unsafe fn query_vbios<T>(
        &self,
        info_id: ::core::ffi::c_uint,
    ) -> Result<T, i32> {
        use bindings::{drmCommandWrite, drm_amdgpu_info, AMDGPU_INFO_VBIOS};

        let mut vbios: MaybeUninit<T> = MaybeUninit::uninit();

        // std::ptr::write_bytes(device_info.as_mut_ptr(), 0x0, 1);
        let mut device_info: drm_amdgpu_info = core::mem::zeroed();

        device_info.return_pointer = vbios.as_mut_ptr() as u64;
        device_info.return_size = size_of::<T>() as u32;
        device_info.query = AMDGPU_INFO_VBIOS;

        device_info.__bindgen_anon_1.vbios_info.type_ = info_id;

        // println!("vbios type: {}", device_info.__bindgen_anon_1.vbios_info.type_);

        let mut device_info = MaybeUninit::new(device_info);

        let r = drmCommandWrite(
            self.get_fd(),
            bindings::DRM_AMDGPU_INFO as u64,
            device_info.as_mut_ptr() as *mut ::core::ffi::c_void,
            size_of::<drm_amdgpu_info> as u64,
        );

        query_error!(r);

        let _ = device_info.assume_init();
        let vbios = vbios.assume_init();

        return Ok(vbios);
    }

    pub unsafe fn vbios_info(&self) -> Result<bindings::drm_amdgpu_info_vbios, i32> {
        use bindings::{AMDGPU_INFO_VBIOS_INFO};

        Self::query_vbios(self, AMDGPU_INFO_VBIOS_INFO)
    }

    pub unsafe fn vbios_size(&self) -> Result<u32, i32> {
        use bindings::AMDGPU_INFO_VBIOS_SIZE;

        Self::query_vbios(self, AMDGPU_INFO_VBIOS_SIZE)
    }

    #[cfg(feature = "std")]
    pub unsafe fn vbios_image(&self, vbios_size: usize) -> Result<Vec<u8>, i32> {
        use bindings::{drmCommandWrite, drm_amdgpu_info, AMDGPU_INFO_VBIOS};
        use bindings::AMDGPU_INFO_VBIOS_IMAGE;

        let mut vbios_image = vec![0; vbios_size];

        let mut device_info: drm_amdgpu_info = core::mem::zeroed();

        device_info.return_pointer = vbios_image.as_mut_ptr() as u64;
        device_info.return_size = vbios_size as u32;
        device_info.query = AMDGPU_INFO_VBIOS;

        device_info.__bindgen_anon_1.vbios_info.type_ = AMDGPU_INFO_VBIOS_IMAGE;

        let mut device_info = MaybeUninit::new(device_info);

        let r = drmCommandWrite(
            self.get_fd(),
            bindings::DRM_AMDGPU_INFO as u64,
            device_info.as_mut_ptr() as *mut ::core::ffi::c_void,
            size_of::<drm_amdgpu_info> as u64,
        );

        query_error!(r);

        let _ = device_info.assume_init();

        return Ok(vbios_image);
    }
}
