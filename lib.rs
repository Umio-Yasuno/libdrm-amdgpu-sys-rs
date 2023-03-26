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
mod pci_bus_info;
#[cfg(not(feature = "buildtime_bindgen"))]
pub use pci_bus_info::*;

#[macro_export]
macro_rules! query_error {
    ($r: expr) => {
        if $r != 0 {
            return Err($r);
        }
    };
}
