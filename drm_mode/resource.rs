use crate::{bindings, drmModeConnector, drmModeCrtc};
use core::ptr::addr_of;

pub use bindings::{drmModeResPtr, drmModeObjectPropertiesPtr, drmModePropertyPtr};

#[derive(Debug, Clone)]
pub struct drmModeRes(pub(crate) drmModeResPtr);

impl drmModeRes {
    pub fn get(fd: i32) -> Option<Self> {
        let drm_mode_res_ptr = unsafe { bindings::drmModeGetResources(fd) };

        if drm_mode_res_ptr.is_null() { return None; }

        Some(Self(drm_mode_res_ptr))
    }

    pub fn get_all_connector_current(&self, fd: i32) -> Vec<drmModeConnector> {
        let ptr = unsafe { addr_of!((*self.0).connectors).read() };

        if ptr.is_null() {
            return Vec::new();
        }

        let count = unsafe { addr_of!((*self.0).count_connectors).read() as usize };
        let connectors = unsafe { std::slice::from_raw_parts(ptr, count) };

        connectors.iter().filter_map(|connector_id| {
            drmModeConnector::get_current(fd, *connector_id)
        }).collect()
    }

    pub fn get_all_connector(&self, fd: i32) -> Vec<drmModeConnector> {
        let ptr = unsafe { addr_of!((*self.0).connectors).read() };

        if ptr.is_null() {
            return Vec::new();
        }

        let count = unsafe { addr_of!((*self.0).count_connectors).read() as usize };
        let connectors = unsafe { std::slice::from_raw_parts(ptr, count) };

        connectors.iter().filter_map(|connector_id| {
            drmModeConnector::get(fd, *connector_id)
        }).collect()
    }

    pub fn get_all_crtcs(&self, fd: i32) -> Vec<drmModeCrtc> {
        let ptr = unsafe { addr_of!((*self.0).crtcs).read() };

        if ptr.is_null() {
            return Vec::new();
        }

        let count = unsafe { addr_of!((*self.0).count_crtcs).read() as usize };
        let crtcs = unsafe { std::slice::from_raw_parts(ptr, count) };

        crtcs.iter().filter_map(|crtc_id| {
            drmModeCrtc::get(fd, *crtc_id)
        }).collect()
    }
}

impl Drop for drmModeRes {
    fn drop(&mut self) {
	    unsafe { bindings::drmModeFreeResources(self.0); }
    }
}
