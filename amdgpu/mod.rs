use crate::*;

pub const GRBM_OFFSET: u32 = 0x2004;
pub const GRBM2_OFFSET: u32 = 0x2002;
pub const SRBM_OFFSET: u32 = 0x394;
pub const SRBM2_OFFSET: u32 = 0x393;
pub const SRBM3_OFFSET: u32 = 0x395;
pub const CP_STAT_OFFSET: u32 = 0x21A0;

// pub(crate) type DEVICE = bindings::amdgpu_device;
pub(crate) type DEVICE_HANDLE = bindings::amdgpu_device_handle;

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

mod vbios;
pub use vbios::*;

mod video_caps;
pub use video_caps::*;

mod hw_ip;
pub use hw_ip::*;

mod fw_version;
pub use fw_version::*;

// mod amdgpu_cs;
// pub use amdgpu_cs::*;

mod sensor_info;
pub use sensor_info::*;
