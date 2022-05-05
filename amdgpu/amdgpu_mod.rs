#[path = "./device_handle.rs"]
mod device_handle;
pub use device_handle::*;

#[path = "amdgpu_family.rs"]
mod amdgpu_family;
pub use amdgpu_family::*;

#[path = "amdgpu_vram.rs"]
mod amdgpu_vram;
pub use amdgpu_vram::*;

#[path = "amdgpu_asic.rs"]
mod amdgpu_asic;
pub use amdgpu_asic::*;

#[path = "amdgpu_chip_class.rs"]
mod amdgpu_chip_class;
pub use amdgpu_chip_class::*;

/*
#[path = "amdgpu_vbios.rs"]
mod amdgpu_vbios;
pub use amdgpu_vbios::*;
*/

#[macro_export]
macro_rules! query_error {
    ($r: expr) => {
        if $r != 0 {
            return Err($r);
        }
    };
}
