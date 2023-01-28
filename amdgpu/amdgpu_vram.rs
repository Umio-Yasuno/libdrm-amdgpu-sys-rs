use crate::*;

use bindings::{
    AMDGPU_VRAM_TYPE_DDR2, AMDGPU_VRAM_TYPE_DDR3, AMDGPU_VRAM_TYPE_DDR4, AMDGPU_VRAM_TYPE_DDR5,
    AMDGPU_VRAM_TYPE_GDDR1, AMDGPU_VRAM_TYPE_GDDR3, AMDGPU_VRAM_TYPE_GDDR4, AMDGPU_VRAM_TYPE_GDDR5,
    AMDGPU_VRAM_TYPE_GDDR6, AMDGPU_VRAM_TYPE_HBM, AMDGPU_VRAM_TYPE_UNKNOWN,
};

const AMDGPU_VRAM_TYPE_LPDDR4: u32 = 11;
const AMDGPU_VRAM_TYPE_LPDDR5: u32 = 12;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum VRAM_TYPE {
    UNKNOWN = 0,
    GDDR1,
    DDR2,
    GDDR3,
    GDDR4,
    GDDR5,
    HBM,
    DDR3,
    DDR4,
    GDDR6,
    DDR5,
    LPDDR4,
    LPDDR5,
}

impl From<u32> for VRAM_TYPE {
    fn from(type_id: u32) -> Self {
        match type_id {
            AMDGPU_VRAM_TYPE_GDDR1 => Self::GDDR1,
            AMDGPU_VRAM_TYPE_DDR2 => Self::DDR2,
            AMDGPU_VRAM_TYPE_GDDR3 => Self::GDDR3,
            AMDGPU_VRAM_TYPE_GDDR4 => Self::GDDR4,
            AMDGPU_VRAM_TYPE_GDDR5 => Self::GDDR5,
            AMDGPU_VRAM_TYPE_HBM => Self::HBM,
            AMDGPU_VRAM_TYPE_DDR3 => Self::DDR3,
            AMDGPU_VRAM_TYPE_DDR4 => Self::DDR4,
            AMDGPU_VRAM_TYPE_GDDR6 => Self::GDDR6,
            AMDGPU_VRAM_TYPE_DDR5 => Self::DDR5,
            AMDGPU_VRAM_TYPE_LPDDR4 => Self::LPDDR4,
            AMDGPU_VRAM_TYPE_LPDDR5 => Self::LPDDR5,
            AMDGPU_VRAM_TYPE_UNKNOWN | _ => Self::UNKNOWN,
        }
    }
}

impl VRAM_TYPE {
    /* https://www.kernel.org/doc/html/latest/gpu/amdgpu/thermal.html#pp-od-clk-voltage */
    /*
    fn clk_rate(&self) -> u64 {
        match self {
            Self::GDDR6 => 2,
            _ => 1,
        }
    }
    fn date_rate(&self) -> u64 {
        match self {
            Self::DDR2 |
            Self::DDR3 |
            Self::DDR4 |
            Self::HBM => 2,
            Self::GDDR5 => 4,
            Self::GDDR6 => 8,
            _ => 1,
        }
    }
    */
    /* https://github.com/GPUOpen-Drivers/pal/blob/dev/src/core/device.cpp */
    fn memory_ops_per_clock(&self) -> u64 {
        match self {
            Self::DDR2 | Self::DDR3 | Self::DDR4 | Self::HBM | Self::LPDDR4 => 2,
            Self::GDDR5 | Self::DDR5 | Self::LPDDR5 => 4,
            Self::GDDR6 => 16,
            _ => 1,
        }
    }

    pub fn peak_bw(&self, max_mem_clk_khz: u64, vram_bit_width: u32) -> u64 {
        let eff_mem_clk_mhz = (max_mem_clk_khz / 1000) * self.memory_ops_per_clock();

        eff_mem_clk_mhz * (vram_bit_width as u64) / 8
    }

    pub fn peak_bw_gb(&self, max_mem_clk_khz: u64, vram_bit_width: u32) -> u32 {
        (self.peak_bw(max_mem_clk_khz, vram_bit_width) / 1000) as u32
    }
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for VRAM_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
