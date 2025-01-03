#![doc = include_str!("./README.md")]

#![cfg_attr(not(feature = "std"), no_std)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(clippy::all)]
#[cfg(not(feature = "buildtime_bindgen"))]
mod bindings {
    #[cfg(feature = "link-drm")]
    mod drm;
    #[cfg(feature = "link-drm")]
    pub use drm::*;

    #[cfg(feature = "dynamic_loading")]
    mod dyn_drm;
    #[cfg(feature = "dynamic_loading")]
    pub use dyn_drm::*;

    #[cfg(feature = "dynamic_loading")]
    mod dyn_drm_amdgpu;
    #[cfg(feature = "dynamic_loading")]
    pub use dyn_drm_amdgpu::*;

    mod amdgpu_ids;
    pub use amdgpu_ids::AMDGPU_IDS;

    pub mod ppt {
        pub mod smu_v11_0_0_ppt;
        pub mod smu_v11_0_7_ppt;
        pub mod smu_v13_0_0_ppt;
        pub mod smu_v13_0_7_ppt;
    }
}

#[cfg(feature = "dynamic_loading")]
use std::sync::Arc;
#[cfg(feature = "dynamic_loading")]
use bindings::{DynLibDrm, DynLibDrmAmdgpu};

#[cfg(feature = "dynamic_loading")]
pub struct LibDrm {
    pub(crate) libdrm: Arc<DynLibDrm>,
}

#[cfg(feature = "dynamic_loading")]
impl LibDrm {
    pub fn new() -> Result<Self, ::libloading::Error> {
        let libdrm = unsafe { Arc::new(DynLibDrm::new("libdrm.so")?) };

        Ok(Self { libdrm })
    }
}

#[cfg(feature = "dynamic_loading")]
impl From<LibDrmAmdgpu> for LibDrm {
    fn from(lib: LibDrmAmdgpu) -> Self {
        Self { libdrm: lib.libdrm.clone() }
    }
}

#[cfg(feature = "dynamic_loading")]
pub struct LibDrmAmdgpu {
    pub(crate) libdrm: Arc<DynLibDrm>,
    pub(crate) libdrm_amdgpu: Arc<DynLibDrmAmdgpu>,
}

#[cfg(feature = "dynamic_loading")]
impl LibDrmAmdgpu {
    pub fn new() -> Result<Self, ::libloading::Error> {
        let libdrm = unsafe { Arc::new(DynLibDrm::new("libdrm.so")?) };
        let libdrm_amdgpu = unsafe { Arc::new(DynLibDrmAmdgpu::new("libdrm_amdgpu.so")?) };

        Ok(Self { libdrm, libdrm_amdgpu })
    }

    pub fn new_with_libdrm(lib: LibDrm) -> Result<Self, ::libloading::Error> {
        let libdrm_amdgpu = unsafe { Arc::new(DynLibDrmAmdgpu::new("libdrm_amdgpu.so")?) };

        Ok(Self { libdrm: lib.libdrm.clone(), libdrm_amdgpu })
    }
}

#[cfg(not(feature = "buildtime_bindgen"))]
#[cfg(feature = "std")]
mod amdgpu;
#[cfg(not(feature = "buildtime_bindgen"))]
#[cfg(feature = "std")]
pub mod AMDGPU {
    pub use super::amdgpu::*;
}

#[cfg(not(feature = "buildtime_bindgen"))]
#[cfg(feature = "std")]
mod pci;
#[cfg(not(feature = "buildtime_bindgen"))]
#[cfg(feature = "std")]
pub mod PCI {
    pub use super::pci::*;
}

#[cfg(not(feature = "buildtime_bindgen"))]
#[cfg(feature = "std")]
mod drm_version;
#[cfg(not(feature = "buildtime_bindgen"))]
#[cfg(feature = "std")]
pub use drm_version::*;

/// TODO: dynamic link
#[cfg(not(feature = "buildtime_bindgen"))]
#[cfg(feature = "std")]
#[cfg(feature = "link-drm")]
mod drm_mode;
/// TODO: dynamic link
#[cfg(all(feature = "std", feature = "link-drm"))]
#[cfg(feature = "std")]
#[cfg(feature = "link-drm")]
pub use drm_mode::*;

/// Convert `errno` to `Err(i32)`
#[macro_export]
macro_rules! query_error {
    ($r: expr) => {
        if $r != 0 {
            return Err($r);
        }
    };
}

#[cfg(feature = "std")]
use std::path::PathBuf;

#[cfg(feature = "std")]
pub(crate) fn get_min_max_from_dpm<
    T: std::cmp::Ord + std::marker::Copy,
    P: Into<PathBuf>
>(
    sysfs_path: P,
    parse: fn(&str) -> Option<T>,
) -> Option<[T; 2]> {
    let sysfs_path = sysfs_path.into();
    let s = std::fs::read_to_string(sysfs_path).ok()?;
    let mut lines = s.lines();

    let first = parse(lines.next()?)?;
    let last = match lines.last() {
        Some(last) => parse(last)?,
        None => return Some([first; 2]),
    };

    Some([
        std::cmp::min(first, last),
        std::cmp::max(first, last),
    ])
}
