use crate::*;
use crate::AMDGPU::ASIC_NAME;
use bindings::{
    AMDGPU_FAMILY_AI,
    AMDGPU_FAMILY_CI,
    AMDGPU_FAMILY_CZ,
    AMDGPU_FAMILY_KV,
    AMDGPU_FAMILY_NV,
    AMDGPU_FAMILY_RV,
    AMDGPU_FAMILY_SI,
    AMDGPU_FAMILY_UNKNOWN,
    AMDGPU_FAMILY_VGH,
    AMDGPU_FAMILY_VI,
    AMDGPU_FAMILY_YC,
    AMDGPU_FAMILY_GC_11_0_0,
    AMDGPU_FAMILY_GC_11_0_1,
    AMDGPU_FAMILY_GC_10_3_6,
    AMDGPU_FAMILY_GC_10_3_7,
    AMDGPU_FAMILY_GC_11_5_0,
};

const AMDGPU_FAMILY_GC_12_0_0: u32 = 152;

/// List of AMDGPU Family names
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u32)]
pub enum FAMILY_NAME {
    UNKNOWN,
    SI = AMDGPU_FAMILY_SI,
    CI = AMDGPU_FAMILY_CI,
    KV = AMDGPU_FAMILY_KV,
    VI = AMDGPU_FAMILY_VI,
    CZ = AMDGPU_FAMILY_CZ,
    AI = AMDGPU_FAMILY_AI,
    RV = AMDGPU_FAMILY_RV,
    NV = AMDGPU_FAMILY_NV,
    VGH = AMDGPU_FAMILY_VGH,
    GC_11_0_0 = AMDGPU_FAMILY_GC_11_0_0,
    YC = AMDGPU_FAMILY_YC,
    GC_11_0_1 = AMDGPU_FAMILY_GC_11_0_1,
    GC_10_3_6 = AMDGPU_FAMILY_GC_10_3_6,
    GC_10_3_7 = AMDGPU_FAMILY_GC_10_3_7,
    GC_11_5_0 = AMDGPU_FAMILY_GC_11_5_0,
    GC_12_0_0 = AMDGPU_FAMILY_GC_12_0_0,
}

impl From<u32> for FAMILY_NAME {
    fn from(family_id: u32) -> Self {
        match family_id {
            AMDGPU_FAMILY_SI => Self::SI,
            AMDGPU_FAMILY_CI => Self::CI,
            AMDGPU_FAMILY_KV => Self::KV,
            AMDGPU_FAMILY_VI => Self::VI,
            AMDGPU_FAMILY_CZ => Self::CZ,
            AMDGPU_FAMILY_AI => Self::AI,
            AMDGPU_FAMILY_RV => Self::RV,
            AMDGPU_FAMILY_NV => Self::NV,
            AMDGPU_FAMILY_VGH => Self::VGH,
            AMDGPU_FAMILY_GC_11_0_0 => Self::GC_11_0_0,
            AMDGPU_FAMILY_YC => Self::YC,
            AMDGPU_FAMILY_GC_11_0_1 => Self::GC_11_0_1,
            AMDGPU_FAMILY_GC_10_3_6 => Self::GC_10_3_6,
            AMDGPU_FAMILY_GC_10_3_7 => Self::GC_10_3_7,
            AMDGPU_FAMILY_GC_11_5_0 => Self::GC_11_5_0,
            AMDGPU_FAMILY_GC_12_0_0 => Self::GC_12_0_0,
            AMDGPU_FAMILY_UNKNOWN | _ => Self::UNKNOWN,
        }
    }
}

impl FAMILY_NAME {
    /// Get [ASIC_NAME] from [FAMILY_NAME]
    pub fn asic_name(&self, chip_external_rev: u32) -> ASIC_NAME {
        ASIC_NAME::get(*self, chip_external_rev)
    }
}

use std::fmt;
impl fmt::Display for FAMILY_NAME {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SI => write!(f, "Southern Islands (SI)"),
            Self::CI => write!(f, "Sea Islands (CI)"),
            Self::KV => write!(f, "Kaveri (KV)"),
            Self::VI => write!(f, "Volcanic Islands/Polaris (VI)"),
            Self::CZ => write!(f, "Carrizo (CZ)"),
            Self::AI => write!(f, "Arctic Islands (AI)"),
            Self::RV => write!(f, "Raven (RV)"),
            Self::NV => write!(f, "Navi (NV)"),
            Self::VGH => write!(f, "VanGogh (VGH)"),
            Self::GC_11_0_0 => write!(f, "GC 11.0.0"),
            Self::YC => write!(f, "Yellow Carp (YC)"),
            Self::GC_11_0_1 => write!(f, "GC 11.0.1"),
            Self::GC_10_3_6 => write!(f, "GC 10.3.6"),
            Self::GC_10_3_7 => write!(f, "GC 10.3.7"),
            Self::GC_11_5_0 => write!(f, "GC 11.5.0"),
            Self::GC_12_0_0 => write!(f, "GC 12.0.0"),
            Self::UNKNOWN => write!(f, "Unknown"),
        }
    }
}
