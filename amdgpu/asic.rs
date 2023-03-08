/*
************************************************************************************************************************
*
*  Copyright (C) 2017-2022 Advanced Micro Devices, Inc.  All rights reserved.
*
* Permission is hereby granted, free of charge, to any person obtaining a
* copy of this software and associated documentation files (the "Software"),
* to deal in the Software without restriction, including without limitation
* the rights to use, copy, modify, merge, publish, distribute, sublicense,
* and/or sell copies of the Software, and to permit persons to whom the
* Software is furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
* all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
* THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
* OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
* ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
* OTHER DEALINGS IN THE SOFTWARE
*
***********************************************************************************************************************/

/*
    https://gitlab.freedesktop.org/mesa/mesa/-/blob/main/src/amd/common/amd_family.h
    Commit: dda718d2bfe9309145d8e521c59c617e7674045a
*/

use crate::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u32)]
pub enum ASIC_NAME {
    CHIP_UNKNOWN = 0,
    /* R3xx-based cores. (GFX2) */
    CHIP_R300,
    CHIP_R350,
    CHIP_RV350,
    CHIP_RV370,
    CHIP_RV380,
    CHIP_RS400,
    CHIP_RC410,
    CHIP_RS480,
    /* R4xx-based cores. (GFX2) */
    CHIP_R420,
    CHIP_R423,
    CHIP_R430,
    CHIP_R480,
    CHIP_R481,
    CHIP_RV410,
    CHIP_RS600,
    CHIP_RS690,
    CHIP_RS740,
    /* R5xx-based cores. (GFX2) */
    CHIP_RV515,
    CHIP_R520,
    CHIP_RV530,
    CHIP_R580,
    CHIP_RV560,
    CHIP_RV570,
    /* GFX3 (R6xx) */
    CHIP_R600,
    CHIP_RV610,
    CHIP_RV630,
    CHIP_RV670,
    CHIP_RV620,
    CHIP_RV635,
    CHIP_RS780,
    CHIP_RS880,
    /* GFX3 (R7xx) */
    CHIP_RV770,
    CHIP_RV730,
    CHIP_RV710,
    CHIP_RV740,
    /* GFX4 (Evergreen) */
    CHIP_CEDAR,
    CHIP_REDWOOD,
    CHIP_JUNIPER,
    CHIP_CYPRESS,
    CHIP_HEMLOCK,
    CHIP_PALM,
    CHIP_SUMO,
    CHIP_SUMO2,
    CHIP_BARTS,
    CHIP_TURKS,
    CHIP_CAICOS,
    /* GFX5 (Northern Islands) */
    CHIP_CAYMAN,
    CHIP_ARUBA,
    /* GFX6 (Southern Islands) */
    CHIP_TAHITI,
    CHIP_PITCAIRN,
    CHIP_VERDE,
    CHIP_OLAND,
    CHIP_HAINAN,
    /* GFX7 (Sea Islands) */
    CHIP_BONAIRE,
    CHIP_KAVERI,
    CHIP_KABINI,
    CHIP_HAWAII, /* Radeon 290, 390 */
    /* GFX8 (Volcanic Islands & Polaris) */
    CHIP_TONGA, /* Radeon 285, 380 */
    CHIP_ICELAND,
    CHIP_CARRIZO,
    CHIP_FIJI, /* Radeon Fury */
    CHIP_STONEY,
    CHIP_POLARIS10, /* Radeon 470, 480, 570, 580, 590 */
    CHIP_POLARIS11, /* Radeon 460, 560 */
    CHIP_POLARIS12, /* Radeon 540, 550 */
    CHIP_VEGAM,
    /* GFX9 (Vega) */
    CHIP_VEGA10, /* Vega 56, 64 */
    CHIP_VEGA12,
    CHIP_VEGA20,    /* Radeon VII, MI50 */
    CHIP_RAVEN,     /* Ryzen 2000, 3000 */
    CHIP_RAVEN2,    /* Ryzen 2200U, 3200U */
    CHIP_RENOIR,    /* Ryzen 4000, 5000 */
    CHIP_ARCTURUS,  /* MI100 */
    CHIP_ALDEBARAN, /* MI200 */
    /* GFX10.1 (RDNA 1) */
    CHIP_NAVI10, /* Radeon 5600, 5700 */
    CHIP_NAVI12, /* Radeon Pro 5600M */
    CHIP_NAVI14, /* Radeon 5300, 5500 */
    /* GFX10.3 (RDNA 2) */
    CHIP_NAVI21,    /* Radeon 6800, 6900 */
    CHIP_NAVI22,    /* Radeon 6700 */
    CHIP_VANGOGH,   /* Steam Deck */
    CHIP_NAVI23,    /* Radeon 6600 */
    CHIP_NAVI24,    /* Radeon 6400, 6500 */
    CHIP_REMBRANDT, /* Ryzen 6000 */
    CHIP_GFX1036,
    CHIP_GFX1100,
    CHIP_GFX1101,
    CHIP_GFX1102,
    CHIP_GFX1103_R1,
    CHIP_GFX1103_R2,
}

use crate::AMDGPU::FAMILY_NAME;

impl ASIC_NAME {
    pub fn get(family: FAMILY_NAME, chip_external_rev: u32) -> Self {
        /*
            https://gitlab.freedesktop.org/mesa/mesa/blob/main/src/amd/addrlib/src/amdgpu_asic_addr.h
            Commit: fd3451babd6cded6794561d74c8919576ba1f97d
        */
        let rev = chip_external_rev;

        match family {
            FAMILY_NAME::SI => match rev {
                0x05..=0x14 => Self::CHIP_TAHITI,
                0x15..=0x28 => Self::CHIP_PITCAIRN,
                0x29..=0x3B => Self::CHIP_VERDE,
                0x3C..=0x45 => Self::CHIP_OLAND,
                0x46..=0xFF => Self::CHIP_HAINAN,
                _ => Self::CHIP_UNKNOWN,
            },
            FAMILY_NAME::CI => match rev {
                0x14..=0x27 => Self::CHIP_BONAIRE,
                0x28..=0x3B => Self::CHIP_HAWAII,
                _ => Self::CHIP_UNKNOWN,
            },
            FAMILY_NAME::KV => {
                match rev {
                    /* Spectre, Spooky */
                    0x01..=0x40 | 0x41..=0x80 => Self::CHIP_KAVERI,
                    /* Kalindi, Godavari */
                    0x81..=0xA0 | 0xA1..=0xFF => Self::CHIP_KABINI,
                    _ => Self::CHIP_UNKNOWN,
                }
            }
            FAMILY_NAME::VI => match rev {
                0x01..=0x13 => Self::CHIP_ICELAND,
                0x14..=0x3B => Self::CHIP_TONGA,
                0x3C..=0x4F => Self::CHIP_FIJI,
                0x50..=0x59 => Self::CHIP_POLARIS10,
                0x5A..=0x63 => Self::CHIP_POLARIS11,
                0x64..=0x6D => Self::CHIP_POLARIS12,
                0x6E..=0xFF => Self::CHIP_VEGAM,
                _ => Self::CHIP_UNKNOWN,
            },
            FAMILY_NAME::AI => match rev {
                0x01..=0x13 => Self::CHIP_VEGA10,
                0x14..=0x27 => Self::CHIP_VEGA12,
                0x28..=0x31 => Self::CHIP_VEGA20,
                0x32..=0x3B => Self::CHIP_ARCTURUS,
                0x3C..=0xFF => Self::CHIP_ALDEBARAN,
                _ => Self::CHIP_UNKNOWN,
            },
            FAMILY_NAME::RV => match rev {
                0x01..=0x80 => Self::CHIP_RAVEN,
                0x81..=0x90 => Self::CHIP_RAVEN2,
                0x91..=0xFF => Self::CHIP_RENOIR,
                _ => Self::CHIP_UNKNOWN,
            },
            FAMILY_NAME::NV => match rev {
                0x01..=0x09 => Self::CHIP_NAVI10,
                0x0A..=0x13 => Self::CHIP_NAVI12,
                0x14..=0x27 => Self::CHIP_NAVI14,
                0x28..=0x31 => Self::CHIP_NAVI21,
                0x32..=0x3B => Self::CHIP_NAVI22,
                0x3C..=0x45 => Self::CHIP_NAVI23,
                0x46..=0x4F => Self::CHIP_NAVI24,
                _ => Self::CHIP_UNKNOWN,
            },
            FAMILY_NAME::VGH => Self::CHIP_VANGOGH,
            FAMILY_NAME::GC_11_0_0 => match rev {
                0x01..=0x09 => Self::CHIP_GFX1100,
                0x10..=0x19 => Self::CHIP_GFX1102,
                0x20..=0xFF => Self::CHIP_GFX1101,
                _ => Self::CHIP_UNKNOWN,
            },
            FAMILY_NAME::YC => Self::CHIP_REMBRANDT,
            FAMILY_NAME::GC_11_0_1 => match rev {
                0x01..=0x09 => Self::CHIP_GFX1103_R1,
                0x80..=0xFF => Self::CHIP_GFX1103_R2,
                _ => Self::CHIP_UNKNOWN,
            },
            FAMILY_NAME::GC_10_3_6 |
            FAMILY_NAME::GC_10_3_7 => Self::CHIP_GFX1036,
            _ => Self::CHIP_UNKNOWN,
        }
    }

    pub fn chip_class(&self) -> AMDGPU::CHIP_CLASS {
        AMDGPU::CHIP_CLASS::from(*self)
    }
    /*
        https://gitlab.freedesktop.org/mesa/mesa/blob/main/src/amd/common/ac_gpu_info.c
    */
    fn has_rbplus(&self) -> bool {
        *self == Self::CHIP_STONEY || *self >= Self::CHIP_VEGA10
    }

    pub fn rbplus_allowed(&self) -> bool {
        self.has_rbplus()
            && (*self == Self::CHIP_STONEY
                || *self == Self::CHIP_VEGA12
                || *self == Self::CHIP_RAVEN
                || *self == Self::CHIP_RAVEN2
                || *self == Self::CHIP_RENOIR
                || *self >= Self::CHIP_NAVI21)
    }

    pub fn has_packed_math_16bit(&self) -> bool {
        *self >= Self::CHIP_VEGA10
    }

    pub fn has_accelerated_dot_product(&self) -> bool {
        *self == Self::CHIP_ARCTURUS
            || *self == Self::CHIP_ALDEBARAN
            || *self == Self::CHIP_VEGA20
            || *self >= Self::CHIP_NAVI12
    }

    pub fn max_wave64_per_simd(&self) -> u8 {
        if *self >= Self::CHIP_NAVI21 {
            16
        } else if *self >= Self::CHIP_NAVI10 {
            20
        } else if *self >= Self::CHIP_POLARIS10 && *self <= Self::CHIP_VEGAM {
            8
        } else {
            10
        }
    }

    pub fn num_simd_per_cu(&self) -> u8 {
        if *self >= Self::CHIP_NAVI10 {
            2
        } else {
            4
        }
    }

    pub fn cu_group(&self) -> u8 {
        if *self >= Self::CHIP_NAVI10 {
            2
        } else {
            1
        }
    }

    pub fn l1_cache_size(&self) -> u32 {
        if *self >= Self::CHIP_GFX1100 {
            32 * 1024 // KiB
        } else {
            16 * 1024 // KiB
        }
    }

    /* RDNA L1cache, per ShaderArray */
    pub fn gl1_cache_size(&self) -> u32 {
        if *self >= Self::CHIP_GFX1100 {
            256 * 1024 // KiB
        } else if *self >= Self::CHIP_NAVI10 {
            128 * 1024 // KiB
        } else {
            0
        }
    }

    pub fn l2_cache_size_per_block(&self) -> u32 {
        match self {
            Self::CHIP_TAHITI
            | Self::CHIP_PITCAIRN
            | Self::CHIP_OLAND
            | Self::CHIP_HAWAII
            | Self::CHIP_KABINI
            | Self::CHIP_TONGA
            | Self::CHIP_STONEY
            | Self::CHIP_RAVEN2 => 64 * 1024,
            Self::CHIP_VERDE
            | Self::CHIP_HAINAN
            | Self::CHIP_BONAIRE
            | Self::CHIP_KAVERI
            | Self::CHIP_ICELAND
            | Self::CHIP_CARRIZO
            | Self::CHIP_FIJI
            | Self::CHIP_POLARIS12
            | Self::CHIP_VEGAM => 128 * 1024,
            Self::CHIP_REMBRANDT => 512 * 1024,
            _ => 256 * 1024,
        }
    }

    pub fn l2_cache_line_size(&self) -> u32 {
        if *self >= Self::CHIP_NAVI10 || *self == Self::CHIP_ALDEBARAN {
            128
        } else {
            64
        }
    }

    pub fn l3_cache_size_mb_per_channel(&self) -> u32 {
        match self {
            Self::CHIP_NAVI21 | Self::CHIP_NAVI22 => 8,
            Self::CHIP_NAVI23 | Self::CHIP_NAVI24 => 4,
            _ => 0,
        }
    }

    #[cfg(feature = "std")]
    pub fn get_llvm_processor_name(&self) -> String {
        match self {
            Self::CHIP_TAHITI => "tahiti",
            Self::CHIP_PITCAIRN => "pitcairn",
            Self::CHIP_VERDE => "verde",
            Self::CHIP_OLAND => "oland",
            Self::CHIP_HAINAN => "hainan",
            Self::CHIP_BONAIRE => "bonaire",
            Self::CHIP_KABINI => "kabini",
            Self::CHIP_KAVERI => "kaveri",
            Self::CHIP_HAWAII => "hawaii",
            Self::CHIP_TONGA => "tonga",
            Self::CHIP_ICELAND => "iceland",
            Self::CHIP_CARRIZO => "carrizo",
            Self::CHIP_FIJI => "fiji",
            Self::CHIP_STONEY => "stoney",
            Self::CHIP_POLARIS10 => "polaris10",
            Self::CHIP_POLARIS11 | Self::CHIP_POLARIS12 | Self::CHIP_VEGAM => "polaris11",
            Self::CHIP_VEGA10 => "gfx900",
            Self::CHIP_RAVEN => "gfx902",
            Self::CHIP_VEGA12 => "gfx904",
            Self::CHIP_VEGA20 => "gfx906",
            Self::CHIP_RAVEN2 | Self::CHIP_RENOIR => "gfx909",
            Self::CHIP_ARCTURUS => "gfx908",
            Self::CHIP_ALDEBARAN => "gfx90a",
            Self::CHIP_NAVI10 => "gfx1010",
            Self::CHIP_NAVI12 => "gfx1011",
            Self::CHIP_NAVI14 => "gfx1012",
            Self::CHIP_NAVI21 => "gfx1030",
            Self::CHIP_NAVI22 => "gfx1030",
            /*
                if cfg!(version("1.52")) { // LLVM 12
                    "gfx1031"
                } else {
                    "gfx1030"
                },
            */
            Self::CHIP_NAVI23 => "gfx1030",
            /*
                if cfg!(version("1.52")) { // LLVM 12
                    "gfx1032"
                } else {
                    "gfx1030"
                },
            */
            Self::CHIP_VANGOGH => "gfx1030",
            /*
                if cfg!(version("1.52")) { // LLVM 12
                    "gfx1033"
                } else {
                    "gfx1030"
                },
            */
            Self::CHIP_NAVI24 => "gfx1030",
            /*
                if cfg!(version("1.56")) { // LLVM 13
                    "gfx1034"
                } else {
                    "gfx1030"
                },
            */
            Self::CHIP_REMBRANDT => "gfx1030",
            /*
                if cfg!(version("1.56")) { // LLVM 13
                    "gfx1035"
                } else {
                    "gfx1030"
                },
            */
            Self::CHIP_GFX1036 => "gfx1030",
            /*
            Self::CHIP_GFX1100 =>
                "gfx1100",
            Self::CHIP_GFX1101 =>
                "gfx1101",
            Self::CHIP_GFX1102 =>
                "gfx1102",
            Self::CHIP_GFX1103 =>
                "gfx1103",
            */
            _ => "",
        }
        .to_string()
    }
}

#[test]
fn test_asic_name_get() {
    assert_eq!(
        ASIC_NAME::get(FAMILY_NAME::VI, 0x5C),
        ASIC_NAME::CHIP_POLARIS11,
    );
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for ASIC_NAME {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CHIP_UNKNOWN => write!(f, "Unknown"),
            /* R3xx-based cores. (GFX2) */
            Self::CHIP_R300 => write!(f, "R300"),
            Self::CHIP_R350 => write!(f, "R350"),
            Self::CHIP_RV350 => write!(f, "RV350"),
            Self::CHIP_RV370 => write!(f, "RV370"),
            Self::CHIP_RV380 => write!(f, "RV380"),
            Self::CHIP_RS400 => write!(f, "RS400"),
            Self::CHIP_RC410 => write!(f, "RC410"),
            Self::CHIP_RS480 => write!(f, "RS480"),
            /* R4xx-based cores. (GFX2) */
            Self::CHIP_R420 => write!(f, "R420"),
            Self::CHIP_R423 => write!(f, "R423"),
            Self::CHIP_R430 => write!(f, "R430"),
            Self::CHIP_R480 => write!(f, "R480"),
            Self::CHIP_R481 => write!(f, "R481"),
            Self::CHIP_RV410 => write!(f, "RV410"),
            Self::CHIP_RS600 => write!(f, "RS600"),
            Self::CHIP_RS690 => write!(f, "RS690"),
            Self::CHIP_RS740 => write!(f, "RS740"),
            /* R5xx-based cores. (GFX2) */
            Self::CHIP_RV515 => write!(f, "RV515"),
            Self::CHIP_R520 => write!(f, "R520"),
            Self::CHIP_RV530 => write!(f, "RV530"),
            Self::CHIP_R580 => write!(f, "R580"),
            Self::CHIP_RV560 => write!(f, "RV560"),
            Self::CHIP_RV570 => write!(f, "RV570"),
            /* GFX3 (R6xx) */
            Self::CHIP_R600 => write!(f, "R600"),
            Self::CHIP_RV610 => write!(f, "RV610"),
            Self::CHIP_RV630 => write!(f, "RV630"),
            Self::CHIP_RV670 => write!(f, "RV670"),
            Self::CHIP_RV620 => write!(f, "RV620"),
            Self::CHIP_RV635 => write!(f, "RV635"),
            Self::CHIP_RS780 => write!(f, "RS780"),
            Self::CHIP_RS880 => write!(f, "RS880"),
            /* GFX3 (R7xx) */
            Self::CHIP_RV770 => write!(f, "RV770"),
            Self::CHIP_RV730 => write!(f, "RV730"),
            Self::CHIP_RV710 => write!(f, "RV710"),
            Self::CHIP_RV740 => write!(f, "RV740"),
            /* GFX4 (Evergreen) */
            Self::CHIP_CEDAR => write!(f, "Cedar"),
            Self::CHIP_REDWOOD => write!(f, "Redwood"),
            Self::CHIP_JUNIPER => write!(f, "Jupiter"),
            Self::CHIP_CYPRESS => write!(f, "Cypress"),
            Self::CHIP_HEMLOCK => write!(f, "Hemlock"),
            Self::CHIP_PALM => write!(f, "Palm"),
            Self::CHIP_SUMO => write!(f, "Sumo"),
            Self::CHIP_SUMO2 => write!(f, "Sumo2"),
            Self::CHIP_BARTS => write!(f, "Barts"),
            Self::CHIP_TURKS => write!(f, "Turks"),
            Self::CHIP_CAICOS => write!(f, "Caicos"),
            /* GFX5 (Northern Islands) */
            Self::CHIP_CAYMAN => write!(f, "Cayman"),
            Self::CHIP_ARUBA => write!(f, "Aruba"),
            /* GFX6 (Southern Islands) */
            Self::CHIP_TAHITI => write!(f, "Tahiti"),
            Self::CHIP_PITCAIRN => write!(f, "Pitcairn"),
            Self::CHIP_VERDE => write!(f, "Verde"),
            Self::CHIP_OLAND => write!(f, "Oland"),
            Self::CHIP_HAINAN => write!(f, "Hainan"),
            /* GFX7 (Sea Islands) */
            Self::CHIP_BONAIRE => write!(f, "Bonaire"),
            Self::CHIP_KAVERI => write!(f, "Kaveri"),
            Self::CHIP_KABINI => write!(f, "Kabini"),
            Self::CHIP_HAWAII => write!(f, "Hawaii"),
            /* GFX8 (Volcanic Islands & Polaris) */
            Self::CHIP_TONGA => write!(f, "Tonga"),
            Self::CHIP_ICELAND => write!(f, "Iceland"),
            Self::CHIP_CARRIZO => write!(f, "Carrizo"),
            Self::CHIP_FIJI => write!(f, "Fiji"),
            Self::CHIP_STONEY => write!(f, "Stoney"),
            Self::CHIP_POLARIS10 => write!(f, "Polaris10"),
            Self::CHIP_POLARIS11 => write!(f, "Polaris11"),
            Self::CHIP_POLARIS12 => write!(f, "Polaris12"),
            Self::CHIP_VEGAM => write!(f, "VegaM"),
            /* GFX9 (Vega) */
            Self::CHIP_VEGA10 => write!(f, "Vega10"),
            Self::CHIP_VEGA12 => write!(f, "Vega12"),
            Self::CHIP_VEGA20 => write!(f, "Vega20"),
            Self::CHIP_RAVEN => write!(f, "Raven"),
            Self::CHIP_RAVEN2 => write!(f, "Raven2"),
            Self::CHIP_RENOIR => write!(f, "Renoir"),
            Self::CHIP_ARCTURUS => write!(f, "Arcturus"),
            Self::CHIP_ALDEBARAN => write!(f, "Aldebaran"),
            /* GFX10.1 (RDNA 1) */
            Self::CHIP_NAVI10 => write!(f, "Navi10"),
            Self::CHIP_NAVI12 => write!(f, "Navi12"),
            Self::CHIP_NAVI14 => write!(f, "Navi14"),
            /* GFX10.3 (RDNA 2) */
            Self::CHIP_NAVI21 => write!(f, "Sienna Cichlid/Navi21"),
            Self::CHIP_NAVI22 => write!(f, "Navy Flounder/Navi22"),
            Self::CHIP_VANGOGH => write!(f, "VanGogh"),
            Self::CHIP_NAVI23 => write!(f, "Dimgrey Cavefish/Navi23"),
            Self::CHIP_NAVI24 => write!(f, "Beige Goby/Navi24"),
            Self::CHIP_REMBRANDT => write!(f, "Yellow Carp/Rembrandt"),
            Self::CHIP_GFX1036 => write!(f, "GFX1036"),
            Self::CHIP_GFX1100 => write!(f, "GFX1100"),
            Self::CHIP_GFX1101 => write!(f, "GFX1101"),
            Self::CHIP_GFX1102 => write!(f, "GFX1102"),
            Self::CHIP_GFX1103_R1 => write!(f, "GFX1103_R1"),
            Self::CHIP_GFX1103_R2 => write!(f, "GFX1103_R2"),
        }
    }
}
