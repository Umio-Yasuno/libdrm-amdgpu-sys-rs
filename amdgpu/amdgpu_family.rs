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

impl FAMILY_NAME {
    pub fn from_id(id: u32) -> Self {
        match id {
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

    pub fn asic_name(&self, chip_external_rev: u32) -> AMDGPU::ASIC_NAME {
        AMDGPU::ASIC_NAME::get(*self, chip_external_rev)
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
            Self::YC => write!(f, "Yellow Carp (YC)"),
            Self::UNKNOWN => write!(f, "Unknown"),
        }
    }
}
