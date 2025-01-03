#[cfg(feature = "link_drm")]
use crate::bindings;
use crate::AMDGPU::DeviceHandle;
use core::ffi::CStr;

#[cfg(feature = "std")]
#[derive(Debug, Clone)]
pub struct drmVersion {
    pub version_major: i32,
    pub version_minor: i32,
    pub version_patchlevel: i32,
    pub name: String,
    pub date: String,
    pub desc: String,
}

#[cfg(feature = "std")]
impl DeviceHandle {
    pub fn get_drm_version_struct(&self) -> Result<drmVersion, i32> {
        #[cfg(feature = "link_drm")]
        let (get_func, free_func) = (bindings::drmGetVersion, bindings::drmFreeVersion);
        #[cfg(feature = "dynamic_loading")]
        let (get_func, free_func) = (self.libdrm.drmGetVersion, self.libdrm.drmFreeVersion);

        let drm_ver_ptr = unsafe { get_func(self.fd) };

        if drm_ver_ptr.is_null() {
            return Err(-libc::EFAULT);
        }

        let ver = unsafe { core::ptr::read(drm_ver_ptr) };

        let [name, date, desc] = [ver.name, ver.date, ver.desc].map(|v| {
            if v.is_null() {
                String::new()
            } else {
                unsafe { CStr::from_ptr(v).to_str().map(|s| s.to_string()).unwrap_or_default() }
            }
        });

        unsafe { free_func(drm_ver_ptr) }

        Ok(drmVersion {
            version_major: ver.version_major,
            version_minor: ver.version_minor,
            version_patchlevel: ver.version_patchlevel,
            name,
            date,
            desc,
        })
    }
}
