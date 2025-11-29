use crate::AMDGPU::DeviceHandle;
use std::fs;
use std::path::PathBuf;

/* ref: drivers/gpu/drm/amd/include/kgd_pp_interface.h */

const POWER_PROFILE_BOOTUP_DEFAULT: u32 = 0x0;
const POWER_PROFILE_FULLSCREEN3D: u32 = 0x1;
const POWER_PROFILE_POWERSAVING: u32 = 0x2;
const POWER_PROFILE_VIDEO: u32 = 0x3;
const POWER_PROFILE_VR: u32 = 0x4;
const POWER_PROFILE_COMPUTE: u32 = 0x5;
const POWER_PROFILE_CUSTOM: u32 = 0x6;
const POWER_PROFILE_WINDOW3D: u32 = 0x7;
const POWER_PROFILE_CAPPED: u32 = 0x8;
const POWER_PROFILE_UNCAPPED: u32 = 0x9;

const FILE_NAME: &str = "pp_power_profile_mode";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u32)]
pub enum PowerProfile {
    BOOTUP_DEFAULT = self::POWER_PROFILE_BOOTUP_DEFAULT,
    FULLSCREEN3D = self::POWER_PROFILE_FULLSCREEN3D,
    POWERSAVING = self::POWER_PROFILE_POWERSAVING,
    VIDEO = self::POWER_PROFILE_VIDEO,
    VR = self::POWER_PROFILE_VR,
    COMPUTE = self::POWER_PROFILE_COMPUTE,
    CUSTOM = self::POWER_PROFILE_CUSTOM,
    WINDOW3D = self::POWER_PROFILE_WINDOW3D,
    CAPPED = self::POWER_PROFILE_CAPPED,
    UNCAPPED = self::POWER_PROFILE_UNCAPPED,
    COUNT,
}

impl DeviceHandle {
    pub fn get_all_supported_profiles(&self) -> Vec<PowerProfile> {
        let Ok(sysfs_path) = self.get_sysfs_path() else { return Vec::new() };

        PowerProfile::get_all_supported_profiles_from_sysfs(sysfs_path)
    }

    pub fn get_current_profile(&self) -> Option<PowerProfile> {
        let sysfs_path = self.get_sysfs_path().ok()?;

        PowerProfile::get_current_profile_from_sysfs(sysfs_path)
    }
}

impl PowerProfile {
    pub fn get_all_supported_profiles_from_sysfs<P: Into<PathBuf>>(sysfs: P) -> Vec<Self> {
        let sysfs = sysfs.into();
        let Ok(s) = fs::read_to_string(sysfs.join(FILE_NAME)) else { return Vec::new() };

        s.lines().filter_map(|line| PowerProfile::parse_line(line)).collect()
    }

    pub fn get_current_profile_from_sysfs<P: Into<PathBuf>>(sysfs: P) -> Option<Self> {
        let sysfs = sysfs.into();
        let s = fs::read_to_string(sysfs.join(FILE_NAME)).ok()?;

        s.lines().find_map(|line| {
            let profile = Self::parse_line(line)?;

            if line.ends_with('*') || line.contains("*:") {
                Some(profile)
            } else {
                None
            }
        })
    }

    /*
        TODO: This code does not work correctly in SMU v13.0.7 (GC11.0.2?/Navi33?/GFX1102?).
        ref: drivers/gpu/drm/amd/pm/swsmu/smu13/smu_v13_0_7_ppt.c
    */
    fn parse_line(s: &str) -> Option<PowerProfile> {
        let line = s.trim_start();
        let i = line.find(' ')?;
        let profile = line.get(..i)?.parse::<u32>().ok()?;
        
        PowerProfile::try_from(profile).ok()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnknownPowerProfile;

impl TryFrom<u32> for PowerProfile {
    type Error = UnknownPowerProfile;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            self::POWER_PROFILE_BOOTUP_DEFAULT => Ok(Self::BOOTUP_DEFAULT),
            self::POWER_PROFILE_FULLSCREEN3D => Ok(Self::FULLSCREEN3D),
            self::POWER_PROFILE_POWERSAVING => Ok(Self::POWERSAVING),
            self::POWER_PROFILE_VIDEO => Ok(Self::VIDEO),
            self::POWER_PROFILE_VR => Ok(Self::VR),
            self::POWER_PROFILE_COMPUTE => Ok(Self::COMPUTE),
            self::POWER_PROFILE_CUSTOM => Ok(Self::CUSTOM),
            self::POWER_PROFILE_WINDOW3D => Ok(Self::WINDOW3D),
            self::POWER_PROFILE_CAPPED => Ok(Self::CAPPED),
            self::POWER_PROFILE_UNCAPPED => Ok(Self::UNCAPPED),
            _ => Err(UnknownPowerProfile),
        }
    }
}

use std::fmt;
impl fmt::Display for PowerProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BOOTUP_DEFAULT => write!(f, "BOOTUP_DEFAULT"),
            Self::FULLSCREEN3D => write!(f, "3D_FULL_SCREEN"),
            Self::POWERSAVING => write!(f, "POWER_SAVING"),
            Self::VIDEO => write!(f, "VIDEO"),
            Self::VR => write!(f, "VR"),
            Self::COMPUTE => write!(f, "COMPUTE"),
            Self::CUSTOM => write!(f, "CUSTOM"),
            Self::WINDOW3D => write!(f, "WINDOW_3D"),
            Self::CAPPED => write!(f, "CAPPED"),
            Self::UNCAPPED => write!(f, "UNCAPPED"),
            Self::COUNT => write!(f, ""),
        }
    }
}
