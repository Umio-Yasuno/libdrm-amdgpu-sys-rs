use crate::bindings;
use crate::drmModeObjectProperties;
use bindings::drmModeCrtcPtr;
pub use bindings::drmModeCrtc;

struct WrapperDrmModeCrtcPtr(pub(crate) drmModeCrtcPtr);

impl Drop for WrapperDrmModeCrtcPtr {
    fn drop(&mut self) {
	    unsafe { bindings::drmModeFreeCrtc(self.0); }
    }
}

impl drmModeCrtc {
    pub fn get(fd: i32, crtc_id: u32) -> Option<Self> {
        let ptr = unsafe { bindings::drmModeGetCrtc(fd, crtc_id) };

        if ptr.is_null() { return None; }

        let ptr = WrapperDrmModeCrtcPtr(ptr);

        Some(unsafe { ptr.0.read() })
    }

    pub fn mode_valid(&self) -> bool {
        self.mode_valid == 1
    }

    pub fn get_crtc_props(&self, fd: i32) -> Option<drmModeObjectProperties> {
        drmModeObjectProperties::get(
            fd,
            self.crtc_id,
            bindings::DRM_MODE_OBJECT_CRTC,
        )
    }
}
