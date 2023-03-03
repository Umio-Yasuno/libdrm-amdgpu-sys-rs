use crate::*;

use bindings::{
    AMDGPU_FAMILY_AI, AMDGPU_FAMILY_CI, AMDGPU_FAMILY_CZ, AMDGPU_FAMILY_KV, AMDGPU_FAMILY_NV,
    AMDGPU_FAMILY_RV, AMDGPU_FAMILY_SI, AMDGPU_FAMILY_UNKNOWN, AMDGPU_FAMILY_VGH, AMDGPU_FAMILY_VI,
    AMDGPU_FAMILY_YC,
};

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum FAMILY_NAME {
    UNKNOWN,
    SI,
    CI,
    KV,
    VI,
    CZ,
    AI,
    RV,
    NV,
    VGH,
    YC,
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
            AMDGPU_FAMILY_YC => Self::YC,
            AMDGPU_FAMILY_UNKNOWN | _ => Self::UNKNOWN,
        }
    }
}

impl FAMILY_NAME {
    pub fn asic_name(&self, chip_external_rev: u32) -> AMDGPU::ASIC_NAME {
        AMDGPU::ASIC_NAME::get(*self, chip_external_rev)
    }
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
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
            Self::YC => write!(f, "Yellow Carp (YC)"),
            Self::UNKNOWN => write!(f, "Unknown"),
        }
    }
}
