#![cfg_attr(not(feature = "std"), no_std)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]

mod bindings {
    include!("./bindings/drm.rs");
}

#[path = "./"]
pub mod AMDGPU {
    #[path = "amdgpu_mod.rs"]
    mod amdgpu_mod;
    pub use amdgpu_mod::*;
}

#[cfg(feature = "std")]
mod pci_bus_info;
#[cfg(feature = "std")]
pub use pci_bus_info::*;

pub unsafe fn drmGetVersion(fd: ::core::ffi::c_int) -> bindings::_drmVersion {
    let drm_ver = bindings::drmGetVersion(fd);

    return *drm_ver;
}
