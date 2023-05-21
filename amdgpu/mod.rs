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

#[cfg(feature = "std")]
mod metrics_table;
#[cfg(feature = "std")]
pub use metrics_table::*;
#[cfg(feature = "std")]
mod gpu_metrics;
#[cfg(feature = "std")]
pub use gpu_metrics::*;
#[cfg(feature = "std")]
mod power_cap;
#[cfg(feature = "std")]
pub use power_cap::*;

mod vbios;
mod video_caps;
mod hw_ip;
mod fw_version;
mod sensor_info;

pub mod VBIOS {
    pub use super::vbios::*;
}

/// # Video Encode/Decode Capabilities
/// ## Examples
///
/// ```
/// use AMDGPU::VIDEO_CAPS::*;
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
