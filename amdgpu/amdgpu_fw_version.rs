use crate::*;
use crate::AMDGPU::*;

use std::mem::MaybeUninit;

pub struct FwVer {
    pub version: u32,
    pub feature: u32,
}

pub trait QUERY_FW_VERSION {
    fn query_firmware_version(
        self,
        fw_type: FW_TYPE,
        ip_instance: ::std::os::raw::c_uint,
        index: ::std::os::raw::c_uint,
    ) -> Result<FwVer, i32>;
}

impl QUERY_FW_VERSION for DEVICE_HANDLE {
    fn query_firmware_version(
        self,
        fw_type: FW_TYPE,
        ip_instance: ::std::os::raw::c_uint,
        index: ::std::os::raw::c_uint,
    ) -> Result<FwVer, i32> {
        unsafe {
            let mut version: MaybeUninit<u32> = MaybeUninit::zeroed();
            let mut feature: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_firmware_version(
                self,
                fw_type as u32,
                ip_instance,
                index,
                version.as_mut_ptr(),
                feature.as_mut_ptr(),
            );

            query_error!(r);

            let fw_ver = FwVer {
                version: version.assume_init(),
                feature: feature.assume_init(),
            };

            return Ok(fw_ver);
        }
    }
}

pub use crate::bindings::{
    AMDGPU_INFO_FW_VCE,
    AMDGPU_INFO_FW_UVD,
    AMDGPU_INFO_FW_GMC,
    AMDGPU_INFO_FW_GFX_ME,
    AMDGPU_INFO_FW_GFX_PFP,
    AMDGPU_INFO_FW_GFX_CE,
    AMDGPU_INFO_FW_GFX_RLC,
    AMDGPU_INFO_FW_GFX_MEC,
    AMDGPU_INFO_FW_SMC,
    AMDGPU_INFO_FW_SDMA,
    AMDGPU_INFO_FW_SOS,
    AMDGPU_INFO_FW_ASD,
    AMDGPU_INFO_FW_VCN,
    AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_CNTL,
    AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_GPM_MEM,
    AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_SRM_MEM,
    AMDGPU_INFO_FW_DMCU,
    AMDGPU_INFO_FW_TA,
    AMDGPU_INFO_FW_DMCUB,
    AMDGPU_INFO_FW_TOC,
};


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
