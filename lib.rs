#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]

mod bindings {
    include!("./bindings/drm.rs");
}

#[path = "./amdgpu/"]
pub mod AMDGPU {
    #[path = "amdgpu_mod.rs"]
    mod amdgpu_mod;
    pub use amdgpu_mod::*;
}

#[path = "./pci_bus_info.rs"]
mod pci_bus_info;
pub use pci_bus_info::*;

pub fn null_control_to_space(src: Vec<u8>) -> Vec<u8> {
    let mut flag = false;

    let tmp: Vec<u8> = src.iter().map(|&v| {
        if v == 0 { flag = true; }

        if flag || v < 0x20 {
            0x20
        } else {
            v
        }
    }).collect();

    return tmp;
}

pub unsafe fn drmGetVersion(fd: ::std::os::raw::c_int) -> bindings::_drmVersion {
    let drm_ver = bindings::drmGetVersion(fd);

    return *drm_ver;
}
