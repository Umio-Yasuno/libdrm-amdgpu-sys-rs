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

use crate::AMDGPU::ASIC_NAME;

/// List of AMDGPU chip class (generation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u32)]
pub enum CHIP_CLASS {
    CLASS_UNKNOWN = 0,
    R300,
    R400,
    R500,
    R600,
    R700,
    EVERGREEN,
    CAYMAN,
    GFX6,
    GFX7,
    GFX8,
    GFX9,
    GFX10,
    GFX10_3,
    GFX11,
    GFX12,
}

impl From<ASIC_NAME> for CHIP_CLASS {
    fn from(asic_name: ASIC_NAME) -> Self {
        if asic_name >= ASIC_NAME::CHIP_GFX1100 {
            Self::GFX12
        } else if asic_name >= ASIC_NAME::CHIP_GFX1100 {
            Self::GFX11
        } else if asic_name >= ASIC_NAME::CHIP_NAVI21 {
            Self::GFX10_3
        } else if asic_name >= ASIC_NAME::CHIP_NAVI10 {
            Self::GFX10
        } else if asic_name >= ASIC_NAME::CHIP_VEGA10 {
            Self::GFX9
        } else if asic_name >= ASIC_NAME::CHIP_TONGA {
            Self::GFX8
        } else if asic_name >= ASIC_NAME::CHIP_BONAIRE {
            Self::GFX7
        } else if asic_name >= ASIC_NAME::CHIP_TAHITI {
            Self::GFX6
        } else {
            Self::CLASS_UNKNOWN
        }
    }
}

impl CHIP_CLASS {
    pub fn has_packed_math_16bit(&self) -> bool {
        *self >= Self::GFX9
    }

    pub fn cu_group(&self) -> u8 {
        if *self >= Self::GFX10 {
            2
        } else {
            1
        }
    }
}

#[test]
fn test_amdgpu_chip_class() {
    assert_eq!(ASIC_NAME::CHIP_POLARIS11.chip_class(), CHIP_CLASS::GFX8,)
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for CHIP_CLASS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CLASS_UNKNOWN => write!(f, "Unknown"),
            Self::R300 => write!(f, "R300"),
            Self::R400 => write!(f, "R400"),
            Self::R500 => write!(f, "R500"),
            Self::R600 => write!(f, "R600"),
            Self::R700 => write!(f, "R700"),
            Self::EVERGREEN => write!(f, "Evergreen"),
            Self::CAYMAN => write!(f, "Cayman"),
            Self::GFX6 => write!(f, "GFX6"),
            Self::GFX7 => write!(f, "GFX7"),
            Self::GFX8 => write!(f, "GFX8"),
            Self::GFX9 => write!(f, "GFX9"),
            Self::GFX10 => write!(f, "GFX10"),
            Self::GFX10_3 => write!(f, "GFX10_3"),
            Self::GFX11 => write!(f, "GFX11"),
            Self::GFX12 => write!(f, "GFX12"),
        }
    }
}
