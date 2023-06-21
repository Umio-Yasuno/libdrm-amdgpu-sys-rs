#[cfg(feature = "std")]
use std::fmt;

#[derive(Debug, Clone)]
pub struct ThrottleStatus(u64);

impl ThrottleStatus {
    pub fn new(val: u64) -> Self {
        Self(val)
    }

    pub fn check_throttler(&self, thr: ThrottlerBit) -> bool {
        ((self.0 >> thr as u64) & 0b1) == 1
    }

    pub fn get_all_throttler(&self) -> Vec<ThrottlerBit> {
        THROTTLER_LIST.iter().copied().filter(|thr| self.check_throttler(*thr)).collect()
    }

    pub fn get_all_throttler_type(&self) -> Vec<ThrottlerType> {
        Self::get_all_throttler_type_from_vec(&self.get_all_throttler())
    }

    pub fn get_all_throttler_type_from_vec(thrs: &[ThrottlerBit]) -> Vec<ThrottlerType> {
        use std::collections::HashSet;
        let mut set: HashSet<ThrottlerType> = HashSet::new();

        for thr in thrs {
            set.insert(thr.throttler_type());
        }

        set.into_iter().collect()
    }
}

/// ref: drivers/gpu/drm/amd/pm/swsmu/inc/amdgpu_smu.h
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum ThrottlerBit {
    // Power
    PPT0 = 0,
    PPT1 = 1,
    PPT2 = 2,
    PPT3 = 3,
    SPL = 4,
    FPPT = 5,
    SPPT = 6,
    SPPT_APU = 7,
    // Current
    TDC_GFX = 16,
    TDC_SOC = 17,
    TDC_MEM = 18,
    TDC_VDD = 19,
    TDC_CVIP = 20,
    EDC_CPU = 21,
    EDC_GFX = 22,
    APCC = 23,
    // Temperature
    TEMP_GPU = 32,
    TEMP_CORE = 33,
    TEMP_MEM = 34,
    TEMP_EDGE = 35,
    TEMP_HOTSPOT = 36,
    TEMP_SOC = 37,
    TEMP_VR_GFX = 38,
    TEMP_VR_SOC = 39,
    TEMP_VR_MEM0 = 40,
    TEMP_VR_MEM1 = 41,
    TEMP_LIQUID0 = 42,
    TEMP_LIQUID1 = 43,
    VRHOT0 = 44,
    VRHOT1 = 45,
    PROCHOT_CPU = 46,
    PROCHOT_GPU = 47,
    // Other
    PPM = 56,
    FIT = 57,
}

impl ThrottlerBit {
    pub fn throttler_type(&self) -> ThrottlerType {
        ThrottlerType::from(self)
    }
}

#[cfg(feature = "std")]
impl fmt::Display for ThrottlerBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const THROTTLER_LIST: &[ThrottlerBit] = &[
    ThrottlerBit::PPT0,
    ThrottlerBit::PPT1,
    ThrottlerBit::PPT2,
    ThrottlerBit::PPT3,
    ThrottlerBit::SPL,
    ThrottlerBit::FPPT,
    ThrottlerBit::SPPT,
    ThrottlerBit::SPPT_APU,
    ThrottlerBit::TDC_GFX,
    ThrottlerBit::TDC_SOC,
    ThrottlerBit::TDC_MEM,
    ThrottlerBit::TDC_VDD,
    ThrottlerBit::TDC_CVIP,
    ThrottlerBit::EDC_CPU,
    ThrottlerBit::EDC_GFX,
    ThrottlerBit::APCC,
    ThrottlerBit::TEMP_GPU,
    ThrottlerBit::TEMP_CORE,
    ThrottlerBit::TEMP_MEM,
    ThrottlerBit::TEMP_EDGE,
    ThrottlerBit::TEMP_HOTSPOT,
    ThrottlerBit::TEMP_SOC,
    ThrottlerBit::TEMP_VR_GFX,
    ThrottlerBit::TEMP_VR_SOC,
    ThrottlerBit::TEMP_VR_MEM0,
    ThrottlerBit::TEMP_VR_MEM1,
    ThrottlerBit::TEMP_LIQUID0,
    ThrottlerBit::TEMP_LIQUID1,
    ThrottlerBit::VRHOT0,
    ThrottlerBit::VRHOT1,
    ThrottlerBit::PROCHOT_CPU,
    ThrottlerBit::PROCHOT_GPU,
    ThrottlerBit::PPM,
    ThrottlerBit::FIT,
];

/// ref: drivers/gpu/drm/amd/pm/swsmu/inc/amdgpu_smu.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThrottlerType {
    Power,
    Current,
    Temperature,
    Other,
}

#[cfg(feature = "std")]
impl fmt::Display for ThrottlerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&ThrottlerBit> for ThrottlerType {
    fn from(thr: &ThrottlerBit) -> Self {
        match thr {
            ThrottlerBit::PPT0 |
            ThrottlerBit::PPT1 |
            ThrottlerBit::PPT2 |
            ThrottlerBit::PPT3 |
            ThrottlerBit::SPL |
            ThrottlerBit::FPPT |
            ThrottlerBit::SPPT |
            ThrottlerBit::SPPT_APU => Self::Power,
            ThrottlerBit::TDC_GFX |
            ThrottlerBit::TDC_SOC |
            ThrottlerBit::TDC_MEM |
            ThrottlerBit::TDC_VDD |
            ThrottlerBit::TDC_CVIP |
            ThrottlerBit::EDC_CPU |
            ThrottlerBit::EDC_GFX |
            ThrottlerBit::APCC => Self::Current,
            ThrottlerBit::TEMP_GPU |
            ThrottlerBit::TEMP_CORE |
            ThrottlerBit::TEMP_MEM |
            ThrottlerBit::TEMP_EDGE |
            ThrottlerBit::TEMP_HOTSPOT |
            ThrottlerBit::TEMP_SOC |
            ThrottlerBit::TEMP_VR_GFX |
            ThrottlerBit::TEMP_VR_SOC |
            ThrottlerBit::TEMP_VR_MEM0 |
            ThrottlerBit::TEMP_VR_MEM1 |
            ThrottlerBit::TEMP_LIQUID0 |
            ThrottlerBit::TEMP_LIQUID1 |
            ThrottlerBit::VRHOT0 |
            ThrottlerBit::VRHOT1 |
            ThrottlerBit::PROCHOT_CPU |
            ThrottlerBit::PROCHOT_GPU => Self::Temperature,
            ThrottlerBit::PPM |
            ThrottlerBit::FIT => Self::Other,
        }
    }
}

impl From<ThrottlerBit> for ThrottlerType {
    fn from(thr: ThrottlerBit) -> Self {
        Self::from(&thr)
    }
}
