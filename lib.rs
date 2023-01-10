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

#[path = "./pci_bus_info.rs"]
mod pci_bus_info;
pub use pci_bus_info::*;

/* TODO: CStr::from_bytes_until_nul */
pub trait BindingsStr {
    fn null_ctrl_to_space(&self) -> Vec<u8>;
}

impl BindingsStr for Vec<u8> {
    fn null_ctrl_to_space(&self) -> Vec<u8> {
        let mut null_char_flag = false;

        let tmp: Vec<u8> = self
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
            .collect();

        return tmp;
    }
}

pub unsafe fn drmGetVersion(fd: ::std::os::raw::c_int) -> bindings::_drmVersion {
    let drm_ver = bindings::drmGetVersion(fd);

    return *drm_ver;
}
