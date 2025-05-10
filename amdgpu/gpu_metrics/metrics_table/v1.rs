use super::{
    MetricsInfo,
    metrics_table_header,
    gpu_metrics_v1_0,
    gpu_metrics_v1_1,
    gpu_metrics_v1_2,
    gpu_metrics_v1_3,
    NUM_HBM_INSTANCES,
    NUM_VCN,
    NUM_JPEG_ENG,
    NUM_XGMI_LINKS,
    MAX_CLKS,
    MAX_GFX_CLKS,
};
use crate::AMDGPU::{ThrottlerBit, ThrottleStatus};

macro_rules! v1_impl {
    () => {
        fn get_header(&self) -> Option<metrics_table_header> {
            Some(self.common_header.clone())
        }

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

        fn get_temperature_gfx(&self) -> Option<u16> { None }
        fn get_temperature_soc(&self) -> Option<u16> { None }
        fn get_temperature_core(&self) -> Option<Vec<u16>> { None }
        fn get_temperature_l3(&self) -> Option<Vec<u16>> { None }
        fn get_temperature_skin(&self) -> Option<u16> { None }

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

        fn get_average_socket_power(&self) -> Option<u32> {
            Some(self.average_socket_power as u32)
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

        fn get_average_fclk_frequency(&self) -> Option<u16> { None }

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

        fn get_current_fclk(&self) -> Option<u16> { None }

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

        fn get_current_coreclk(&self) -> Option<Vec<u16>> { None }
        fn get_current_l3clk(&self) -> Option<Vec<u16>> { None }
        fn get_current_core_maxfreq(&self) -> Option<u16> { None }
        fn get_current_gfx_maxfreq(&self) -> Option<u16> { None }

        fn get_throttle_status(&self) -> Option<u32> {
            Some(self.throttle_status)
        }

        fn get_current_fan_speed(&self) -> Option<u16> {
            Some(self.current_fan_speed)
        }

        fn get_fan_pwm(&self) -> Option<u16> { None }

        fn get_pcie_link_width(&self) -> Option<u16> {
            Some(self.pcie_link_width as u16)
        }

        fn get_pcie_link_speed(&self) -> Option<u16> {
            Some(self.pcie_link_speed as u16)
        }

        fn get_average_vpeclk_frequency(&self) -> Option<u16> { None }
        fn get_average_ipuclk_frequency(&self) -> Option<u16> { None }
        fn get_average_mpipu_frequency(&self) -> Option<u16> { None }

        fn get_average_ipu_activity(&self) -> Option<Vec<u16>> { None }
        fn get_average_core_c0_activity(&self) -> Option<Vec<u16>> { None }
        fn get_average_dram_reads(&self) -> Option<u16> { None }
        fn get_average_dram_writes(&self) -> Option<u16> { None }
        fn get_average_ipu_reads(&self) -> Option<u16> { None }
        fn get_average_ipu_writes(&self) -> Option<u16> { None }

        fn get_average_cpu_power(&self) -> Option<u16> { None }
        fn get_average_soc_power(&self) -> Option<u16> { None }
        fn get_average_gfx_power(&self) -> Option<u16> { None }
        fn get_average_gfx_power_u32(&self) -> Option<u32> { None }
        fn get_average_core_power(&self) -> Option<Vec<u16>> { None }
        fn get_average_ipu_power(&self) -> Option<u16> { None }
        fn get_average_apu_power(&self) -> Option<u32> { None }
        fn get_average_dgpu_power(&self) -> Option<u32> { None }
        fn get_average_all_core_power(&self) -> Option<u32> { None }
        fn get_average_sys_power(&self) -> Option<u16> { None }
        fn get_stapm_power_limit(&self) -> Option<u16> { None }
        fn get_current_stapm_power_limit(&self) -> Option<u16> { None }

        fn get_xgmi_link_width(&self) -> Option<u16> { None }
        fn get_xgmi_link_speed(&self) -> Option<u16> { None }
        fn get_xgmi_read_data_acc(&self) -> Option<[u64; NUM_XGMI_LINKS as usize]> { None }
        fn get_xgmi_write_data_acc(&self) -> Option<[u64; NUM_XGMI_LINKS as usize]> { None }
        fn get_pcie_bandwidth_acc(&self) -> Option<u64> { None }
        fn get_pcie_bandwidth_inst(&self) -> Option<u64> { None }
        fn get_average_temperature_gfx(&self) -> Option<u16> { None }
        fn get_average_temperature_soc(&self) -> Option<u16> { None }
        fn get_average_temperature_core(&self) -> Option<Vec<u16>> { None }
        fn get_average_temperature_l3(&self) -> Option<Vec<u16>> { None }
        fn get_average_cpu_voltage(&self) -> Option<u16> { None }
        fn get_average_soc_voltage(&self) -> Option<u16> { None }
        fn get_average_gfx_voltage(&self) -> Option<u16> { None }
        fn get_average_cpu_current(&self) -> Option<u16> { None }
        fn get_average_soc_current(&self) -> Option<u16> { None }
        fn get_average_gfx_current(&self) -> Option<u16> { None }
        fn get_gfxclk_lock_status(&self) -> Option<u32> { None }
        fn get_current_socket_power(&self) -> Option<u16> { None }
        fn get_all_instances_current_gfxclk(&self) -> Option<[u16; MAX_GFX_CLKS as usize]> { None }
        fn get_all_instances_current_socclk(&self) -> Option<[u16; MAX_CLKS as usize]> { None }
        fn get_all_instances_current_vclk0(&self) -> Option<[u16; MAX_CLKS as usize]> { None }
        fn get_all_instances_current_dclk0(&self) -> Option<[u16; MAX_CLKS as usize]> { None }
        fn get_all_vcn_activity(&self) -> Option<[u16; NUM_VCN as usize]> { None }
        fn get_all_jpeg_activity(&self) -> Option<[u16; NUM_JPEG_ENG as usize]> { None }
    }
}

impl MetricsInfo for gpu_metrics_v1_0 {
    v1_impl!();

    fn get_gfx_activity_acc(&self) -> Option<u32> { None }
    fn get_mem_activity_acc(&self) -> Option<u32> { None }
    fn get_temperature_hbm(&self) -> Option<[u16; NUM_HBM_INSTANCES as usize]> { None }
    fn get_voltage_soc(&self) -> Option<u16> { None }
    fn get_voltage_gfx(&self) -> Option<u16> { None }
    fn get_voltage_mem(&self) -> Option<u16> { None }
    fn get_indep_throttle_status(&self) -> Option<u64> { None }
}

impl MetricsInfo for gpu_metrics_v1_1 {
    v1_impl!();

    fn get_gfx_activity_acc(&self) -> Option<u32> {
        Some(self.gfx_activity_acc)
    }

    fn get_mem_activity_acc(&self) -> Option<u32> {
        Some(self.mem_activity_acc)
    }

    fn get_temperature_hbm(&self) -> Option<[u16; NUM_HBM_INSTANCES as usize]> {
        Some(self.temperature_hbm)
    }

    fn get_voltage_soc(&self) -> Option<u16> { None }
    fn get_voltage_gfx(&self) -> Option<u16> { None }
    fn get_voltage_mem(&self) -> Option<u16> { None }
    fn get_indep_throttle_status(&self) -> Option<u64> { None }
}

impl MetricsInfo for gpu_metrics_v1_2 {
    v1_impl!();

    fn get_gfx_activity_acc(&self) -> Option<u32> {
        Some(self.gfx_activity_acc)
    }

    fn get_mem_activity_acc(&self) -> Option<u32> {
        Some(self.mem_activity_acc)
    }

    fn get_temperature_hbm(&self) -> Option<[u16; NUM_HBM_INSTANCES as usize]> {
        Some(self.temperature_hbm)
    }

    fn get_voltage_soc(&self) -> Option<u16> { None }
    fn get_voltage_gfx(&self) -> Option<u16> { None }
    fn get_voltage_mem(&self) -> Option<u16> { None }
    fn get_indep_throttle_status(&self) -> Option<u64> { None }
}

impl MetricsInfo for gpu_metrics_v1_3 {
    v1_impl!();

    fn get_gfx_activity_acc(&self) -> Option<u32> {
        Some(self.gfx_activity_acc)
    }

    fn get_mem_activity_acc(&self) -> Option<u32> {
        Some(self.mem_activity_acc)
    }

    fn get_temperature_hbm(&self) -> Option<[u16; NUM_HBM_INSTANCES as usize]> {
        Some(self.temperature_hbm)
    }

    fn get_voltage_soc(&self) -> Option<u16> {
        Some(self.voltage_soc)
    }

    fn get_voltage_gfx(&self) -> Option<u16> {
        Some(self.voltage_gfx)
    }

    fn get_voltage_mem(&self) -> Option<u16> {
        Some(self.voltage_mem)
    }

    fn get_indep_throttle_status(&self) -> Option<u64> {
        Some(self.indep_throttle_status)
    }

    fn get_throttle_status_info(&self) -> Option<ThrottleStatus> {
        const TEMP_HOTSPOT_BIT: u64 = ThrottlerBit::TEMP_HOTSPOT as u64;
        let mut indep = self.indep_throttle_status;
        let temp_hotspot_flag = ((indep >> TEMP_HOTSPOT_BIT) & 0b1) == 1;

        // ThrottlingPercentage for TEMP_HOTSPOT on SMU v13.0.0/7 is almost always greater then or equal to 1.
        // So the AMDGPU driver set the TEMP_HOTSPOT bit even in idle state.
        // ref: https://gitlab.freedesktop.org/drm/amd/-/issues/3251
        if temp_hotspot_flag && self.temperature_hotspot >= 90 {
            indep |= 1 << TEMP_HOTSPOT_BIT;
        } else {
            indep &= !(1 << TEMP_HOTSPOT_BIT as u64);
        }

        Some(ThrottleStatus::new(indep))
    }
}
