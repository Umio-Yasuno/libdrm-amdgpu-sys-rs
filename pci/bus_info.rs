use super::{BUS_INFO, LINK, STATUS};

#[cfg(feature = "std")]
use std::path::PathBuf;

impl BUS_INFO {
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

        Ok(Self {
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
    pub fn get_link_info(&self, status: STATUS) -> LINK {
        LINK::get_from_sysfs_with_status(self.get_sysfs_path(), status).unwrap_or_default()
    }

    #[cfg(feature = "std")]
    pub fn get_min_max_link_info_from_dpm(&self) -> Option<[LINK; 2]> {
        LINK::get_min_max_link_info_from_dpm(self.get_sysfs_path())
    }

    #[cfg(feature = "std")]
    pub fn get_current_link_info_from_dpm(&self) -> Option<LINK> {
        LINK::get_current_link_info_from_dpm(self.get_sysfs_path())
    }

    #[cfg(feature = "std")]
    pub fn get_max_gpu_link(&self) -> Option<LINK> {
        let mut tmp = self.get_system_pcie_port_sysfs_path();

        tmp.pop();

        Self::get_max_link(&tmp)
    }

    #[cfg(feature = "std")]
    pub fn get_max_system_link(&self) -> Option<LINK> {
        Self::get_max_link(&self.get_system_pcie_port_sysfs_path())
    }

    #[cfg(feature = "std")]
    fn get_system_pcie_port_sysfs_path(&self) -> PathBuf {
        const NAVI10_UPSTREAM_PORT: &str = "0x1478\n";
        const NAVI10_DOWNSTREAM_PORT: &str = "0x1479\n";

        let mut tmp = self.get_sysfs_path().join("../"); // pcie port

        for _ in 0..2 {
            let Ok(did) = std::fs::read_to_string(&tmp.join("device")) else { break };

            if &did == NAVI10_UPSTREAM_PORT || &did == NAVI10_DOWNSTREAM_PORT {
                tmp.push("../");
            } else {
                break;
            }
        }

        tmp
    }

    #[cfg(feature = "std")]
    fn get_max_link(sysfs_path: &PathBuf) -> Option<LINK> {
        let [s_speed, s_width] = ["max_link_speed", "max_link_width"].map(|name| {
            let mut s = std::fs::read_to_string(sysfs_path.join(name)).ok()?;
            s.pop(); // trim `\n`

            Some(s)
        });

        let gen = match s_speed?.as_str() {
            "2.5 GT/s PCIe" => 1,
            "5.0 GT/s PCIe" => 2,
            "8.0 GT/s PCIe" => 3,
            "16.0 GT/s PCIe" => 4,
            "32.0 GT/s PCIe" => 5,
            "64.0 GT/s PCIe" => 6,
            _ => 0,
        };
        let width = s_width?.parse::<u8>().ok()?;

        Some(LINK { gen, width })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBusInfoError;

#[cfg(feature = "std")]
impl std::str::FromStr for super::BUS_INFO {
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
    let bus = super::BUS_INFO { domain: 0x0, bus: 0xd, dev: 0x0, func: 0x0 };

    assert_eq!(s, Ok(bus));
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for super::BUS_INFO {
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
