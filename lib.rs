#![doc = include_str!("./README.md")]

#![cfg_attr(not(feature = "std"), no_std)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(dead_code)]
#[cfg(not(feature = "buildtime_bindgen"))]
mod bindings {
    mod drm;
    pub use drm::*;
}

#[cfg(not(feature = "buildtime_bindgen"))]
mod amdgpu;
#[cfg(not(feature = "buildtime_bindgen"))]
pub mod AMDGPU {
    pub use super::amdgpu::*;
}

#[cfg(not(feature = "buildtime_bindgen"))]
mod pci;
#[cfg(not(feature = "buildtime_bindgen"))]
pub mod PCI {
    pub use super::pci::*;
}

#[cfg(not(feature = "buildtime_bindgen"))]
mod drm_version;
#[cfg(not(feature = "buildtime_bindgen"))]
pub use drm_version::*;

#[cfg(not(feature = "buildtime_bindgen"))]
mod drm_mode;
#[cfg(not(feature = "buildtime_bindgen"))]
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
