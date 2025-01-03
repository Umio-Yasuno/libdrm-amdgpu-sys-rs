use crate::AMDGPU;

/// PCI information (Domain, Bus, Device, Function)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct BUS_INFO {
    pub domain: u16,
    pub bus: u8,
    pub dev: u8,
    pub func: u8,
}

#[cfg(feature = "std")]
use super::{LINK, STATUS};

#[cfg(feature = "std")]
use std::path::PathBuf;

impl BUS_INFO {
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
        let base = PathBuf::from("/dev/dri");

        let name = format!("by-path/pci-{}-{type_name}", self);
        let pci_by_path = std::fs::canonicalize(base.join(name));

        pci_by_path.or_else(|e| {
            std::fs::read_dir(self.get_sysfs_path().join("drm"))?
                .find_map(|v| {
                    let file_name = v.ok()?.file_name().into_string().ok()?;
                    if file_name.starts_with(type_name) {
                        Some(base.join(file_name))
                    } else {
                        None
                    }
                })
                .ok_or(e)
        })
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

    #[cfg(feature = "std")]
    fn from_pathbuf(path: PathBuf) -> Option<Self> {
        path
            .canonicalize().ok()?
            .file_name()?
            .to_str()?
            .parse().ok()
    }

    /// Recent AMD GPUs have multiple endpoints, and the PCIe speed/width actually
    /// runs in that system for the GPU is output to `pp_dpm_pcie`.
    /// ref: <https://gitlab.freedesktop.org/drm/amd/-/issues/1967>
    #[cfg(feature = "std")]
    pub fn get_gpu_pcie_port_bus(&self) -> Self {
        let mut path = self.get_system_pcie_port_sysfs_path();

        path.pop();

        if let Some(pci) = Self::from_pathbuf(path) {
            pci
        } else {
            *self
        }
    }

    #[cfg(feature = "std")]
    pub fn get_system_pcie_port_bus(&self) -> Self {
        let path = self.get_system_pcie_port_sysfs_path();

        if let Some(pci) = Self::from_pathbuf(path) {
            pci
        } else {
            *self
        }
    }

    /// Recent AMD GPUs have multiple endpoints, and the PCIe speed/width actually
    /// runs in that system for the GPU is output to `pp_dpm_pcie`.
    /// ref: <https://gitlab.freedesktop.org/drm/amd/-/issues/1967>
    #[cfg(feature = "std")]
    fn get_system_pcie_port_sysfs_path(&self) -> PathBuf {
        const VENDOR_ATI: &str = "0x1002\n";
        // 0x6: Bridge, 0x4: PCI-to-PCI Bridge
        const CLASS: &str = "0x060400\n";

        let mut tmp = self.get_sysfs_path().join("../"); // pcie port

        for _ in 0..2 {
            let [Ok(vendor), Ok(class)] = [
                std::fs::read_to_string(&tmp.join("vendor")),
                std::fs::read_to_string(&tmp.join("class")),
            ] else { break };

            if vendor == VENDOR_ATI && class == CLASS {
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

    fn parse_id(&self, file_name: &str) -> Option<u32> {
        let sysfs_path = self.get_sysfs_path();
        let id = std::fs::read_to_string(sysfs_path.join(file_name)).ok()?;

        u32::from_str_radix(id.trim_start_matches("0x").trim_end(), 16).ok()
    }

    /// Get PCI Device ID from sysfs
    pub fn get_device_id(&self) -> Option<u32> {
        self.parse_id("device")
    }

    /// Get PCI Revision ID from sysfs
    pub fn get_revision_id(&self) -> Option<u32> {
        self.parse_id("revision")
    }

    /// Find device marketing name from `amdgpu.ids`  
    /// Link: <https://gitlab.freedesktop.org/mesa/drm/-/blob/main/data/amdgpu.ids>
    pub fn find_device_name(&self) -> Option<String> {
        let device_id = self.get_device_id()?;
        let revision_id = self.get_revision_id()?;

        AMDGPU::find_device_name(device_id, revision_id)
    }

    pub fn find_device_name_or_default_name(&self) -> String {
        self.find_device_name().unwrap_or(AMDGPU::DEFAULT_DEVICE_NAME.to_string())
    }

    pub fn check_if_device_is_active(&self) -> bool {
        let path = self.get_sysfs_path().join("power/runtime_status");
        let Ok(s) = std::fs::read_to_string(path) else { return false };

        s.starts_with("active")
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
