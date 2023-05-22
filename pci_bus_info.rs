pub mod PCI {
    /// PCI information (Domain, Bus, Device, Function)
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct BUS_INFO {
        pub domain: u16,
        pub bus: u8,
        pub dev: u8,
        pub func: u8,
    }

    /// PCI link status
    pub enum STATUS {
        Current,
        Max,
    }

    /// PCI link speed information
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LINK {
        pub gen: u8,
        pub width: u8,
    }
}

#[cfg(feature = "std")]
use std::path::PathBuf;

#[cfg(feature = "std")]
const PCIE_DPM: &str = "pp_dpm_pcie";

impl PCI::BUS_INFO {
    pub(crate) fn drm_get_device2(
        fd: ::core::ffi::c_int,
        //  flags: u32,
    ) -> Result<Self, i32> {
        let pci = unsafe {
            let mut dev_info = __drmGetDevice2(fd, 0)?;
            let pci = core::ptr::read((*dev_info).businfo.pci);
            __drmFreeDevice(&mut dev_info);

            pci
        };

        Ok(PCI::BUS_INFO {
            domain: pci.domain,
            bus: pci.bus,
            dev: pci.dev,
            func: pci.func,
        })
    }

    /// Convert a string ("0000:01:00.0") to [PCI::BUS_INFO]
    #[cfg(feature = "std")]
    pub fn from_number_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }

    #[cfg(feature = "std")]
    pub fn get_sysfs_path(&self) -> PathBuf {
        PathBuf::from("/sys/bus/pci/devices/").join(self.to_string())
    }

    #[cfg(feature = "std")]
    pub fn get_hwmon_path(&self) -> Option<PathBuf> {
        /*
            use std::ffi::OsString;

            let base = PathBuf::from("/sys/class/hwmon");
            let hwmon_dir = std::fs::read_dir(&base).ok()?;

            for hwmon in hwmon_dir {
                let Ok(hwmon) = hwmon else { continue };
                let link = std::fs::read_link(hwmon.path()).ok()?;
                // "../../devices/pci0000:00/0000:00:01.1/0000:01:00.0/hwmon/hwmon1"
                let pci = link.iter().skip(5).next()?;

                if pci.to_os_string() == OsString::from(self.to_string()) {
                    return std::fs::canonicalize(base.join(link)).ok();
                }
            }

            None
        */
        let base = self.get_sysfs_path().join("hwmon");
        let hwmon_dir = std::fs::read_dir(base).ok()?;

        for entry in hwmon_dir {
            let entry = entry.ok()?;
            if entry.metadata().ok()?.is_dir() {
                return Some(entry.path());
            }
        }

        None
    }

    /// Returns paths to sysfs for PCI information
    #[cfg(feature = "std")]
    pub fn get_link_sysfs_path(&self, status: PCI::STATUS) -> [PathBuf; 2] {
        let status = match status {
            PCI::STATUS::Current => "current",
            PCI::STATUS::Max => "max",
        };
        let path = PathBuf::from(format!("/sys/bus/pci/devices/{}/", self));

        [
            format!("{status}_link_speed"),
            format!("{status}_link_width"),
        ]
        .map(|file_name| path.join(file_name))
    }

    /// Returns [PCI::LINK]
    #[cfg(feature = "std")]
    pub fn get_link_info(&self, status: PCI::STATUS) -> PCI::LINK {
        let [speed, width] = Self::get_link_sysfs_path(self, status)
            .map(|path| std::fs::read_to_string(path).unwrap_or_default());

        let gen = Self::speed_to_gen(speed.trim());
        let width: u8 = width.trim().parse().unwrap_or(0);

        PCI::LINK {
            gen,
            width
        }
    }

    #[cfg(feature = "std")]
    fn parse_dpm_line(s: &str) -> Option<PCI::LINK> {
        let mut link = PCI::LINK { gen: 0, width: 0 };

        for tmp in s.split(", ") {
            if tmp.ends_with("GT/s") {
                // "0: 2.5GT/s"
                let Some(pos) = tmp.find(' ') else { continue };
                link.gen = match &tmp[(pos+1)..] {
                    "2.5GT/s" => 1,
                    "5.0GT/s" => 2,
                    "8.0GT/s" => 3,
                    "16.0GT/s" => 4,
                    "32.0GT/s" => 5,
                    "64.0GT/s" => 6,
                    _ => 0,
                };
                continue;
            }

            if tmp.starts_with('x') {
                // "x8 ", "x16 * "
                let tmp = tmp.trim_start_matches('x');
                let Some(space_pos) = tmp.find(' ') else { continue };
                link.width = tmp[..space_pos].parse().unwrap_or(0);
                continue;
            }
        }

        if link.gen != 0 && link.width != 0 {
            Some(link)
        } else {
            None
        }
    }

    #[cfg(feature = "std")]
    pub fn get_min_max_link_info_from_dpm(&self) -> Option<[PCI::LINK; 2]> {
        let sysfs_path = self.get_sysfs_path();
        let s = std::fs::read_to_string(sysfs_path.join(PCIE_DPM)).ok()?;
        let mut lines = s.lines();

        let first = Self::parse_dpm_line(lines.next()?)?;
        let last = match lines.last() {
            Some(last) => Self::parse_dpm_line(last)?,
            None => return Some([first; 2]),
        };

        Some([
            std::cmp::min(first, last),
            std::cmp::max(first, last),
        ])
    }

    #[cfg(feature = "std")]
    pub fn get_current_link_info_from_dpm(&self) -> Option<PCI::LINK> {
        let sysfs_path = self.get_sysfs_path();
        let s = std::fs::read_to_string(sysfs_path.join(PCIE_DPM)).ok()?;
        let cur = s.lines().find(|&line| line.ends_with(" *"))?;

        Self::parse_dpm_line(cur)
    }

    /// Convert PCIe speed to PCIe gen
    #[cfg(feature = "std")]
    fn speed_to_gen(speed: &str) -> u8 {
        match speed {
            "2.5 GT/s PCIe" => 1,
            "5.0 GT/s PCIe" => 2,
            "8.0 GT/s PCIe" => 3,
            "16.0 GT/s PCIe" => 4,
            "32.0 GT/s PCIe" => 5,
            "64.0 GT/s PCIe" => 6,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBusInfoError;

#[cfg(feature = "std")]
impl std::str::FromStr for PCI::BUS_INFO {
    type Err = ParseBusInfoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(&[':', '.']).take(4);
        let [domain, bus, dev, func] = [split.next(), split.next(), split.next(), split.next()]
            .map(|s| s.ok_or(ParseBusInfoError));
        let domain = u16::from_str_radix(domain?, 16).map_err(|_| ParseBusInfoError);
        let [bus, dev, func] = [bus, dev, func].map(|v| {
            u8::from_str_radix(v?, 16).map_err(|_| ParseBusInfoError)
        });

        Ok(Self {
            domain: domain?,
            bus: bus?,
            dev: dev?,
            func: func?,
        })
    }
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for PCI::BUS_INFO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{:04x}:{:02x}:{:02x}.{:01x}",
            self.domain, self.bus, self.dev, self.func
        )
    }
}

use crate::bindings::{self, drmDevicePtr, drmFreeDevice};
use crate::query_error;
use core::mem::MaybeUninit;

unsafe fn __drmGetDevice2(fd: ::core::ffi::c_int, flags: u32) -> Result<drmDevicePtr, i32> {
    let mut drm_dev_info: MaybeUninit<drmDevicePtr> = MaybeUninit::uninit();

    let r = bindings::drmGetDevice2(fd, flags, drm_dev_info.as_mut_ptr());

    let drm_dev_info = drm_dev_info.assume_init();

    if drm_dev_info.is_null() {
        return Err(r);
    }

    query_error!(r);

    Ok(drm_dev_info)
}

unsafe fn __drmFreeDevice(device: *mut drmDevicePtr) {
    drmFreeDevice(device)
}
