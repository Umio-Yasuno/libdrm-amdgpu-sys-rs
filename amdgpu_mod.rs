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

#[path = "amdgpu/family.rs"]
mod family;
pub use family::*;

#[path = "amdgpu/vram.rs"]
mod vram;
pub use vram::*;

#[path = "amdgpu/asic.rs"]
mod asic;
pub use asic::*;

#[path = "amdgpu/chip_class.rs"]
mod chip_class;
pub use chip_class::*;

#[path = "amdgpu/gpu_info.rs"]
mod gpu_info;
pub use gpu_info::*;

#[path = "amdgpu/"]
pub mod VBIOS {
    mod vbios;
    pub use vbios::*;
}

#[path = "amdgpu/"]
pub mod VIDEO_CAPS {
    mod video_caps;
    pub use video_caps::*;
}

#[path = "amdgpu/"]
pub mod HW_IP {
    mod hw_ip;
    pub use hw_ip::*;
}

#[path = "amdgpu/"]
pub mod FW_VERSION {
    mod fw_version;
    pub use fw_version::*;
}

/*
#[path = "amdgpu/"]
pub mod CS {
    mod amdgpu_cs;
    pub use amdgpu_cs::*;
}
*/

#[path = "amdgpu/"]
pub mod SENSOR_INFO {
    mod sensor_info;
    pub use sensor_info::*;
}
