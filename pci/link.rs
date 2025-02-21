use std::path::PathBuf;

const PCIE_DPM: &str = "pp_dpm_pcie";

/// PCI link speed information
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct LINK {
    pub r#gen: u8,
    pub width: u8,
}

use super::STATUS;

impl LINK {
    pub fn get_from_sysfs_with_status<P: Into<PathBuf>>(
        sysfs_path: P,
        status: STATUS,
    ) -> Option<Self> {
        let base_path = sysfs_path.into();
        let [s_speed, s_width] = status.to_sysfs_file_name().map(|name| {
            let mut s = std::fs::read_to_string(base_path.join(name)).ok()?;
            s.pop(); // trim `\n`

            Some(s)
        });

        let r#gen = Self::speed_to_gen(&s_speed?)?;
        let width = s_width?.parse::<u8>().ok()?;

        Some(Self { r#gen, width })
    }

    /// Convert PCIe speed str to PCIe gen
    pub fn speed_to_gen(speed: &str) -> Option<u8> {
        let r#gen = match speed {
            "2.5 GT/s PCIe" => 1,
            "5.0 GT/s PCIe" => 2,
            "8.0 GT/s PCIe" => 3,
            "16.0 GT/s PCIe" => 4,
            "32.0 GT/s PCIe" => 5,
            "64.0 GT/s PCIe" => 6,
            _ => return None,
        };

        Some(r#gen)
    }

    fn parse_dpm_line(s: &str) -> Option<Self> {
        let mut r#gen: Option<u8> = None;
        let mut width: Option<u8> = None;

        for tmp in s.split(", ") {
            if tmp.ends_with("GT/s") {
                // "0: 2.5GT/s"
                let Some(pos) = tmp.find(' ') else { continue };
                r#gen = {
                    let tmp = match tmp.get(pos+1..)? {
                        "2.5GT/s" => 1,
                        "5.0GT/s" => 2,
                        "8.0GT/s" => 3,
                        "16.0GT/s" => 4,
                        "32.0GT/s" => 5,
                        "64.0GT/s" => 6,
                        _ => 0,
                    };

                    (tmp != 0).then_some(tmp)
                };
                continue;
            }

            if tmp.starts_with('x') {
                // "x8 ", "x16 * "
                let tmp = tmp.trim_start_matches('x');
                let Some(space_pos) = tmp.find(' ') else { continue };
                width = tmp.get(..space_pos)?.parse().ok();
                continue;
            }
        }

        Some(Self { r#gen: r#gen?, width: width? })
    }

    pub fn get_min_max_link_info_from_dpm<P: Into<PathBuf>>(sysfs_path: P) -> Option<[LINK; 2]> {
        use crate::get_min_max_from_dpm;

        get_min_max_from_dpm(sysfs_path.into().join(PCIE_DPM), Self::parse_dpm_line)
    }

    pub fn get_current_link_info_from_dpm<P: Into<PathBuf>>(sysfs_path: P) -> Option<LINK> {
        let sysfs_path = sysfs_path.into();
        let s = std::fs::read_to_string(sysfs_path.join(PCIE_DPM)).ok()?;
        let cur = s.lines().find(|&line| line.ends_with(" *"))?;

        Self::parse_dpm_line(cur)
    }

    pub(crate) fn get_max_link(sysfs_path: &PathBuf) -> Option<Self> {
        let [s_speed, s_width] = STATUS::Max.to_sysfs_file_name().map(|name| {
            let mut s = std::fs::read_to_string(sysfs_path.join(name)).ok()?;
            s.pop(); // trim `\n`

            Some(s)
        });

        let r#gen = Self::speed_to_gen(&s_speed?)?;
        let width = s_width?.parse::<u8>().ok()?;

        Some(Self { r#gen, width })
    }
}
