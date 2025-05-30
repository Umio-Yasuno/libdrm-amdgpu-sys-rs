use crate::{bindings, LibDrm};
use core::ptr::addr_of;

pub use bindings::{drmModePropertyPtr, drm_mode_property_enum};

#[allow(dead_code)]
#[derive(Clone)]
pub struct drmModeProperty {
    pub(crate) ptr: drmModePropertyPtr,
    pub(crate) lib: LibDrm,
}

impl LibDrm {
    pub fn get_drm_mode_property(&self, fd: i32, property_id: u32) -> Option<drmModeProperty> {
        #[cfg(feature = "link_drm")]
        let func = bindings::drmModeGetProperty;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm.drmModeGetProperty;

        let prop_ptr = unsafe { func(fd, property_id) };

        if prop_ptr.is_null() {
            None
        } else {
            Some(drmModeProperty { ptr: prop_ptr, lib: self.clone() })
        }
    }
}

impl drmModeProperty {
    #[cfg(feature = "link_drm")]
    pub fn get(fd: i32, property_id: u32) -> Option<Self> {
        let prop_ptr = unsafe { bindings::drmModeGetProperty(
            fd,
            property_id,
        ) };

        if prop_ptr.is_null() {
            None
        } else {
            Some(Self { ptr: prop_ptr, lib: LibDrm::new().unwrap() })
        }
    }

    pub fn name(&self) -> String {
        let c_name = unsafe { addr_of!((*self.ptr).name).read() };

        super::c_char_to_string(&c_name)
    }

    pub fn prop_id(&self) -> u32 {
        unsafe { addr_of!((*self.ptr).prop_id).read() }
    }

    pub fn flags(&self) -> u32 {
        unsafe { addr_of!((*self.ptr).flags).read() }
    }

    pub fn property_type(&self) -> drmModePropType {
        let flags = self.flags();
        let type_ = flags & (bindings::DRM_MODE_PROP_LEGACY_TYPE | bindings::DRM_MODE_PROP_EXTENDED_TYPE);

        drmModePropType::from(type_)
    }

    pub fn is_atomic(&self) -> bool {
        let flags = self.flags();
        (flags & DRM_MODE_PROP_ATOMIC) != 0
    }

    pub fn is_pending(&self) -> bool {
        let flags = self.flags();
        (flags & DRM_MODE_PROP_PENDING) != 0
    }

    pub fn is_immutable(&self) -> bool {
        let flags = self.flags();
        (flags & DRM_MODE_PROP_IMMUTABLE) != 0
    }

    pub fn values(&self) -> Vec<u64> {
        let ptr = unsafe { addr_of!((*self.ptr).values).read() };
        let count = unsafe { addr_of!((*self.ptr).count_values).read() as usize };

        if ptr.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(ptr, count) }.to_vec()
        }
    }

    pub fn blob_ids(&self) -> Vec<u32> {
        let ptr = unsafe { addr_of!((*self.ptr).blob_ids).read() };
        let count = unsafe { addr_of!((*self.ptr).count_blobs).read() as usize };

        if ptr.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(ptr, count) }.to_vec()
        }
    }

    pub fn enums(&self) -> Vec<drm_mode_property_enum> {
        let ptr = unsafe { addr_of!((*self.ptr).enums).read() };
        let count = unsafe { addr_of!((*self.ptr).count_enums).read() as usize };

        if ptr.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(ptr, count) }.to_vec()
        }
    }
}

impl Drop for drmModeProperty {
    fn drop(&mut self) {
        #[cfg(feature = "link_drm")]
        let func = bindings::drmModeFreeProperty;
        #[cfg(feature = "dynamic_loading")]
        let func = self.lib.libdrm.drmModeFreeProperty;

	    unsafe { func(self.ptr); }
    }
}

pub(crate) const DRM_MODE_PROP_OBJECT: u32 = 1 << 6;
pub(crate) const DRM_MODE_PROP_SIGNED_RANGE: u32 = 2 << 6;

use bindings::{
    DRM_MODE_PROP_PENDING,
    DRM_MODE_PROP_RANGE,
    DRM_MODE_PROP_IMMUTABLE,
    DRM_MODE_PROP_ENUM,
    DRM_MODE_PROP_BLOB,
    DRM_MODE_PROP_BITMASK,
    DRM_MODE_PROP_LEGACY_TYPE,
    DRM_MODE_PROP_EXTENDED_TYPE,
    DRM_MODE_PROP_ATOMIC,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum drmModePropType {
    RANGE = DRM_MODE_PROP_RANGE,
    ENUM = DRM_MODE_PROP_ENUM,
    BLOB = DRM_MODE_PROP_BLOB,
    BITMASK = DRM_MODE_PROP_BITMASK,
    LEGACY_TYPE = DRM_MODE_PROP_LEGACY_TYPE,
    EXTENDED_TYPE = DRM_MODE_PROP_EXTENDED_TYPE,
    ATOMIC = DRM_MODE_PROP_ATOMIC,
    OBJECT = DRM_MODE_PROP_OBJECT,
    SIGNED_RANGE = DRM_MODE_PROP_SIGNED_RANGE,
    UNKNOWN(u32),
}

impl From<u32> for drmModePropType {
    fn from(value: u32) -> Self {
        match value {
            DRM_MODE_PROP_RANGE => Self::RANGE,
            DRM_MODE_PROP_ENUM => Self::ENUM,
            DRM_MODE_PROP_BLOB => Self::BLOB,
            DRM_MODE_PROP_BITMASK => Self::BITMASK,
            DRM_MODE_PROP_LEGACY_TYPE => Self::LEGACY_TYPE,
            DRM_MODE_PROP_EXTENDED_TYPE => Self::EXTENDED_TYPE,
            DRM_MODE_PROP_ATOMIC => Self::ATOMIC,
            DRM_MODE_PROP_OBJECT => Self::OBJECT,
            DRM_MODE_PROP_SIGNED_RANGE => Self::SIGNED_RANGE,
            _ => Self::UNKNOWN(value),
        }
    }
}

use std::fmt;
impl fmt::Display for drmModePropType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl drm_mode_property_enum {
    pub fn name(&self) -> String {
        super::c_char_to_string(&self.name)
    }
}
