use crate::*;

pub const GRBM_OFFSET: u32 = 0x2004;
pub const GRBM2_OFFSET: u32 = 0x2002;
pub const SRBM_OFFSET: u32 = 0x394;
pub const SRBM2_OFFSET: u32 = 0x393;
pub const SRBM3_OFFSET: u32 = 0x395;
pub const CP_STAT_OFFSET: u32 = 0x21A0;

// pub(crate) type DEVICE = bindings::amdgpu_device;
pub(crate) type DEVICE_HANDLE = bindings::amdgpu_device_handle;

// ref: https://gitlab.freedesktop.org/mesa/drm/-/blob/a81b9ab8f3fb6840b36f732c1dd25fe5e0d68d0a/amdgpu/amdgpu_device.c#L293
pub const DEFAULT_DEVICE_NAME: &str = "AMD Radeon Graphics";

mod device_handle;
pub use device_handle::*;

mod family;
pub use family::*;

mod vram;
pub use vram::*;

mod asic;
pub use asic::*;

mod chip_class;
pub use chip_class::*;

mod gpu_info;
pub use gpu_info::*;

mod gfx_target_version;
pub use gfx_target_version::GfxTargetVersion;

mod context;
pub use context::*;

mod gpu_metrics;
pub use gpu_metrics::*;

mod throttle_status;
pub use throttle_status::*;

mod hwmon_temp;
pub use hwmon_temp::*;

mod power_cap;
pub use power_cap::*;

mod power_profile;
pub use power_profile::*;

mod dpm_forced_level;
pub use dpm_forced_level::*;

mod ip_discovery;
pub use ip_discovery::*;

mod ras_features;
pub use ras_features::*;

mod dpm;
pub use dpm::*;

pub(crate) fn parse_hwmon<T: std::str::FromStr, P: Into<std::path::PathBuf>>(path: P) -> Option<T> {
    std::fs::read_to_string(path.into()).ok()
        .and_then(|file| file.trim_end().parse::<T>().ok())
}

mod vbios;
mod vbios_parser;
mod video_caps;
mod hw_ip;
mod fw_version;
mod sensor_info;

pub mod pp_table;
pub use pp_table::PPTable;

pub mod VBIOS {
    pub use super::vbios::*;
    pub use super::vbios_parser::*;
}

/// # Video Encode/Decode Capabilities
/// ## Examples
///
/// ```
/// use libdrm_amdgpu_sys::LibDrmAmdgpu;
/// use libdrm_amdgpu_sys::AMDGPU::{DeviceHandle, VIDEO_CAPS::*};
/// let libdrm_amdgpu = LibDrmAmdgpu::new().unwrap();
/// let (amdgpu_dev, drm_major, drm_minor) = {
///     use std::fs::File;
///     use std::os::fd::IntoRawFd;
///
///     let fd = File::open("/dev/dri/renderD128").unwrap();
///
///     libdrm_amdgpu.init_device_handle(fd.into_raw_fd()).unwrap()
/// };
/// for cap_type in [
///     amdgpu_dev.get_video_caps(CAP_TYPE::DECODE).unwrap(),
///     amdgpu_dev.get_video_caps(CAP_TYPE::ENCODE).unwrap(),
/// ] {
///     let codec_list = [
///         CODEC::MPEG2,
///         CODEC::MPEG4,
///         CODEC::VC1,
///         CODEC::MPEG4_AVC,
///         CODEC::HEVC,
///         CODEC::JPEG,
///         CODEC::VP9,
///         CODEC::AV1,
///     ];
///
///     for codec in &codec_list {
///         let cap = cap_type.get_codec_info(*codec);
///         println!("{cap:?}");
///     }
/// }
/// // or
/// if let Ok(dec_info) = amdgpu_dev.get_video_caps_info(CAP_TYPE::DECODE) {
///     println!("{dec_info:#?}");
/// }
/// if let Ok(enc_info) = amdgpu_dev.get_video_caps_info(CAP_TYPE::ENCODE) {
///     println!("{enc_info:#?}");
/// }
/// ```
pub mod VIDEO_CAPS {
    pub use super::video_caps::*;
}
pub mod HW_IP {
    pub use super::hw_ip::*;
}
pub mod FW_VERSION {
    pub use super::fw_version::*;
}
pub mod SENSOR_INFO {
    pub use super::sensor_info::*;
}

pub fn get_all_amdgpu_pci_bus() -> Vec<PCI::BUS_INFO> {
    let Ok(amdgpu_devices) = std::fs::read_dir("/sys/bus/pci/drivers/amdgpu") else { return Vec::new() };

    amdgpu_devices.flat_map(|v| {
        let name = v.ok()?.file_name();

        /* 0000:00:00.0 */
        if name.len() != 12 { return None; }

        name.into_string().ok()?.parse::<PCI::BUS_INFO>().ok()
    }).collect()
}
