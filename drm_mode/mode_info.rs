use crate::bindings;
pub use bindings::drmModeModeInfo;
use bindings::{
    DRM_MODE_TYPE_PREFERRED,
    DRM_MODE_TYPE_USERDEF,
    DRM_MODE_TYPE_DRIVER,
};
use bindings::{
    DRM_MODE_FLAG_PHSYNC,
    DRM_MODE_FLAG_NHSYNC,
    DRM_MODE_FLAG_PVSYNC,
    DRM_MODE_FLAG_NVSYNC,
    DRM_MODE_FLAG_INTERLACE,
    DRM_MODE_FLAG_DBLSCAN,
    DRM_MODE_FLAG_CSYNC,
    DRM_MODE_FLAG_PCSYNC,
    DRM_MODE_FLAG_NCSYNC,
    DRM_MODE_FLAG_HSKEW,
    DRM_MODE_FLAG_DBLCLK,
    DRM_MODE_FLAG_CLKDIV2,
};

macro_rules! impl_mode_info {
    ($name: tt, $flag: expr) => {
        pub fn $name(&self) -> bool {
            (self.type_ & $flag) != 0
        }
    }
}

macro_rules! impl_flag {
    ($name: tt, $flag: expr) => {
        pub fn $name(&self) -> bool {
            (self.flags & $flag) != 0
        }
    }
}

impl drmModeModeInfo {
    pub fn name(&self) -> String {
        super::c_char_to_string(&self.name)
    }

    // ref: https://gitlab.freedesktop.org/mesa/drm/-/blob/main/tests/modetest/modetest.c
    pub fn refresh_rate(&self) -> f32 {
        let mut num = self.clock as f32;
        let mut den = self.htotal as f32 * self.vtotal as f32;

        if self.is_interlace() {
            num *= 2.0;
        }

        if self.is_dblscan() {
            den *= 2.0;
        }

        if self.vscan > 1 {
            den *= self.vscan as f32;
        }

        (num * 1000.0) / den
    }

    impl_mode_info!(type_is_preferred, DRM_MODE_TYPE_PREFERRED);
    impl_mode_info!(type_is_userdef, DRM_MODE_TYPE_USERDEF);
    impl_mode_info!(type_is_driver, DRM_MODE_TYPE_DRIVER);
    impl_flag!(is_phsync, DRM_MODE_FLAG_PHSYNC);
    impl_flag!(is_nhsync, DRM_MODE_FLAG_NHSYNC);
    impl_flag!(is_pvsync, DRM_MODE_FLAG_PVSYNC);
    impl_flag!(is_nvsync, DRM_MODE_FLAG_NVSYNC);
    impl_flag!(is_interlace, DRM_MODE_FLAG_INTERLACE);
    impl_flag!(is_dblscan, DRM_MODE_FLAG_DBLSCAN);
    impl_flag!(is_csync, DRM_MODE_FLAG_CSYNC);
    impl_flag!(is_pcsync, DRM_MODE_FLAG_PCSYNC);
    impl_flag!(is_ncsync, DRM_MODE_FLAG_NCSYNC);
    impl_flag!(is_hskew, DRM_MODE_FLAG_HSKEW);
    impl_flag!(is_dblclk, DRM_MODE_FLAG_DBLCLK);
    impl_flag!(is_clkdiv2, DRM_MODE_FLAG_CLKDIV2);
}
