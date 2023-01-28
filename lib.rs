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

/* TODO: CStr::from_bytes_until_nul */
#[cfg(feature = "std")]
pub trait BindingsStr {
    fn null_ctrl_to_space(&self) -> Vec<u8>;
}

#[cfg(feature = "std")]
impl BindingsStr for Vec<u8> {
    fn null_ctrl_to_space(&self) -> Vec<u8> {
        let mut null_char_flag = false;

        self
            .iter()
            .map(|&v| {
                /* '\0' */
                if v == 0 {
                    null_char_flag = true;
                }

                /* replace from <Control> \u0020 (<Space>) */
                if null_char_flag || char::from(v).is_control() {
                    0x20
                } else {
                    v
                }
            })
            .collect()
    }
}

pub unsafe fn drmGetVersion(fd: ::core::ffi::c_int) -> bindings::_drmVersion {
    let drm_ver = bindings::drmGetVersion(fd);

    return *drm_ver;
}
