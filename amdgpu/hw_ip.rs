use crate::AMDGPU::*;
use crate::*;

pub use crate::bindings::drm_amdgpu_info_hw_ip;
use core::mem::MaybeUninit;

impl DeviceHandle {
    pub fn query_hw_ip_count(
        &self,
        type_: HW_IP_TYPE,
    ) -> Result<u32, i32> {
        unsafe {
            let mut hw_ip_count: MaybeUninit<u32> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_hw_ip_count(
                self.0,
                type_ as ::core::ffi::c_uint,
                hw_ip_count.as_mut_ptr(),
            );

            let hw_ip_count = hw_ip_count.assume_init();

            query_error!(r);

            Ok(hw_ip_count)
        }
    }

    /// Note: `ip_instance` must be less than `AMDGPU_HW_IP_INSTANCE_MAX_COUNT` (`0` recommended)
    pub fn query_hw_ip_info(
        &self,
        type_: HW_IP_TYPE,
        ip_instance: ::core::ffi::c_uint,
    ) -> Result<drm_amdgpu_info_hw_ip, i32> {
        unsafe {
            let mut hw_ip_info: MaybeUninit<drm_amdgpu_info_hw_ip> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_hw_ip_info(
                self.0,
                type_ as ::core::ffi::c_uint,
                ip_instance as ::core::ffi::c_uint,
                hw_ip_info.as_mut_ptr(),
            );

            let hw_ip_info = hw_ip_info.assume_init();

            query_error!(r);

            Ok(hw_ip_info)
        }
    }
}

impl drm_amdgpu_info_hw_ip {
    pub fn num_queues(&self) -> u32 {
        self.available_rings.count_ones()
    }
    pub fn version(&self) -> (u32, u32) {
        (self.hw_ip_version_major, self.hw_ip_version_minor)
    }
}

use crate::bindings::{
    AMDGPU_HW_IP_COMPUTE,
    AMDGPU_HW_IP_DMA,
    AMDGPU_HW_IP_GFX,
    AMDGPU_HW_IP_UVD,
    AMDGPU_HW_IP_UVD_ENC,
    AMDGPU_HW_IP_VCE,
    AMDGPU_HW_IP_VCN_DEC,
    AMDGPU_HW_IP_VCN_ENC,
    AMDGPU_HW_IP_VCN_JPEG,
    // AMDGPU_HW_IP_NUM,
    // AMDGPU_HW_IP_INSTANCE_MAX_COUNT,
};

/// Used for [DeviceHandle::query_hw_ip_info] and [DeviceHandle::query_hw_ip_count]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum HW_IP_TYPE {
    GFX = AMDGPU_HW_IP_GFX,
    COMPUTE = AMDGPU_HW_IP_COMPUTE,
    DMA = AMDGPU_HW_IP_DMA,
    UVD = AMDGPU_HW_IP_UVD,
    VCE = AMDGPU_HW_IP_VCE,
    UVD_ENC = AMDGPU_HW_IP_UVD_ENC,
    VCN_DEC = AMDGPU_HW_IP_VCN_DEC,
    VCN_ENC = AMDGPU_HW_IP_VCN_ENC,
    VCN_JPEG = AMDGPU_HW_IP_VCN_JPEG,
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for HW_IP_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
