use crate::*;

// pub type DEVICE = bindings::amdgpu_device;
pub type DEVICE_HANDLE = bindings::amdgpu_device_handle;

#[macro_export]
macro_rules! query_error {
    ($r: expr) => {
        if $r != 0 {
            return Err($r);
        }
    };
}

#[path = "amdgpu/device_handle.rs"]
mod device_handle;
pub use device_handle::*;

#[path = "amdgpu/amdgpu_family.rs"]
mod amdgpu_family;
pub use amdgpu_family::*;

#[path = "amdgpu/amdgpu_vram.rs"]
mod amdgpu_vram;
pub use amdgpu_vram::*;

#[path = "amdgpu/amdgpu_asic.rs"]
mod amdgpu_asic;
pub use amdgpu_asic::*;

#[path = "amdgpu/amdgpu_chip_class.rs"]
mod amdgpu_chip_class;
pub use amdgpu_chip_class::*;

#[path = "amdgpu/gpu_info.rs"]
mod gpu_info;
pub use gpu_info::*;

#[path = "amdgpu/"]
pub mod VBIOS {
    mod amdgpu_vbios;
    pub use amdgpu_vbios::*;
}

#[path = "amdgpu/"]
pub mod VIDEO_CAPS {
    mod amdgpu_video_caps;
    pub use amdgpu_video_caps::*;
}

#[path = "amdgpu/"]
pub mod HW_IP {
    mod amdgpu_hw_ip;
    pub use amdgpu_hw_ip::*;
}

#[path = "amdgpu/"]
pub mod FW_VERSION {
    mod amdgpu_fw_version;
    pub use amdgpu_fw_version::*;
}

#[path = "amdgpu/"]
pub mod CS {
    mod amdgpu_cs;
    pub use amdgpu_cs::*;
}

#[path = "amdgpu/"]
pub mod SENSOR_INFO {
    mod amdgpu_sensor_info;
    pub use amdgpu_sensor_info::*;
}
