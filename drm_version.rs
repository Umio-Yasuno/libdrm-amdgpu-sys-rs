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
impl drmVersion {
    pub fn get(fd: i32) -> Result<drmVersion, i32> {
        use crate::bindings;
        use core::ffi::CStr;

        let drm_ver_ptr = unsafe { bindings::drmGetVersion(fd) };

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

        unsafe { bindings::drmFreeVersion(drm_ver_ptr) }

        Ok(Self {
            version_major: ver.version_major,
            version_minor: ver.version_minor,
            version_patchlevel: ver.version_patchlevel,
            name,
            date,
            desc,
        })
    }
}
