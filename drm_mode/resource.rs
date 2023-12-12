use crate::{bindings, drmModeConnector};
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
        let connectors = unsafe { std::slice::from_raw_parts(
            addr_of!((*self.0).connectors).read(),
            addr_of!((*self.0).count_connectors).read() as usize,
        ) };

        connectors.iter().filter_map(|connector_id| {
            drmModeConnector::get_current(fd, *connector_id)
        }).collect()
    }

    pub fn get_all_connector(&self, fd: i32) -> Vec<drmModeConnector> {
        let connectors = unsafe { std::slice::from_raw_parts(
            addr_of!((*self.0).connectors).read(),
            addr_of!((*self.0).count_connectors).read() as usize,
        ) };

        connectors.iter().filter_map(|connector_id| {
            drmModeConnector::get(fd, *connector_id)
        }).collect()
    }
}

impl Drop for drmModeRes {
    fn drop(&mut self) {
	    unsafe { bindings::drmModeFreeResources(self.0); }
    }
}
