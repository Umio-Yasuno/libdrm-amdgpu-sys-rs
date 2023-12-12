use crate::bindings;
use core::ptr::addr_of;

pub use bindings::{drmModePropertyPtr, drm_mode_property_enum};

#[derive(Debug, Clone)]
pub struct drmModeProperty(pub(crate) drmModePropertyPtr);

impl drmModeProperty {
    pub fn get(fd: i32, property_id: u32) -> Option<Self> {
        let prop_ptr = unsafe { bindings::drmModeGetProperty(
            fd,
            property_id,
        ) };

        if prop_ptr.is_null() {
            None
        } else {
            Some(Self(prop_ptr))
        }
    }

    pub fn name(&self) -> String {
        let c_name = unsafe { addr_of!((*self.0).name).read() };

        c_char_to_string(&c_name)
    }

    pub fn prop_id(&self) -> u32 {
        unsafe { addr_of!((*self.0).prop_id).read() }
    }

    pub fn flags(&self) -> u32 {
        unsafe { addr_of!((*self.0).flags).read() }
    }

    pub fn property_type(&self) -> drmModePropType {
        let flags = self.flags();
        let type_ = flags & (bindings::DRM_MODE_PROP_LEGACY_TYPE | bindings::DRM_MODE_PROP_EXTENDED_TYPE);

        drmModePropType::from(type_)
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
        unsafe { std::slice::from_raw_parts(
            addr_of!((*self.0).values).read(),
            addr_of!((*self.0).count_values).read() as usize,
        ) }.to_vec()
    }

    pub fn blob_ids(&self) -> Vec<u32> {
        unsafe { std::slice::from_raw_parts(
            addr_of!((*self.0).blob_ids).read(),
            addr_of!((*self.0).count_blobs).read() as usize,
        ) }.to_vec()
    }

    pub fn enums(&self) -> Vec<drm_mode_property_enum> {
        unsafe { std::slice::from_raw_parts(
            addr_of!((*self.0).enums).read(),
            addr_of!((*self.0).count_enums).read() as usize,
        ) }.to_vec()
    }
}

impl Drop for drmModeProperty {
    fn drop(&mut self) {
	    unsafe { bindings::drmModeFreeProperty(self.0); }
    }
}

fn c_char_to_string(c: &[core::ffi::c_char]) -> String {
    let c_name: Vec<u8> = c.iter().map(|c| c.unsigned_abs()).collect();

    if let Some(index) = c_name.iter().position(|&x| x == 0) {
        String::from_utf8_lossy(c_name.get(..index).unwrap_or_default())
    } else {
        String::from_utf8_lossy(&c_name)
    }.to_string()
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
    UNKNOWN,
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
            _ => Self::UNKNOWN,
        }
    }
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for drmModePropType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl drm_mode_property_enum {
    pub fn name(&self) -> String {
        c_char_to_string(&self.name)
    }
}
