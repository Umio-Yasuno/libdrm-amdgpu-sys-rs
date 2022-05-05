#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]

mod bindings {
    include!("./bindings/drm.rs");
}

#[path = "./amdgpu/"]
pub mod AMDGPU {
    #[path = "amdgpu.rs"]
    mod amdgpu;
    pub use amdgpu::*;

    #[path = "amdgpu_family.rs"]
    mod amdgpu_family;
    pub use amdgpu_family::*;

    #[path = "amdgpu_vram.rs"]
    mod amdgpu_vram;
    pub use amdgpu_vram::*;

    #[path = "amdgpu_asic.rs"]
    mod amdgpu_asic;
    pub use amdgpu_asic::*;

    #[path = "amdgpu_chip_class.rs"]
    mod amdgpu_chip_class;
    pub use amdgpu_chip_class::*;
}

#[macro_export]
macro_rules! query_error {
    ($r: expr) => {
        if $r != 0 {
            return Err($r);
        }
    };
}

pub fn drmGetVersion(fd: ::std::os::raw::c_int) -> bindings::_drmVersion {
    unsafe {
        let drm_ver = bindings::drmGetVersion(fd);

        return bindings::_drmVersion {
                version_major:      (*drm_ver).version_major,
                version_minor:      (*drm_ver).version_minor,
                version_patchlevel: (*drm_ver).version_patchlevel,
                name_len:           (*drm_ver).name_len,
                name:               (*drm_ver).name,
                date_len:           (*drm_ver).date_len,
                date:               (*drm_ver).date,
                desc_len:           (*drm_ver).desc_len,
                desc:               (*drm_ver).desc,
        };
    }
}
