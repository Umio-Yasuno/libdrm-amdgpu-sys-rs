use crate::bindings;
use core::ptr::addr_of;

pub use bindings::drmModePropertyBlobPtr;

#[derive(Debug, Clone)]
pub struct drmModePropertyBlob(pub(crate) drmModePropertyBlobPtr);

impl drmModePropertyBlob {
    pub fn get(fd: i32, blob_id: u32) -> Option<Self> {
        let blob_ptr = unsafe { bindings::drmModeGetPropertyBlob(
            fd,
            blob_id,
        ) };

        if blob_ptr.is_null() {
            None
        } else {
            Some(Self(blob_ptr))
        }
    }

    pub fn id(&self) -> u32 {
        unsafe { addr_of!((*self.0).id).read() }
    }

    pub fn length(&self) -> u32 {
        unsafe { addr_of!((*self.0).length).read() }
    }

    pub fn data(&self) -> Vec<u8> {
        let ptr = unsafe { addr_of!((*self.0).data).read() };
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
	    unsafe { bindings::drmModeFreePropertyBlob(self.0); }
    }
}
