mod resource;
pub use resource::*;

mod connector;
pub use connector::*;

mod crtc;
pub use crtc::*;

mod object_property;
pub use object_property::*;

mod property;
pub use property::*;

mod property_blob;
pub use property_blob::*;

mod mode_info;
#[allow(unused_imports)]
pub use mode_info::*;

// TODO: encoder

pub(crate) fn c_char_to_string(c: &[core::ffi::c_char]) -> String {
    let c_name: Vec<u8> = c.iter().map(|c| *c as u8).collect();

    if let Some(index) = c_name.iter().position(|&x| x == 0) {
        String::from_utf8_lossy(c_name.get(..index).unwrap_or_default())
    } else {
        String::from_utf8_lossy(&c_name)
    }.to_string()
}

use crate::bindings;

pub use crate::bindings::{
    DRM_CLIENT_CAP_STEREO_3D,
    DRM_CLIENT_CAP_UNIVERSAL_PLANES,
    DRM_CLIENT_CAP_ATOMIC,
    DRM_CLIENT_CAP_ASPECT_RATIO,
    DRM_CLIENT_CAP_WRITEBACK_CONNECTORS,
    DRM_CLIENT_CAP_CURSOR_PLANE_HOTSPOT,
};

#[cfg(feature = "link-drm")]
pub fn set_client_caps(fd: i32, cap: u64, val: u64) -> i32 {
    unsafe { bindings::drmSetClientCap(fd, cap, val) }
}

#[cfg(feature = "link-drm")]
pub fn set_all_client_caps(fd: i32) {
    for cap in [
        DRM_CLIENT_CAP_STEREO_3D,
        DRM_CLIENT_CAP_UNIVERSAL_PLANES,
        DRM_CLIENT_CAP_ATOMIC,
        DRM_CLIENT_CAP_ASPECT_RATIO,
        DRM_CLIENT_CAP_WRITEBACK_CONNECTORS,
        DRM_CLIENT_CAP_CURSOR_PLANE_HOTSPOT,
    ] {
        let _ = unsafe { bindings::drmSetClientCap(fd, cap as u64, 1) };
    }
}
