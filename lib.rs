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

mod pci_bus_info;
pub use pci_bus_info::*;
