use std::fmt;
use std::path::PathBuf;
use std::io;

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

#[derive(Debug, Clone, Copy)]
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

impl RasBlock {
    /// ref: drivers/gpu/drm/amd/amdgpu/amdgpu_ras.c
    pub fn to_sysfs_name_prefix(&self) -> &str {
        match self {
            Self::UMC => "umc",
            Self::SDMA => "sdma",
            Self::GFX => "gfx",
            Self::MMHUB => "mmhub",
            Self::ATHUB => "athub",
            Self::PCIE => "pcie_bif",
            Self::HDP => "hdp",
            Self::XGMI => "xgmi_wafl",
            Self::DF => "df",
            Self::SMN => "smn",
            Self::SEM => "sem",
            Self::MP0 => "mp0",
            Self::MP1 => "mp1",
            Self::FUSE => "fuse",
        }
    }
}

impl fmt::Display for RasBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct RasErrorCount {
    pub uncorrected: u64,
    pub corrected: u64,
}

impl RasErrorCount {
    pub fn get_from_sysfs_with_ras_block<P: Into<PathBuf>>(
        sysfs_path: P,
        ras_block: RasBlock,
    ) -> io::Result<Self> {
        let s = {
            let pre = ras_block.to_sysfs_name_prefix();
            let path = sysfs_path.into().join("ras").join(format!("{pre}_err_count"));

            std::fs::read_to_string(path)?
        };

        let mut lines = s.lines();

        let [ue, ce] = [lines.next(), lines.next()].map(|line| -> io::Result<u64> {
            const PRE: usize = "ue: ".len();

            line
                .and_then(|l| l.get(PRE..))
                .and_then(|s| s.parse().ok())
                .ok_or(io::Error::other("Parse Error"))
        });

        Ok(Self { uncorrected: ue?, corrected: ce? })
    }
}
