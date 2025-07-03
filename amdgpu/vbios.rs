use crate::AMDGPU::*;
use crate::*;

use core::mem::{size_of, MaybeUninit};
use core::ptr;

use bindings::{AMDGPU_INFO_VBIOS, DRM_AMDGPU_INFO, drm_amdgpu_info};

/// VBIOS information
#[derive(Debug, Clone)]
pub struct VbiosInfo {
    pub name: String,
    pub pn: String,
    pub ver: String,
    pub date: String,
    pub size: u32,
}

impl DeviceHandle {
    pub fn get_vbios_info(&self) -> Result<VbiosInfo, i32> {
        let vbios = self.vbios_info()?;
        let size = self.vbios_size()?;

        let [name, pn, ver, date] = [
            vbios.name.to_vec(),
            vbios.vbios_pn.to_vec(),
            vbios.vbios_ver_str.to_vec(),
            vbios.date.to_vec(),
        ]
        .map(|v| {
            if let Some(index) = v.iter().position(|&x| x == 0) {
                String::from_utf8(v.get(..index).unwrap_or_default().to_vec())
            } else {
                String::from_utf8(v)
            }.unwrap_or_default()
        });

        Ok(VbiosInfo { name, pn, ver, date, size })
    }

    unsafe fn query_vbios<T>(
        &self,
        info_id: ::core::ffi::c_uint,
    ) -> Result<T, i32> { unsafe {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::drmCommandWrite;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm.drmCommandWrite;

        let mut vbios: MaybeUninit<T> = MaybeUninit::zeroed();
        let mut device_info: MaybeUninit<drm_amdgpu_info> = MaybeUninit::zeroed();

        {
            let ptr = device_info.as_mut_ptr();

            ptr::addr_of_mut!((*ptr).return_pointer).write(vbios.as_mut_ptr() as u64);
            ptr::addr_of_mut!((*ptr).return_size).write(size_of::<T>() as u32);
            ptr::addr_of_mut!((*ptr).query).write(AMDGPU_INFO_VBIOS);

            ptr::addr_of_mut!((*ptr).__bindgen_anon_1.vbios_info.type_).write(info_id);
        }

        let r = func(
            self.fd,
            DRM_AMDGPU_INFO as ::core::ffi::c_ulong,
            device_info.as_mut_ptr() as *mut ::core::ffi::c_void,
            size_of::<drm_amdgpu_info>() as ::core::ffi::c_ulong,
        );

        let (_, vbios) = (device_info.assume_init(), vbios.assume_init());

        query_error!(r);

        Ok(vbios)
    }}

    pub fn vbios_info(&self) -> Result<bindings::drm_amdgpu_info_vbios, i32> {
        use bindings::AMDGPU_INFO_VBIOS_INFO;

        unsafe { Self::query_vbios(self, AMDGPU_INFO_VBIOS_INFO) }
    }

    pub fn vbios_size(&self) -> Result<u32, i32> {
        use bindings::AMDGPU_INFO_VBIOS_SIZE;

        unsafe { Self::query_vbios(self, AMDGPU_INFO_VBIOS_SIZE) }
    }

    unsafe fn get_vbios_image_with_size(&self, vbios_size: u32) -> Result<Vec<u8>, i32> { unsafe {
        use bindings::AMDGPU_INFO_VBIOS_IMAGE;

        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::drmCommandWrite;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm.drmCommandWrite;

        let mut vbios_image = vec![0; vbios_size as usize];
        let mut device_info: MaybeUninit<drm_amdgpu_info> = MaybeUninit::zeroed();

        {
            let ptr = device_info.as_mut_ptr();

            ptr::addr_of_mut!((*ptr).return_pointer).write(vbios_image.as_mut_ptr() as u64);
            ptr::addr_of_mut!((*ptr).return_size).write(vbios_size);
            ptr::addr_of_mut!((*ptr).query).write(AMDGPU_INFO_VBIOS);

            ptr::addr_of_mut!((*ptr).__bindgen_anon_1.vbios_info.type_)
                .write(AMDGPU_INFO_VBIOS_IMAGE);
        }

        let r = func(
            self.fd,
            DRM_AMDGPU_INFO as ::core::ffi::c_ulong,
            device_info.as_mut_ptr() as *mut ::core::ffi::c_void,
            size_of::<drm_amdgpu_info>() as ::core::ffi::c_ulong,
        );

        let _ = device_info.assume_init();

        query_error!(r);

        Ok(vbios_image)
    }}


    pub fn get_vbios_image(&self) -> Result<Vec<u8>, i32> {
        let size = self.vbios_size()?;

        unsafe { self.get_vbios_image_with_size(size) }
    }
}
