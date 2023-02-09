use crate::AMDGPU::*;
use crate::*;

use core::mem::{size_of, MaybeUninit};
use core::ptr;

impl DeviceHandle {
    unsafe fn query_vbios<T>(
        &self,
        info_id: ::core::ffi::c_uint,
    ) -> Result<T, i32> {
        use bindings::{drmCommandWrite, drm_amdgpu_info, AMDGPU_INFO_VBIOS};

        let mut vbios: MaybeUninit<T> = MaybeUninit::uninit();
        let mut device_info: MaybeUninit<drm_amdgpu_info> = MaybeUninit::uninit();

        {
            let ptr = device_info.as_mut_ptr();

            ptr::addr_of_mut!((*ptr).return_pointer).write(vbios.as_mut_ptr() as u64);
            ptr::addr_of_mut!((*ptr).return_size).write(size_of::<T> as u32);
            ptr::addr_of_mut!((*ptr).query).write(AMDGPU_INFO_VBIOS);

            ptr::addr_of_mut!((*ptr).__bindgen_anon_1.vbios_info.type_).write(info_id);
        }

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
        let mut device_info: MaybeUninit<drm_amdgpu_info> = MaybeUninit::uninit();

        {
            let ptr = device_info.as_mut_ptr();

            ptr::addr_of_mut!((*ptr).return_pointer).write(vbios_image.as_mut_ptr() as u64);
            ptr::addr_of_mut!((*ptr).return_size).write(vbios_size as u32);
            ptr::addr_of_mut!((*ptr).query).write(AMDGPU_INFO_VBIOS);

            ptr::addr_of_mut!((*ptr).__bindgen_anon_1.vbios_info.type_)
                .write(AMDGPU_INFO_VBIOS_IMAGE);
        }

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
