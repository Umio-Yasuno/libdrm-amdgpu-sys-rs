use super::BUS_INFO;
#[cfg(feature = "std")]
use super::{LINK, STATUS};

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

    /// Get device sysfs path
    #[cfg(feature = "std")]
    pub fn get_sysfs_path(&self) -> PathBuf {
        PathBuf::from("/sys/bus/pci/devices/").join(self.to_string())
    }

    /// Get device hwmon path
    #[cfg(feature = "std")]
    pub fn get_hwmon_path(&self) -> Option<PathBuf> {
        let base = self.get_sysfs_path().join("hwmon");
        let entry = std::fs::read_dir(base).ok()?.next()?.ok()?;

        Some(entry.path())
    }

    #[cfg(feature = "std")]
    fn get_drm_path(&self, type_name: &str) -> std::io::Result<PathBuf> {
        let base = PathBuf::from("/dev/dri/by-path");

        let name = format!("pci-{}-{type_name}", self);
        let link = std::fs::read_link(base.join(name))?;

        std::fs::canonicalize(base.join(link))
    }

    /// Get DRM render path
    #[cfg(feature = "std")]
    pub fn get_drm_render_path(&self) -> std::io::Result<PathBuf> {
        self.get_drm_path("render")
    }

    /// Get DRM card path
    #[cfg(feature = "std")]
    pub fn get_drm_card_path(&self) -> std::io::Result<PathBuf> {
        self.get_drm_path("card")
    }

    /// Get device debug path
    #[cfg(feature = "std")]
    pub fn get_debug_dri_path(&self) -> std::io::Result<PathBuf> {
        let s = format!("amdgpu dev={}", self);

        std::fs::read_dir("/sys/kernel/debug/dri/")?
            .filter_map(|entry| Some(entry.ok()?.path()))
            .find(|path| {
                let Ok(name) = std::fs::read_to_string(path.join("name")) else { return false };

                name.starts_with(&s)
            })
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))
    }

    /// Get GPU maximum/minimum link speed/width from DPM
    #[cfg(feature = "std")]
    pub fn get_min_max_link_info_from_dpm(&self) -> Option<[LINK; 2]> {
        LINK::get_min_max_link_info_from_dpm(self.get_sysfs_path())
    }

    /// Get GPU current link speed/width from DPM
    #[cfg(feature = "std")]
    pub fn get_current_link_info_from_dpm(&self) -> Option<LINK> {
        LINK::get_current_link_info_from_dpm(self.get_sysfs_path())
    }

    /// Get GPU maximum link speed/width from sysfs
    #[cfg(feature = "std")]
    pub fn get_max_gpu_link(&self) -> Option<LINK> {
        let mut tmp = self.get_system_pcie_port_sysfs_path();

        tmp.pop();

        LINK::get_max_link(&tmp)
    }

    /// Get system maximum link speed/width from sysfs
    #[cfg(feature = "std")]
    pub fn get_max_system_link(&self) -> Option<LINK> {
        LINK::get_max_link(&self.get_system_pcie_port_sysfs_path())
    }

    /// The AMDGPU driver reports maximum number of PCIe lanes of Polaris11/Polaris12 as x16
    /// in `pp_dpm_pcie` (actually x8), so we use `{current,max}_link_{speed,width}`.
    /// ref: drivers/gpu/drm/amd/pm/powerplay/hwmgr/smu7_hwmgr.c
    ///
    /// Recent AMD GPUs have multiple endpoints, and the PCIe speed/width actually
    /// runs in that system for the GPU is output to `pp_dpm_pcie`.
    /// ref: <https://gitlab.freedesktop.org/drm/amd/-/issues/1967>
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

    /// Get GPU current link speed/width from sysfs
    #[cfg(feature = "std")]
    pub fn get_current_link_info(&self) -> Option<LINK> {
        LINK::get_from_sysfs_with_status(self.get_sysfs_path(), STATUS::Current)
    }

    /// Get GPU maximum link speed/width from sysfs
    #[cfg(feature = "std")]
    pub fn get_max_link_info(&self) -> Option<LINK> {
        LINK::get_from_sysfs_with_status(self.get_sysfs_path(), STATUS::Max)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBusInfoError;

#[cfg(feature = "std")]
impl std::str::FromStr for BUS_INFO {
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
    let bus = BUS_INFO { domain: 0x0, bus: 0xd, dev: 0x0, func: 0x0 };

    assert_eq!(s, Ok(bus));
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for BUS_INFO {
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
