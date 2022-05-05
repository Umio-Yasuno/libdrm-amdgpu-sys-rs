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

pub unsafe fn drmGetVersion(fd: ::std::os::raw::c_int) -> bindings::_drmVersion {
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
