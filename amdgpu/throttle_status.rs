use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct ThrottleStatus(u64);

impl ThrottleStatus {
    pub fn new(val: u64) -> Self {
        Self(val)
    }

    pub fn check_throttler(&self, thr: ThrottlerBit) -> bool {
        ((self.0 >> thr as u64) & 0b1) == 1
    }

    pub fn get_all_throttler(&self) -> Vec<ThrottlerBit> {
        let mut vec: Vec<ThrottlerBit> = Vec::with_capacity(64);
        let mut i = 0;
        let mut n = self.0;

        while n != 0 {
            if (n & 0b1) == 1 {
                vec.push(ThrottlerBit::from(i));
            }
            n >>= 0b1;
            i += 1;
        }

        vec
    }

    pub fn get_all_throttler_type(&self) -> Vec<ThrottlerType> {
        Self::get_all_throttler_type_from_slice(&self.get_all_throttler())
    }

    pub fn get_all_throttler_type_from_slice(thrs: &[ThrottlerBit]) -> Vec<ThrottlerType> {
        use std::collections::HashSet;
        let mut set: HashSet<ThrottlerType> = HashSet::new();

        for thr in thrs {
            set.insert(thr.throttler_type());
        }

        set.into_iter().collect()
    }
}

/// ref: drivers/gpu/drm/amd/pm/swsmu/inc/amdgpu_smu.h
struct SmuThrottler;

impl SmuThrottler {
    // Power
    const PPT0: u8 = 0;
    const PPT1: u8 = 1;
    const PPT2: u8 = 2;
    const PPT3: u8 = 3;
    const SPL: u8 = 4;
    const FPPT: u8 = 5;
    const SPPT: u8 = 6;
    const SPPT_APU: u8 = 7;
    // Current
    const TDC_GFX: u8 = 16;
    const TDC_SOC: u8 = 17;
    const TDC_MEM: u8 = 18;
    const TDC_VDD: u8 = 19;
    const TDC_CVIP: u8 = 20;
    const EDC_CPU: u8 = 21;
    const EDC_GFX: u8 = 22;
    const APCC: u8 = 23;
    // Temperature
    const TEMP_GPU: u8 = 32;
    const TEMP_CORE: u8 = 33;
    const TEMP_MEM: u8 = 34;
    const TEMP_EDGE: u8 = 35;
    const TEMP_HOTSPOT: u8 = 36;
    const TEMP_SOC: u8 = 37;
    const TEMP_VR_GFX: u8 = 38;
    const TEMP_VR_SOC: u8 = 39;
    const TEMP_VR_MEM0: u8 = 40;
    const TEMP_VR_MEM1: u8 = 41;
    const TEMP_LIQUID0: u8 = 42;
    const TEMP_LIQUID1: u8 = 43;
    const VRHOT0: u8 = 44;
    const VRHOT1: u8 = 45;
    const PROCHOT_CPU: u8 = 46;
    const PROCHOT_GPU: u8 = 47;
    // Other
    const PPM: u8 = 56;
    const FIT: u8 = 57;
    // Unknown;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ThrottlerBit {
    // Power
    PPT0 = SmuThrottler::PPT0,
    PPT1 = SmuThrottler::PPT1,
    PPT2 = SmuThrottler::PPT2,
    PPT3 = SmuThrottler::PPT3,
    SPL = SmuThrottler::SPL,
    FPPT = SmuThrottler::FPPT,
    SPPT = SmuThrottler::SPPT,
    SPPT_APU = SmuThrottler::SPPT_APU,
    // Current
    TDC_GFX = SmuThrottler::TDC_GFX,
    TDC_SOC = SmuThrottler::TDC_SOC,
    TDC_MEM = SmuThrottler::TDC_MEM,
    TDC_VDD = SmuThrottler::TDC_VDD,
    TDC_CVIP = SmuThrottler::TDC_CVIP,
    EDC_CPU = SmuThrottler::EDC_CPU,
    EDC_GFX = SmuThrottler::EDC_GFX,
    APCC = SmuThrottler::APCC,
    // Temperature
    TEMP_GPU = SmuThrottler::TEMP_GPU,
    TEMP_CORE = SmuThrottler::TEMP_CORE,
    TEMP_MEM = SmuThrottler::TEMP_MEM,
    TEMP_EDGE = SmuThrottler::TEMP_EDGE,
    TEMP_HOTSPOT = SmuThrottler::TEMP_HOTSPOT,
    TEMP_SOC = SmuThrottler::TEMP_SOC,
    TEMP_VR_GFX = SmuThrottler::TEMP_VR_GFX,
    TEMP_VR_SOC = SmuThrottler::TEMP_VR_SOC,
    TEMP_VR_MEM0 = SmuThrottler::TEMP_VR_MEM0,
    TEMP_VR_MEM1 = SmuThrottler::TEMP_VR_MEM1,
    TEMP_LIQUID0 = SmuThrottler::TEMP_LIQUID0,
    TEMP_LIQUID1 = SmuThrottler::TEMP_LIQUID1,
    VRHOT0 = SmuThrottler::VRHOT0,
    VRHOT1 = SmuThrottler::VRHOT1,
    PROCHOT_CPU = SmuThrottler::PROCHOT_CPU,
    PROCHOT_GPU = SmuThrottler::PROCHOT_GPU,
    // Other
    PPM = SmuThrottler::PPM,
    FIT = SmuThrottler::FIT,
    Unknown,
}

impl ThrottlerBit {
    pub fn throttler_type(&self) -> ThrottlerType {
        ThrottlerType::from(self)
    }
}

impl From<u8> for ThrottlerBit {
    fn from(pos: u8) -> Self {
        match pos {
            SmuThrottler::PPT0 => Self::PPT0,
            SmuThrottler::PPT1 => Self::PPT1,
            SmuThrottler::PPT2 => Self::PPT2,
            SmuThrottler::PPT3 => Self::PPT3,
            SmuThrottler::SPL => Self::SPL,
            SmuThrottler::FPPT => Self::FPPT,
            SmuThrottler::SPPT => Self::SPPT,
            SmuThrottler::SPPT_APU => Self::SPPT_APU,
            // Current
            SmuThrottler::TDC_GFX => Self::TDC_GFX,
            SmuThrottler::TDC_SOC => Self::TDC_SOC,
            SmuThrottler::TDC_MEM => Self::TDC_MEM,
            SmuThrottler::TDC_VDD => Self::TDC_VDD,
            SmuThrottler::TDC_CVIP => Self::TDC_CVIP,
            SmuThrottler::EDC_CPU => Self::EDC_CPU,
            SmuThrottler::EDC_GFX => Self::EDC_GFX,
            SmuThrottler::APCC => Self::APCC,
            // Temperature
            SmuThrottler::TEMP_GPU => Self::TEMP_GPU,
            SmuThrottler::TEMP_CORE => Self::TEMP_CORE,
            SmuThrottler::TEMP_MEM => Self::TEMP_MEM,
            SmuThrottler::TEMP_EDGE => Self::TEMP_EDGE,
            SmuThrottler::TEMP_HOTSPOT => Self::TEMP_HOTSPOT,
            SmuThrottler::TEMP_SOC => Self::TEMP_SOC,
            SmuThrottler::TEMP_VR_GFX => Self::TEMP_VR_GFX,
            SmuThrottler::TEMP_VR_SOC => Self::TEMP_VR_SOC,
            SmuThrottler::TEMP_VR_MEM0 => Self::TEMP_VR_MEM0,
            SmuThrottler::TEMP_VR_MEM1 => Self::TEMP_VR_MEM1,
            SmuThrottler::TEMP_LIQUID0 => Self::TEMP_LIQUID0,
            SmuThrottler::TEMP_LIQUID1 => Self::TEMP_LIQUID1,
            SmuThrottler::VRHOT0 => Self::VRHOT0,
            SmuThrottler::VRHOT1 => Self::VRHOT1,
            SmuThrottler::PROCHOT_CPU => Self::PROCHOT_CPU,
            SmuThrottler::PROCHOT_GPU => Self::PROCHOT_GPU,
            // Other
            SmuThrottler::PPM => Self::PPM,
            SmuThrottler::FIT => Self::FIT,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for ThrottlerBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ref: drivers/gpu/drm/amd/pm/swsmu/inc/amdgpu_smu.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThrottlerType {
    Power,
    Current,
    Temperature,
    Other,
}

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
            ThrottlerBit::FIT |
            ThrottlerBit::Unknown => Self::Other,
        }
    }
}

impl From<ThrottlerBit> for ThrottlerType {
    fn from(thr: ThrottlerBit) -> Self {
        Self::from(&thr)
    }
}
