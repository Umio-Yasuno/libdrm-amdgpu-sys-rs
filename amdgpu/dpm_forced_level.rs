use std::io;
use std::path::PathBuf;

const SYSFS_NAME: &str = "power_dpm_force_performance_level";

#[derive(Debug, Copy, Clone)]
pub enum DpmForcedLevel {
    Auto,
    Manual,
    Low,
    High,
    ProfileStandard,
    ProfileMinSclk,
    ProfileMinMclk,
    ProfilePeak,
    ProfileExit,
    PerfDeterminism,
}

impl DpmForcedLevel {
    pub fn get_from_sysfs<P: Into<PathBuf>>(sysfs_path: P) -> io::Result<Self> {
        let sysfs_path = sysfs_path.into();
        let s = std::fs::read_to_string(sysfs_path.join(SYSFS_NAME))?;

        let level = match s.get(0..4).unwrap_or("") {
            "auto" => Self::Auto,
            "low\n" => Self::Low,
            "high" => Self::High,
            "manu" => Self::Manual,
            "prof" => {
                const PRE_LEN: usize = "profile_".len();

                match s.get(PRE_LEN..).unwrap_or("") {
                    "standard\n" => Self::ProfileStandard,
                    "peak\n" => Self::ProfilePeak,
                    "min_sclk\n" => Self::ProfileMinSclk,
                    "min_mclk\n" => Self::ProfileMinMclk,
                    "exit\n" => Self::ProfileExit,
                    _ => return Err(io::Error::other("Unknown Level")),
                }
            },
            "perf" => Self::PerfDeterminism,
            _ => return Err(io::Error::other("Unknown Level")),
        };

        Ok(level)
    }

    pub const fn to_arg(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Low => "low",
            Self::High => "high",
            Self::Manual => "manual",
            Self::ProfileStandard => "profile_standard",
            Self::ProfilePeak => "profile_peak",
            Self::ProfileMinSclk => "profile_min_sclk",
            Self::ProfileMinMclk => "profile_min_mclk",
            Self::ProfileExit => "profile_exit",
            Self::PerfDeterminism => "perf_determinism",
        }
    }
}
