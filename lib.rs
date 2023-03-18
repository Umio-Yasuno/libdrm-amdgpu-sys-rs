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
#[path = "./"]
pub mod AMDGPU {
    #[path = "amdgpu_mod.rs"]
    mod amdgpu_mod;
    pub use amdgpu_mod::*;
}

#[cfg(not(feature = "buildtime_bindgen"))]
mod pci_bus_info;
#[cfg(not(feature = "buildtime_bindgen"))]
pub use pci_bus_info::*;
