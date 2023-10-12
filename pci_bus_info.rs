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
    #[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
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

    #[cfg(feature = "std")]
    pub fn get_link_info(&self, status: PCI::STATUS) -> PCI::LINK {
        PCI::LINK::get_from_sysfs_with_status(self.get_sysfs_path(), status).unwrap_or_default()
    }

    #[cfg(feature = "std")]
    pub fn get_min_max_link_info_from_dpm(&self) -> Option<[PCI::LINK; 2]> {
        PCI::LINK::get_min_max_link_info_from_dpm(self.get_sysfs_path())
    }

    #[cfg(feature = "std")]
    pub fn get_current_link_info_from_dpm(&self) -> Option<PCI::LINK> {
        PCI::LINK::get_current_link_info_from_dpm(self.get_sysfs_path())
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

#[test]
fn test_pci_bus_info_parse() {
    let s = "0000:0d:00.0".parse();
    let bus = PCI::BUS_INFO { domain: 0x0, bus: 0xd, dev: 0x0, func: 0x0 };

    assert_eq!(s, Ok(bus));
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

impl PCI::STATUS {
    pub const fn to_sysfs_file_name(&self) -> [&str; 2] {
        match self {
            Self::Current => ["current_link_speed", "current_link_width"],
            Self::Max => ["max_link_speed", "max_link_width"],
        }
    }
}

impl PCI::LINK {
    #[cfg(feature = "std")]
    pub fn get_from_sysfs_with_status<P: Into<PathBuf>>(
        sysfs_path: P,
        status: PCI::STATUS,
    ) -> Option<Self> {
        let base_path = sysfs_path.into();
        let [s_speed, s_width] = status.to_sysfs_file_name().map(|name| {
            let mut s = std::fs::read_to_string(base_path.join(name)).ok()?;
            s.pop(); // trim `\n`

            Some(s)
        });

        let gen = Self::speed_to_gen(&s_speed?)?;
        let width = s_width?.parse::<u8>().ok()?;

        Some(Self { gen, width })
    }

    /// Convert PCIe speed str to PCIe gen
    #[cfg(feature = "std")]
    pub fn speed_to_gen(speed: &str) -> Option<u8> {
        let gen = match speed {
            "2.5 GT/s PCIe" => 1,
            "5.0 GT/s PCIe" => 2,
            "8.0 GT/s PCIe" => 3,
            "16.0 GT/s PCIe" => 4,
            "32.0 GT/s PCIe" => 5,
            "64.0 GT/s PCIe" => 6,
            _ => return None,
        };

        Some(gen)
    }

    #[cfg(feature = "std")]
    fn parse_dpm_line(s: &str) -> Option<Self> {
        let mut gen: Option<u8> = None;
        let mut width: Option<u8> = None;

        for tmp in s.split(", ") {
            if tmp.ends_with("GT/s") {
                // "0: 2.5GT/s"
                let Some(pos) = tmp.find(' ') else { continue };
                gen = {
                    let tmp = match &tmp[(pos+1)..] {
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
                width = tmp[..space_pos].parse().ok();
                continue;
            }
        }

        Some(Self { gen: gen?, width: width? })
    }

    #[cfg(feature = "std")]
    pub fn get_min_max_link_info_from_dpm<P: Into<PathBuf>>(
        sysfs_path: P,
    ) -> Option<[PCI::LINK; 2]> {
        use crate::get_min_max_from_dpm;

        get_min_max_from_dpm(sysfs_path.into().join(PCIE_DPM), Self::parse_dpm_line)
    }

    #[cfg(feature = "std")]
    pub fn get_current_link_info_from_dpm<P: Into<PathBuf>>(sysfs_path: P) -> Option<PCI::LINK> {
        let sysfs_path = sysfs_path.into();
        let s = std::fs::read_to_string(sysfs_path.join(PCIE_DPM)).ok()?;
        let cur = s.lines().find(|&line| line.ends_with(" *"))?;

        Self::parse_dpm_line(cur)
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
