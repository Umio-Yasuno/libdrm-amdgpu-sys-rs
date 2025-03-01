use crate::*;

use bindings::{
    AMDGPU_VRAM_TYPE_DDR2,
    AMDGPU_VRAM_TYPE_DDR3,
    AMDGPU_VRAM_TYPE_DDR4,
    AMDGPU_VRAM_TYPE_DDR5,
    AMDGPU_VRAM_TYPE_GDDR1,
    AMDGPU_VRAM_TYPE_GDDR3,
    AMDGPU_VRAM_TYPE_GDDR4,
    AMDGPU_VRAM_TYPE_GDDR5,
    AMDGPU_VRAM_TYPE_GDDR6,
    AMDGPU_VRAM_TYPE_HBM,
    AMDGPU_VRAM_TYPE_UNKNOWN,
};

const AMDGPU_VRAM_TYPE_LPDDR4: u32 = 11;
const AMDGPU_VRAM_TYPE_LPDDR5: u32 = 12;

/// List of AMDGPU VRAM types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u32)]
pub enum VRAM_TYPE {
    GDDR1 = AMDGPU_VRAM_TYPE_GDDR1,
    DDR2 = AMDGPU_VRAM_TYPE_DDR2,
    GDDR3 = AMDGPU_VRAM_TYPE_GDDR3,
    GDDR4 = AMDGPU_VRAM_TYPE_GDDR4,
    GDDR5 = AMDGPU_VRAM_TYPE_GDDR5,
    HBM = AMDGPU_VRAM_TYPE_HBM,
    DDR3 = AMDGPU_VRAM_TYPE_DDR3,
    DDR4 = AMDGPU_VRAM_TYPE_DDR4,
    GDDR6 = AMDGPU_VRAM_TYPE_GDDR6,
    DDR5 = AMDGPU_VRAM_TYPE_DDR5,
    LPDDR4 = AMDGPU_VRAM_TYPE_LPDDR4,
    LPDDR5 = AMDGPU_VRAM_TYPE_LPDDR5,
    UNKNOWN = AMDGPU_VRAM_TYPE_UNKNOWN,
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
    pub fn clock_rate(&self) -> u64 {
        match self {
            Self::GDDR6 => 2,
            _ => 1,
        }
    }
    /*
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

    pub fn bit_width_per_channel(&self) -> u32 {
        match self {
            Self::DDR2 |
            Self::DDR3 |
            Self::DDR4 |
            Self::DDR5 => 64,
            /*
                The AMDGPU drivers always calculate the width per memory channel on APU as 64-bit.  
                https://gitlab.freedesktop.org/drm/amd/-/issues/2468
            */
            Self::LPDDR4 |
            Self::LPDDR5 => 64,
            Self::HBM => 128,
            Self::GDDR1 |
            Self::GDDR3 |
            Self::GDDR4 |
            Self::GDDR5 => 32,
            Self::GDDR6 => 16,
            Self::UNKNOWN => 1,
        }
    }

    /// Memory ops per clock  
    /// ref: https://github.com/GPUOpen-Drivers/pal/blob/dev/src/core/device.cpp
    fn memory_ops_per_clock(&self) -> u64 {
        match self {
            Self::DDR2 |
            Self::DDR3 |
            Self::DDR4 |
            /*
                ref: https://gitlab.freedesktop.org/mesa/mesa/-/issues/9259#note_1978834
            */
            Self::DDR5 |
            Self::HBM |
            Self::LPDDR4 => 2,
            Self::GDDR5 |
            // Self::DDR5 |
            Self::LPDDR5 => 4,
            Self::GDDR6 => 16,
            _ => 1,
        }
    }

    /// Peak Memory Bandwidth (MB/s)
    pub fn peak_bw(&self, max_mem_clk_khz: u64, vram_bit_width: u32) -> u64 {
        let eff_mem_clk_mhz = (max_mem_clk_khz / 1000) * self.memory_ops_per_clock();

        eff_mem_clk_mhz * (vram_bit_width as u64) / 8
    }

    /// Peak Memory Bandwidth (GB/s)
    pub fn peak_bw_gb(&self, max_mem_clk_khz: u64, vram_bit_width: u32) -> u32 {
        (self.peak_bw(max_mem_clk_khz, vram_bit_width) / 1000) as u32
    }
}

use std::fmt;
impl fmt::Display for VRAM_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
