use crate::AMDGPU::DeviceHandle;
use std::str::FromStr;
use std::path::PathBuf;
use super::parse_hwmon;

impl DeviceHandle {
    pub fn get_power_cap(&self) -> Option<PowerCap> {
        let hwmon_path = self.get_hwmon_path()?;

        PowerCap::from_hwmon_path(hwmon_path)
    }
}

#[derive(Clone, Debug)]
pub struct PowerCap {
    pub type_: PowerCapType,
    pub current: u32, // W
    pub default: u32, // W
    pub min: u32, // W
    pub max: u32, // W
}

impl PowerCap {
    pub fn from_hwmon_path<P: Into<PathBuf>>(path: P) -> Option<Self> {
        let path = path.into();

        let label = match std::fs::read_to_string(path.join("power1_label")) {
            Ok(s) => s,
            Err(_) => std::fs::read_to_string(path.join("power2_label")).ok()?,
        };
        let type_ = PowerCapType::from_str(label.as_str().trim_end()).ok()?;
        let [current, default, min, max] = type_.file_names().map(|name| {
            parse_hwmon::<u32, _>(path.join(name)).map(|v| v.saturating_div(1_000_000))
        });

        Some(Self {
            type_,
            current: current?,
            default: default?,
            min: min?,
            max: max?,
        })
    }

    /// ref: drivers/gpu/drm/amd/pm/swsmu/smu13/aldebaran_ppt.c
    /// ref: <https://github.com/RadeonOpenCompute/rocm_smi_lib/blob/master/python_smi_tools/rocm_smi.py>
    pub fn check_if_secondary_die(&self) -> bool {
        self.current == 0 && self.default == 0 && self.max == 0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PowerCapType {
    PPT,
    FastPPT,
    SlowPPT,
}

impl PowerCapType {
    const fn file_names(&self) -> [&str; 4] {
        match self {
            Self::PPT =>
                ["power1_cap", "power1_cap_default", "power1_cap_min", "power1_cap_max"],
            // for VanGogh APU
            Self::FastPPT |
            Self::SlowPPT =>
                ["power2_cap", "power2_cap_default", "power2_cap_min", "power2_cap_max"],
        }
    }
}

use std::fmt;
impl fmt::Display for PowerCapType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePowerCapTypeError;

impl FromStr for PowerCapType {
    type Err = ParsePowerCapTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PPT" => Ok(Self::PPT),
            "fastPPT" => Ok(Self::FastPPT),
            "slowPPT" => Ok(Self::SlowPPT),
            _ => Err(ParsePowerCapTypeError),
        }
    }
}
