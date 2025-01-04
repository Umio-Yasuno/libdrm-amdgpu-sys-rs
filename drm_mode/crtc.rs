use crate::{bindings, drmModeObjectProperties, LibDrm};
use bindings::drmModeCrtcPtr;
pub use bindings::drmModeCrtc;

#[allow(dead_code)]
#[derive(Clone)]
struct WrapperDrmModeCrtcPtr {
    pub(crate) ptr: drmModeCrtcPtr,
    pub(crate) lib: LibDrm,
}

impl LibDrm {
    pub fn get_drm_mode_crtc(&self, fd: i32, crtc_id: u32) -> Option<drmModeCrtc> {
        #[cfg(feature = "link_drm")]
        let func = bindings::drmModeGetCrtc;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm.drmModeGetCrtc;

        let ptr = unsafe { func(fd, crtc_id) };

        if ptr.is_null() { return None; }

        let wrapper = WrapperDrmModeCrtcPtr { ptr, lib: self.clone() };

        Some(unsafe { wrapper.ptr.read() })
    }

    pub fn get_drm_mode_crtc_props(
        &self,
        fd: i32,
        crtc_id: u32,
    ) -> Option<drmModeObjectProperties> {
        self.get_drm_mode_object_properties(
            fd,
            crtc_id,
            bindings::DRM_MODE_OBJECT_CRTC,
        )
    }
}

impl drmModeCrtc {
    #[cfg(feature = "link_drm")]
    pub fn get(fd: i32, crtc_id: u32) -> Option<Self> {
        let ptr = unsafe { bindings::drmModeGetCrtc(fd, crtc_id) };

        if ptr.is_null() { return None; }

        let wrapper = WrapperDrmModeCrtcPtr { ptr, lib: LibDrm::new().unwrap() };

        Some(unsafe { wrapper.ptr.read() })
    }

    #[cfg(feature = "link_drm")]
    pub fn get_crtc_props(&self, fd: i32) -> Option<drmModeObjectProperties> {
        drmModeObjectProperties::get(
            fd,
            self.crtc_id,
            bindings::DRM_MODE_OBJECT_CRTC,
        )
    }

    pub fn mode_valid(&self) -> bool {
        self.mode_valid == 1
    }
}

impl Drop for WrapperDrmModeCrtcPtr {
    fn drop(&mut self) {
        #[cfg(feature = "link_drm")]
        let func = bindings::drmModeFreeCrtc;
        #[cfg(feature = "dynamic_loading")]
        let func = self.lib.libdrm.drmModeFreeCrtc;

	    unsafe { func(self.ptr); }
    }
}
