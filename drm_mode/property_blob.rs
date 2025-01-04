use crate::{bindings, LibDrm};
use core::ptr::addr_of;

pub use bindings::drmModePropertyBlobPtr;

#[allow(dead_code)]
#[derive(Clone)]
pub struct drmModePropertyBlob {
    pub(crate) ptr: drmModePropertyBlobPtr,
    pub(crate) lib: LibDrm,
}

impl LibDrm {
    pub fn get_drm_mode_property_blob(&self, fd: i32, blob_id: u32) -> Option<drmModePropertyBlob> {
        #[cfg(feature = "link_drm")]
        let func = bindings::drmModeGetPropertyBlob;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm.drmModeGetPropertyBlob;

        let blob_ptr = unsafe { func(fd, blob_id) };

        if blob_ptr.is_null() {
            None
        } else {
            Some(drmModePropertyBlob { ptr: blob_ptr, lib: self.clone() })
        }
    }
}

impl drmModePropertyBlob {
    #[cfg(feature = "link_drm")]
    pub fn get(fd: i32, blob_id: u32) -> Option<Self> {
        let blob_ptr = unsafe { bindings::drmModeGetPropertyBlob(
            fd,
            blob_id,
        ) };

        if blob_ptr.is_null() {
            None
        } else {
            Some(Self { ptr: blob_ptr, lib: LibDrm::new().unwrap() })
        }
    }

    pub fn id(&self) -> u32 {
        unsafe { addr_of!((*self.ptr).id).read() }
    }

    pub fn length(&self) -> u32 {
        unsafe { addr_of!((*self.ptr).length).read() }
    }

    pub fn data(&self) -> Vec<u8> {
        let ptr = unsafe { addr_of!((*self.ptr).data).read() };
        let len = self.length() as usize;

        if ptr.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(ptr as *const u8, len) }.to_vec()
        }
    }
}

impl Drop for drmModePropertyBlob {
    fn drop(&mut self) {
        #[cfg(feature = "link_drm")]
        let func = bindings::drmModeFreePropertyBlob;
        #[cfg(feature = "dynamic_loading")]
        let func = self.lib.libdrm.drmModeFreePropertyBlob;

	    unsafe { func(self.ptr); }
    }
}
