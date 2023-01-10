use crate::AMDGPU::*;
use crate::*;

use crate::bindings::drm_amdgpu_info_hw_ip;
use std::mem::MaybeUninit;

impl DeviceHandle {
    pub fn query_hw_ip_count(
        &self,
        type_: HW_IP_TYPE,
    ) -> Result<u32, i32> {
        unsafe {
            let mut hw_ip_count: MaybeUninit<u32> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_hw_ip_count(
                self.0,
                type_ as ::std::os::raw::c_uint,
                hw_ip_count.as_mut_ptr(),
            );

            query_error!(r);

            return Ok(hw_ip_count.assume_init());
        }
    }

    pub fn query_hw_ip_info(
        &self,
        type_: HW_IP_TYPE,
        ip_instance: ::std::os::raw::c_uint,
    ) -> Result<drm_amdgpu_info_hw_ip, i32> {
        unsafe {
            let mut hw_ip_info: MaybeUninit<drm_amdgpu_info_hw_ip> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_hw_ip_info(
                self.0,
                type_ as ::std::os::raw::c_uint,
                ip_instance as ::std::os::raw::c_uint,
                hw_ip_info.as_mut_ptr(),
            );

            query_error!(r);

            return Ok(hw_ip_info.assume_init());
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

use std::fmt;
impl fmt::Display for HW_IP_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
