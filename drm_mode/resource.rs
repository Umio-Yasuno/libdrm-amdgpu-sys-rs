use crate::{bindings, drmModeConnector, drmModeCrtc, LibDrm};
use core::ptr::addr_of;

pub use bindings::{drmModeResPtr, drmModeObjectPropertiesPtr, drmModePropertyPtr};

#[derive(Clone)]
pub struct drmModeRes {
    pub(crate) ptr: drmModeResPtr,
    pub(crate) lib: LibDrm,
}

impl LibDrm {
    pub fn get_drm_mode_resources(&self, fd: i32) -> Option<drmModeRes> {
        #[cfg(feature = "link_drm")]
        let func = bindings::drmModeGetResources;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm.drmModeGetResources;

        let drm_mode_res_ptr = unsafe { func(fd) };

        if drm_mode_res_ptr.is_null() { return None; }

        Some(drmModeRes {
            ptr: drm_mode_res_ptr,
            lib: self.clone(),
        })
    }
}

impl drmModeRes {
    pub fn get_drm_mode_all_connector_current(&self, fd: i32) -> Vec<drmModeConnector> {
        let ptr = unsafe { addr_of!((*self.ptr).connectors).read() };

        if ptr.is_null() {
            return Vec::new();
        }

        let count = unsafe { addr_of!((*self.ptr).count_connectors).read() as usize };
        let connectors = unsafe { std::slice::from_raw_parts(ptr, count) };

        connectors.iter().filter_map(|connector_id| {
            self.lib.get_drm_mode_connector_current(fd, *connector_id)
        }).collect()
    }

    pub fn get_drm_mode_all_connector(&self, fd: i32) -> Vec<drmModeConnector> {
        let ptr = unsafe { addr_of!((*self.ptr).connectors).read() };

        if ptr.is_null() {
            return Vec::new();
        }

        let count = unsafe { addr_of!((*self.ptr).count_connectors).read() as usize };
        let connectors = unsafe { std::slice::from_raw_parts(ptr, count) };

        connectors.iter().filter_map(|connector_id| {
            self.lib.get_drm_mode_connector(fd, *connector_id)
        }).collect()
    }

    pub fn get_drm_mode_all_crtcs(&self, fd: i32) -> Vec<drmModeCrtc> {
        let ptr = unsafe { addr_of!((*self.ptr).crtcs).read() };

        if ptr.is_null() {
            return Vec::new();
        }

        let count = unsafe { addr_of!((*self.ptr).count_crtcs).read() as usize };
        let crtcs = unsafe { std::slice::from_raw_parts(ptr, count) };

        crtcs.iter().filter_map(|crtc_id| {
            self.lib.get_drm_mode_crtc(fd, *crtc_id)
        }).collect()
    }
}

#[cfg(feature = "link_drm")]
impl drmModeRes {
    pub fn get(fd: i32) -> Option<Self> {
        let drm_mode_res_ptr = unsafe { bindings::drmModeGetResources(fd) };

        if drm_mode_res_ptr.is_null() { return None; }

        Some(Self { ptr: drm_mode_res_ptr, lib: LibDrm::new().unwrap() })
    }

    pub fn get_all_connector_current(&self, fd: i32) -> Vec<drmModeConnector> {
        let ptr = unsafe { addr_of!((*self.ptr).connectors).read() };

        if ptr.is_null() {
            return Vec::new();
        }

        let count = unsafe { addr_of!((*self.ptr).count_connectors).read() as usize };
        let connectors = unsafe { std::slice::from_raw_parts(ptr, count) };

        connectors.iter().filter_map(|connector_id| {
            drmModeConnector::get_current(fd, *connector_id)
        }).collect()
    }

    pub fn get_all_connector(&self, fd: i32) -> Vec<drmModeConnector> {
        let ptr = unsafe { addr_of!((*self.ptr).connectors).read() };

        if ptr.is_null() {
            return Vec::new();
        }

        let count = unsafe { addr_of!((*self.ptr).count_connectors).read() as usize };
        let connectors = unsafe { std::slice::from_raw_parts(ptr, count) };

        connectors.iter().filter_map(|connector_id| {
            drmModeConnector::get(fd, *connector_id)
        }).collect()
    }

    pub fn get_all_crtcs(&self, fd: i32) -> Vec<drmModeCrtc> {
        let ptr = unsafe { addr_of!((*self.ptr).crtcs).read() };

        if ptr.is_null() {
            return Vec::new();
        }

        let count = unsafe { addr_of!((*self.ptr).count_crtcs).read() as usize };
        let crtcs = unsafe { std::slice::from_raw_parts(ptr, count) };

        crtcs.iter().filter_map(|crtc_id| {
            drmModeCrtc::get(fd, *crtc_id)
        }).collect()
    }
}

impl Drop for drmModeRes {
    fn drop(&mut self) {
        #[cfg(feature = "link_drm")]
        let func = bindings::drmModeFreeResources;
        #[cfg(feature = "dynamic_loading")]
        let func = self.lib.libdrm.drmModeFreeResources;

	    unsafe { func(self.ptr); }
    }
}
