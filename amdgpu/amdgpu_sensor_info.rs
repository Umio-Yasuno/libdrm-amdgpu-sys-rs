use crate::*;
use crate::AMDGPU::*;
use std::mem::{MaybeUninit, size_of};

pub trait QUERY_SENSOR_INFO {
    fn sensor_info(
        self,
        sensor_type: SENSOR_TYPE,
    ) -> Result<u32, i32>;
}

impl QUERY_SENSOR_INFO for DEVICE_HANDLE {
    fn sensor_info(
        self,
        sensor_type: SENSOR_TYPE,
    ) -> Result<u32, i32> {
        unsafe {
            let mut val: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_sensor_info(
                self,
                sensor_type as u32,
                size_of::<u32>() as u32,
                val.as_mut_ptr() as *mut ::std::os::raw::c_void,
            );

            query_error!(r);

            return Ok(val.assume_init());
        }
    }
}

use bindings::{
    AMDGPU_INFO_SENSOR_GFX_SCLK,
    AMDGPU_INFO_SENSOR_GFX_MCLK,
    AMDGPU_INFO_SENSOR_GPU_TEMP,
    AMDGPU_INFO_SENSOR_GPU_LOAD,
    AMDGPU_INFO_SENSOR_GPU_AVG_POWER,
    AMDGPU_INFO_SENSOR_VDDNB,
    AMDGPU_INFO_SENSOR_VDDGFX,
    AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_SCLK,
    AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_MCLK,
};

#[derive(Debug, Clone, Copy)]
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
