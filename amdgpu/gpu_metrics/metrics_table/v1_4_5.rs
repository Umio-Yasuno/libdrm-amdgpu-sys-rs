use super::{
    MetricsInfo,
    metrics_table_header,
    gpu_metrics_v1_4,
    gpu_metrics_v1_5,
    NUM_HBM_INSTANCES,
    NUM_VCN,
    NUM_JPEG_ENG,
    NUM_XGMI_LINKS,
    MAX_CLKS,
    MAX_GFX_CLKS,
};

macro_rules! v1_4_v1_5_impl {
    () => {
        fn get_header(&self) -> Option<metrics_table_header> {
            Some(self.common_header.clone())
        }

        fn get_temperature_hotspot(&self) -> Option<u16> {
            Some(self.temperature_hotspot)
        }

        fn get_temperature_mem(&self) -> Option<u16> {
            Some(self.temperature_mem)
        }

        fn get_temperature_vrsoc(&self) -> Option<u16> {
            Some(self.temperature_vrsoc)
        }

        fn get_average_gfx_activity(&self) -> Option<u16> {
            Some(self.average_gfx_activity)
        }

        fn get_average_umc_activity(&self) -> Option<u16> {
            Some(self.average_umc_activity)
        }

        fn get_current_socket_power(&self) -> Option<u16> {
            Some(self.curr_socket_power)
        }

        fn get_all_vcn_activity(&self) -> Option<[u16; NUM_VCN as usize]> {
            Some(self.vcn_activity)
        }

        fn get_system_clock_counter(&self) -> Option<u64> {
            Some(self.system_clock_counter)
        }

        fn get_throttle_status(&self) -> Option<u32> {
            Some(self.throttle_status)
        }

        fn get_gfxclk_lock_status(&self) -> Option<u32> {
            Some(self.gfxclk_lock_status)
        }

        fn get_all_instances_current_gfxclk(&self) -> Option<[u16; MAX_GFX_CLKS as usize]> {
            Some(self.current_gfxclk)
        }

        fn get_all_instances_current_socclk(&self) -> Option<[u16; MAX_CLKS as usize]> {
            Some(self.current_socclk)
        }

        fn get_all_instances_current_vclk0(&self) -> Option<[u16; MAX_CLKS as usize]> {
            Some(self.current_vclk0)
        }

        fn get_all_instances_current_dclk0(&self) -> Option<[u16; MAX_CLKS as usize]> {
            Some(self.current_dclk0)
        }

        fn get_pcie_link_width(&self) -> Option<u16> {
            Some(self.pcie_link_width)
        }

        fn get_pcie_link_speed(&self) -> Option<u16> {
            Some(self.pcie_link_speed)
        }

        fn get_pcie_bandwidth_acc(&self) -> Option<u64> {
            Some(self.pcie_bandwidth_acc)
        }

        fn get_pcie_bandwidth_inst(&self) -> Option<u64> {
            Some(self.pcie_bandwidth_inst)
        }

        fn get_xgmi_link_width(&self) -> Option<u16> {
            Some(self.xgmi_link_width)
        }

        fn get_xgmi_link_speed(&self) -> Option<u16> {
            Some(self.xgmi_link_speed)
        }

        fn get_xgmi_read_data_acc(&self) -> Option<[u64; NUM_XGMI_LINKS as usize]> {
            Some(self.xgmi_read_data_acc)
        }

        fn get_xgmi_write_data_acc(&self) -> Option<[u64; NUM_XGMI_LINKS as usize]> {
            Some(self.xgmi_write_data_acc)
        }

        fn get_gfx_activity_acc(&self) -> Option<u32> {
            Some(self.gfx_activity_acc)
        }

        fn get_mem_activity_acc(&self) -> Option<u32> {
            Some(self.mem_activity_acc)
        }

        fn get_current_uclk(&self) -> Option<u16> {
            Some(self.current_uclk)
        }

        fn get_temperature_gfx(&self) -> Option<u16> { None }
        fn get_temperature_soc(&self) -> Option<u16> { None }
        fn get_temperature_core(&self) -> Option<Vec<u16>> { None }
        fn get_temperature_l3(&self) -> Option<Vec<u16>> { None }
        fn get_temperature_skin(&self) -> Option<u16> { None }
        fn get_temperature_edge(&self) -> Option<u16> { None }
        fn get_temperature_vrmem(&self) -> Option<u16> { None }
        fn get_temperature_vrgfx(&self) -> Option<u16> { None }
        fn get_temperature_hbm(&self) -> Option<[u16; NUM_HBM_INSTANCES as usize]> { None }

        fn get_average_vpeclk_frequency(&self) -> Option<u16> { None }
        fn get_average_ipuclk_frequency(&self) -> Option<u16> { None }
        fn get_average_mpipu_frequency(&self) -> Option<u16> { None }

        fn get_average_mm_activity(&self) -> Option<u16> { None }
        fn get_average_ipu_activity(&self) -> Option<Vec<u16>> { None }
        fn get_average_core_c0_activity(&self) -> Option<Vec<u16>> { None }
        fn get_average_dram_reads(&self) -> Option<u16> { None }
        fn get_average_dram_writes(&self) -> Option<u16> { None }
        fn get_average_ipu_reads(&self) -> Option<u16> { None }
        fn get_average_ipu_writes(&self) -> Option<u16> { None }

        fn get_average_socket_power(&self) -> Option<u32> { None }
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

        fn get_average_gfxclk_frequency(&self) -> Option<u16> { None }
        fn get_average_socclk_frequency(&self) -> Option<u16> { None }
        fn get_average_uclk_frequency(&self) -> Option<u16> { None }
        fn get_average_fclk_frequency(&self) -> Option<u16> { None }
        fn get_average_vclk_frequency(&self) -> Option<u16> { None }
        fn get_average_dclk_frequency(&self) -> Option<u16> { None }
        fn get_average_vclk1_frequency(&self) -> Option<u16> { None }
        fn get_average_dclk1_frequency(&self) -> Option<u16> { None }
        fn get_current_coreclk(&self) -> Option<Vec<u16>> { None }
        fn get_current_l3clk(&self) -> Option<Vec<u16>> { None }
        fn get_current_gfxclk(&self) -> Option<u16> { None }
        fn get_current_socclk(&self) -> Option<u16> { None }
        fn get_current_fclk(&self) -> Option<u16> { None }
        fn get_current_vclk(&self) -> Option<u16> { None }
        fn get_current_dclk(&self) -> Option<u16> { None }
        fn get_current_vclk1(&self) -> Option<u16> { None }
        fn get_current_dclk1(&self) -> Option<u16> { None }
        fn get_current_core_maxfreq(&self) -> Option<u16> { None }
        fn get_current_gfx_maxfreq(&self) -> Option<u16> { None }
        fn get_indep_throttle_status(&self) -> Option<u64> { None }
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
        fn get_voltage_soc(&self) -> Option<u16> { None }
        fn get_voltage_gfx(&self) -> Option<u16> { None }
        fn get_voltage_mem(&self) -> Option<u16> { None }
        fn get_current_fan_speed(&self) -> Option<u16> { None }
        fn get_fan_pwm(&self) -> Option<u16> { None }
    }
}

impl MetricsInfo for gpu_metrics_v1_4 {
    v1_4_v1_5_impl!();

    fn get_all_jpeg_activity(&self) -> Option<[u16; NUM_JPEG_ENG as usize]> { None }
}

impl MetricsInfo for gpu_metrics_v1_5 {
    v1_4_v1_5_impl!();

    fn get_all_jpeg_activity(&self) -> Option<[u16; NUM_JPEG_ENG as usize]> {
        Some(self.jpeg_activity)
    }
}
