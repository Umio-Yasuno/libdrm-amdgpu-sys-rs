use crate::AMDGPU::*;
use crate::*;

use core::mem::MaybeUninit;

#[derive(Debug, Clone, Copy)]
pub struct FwVer {
    pub fw_type: FW_TYPE,
    pub ip_instance: u32,
    pub index: u32,
    pub version: u32,
    pub feature: u32,
}

impl DeviceHandle {
    /// Note: `ip_instance` must be `0`.
    pub fn query_firmware_version(
        &self,
        fw_type: FW_TYPE,
        ip_instance: ::core::ffi::c_uint,
        index: ::core::ffi::c_uint,
    ) -> Result<FwVer, i32> {
        unsafe {
            let mut version: MaybeUninit<u32> = MaybeUninit::zeroed();
            let mut feature: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_firmware_version(
                self.0,
                fw_type as u32,
                ip_instance,
                index,
                version.as_mut_ptr(),
                feature.as_mut_ptr(),
            );

            let fw_ver = FwVer {
                fw_type,
                ip_instance,
                index,
                version: version.assume_init(),
                feature: feature.assume_init(),
            };

            query_error!(r);

            Ok(fw_ver)
        }
    }
}

pub use crate::bindings::{
    AMDGPU_INFO_FW_ASD, AMDGPU_INFO_FW_DMCU, AMDGPU_INFO_FW_DMCUB, AMDGPU_INFO_FW_GFX_CE,
    AMDGPU_INFO_FW_GFX_ME, AMDGPU_INFO_FW_GFX_MEC, AMDGPU_INFO_FW_GFX_PFP, AMDGPU_INFO_FW_GFX_RLC,
    AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_CNTL, AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_GPM_MEM,
    AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_SRM_MEM, AMDGPU_INFO_FW_GMC, AMDGPU_INFO_FW_SDMA,
    AMDGPU_INFO_FW_SMC, AMDGPU_INFO_FW_SOS, AMDGPU_INFO_FW_TA, AMDGPU_INFO_FW_TOC,
    AMDGPU_INFO_FW_UVD, AMDGPU_INFO_FW_VCE, AMDGPU_INFO_FW_VCN,
};

/// Used for [DeviceHandle::query_firmware_version]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum FW_TYPE {
    VCE = AMDGPU_INFO_FW_VCE,
    UVD = AMDGPU_INFO_FW_UVD,
    GMC = AMDGPU_INFO_FW_GMC,
    GFX_ME = AMDGPU_INFO_FW_GFX_ME,
    GFX_PFP = AMDGPU_INFO_FW_GFX_PFP,
    GFX_CE = AMDGPU_INFO_FW_GFX_CE,
    GFX_RLC = AMDGPU_INFO_FW_GFX_RLC,
    GFX_MEC = AMDGPU_INFO_FW_GFX_MEC,
    SMC = AMDGPU_INFO_FW_SMC,
    SDMA = AMDGPU_INFO_FW_SDMA,
    SOS = AMDGPU_INFO_FW_SOS,
    ASD = AMDGPU_INFO_FW_ASD,
    VCN = AMDGPU_INFO_FW_VCN,
    GFX_RLC_RESTORE_LIST_CNTL = AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_CNTL,
    GFX_RLC_RESTORE_LIST_GPM_MEM = AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_GPM_MEM,
    GFX_RLC_RESTORE_LIST_SRM_MEM = AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_SRM_MEM,
    DMCU = AMDGPU_INFO_FW_DMCU,
    TA = AMDGPU_INFO_FW_TA,
    DMCUB = AMDGPU_INFO_FW_DMCUB,
    TOC = AMDGPU_INFO_FW_TOC,
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for FW_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
