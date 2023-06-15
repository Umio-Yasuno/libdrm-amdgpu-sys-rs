use crate::AMDGPU::*;
use crate::*;
use core::mem::{size_of, MaybeUninit};

impl DeviceHandle {
    pub fn sensor_info(&self, sensor_type: SENSOR_TYPE) -> Result<u32, i32> {
        unsafe {
            let mut val: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_sensor_info(
                self.0,
                sensor_type as u32,
                size_of::<u32>() as u32,
                val.as_mut_ptr() as *mut ::core::ffi::c_void,
            );

            let val = val.assume_init();

            query_error!(r);

            Ok(val)
        }
    }
}

use bindings::{
    AMDGPU_INFO_SENSOR_GFX_MCLK, AMDGPU_INFO_SENSOR_GFX_SCLK, AMDGPU_INFO_SENSOR_GPU_AVG_POWER,
    AMDGPU_INFO_SENSOR_GPU_LOAD, AMDGPU_INFO_SENSOR_GPU_TEMP,
    AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_MCLK, AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_SCLK,
    AMDGPU_INFO_SENSOR_VDDGFX, AMDGPU_INFO_SENSOR_VDDNB,
};

/// Used for [DeviceHandle::sensor_info]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u32)]
pub enum SENSOR_TYPE {
    GFX_SCLK = AMDGPU_INFO_SENSOR_GFX_SCLK,
    GFX_MCLK = AMDGPU_INFO_SENSOR_GFX_MCLK,
    GPU_TEMP = AMDGPU_INFO_SENSOR_GPU_TEMP,
    GPU_LOAD = AMDGPU_INFO_SENSOR_GPU_LOAD,
    GPU_AVG_POWER = AMDGPU_INFO_SENSOR_GPU_AVG_POWER,
    VDDNB = AMDGPU_INFO_SENSOR_VDDNB,
    VDDGFX = AMDGPU_INFO_SENSOR_VDDGFX,
    STABLE_PSTATE_GFX_SCLK = AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_SCLK,
    STABLE_PSTATE_GFX_MCLK = AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_MCLK,
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for SENSOR_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
