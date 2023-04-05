/// ref: drivers/gpu/drm/amd/include/kgd_pp_interface.h

use std::fs::File;
use std::io::{self, Read};
use crate::AMDGPU::{
    metrics_table_header,
    gpu_metrics_v1_0,
    gpu_metrics_v1_1,
    gpu_metrics_v1_2,
    gpu_metrics_v1_3,
    gpu_metrics_v2_0,
    gpu_metrics_v2_1,
    gpu_metrics_v2_2,
    gpu_metrics_v2_3,
};

impl metrics_table_header {
    pub(crate) fn from_slice(buf: &[u8]) -> Self {
        Self {
            structure_size: u16::from_le_bytes([buf[0], buf[1]]),
            format_revision: buf[2],
            content_revision: buf[3]
        }
    }

    pub fn from_buf(buf: [u8; 4]) -> Self {
        Self::from_slice(&buf)
    }

    pub fn from_sysfs_path(path: &str) -> io::Result<Self> {
        let mut f = File::open(path)?;
        let mut buf = [0u8; 4];

        f.read(&mut buf[..])?;

        Ok(Self::from_buf(buf))
    }
}

pub trait MetricsInfo {
    fn get_temperature_edge(&self) -> Option<u16>;
    fn get_temperature_hotspot(&self) -> Option<u16>;
    fn get_temperature_mem(&self) -> Option<u16>;
    fn get_temperature_vrgfx(&self) -> Option<u16>;
    fn get_temperature_vrsoc(&self) -> Option<u16>;
    fn get_temperature_vrmem(&self) -> Option<u16>;
    fn get_temperature_gfx(&self) -> Option<u16>;
    fn get_temperature_soc(&self) -> Option<u16>;
    fn get_temperature_core(&self) -> Option<[u16; 8]>;
    fn get_temperature_l3(&self) -> Option<[u16; 2]>;
    fn get_average_gfx_activity(&self) -> Option<u16>;
    fn get_average_umc_activity(&self) -> Option<u16>;
    fn get_average_mm_activity(&self) -> Option<u16>;
    fn get_system_clock_counter(&self) -> Option<u64>;
    fn get_average_socket_power(&self) -> Option<u16>;
    fn get_average_cpu_power(&self) -> Option<u16>;
    fn get_average_soc_power(&self) -> Option<u16>;
    fn get_average_gfx_power(&self) -> Option<u16>;
    fn get_average_core_power(&self) -> Option<[u16; 8]>;
    fn get_average_gfxclk_frequency(&self) -> Option<u16>;
    fn get_average_socclk_frequency(&self) -> Option<u16>;
    fn get_average_uclk_frequency(&self) -> Option<u16>;
    fn get_average_fclk_frequency(&self) -> Option<u16>;
    fn get_average_vclk_frequency(&self) -> Option<u16>;
    fn get_average_dclk_frequency(&self) -> Option<u16>;
    fn get_average_vclk1_frequency(&self) -> Option<u16>;
    fn get_average_dclk1_frequency(&self) -> Option<u16>;
    fn get_current_gfxclk(&self) -> Option<u16>;
    fn get_current_socclk(&self) -> Option<u16>;
    fn get_current_uclk(&self) -> Option<u16>;
    fn get_current_fclk(&self) -> Option<u16>;
    fn get_current_vclk(&self) -> Option<u16>;
    fn get_current_dclk(&self) -> Option<u16>;
    fn get_current_vclk1(&self) -> Option<u16>;
    fn get_current_dclk1(&self) -> Option<u16>;
    fn get_current_coreclk(&self) -> Option<[u16; 8]>;
    fn get_current_l3clk(&self) -> Option<[u16; 2]>;
    fn get_current_fan_speed(&self) -> Option<u16>;
    fn get_fan_pwm(&self) -> Option<u16>;
}

macro_rules! v1_impl {
    ($struct: ty) => {
        impl MetricsInfo for $struct {
            fn get_temperature_edge(&self) -> Option<u16> {
                Some(self.temperature_edge)
            }

            fn get_temperature_hotspot(&self) -> Option<u16> {
                Some(self.temperature_hotspot)
            }

            fn get_temperature_mem(&self) -> Option<u16> {
                Some(self.temperature_mem)
            }

            fn get_temperature_vrgfx(&self) -> Option<u16> {
                Some(self.temperature_vrgfx)
            }

            fn get_temperature_vrsoc(&self) -> Option<u16> {
                Some(self.temperature_vrsoc)
            }

            fn get_temperature_vrmem(&self) -> Option<u16> {
                Some(self.temperature_vrmem)
            }

            fn get_temperature_gfx(&self) -> Option<u16> {
                None
            }

            fn get_temperature_soc(&self) -> Option<u16> {
                None
            }

            fn get_temperature_core(&self) -> Option<[u16; 8]> {
                None
            }

            fn get_temperature_l3(&self) -> Option<[u16; 2]> {
                None
            }

            fn get_average_gfx_activity(&self) -> Option<u16> {
                Some(self.average_gfx_activity)
            }

            fn get_average_umc_activity(&self) -> Option<u16> {
                Some(self.average_umc_activity)
            }

            fn get_average_mm_activity(&self) -> Option<u16> {
                Some(self.average_mm_activity)
            }

            fn get_system_clock_counter(&self) -> Option<u64> {
                Some(self.system_clock_counter)
            }

            fn get_average_socket_power(&self) -> Option<u16> {
                Some(self.average_socket_power)
            }

            fn get_average_cpu_power(&self) -> Option<u16> {
                None
            }

            fn get_average_soc_power(&self) -> Option<u16> {
                None
            }

            fn get_average_gfx_power(&self) -> Option<u16> {
                None
            }

            fn get_average_core_power(&self) -> Option<[u16; 8]> {
                None
            }

            fn get_average_gfxclk_frequency(&self) -> Option<u16> {
                Some(self.average_gfxclk_frequency)
            }

            fn get_average_socclk_frequency(&self) -> Option<u16> {
                Some(self.average_socclk_frequency)
            }

            fn get_average_uclk_frequency(&self) -> Option<u16> {
                Some(self.average_uclk_frequency)
            }

            fn get_average_fclk_frequency(&self) -> Option<u16> {
                None
            }

            fn get_average_vclk_frequency(&self) -> Option<u16> {
                Some(self.average_vclk0_frequency)
            }

            fn get_average_dclk_frequency(&self) -> Option<u16> {
                Some(self.average_dclk0_frequency)
            }

            fn get_average_vclk1_frequency(&self) -> Option<u16> {
                Some(self.average_vclk1_frequency)
            }

            fn get_average_dclk1_frequency(&self) -> Option<u16> {
                Some(self.average_dclk1_frequency)
            }

            fn get_current_gfxclk(&self) -> Option<u16> {
                Some(self.current_gfxclk)
            }

            fn get_current_socclk(&self) -> Option<u16> {
                Some(self.current_socclk)
            }

            fn get_current_uclk(&self) -> Option<u16> {
                Some(self.current_uclk)
            }

            fn get_current_fclk(&self) -> Option<u16> {
                None
            }

            fn get_current_vclk(&self) -> Option<u16> {
                Some(self.current_vclk0)
            }

            fn get_current_dclk(&self) -> Option<u16> {
                Some(self.current_dclk0)
            }

            fn get_current_vclk1(&self) -> Option<u16> {
                Some(self.current_vclk1)
            }

            fn get_current_dclk1(&self) -> Option<u16> {
                Some(self.current_dclk1)
            }

            fn get_current_coreclk(&self) -> Option<[u16; 8]> {
                None
            }

            fn get_current_l3clk(&self) -> Option<[u16; 2]> {
                None
            }

            fn get_current_fan_speed(&self) -> Option<u16> {
                Some(self.current_fan_speed)
            }

            fn get_fan_pwm(&self) -> Option<u16> {
                None
            }
        }
    }
}

v1_impl!(gpu_metrics_v1_0);
v1_impl!(gpu_metrics_v1_1);
v1_impl!(gpu_metrics_v1_2);
v1_impl!(gpu_metrics_v1_3);

macro_rules! v2_impl {
    ($struct: ty) => {
        impl MetricsInfo for $struct {
            fn get_temperature_edge(&self) -> Option<u16> {
                None
            }

            fn get_temperature_hotspot(&self) -> Option<u16> {
                None
            }

            fn get_temperature_mem(&self) -> Option<u16> {
                None
            }

            fn get_temperature_vrgfx(&self) -> Option<u16> {
                None
            }

            fn get_temperature_vrsoc(&self) -> Option<u16> {
                None
            }

            fn get_temperature_vrmem(&self) -> Option<u16> {
                None
            }

            fn get_temperature_gfx(&self) -> Option<u16> {
                Some(self.temperature_gfx)
            }

            fn get_temperature_soc(&self) -> Option<u16> {
                Some(self.temperature_soc)
            }

            fn get_temperature_core(&self) -> Option<[u16; 8]> {
                Some(self.temperature_core)
            }

            fn get_temperature_l3(&self) -> Option<[u16; 2]> {
                Some(self.temperature_l3)
            }

            fn get_average_gfx_activity(&self) -> Option<u16> {
                Some(self.average_gfx_activity)
            }

            fn get_average_umc_activity(&self) -> Option<u16> {
                None
            }

            fn get_average_mm_activity(&self) -> Option<u16> {
                Some(self.average_mm_activity)
            }

            fn get_system_clock_counter(&self) -> Option<u64> {
                Some(self.system_clock_counter)
            }

            fn get_average_socket_power(&self) -> Option<u16> {
                Some(self.average_socket_power)
            }

            fn get_average_cpu_power(&self) -> Option<u16> {
                Some(self.average_cpu_power)
            }

            fn get_average_soc_power(&self) -> Option<u16> {
                Some(self.average_soc_power)
            }

            fn get_average_gfx_power(&self) -> Option<u16> {
                Some(self.average_gfx_power)
            }

            fn get_average_core_power(&self) -> Option<[u16; 8]> {
                Some(self.average_core_power)
            }

            fn get_average_gfxclk_frequency(&self) -> Option<u16> {
                Some(self.average_gfxclk_frequency)
            }

            fn get_average_socclk_frequency(&self) -> Option<u16> {
                Some(self.average_socclk_frequency)
            }

            fn get_average_uclk_frequency(&self) -> Option<u16> {
                Some(self.average_uclk_frequency)
            }

            fn get_average_fclk_frequency(&self) -> Option<u16> {
                Some(self.average_fclk_frequency)
            }

            fn get_average_vclk_frequency(&self) -> Option<u16> {
                Some(self.average_vclk_frequency)
            }

            fn get_average_dclk_frequency(&self) -> Option<u16> {
                Some(self.average_dclk_frequency)
            }

            fn get_average_vclk1_frequency(&self) -> Option<u16> {
                None
            }

            fn get_average_dclk1_frequency(&self) -> Option<u16> {
                None
            }

            fn get_current_gfxclk(&self) -> Option<u16> {
                Some(self.current_gfxclk)
            }

            fn get_current_socclk(&self) -> Option<u16> {
                Some(self.current_socclk)
            }

            fn get_current_uclk(&self) -> Option<u16> {
                Some(self.current_uclk)
            }

            fn get_current_fclk(&self) -> Option<u16> {
                Some(self.current_fclk)
            }

            fn get_current_vclk(&self) -> Option<u16> {
                Some(self.current_vclk)
            }

            fn get_current_dclk(&self) -> Option<u16> {
                Some(self.current_dclk)
            }

            fn get_current_vclk1(&self) -> Option<u16> {
                None
            }

            fn get_current_dclk1(&self) -> Option<u16> {
                None
            }

            fn get_current_coreclk(&self) -> Option<[u16; 8]> {
                Some(self.current_coreclk)
            }

            fn get_current_l3clk(&self) -> Option<[u16; 2]> {
                Some(self.current_l3clk)
            }

            fn get_current_fan_speed(&self) -> Option<u16> {
                None
            }

            fn get_fan_pwm(&self) -> Option<u16> {
                Some(self.fan_pwm)
            }
        }
    }
}

v2_impl!(gpu_metrics_v2_0);
v2_impl!(gpu_metrics_v2_1);
v2_impl!(gpu_metrics_v2_2);
v2_impl!(gpu_metrics_v2_3);
