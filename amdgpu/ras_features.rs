#[derive(Debug, Clone, Copy)]
pub struct RasEnabledFeatures(u64);

impl RasEnabledFeatures {
    pub fn new(val: u64) -> Self {
        Self(val)
    }

    pub fn is_supported(&self, ras: RasBlock) -> bool {
        (self.0 & ras as u64) != 0
    }

    pub fn mask_value(&self) -> u64 {
        self.0
    }
}

use crate::AMDGPU::DeviceHandle;
use crate::bindings::AMDGPU_INFO_RAS_ENABLED_FEATURES;

impl DeviceHandle {
    pub fn ras_enabled_features(&self) -> Result<RasEnabledFeatures, i32> {
        let v = Self::query(self, AMDGPU_INFO_RAS_ENABLED_FEATURES)?;

        Ok(RasEnabledFeatures::new(v))
    }
}

use crate::bindings::{
    AMDGPU_INFO_RAS_ENABLED_UMC,
    AMDGPU_INFO_RAS_ENABLED_SDMA,
    AMDGPU_INFO_RAS_ENABLED_GFX,
    AMDGPU_INFO_RAS_ENABLED_MMHUB,
    AMDGPU_INFO_RAS_ENABLED_ATHUB,
    AMDGPU_INFO_RAS_ENABLED_PCIE,
    AMDGPU_INFO_RAS_ENABLED_HDP,
    AMDGPU_INFO_RAS_ENABLED_XGMI,
    AMDGPU_INFO_RAS_ENABLED_DF,
    AMDGPU_INFO_RAS_ENABLED_SMN,
    AMDGPU_INFO_RAS_ENABLED_SEM,
    AMDGPU_INFO_RAS_ENABLED_MP0,
    AMDGPU_INFO_RAS_ENABLED_MP1,
    AMDGPU_INFO_RAS_ENABLED_FUSE,
};

#[repr(u32)]
pub enum RasBlock {
    UMC = AMDGPU_INFO_RAS_ENABLED_UMC,
    SDMA = AMDGPU_INFO_RAS_ENABLED_SDMA,
    GFX = AMDGPU_INFO_RAS_ENABLED_GFX,
    MMHUB = AMDGPU_INFO_RAS_ENABLED_MMHUB,
    ATHUB = AMDGPU_INFO_RAS_ENABLED_ATHUB,
    PCIE = AMDGPU_INFO_RAS_ENABLED_PCIE,
    HDP = AMDGPU_INFO_RAS_ENABLED_HDP,
    XGMI = AMDGPU_INFO_RAS_ENABLED_XGMI,
    DF = AMDGPU_INFO_RAS_ENABLED_DF,
    SMN = AMDGPU_INFO_RAS_ENABLED_SMN,
    SEM = AMDGPU_INFO_RAS_ENABLED_SEM,
    MP0 = AMDGPU_INFO_RAS_ENABLED_MP0,
    MP1 = AMDGPU_INFO_RAS_ENABLED_MP1,
    FUSE = AMDGPU_INFO_RAS_ENABLED_FUSE,
}
