use crate::AMDGPU::*;
use crate::*;
use core::mem::{size_of, MaybeUninit};

impl DeviceHandle {
    pub fn sensor_info(&self, sensor_type: SENSOR_TYPE) -> Result<u32, i32> {
        #[cfg(feature = "link_drm")]
        let func = bindings::amdgpu_query_sensor_info;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm_amdgpu.amdgpu_query_sensor_info;

        unsafe {
            let mut val: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = func(
                self.amdgpu_dev,
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
    AMDGPU_INFO_SENSOR_GFX_MCLK,
    AMDGPU_INFO_SENSOR_GFX_SCLK,
    AMDGPU_INFO_SENSOR_GPU_AVG_POWER,
    AMDGPU_INFO_SENSOR_GPU_LOAD,
    AMDGPU_INFO_SENSOR_GPU_TEMP,
    AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_MCLK,
    AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_SCLK,
    AMDGPU_INFO_SENSOR_VDDGFX,
    AMDGPU_INFO_SENSOR_VDDNB,
    AMDGPU_INFO_SENSOR_PEAK_PSTATE_GFX_MCLK,
    AMDGPU_INFO_SENSOR_PEAK_PSTATE_GFX_SCLK,
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
    PEAK_PSTATE_GFX_SCLK = AMDGPU_INFO_SENSOR_PEAK_PSTATE_GFX_SCLK,
    PEAK_PSTATE_GFX_MCLK = AMDGPU_INFO_SENSOR_PEAK_PSTATE_GFX_MCLK,
    GPU_INPUT_POWER = 0xC,
}

use std::fmt;
impl fmt::Display for SENSOR_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
