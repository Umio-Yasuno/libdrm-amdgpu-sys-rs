use super::{
    MetricsInfo,
    metrics_table_header,
    gpu_metrics_v2_0,
    gpu_metrics_v2_1,
    gpu_metrics_v2_2,
    gpu_metrics_v2_3,
    gpu_metrics_v2_4,
    gpu_metrics_v3_0,
    NUM_HBM_INSTANCES,
    NUM_VCN,
    NUM_JPEG_ENG,
    NUM_XGMI_LINKS,
    MAX_CLKS,
    MAX_GFX_CLKS,
};

macro_rules! v2_impl {
    () => {
        fn get_header(&self) -> Option<metrics_table_header> {
            Some(self.common_header.clone())
        }

        fn get_temperature_edge(&self) -> Option<u16> { None }
        fn get_temperature_hotspot(&self) -> Option<u16> { None }
        fn get_temperature_mem(&self) -> Option<u16> { None }
        fn get_temperature_vrgfx(&self) -> Option<u16> { None }
        fn get_temperature_vrsoc(&self) -> Option<u16> { None }
        fn get_temperature_vrmem(&self) -> Option<u16> { None }

        fn get_temperature_gfx(&self) -> Option<u16> {
            Some(self.temperature_gfx)
        }

        fn get_temperature_soc(&self) -> Option<u16> {
            Some(self.temperature_soc)
        }

        fn get_temperature_core(&self) -> Option<Vec<u16>> {
            Some(self.temperature_core.to_vec())
        }

        fn get_temperature_l3(&self) -> Option<Vec<u16>> {
            Some(self.temperature_l3.to_vec())
        }

        fn get_temperature_skin(&self) -> Option<u16> { None }

        fn get_average_gfx_activity(&self) -> Option<u16> {
            Some(self.average_gfx_activity)
        }

        fn get_average_umc_activity(&self) -> Option<u16> { None }

        fn get_average_mm_activity(&self) -> Option<u16> {
            Some(self.average_mm_activity)
        }

        fn get_system_clock_counter(&self) -> Option<u64> {
            Some(self.system_clock_counter)
        }

        fn get_average_socket_power(&self) -> Option<u32> {
            Some(self.average_socket_power as u32)
        }

        fn get_average_cpu_power(&self) -> Option<u16> {
            Some(self.average_cpu_power)
        }

        fn get_average_soc_power(&self) -> Option<u16> {
            Some(self.average_soc_power)
        }

        fn get_average_core_power(&self) -> Option<Vec<u16>> {
            Some(self.average_core_power.to_vec())
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

        fn get_average_vclk1_frequency(&self) -> Option<u16> { None }
        fn get_average_dclk1_frequency(&self) -> Option<u16> { None }

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

        fn get_current_vclk1(&self) -> Option<u16> { None }
        fn get_current_dclk1(&self) -> Option<u16> { None }

        fn get_current_coreclk(&self) -> Option<Vec<u16>> {
            Some(self.current_coreclk.to_vec())
        }

        fn get_current_l3clk(&self) -> Option<Vec<u16>> {
            Some(self.current_l3clk.to_vec())
        }

        fn get_throttle_status(&self) -> Option<u32> {
            Some(self.throttle_status)
        }

        fn get_current_fan_speed(&self) -> Option<u16> { None }

        fn get_fan_pwm(&self) -> Option<u16> {
            Some(self.fan_pwm)
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

        fn get_average_ipu_power(&self) -> Option<u16> { None }
        fn get_average_apu_power(&self) -> Option<u32> { None }
        fn get_average_dgpu_power(&self) -> Option<u32> { None }
        fn get_average_all_core_power(&self) -> Option<u32> { None }
        fn get_average_sys_power(&self) -> Option<u16> { None }
        fn get_stapm_power_limit(&self) -> Option<u16> { None }
        fn get_current_stapm_power_limit(&self) -> Option<u16> { None }

        fn get_current_core_maxfreq(&self) -> Option<u16> { None }
        fn get_current_gfx_maxfreq(&self) -> Option<u16> { None }

        fn get_pcie_link_width(&self) -> Option<u16> { None }
        fn get_pcie_link_speed(&self) -> Option<u16> { None }
        fn get_pcie_bandwidth_acc(&self) -> Option<u64> { None }
        fn get_pcie_bandwidth_inst(&self) -> Option<u64> { None }
        fn get_xgmi_link_width(&self) -> Option<u16> { None }
        fn get_xgmi_link_speed(&self) -> Option<u16> { None }
        fn get_xgmi_read_data_acc(&self) -> Option<[u64; NUM_XGMI_LINKS as usize]> { None }
        fn get_xgmi_write_data_acc(&self) -> Option<[u64; NUM_XGMI_LINKS as usize]> { None }
        fn get_gfx_activity_acc(&self) -> Option<u32> { None }
        fn get_mem_activity_acc(&self) -> Option<u32> { None }
        fn get_temperature_hbm(&self) -> Option<[u16; NUM_HBM_INSTANCES as usize]> { None }
        fn get_voltage_soc(&self) -> Option<u16> { None }
        fn get_voltage_gfx(&self) -> Option<u16> { None }
        fn get_voltage_mem(&self) -> Option<u16> { None }
        fn get_gfxclk_lock_status(&self) -> Option<u32> { None }
        fn get_current_socket_power(&self) -> Option<u16> { None }
        fn get_all_instances_current_gfxclk(&self) -> Option<[u16; MAX_GFX_CLKS as usize]> { None }
        fn get_all_instances_current_socclk(&self) -> Option<[u16; MAX_CLKS as usize]> { None }
        fn get_all_instances_current_vclk0(&self) -> Option<[u16; MAX_CLKS as usize]> { None }
        fn get_all_instances_current_dclk0(&self) -> Option<[u16; MAX_CLKS as usize]> { None }
        fn get_all_vcn_activity(&self) -> Option<[u16; NUM_VCN as usize]> { None }
        fn get_all_jpeg_activity(&self) -> Option<[u16; NUM_JPEG_ENG as usize]> { None }
        fn get_throttle_residency_prochot(&self) -> Option<u32> { None }
        fn get_throttle_residency_spl(&self) -> Option<u32> { None }
        fn get_throttle_residency_fppt(&self) -> Option<u32> { None }
        fn get_throttle_residency_sppt(&self) -> Option<u32> { None }
        fn get_throttle_residency_thm_core(&self) -> Option<u32> { None }
        fn get_throttle_residency_thm_gfx(&self) -> Option<u32> { None }
        fn get_throttle_residency_thm_soc(&self) -> Option<u32> { None }
    }
}

impl MetricsInfo for gpu_metrics_v2_0 {
    v2_impl!();
    fn get_average_gfx_power(&self) -> Option<u16> { None }
    fn get_average_gfx_power_u32(&self) -> Option<u32> { None }
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
}

// Mendocino, Raphael, Rembrandt (Yellow Carp)
impl MetricsInfo for gpu_metrics_v2_1 {
    v2_impl!();

    fn get_average_gfx_power(&self) -> Option<u16> {
        Some(self.average_gfx_power)
    }

    fn get_average_gfx_power_u32(&self) -> Option<u32> {
        Some(self.average_gfx_power as u32)
    }

    fn get_indep_throttle_status(&self) -> Option<u64> {
        use crate::amdgpu::throttle_status::ThrottlerBit;

        // ref: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu13_driver_if_yellow_carp.h
        // ref: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu13_driver_if_v13_0_4.h
        // ref: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu13_driver_if_v13_0_5.h
        const SPL: u64 = 0;
        const FPPT: u64 = 1;
        const SPPT: u64 = 2;
        const SPPT_APU: u64 = 3;
        const THM_CORE: u64 = 4;
        const THM_GFX: u64 = 5;
        const THM_SOC: u64 = 6;
        const TDC_VDD: u64 = 7;
        const TDC_SOC: u64 = 8;
        const PROCHOT_CPU: u64 = 9;
        const PROCHOT_GFX: u64 = 10;
        const EDC_CPU: u64 = 11;
        const EDC_GFX: u64 = 12;

        let thr_status = self.get_throttle_status()?;
        let mut indep = 0u64;

        for (thr_bit, indep_thr_bit) in [
            (SPL, ThrottlerBit::SPL),
            (FPPT, ThrottlerBit::FPPT),
            (SPPT, ThrottlerBit::SPPT),
            (SPPT_APU, ThrottlerBit::SPPT_APU),
            (THM_CORE, ThrottlerBit::TEMP_CORE),
            (THM_GFX, ThrottlerBit::TEMP_GPU),
            (THM_SOC, ThrottlerBit::TEMP_SOC),
            (TDC_VDD, ThrottlerBit::TDC_VDD),
            (TDC_SOC, ThrottlerBit::TDC_SOC),
            (PROCHOT_CPU, ThrottlerBit::PROCHOT_CPU),
            (PROCHOT_GFX, ThrottlerBit::PROCHOT_GPU),
            (EDC_CPU, ThrottlerBit::EDC_CPU),
            (EDC_GFX, ThrottlerBit::EDC_GFX),
        ] {
            let mask = 1 << thr_bit;
            let bit = (thr_status & mask) >> thr_bit;
            indep |= (bit as u64) << (indep_thr_bit as u64);
        }

        Some(indep)
    }

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
}

// Renoir, Lucienne, Cezanne (Green Sardine), Barcelo, Cyan Skillfish, VanGogh (legacy SMU)
impl MetricsInfo for gpu_metrics_v2_2 {
    v2_impl!();

    /// Renoir, Lucienne, Cezanne (Green Sardine), Barcelo APU dose not support `average_gfx_power`. always returns 65535 (0xFFFF).
    fn get_average_gfx_power(&self) -> Option<u16> {
        Some(self.average_gfx_power)
    }

    fn get_average_gfx_power_u32(&self) -> Option<u32> {
        Some(self.average_gfx_power as u32)
    }

    fn get_indep_throttle_status(&self) -> Option<u64> {
        Some(self.indep_throttle_status)
    }

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
}

// VanGogh
impl MetricsInfo for gpu_metrics_v2_3 {
    v2_impl!();

    fn get_average_gfx_power(&self) -> Option<u16> {
        Some(self.average_gfx_power)
    }

    fn get_average_gfx_power_u32(&self) -> Option<u32> {
        Some(self.average_gfx_power as u32)
    }

    fn get_indep_throttle_status(&self) -> Option<u64> {
        Some(self.indep_throttle_status)
    }

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
}

// VanGogh
impl MetricsInfo for gpu_metrics_v2_4 {
    v2_impl!();

    fn get_average_gfx_power(&self) -> Option<u16> {
        Some(self.average_gfx_power)
    }

    fn get_average_gfx_power_u32(&self) -> Option<u32> {
        Some(self.average_gfx_power as u32)
    }

    fn get_indep_throttle_status(&self) -> Option<u64> {
        Some(self.indep_throttle_status)
    }

    fn get_average_temperature_gfx(&self) -> Option<u16> {
        Some(self.average_temperature_gfx)
    }

    fn get_average_temperature_soc(&self) -> Option<u16> {
        Some(self.average_temperature_soc)
    }

    fn get_average_temperature_core(&self) -> Option<Vec<u16>> {
        Some(self.average_temperature_core.to_vec())
    }

    fn get_average_temperature_l3(&self) -> Option<Vec<u16>> {
        Some(self.average_temperature_l3.to_vec())
    }

    fn get_average_cpu_voltage(&self) -> Option<u16> {
        Some(self.average_cpu_voltage)
    }

    fn get_average_soc_voltage(&self) -> Option<u16> {
        Some(self.average_soc_voltage)
    }

    fn get_average_gfx_voltage(&self) -> Option<u16> {
        Some(self.average_gfx_voltage)
    }

    fn get_average_cpu_current(&self) -> Option<u16> {
        Some(self.average_cpu_current)
    }

    fn get_average_soc_current(&self) -> Option<u16> {
        Some(self.average_soc_current)
    }

    fn get_average_gfx_current(&self) -> Option<u16> {
        Some(self.average_gfx_current)
    }
}

macro_rules! v3_impl {
    () => {
        fn get_header(&self) -> Option<metrics_table_header> {
            Some(self.common_header.clone())
        }
        fn get_temperature_gfx(&self) -> Option<u16> {
            Some(self.temperature_gfx)
        }

        fn get_temperature_soc(&self) -> Option<u16> {
            Some(self.temperature_soc)
        }

        fn get_temperature_skin(&self) -> Option<u16> {
            Some(self.temperature_skin)
        }

        fn get_temperature_core(&self) -> Option<Vec<u16>> {
            Some(self.temperature_core.to_vec())
        }

        fn get_average_gfx_activity(&self) -> Option<u16> {
            Some(self.average_gfx_activity)
        }

        fn get_average_mm_activity(&self) -> Option<u16> {
            Some(self.average_vcn_activity)
        }

        fn get_average_ipu_activity(&self) -> Option<Vec<u16>> {
            Some(self.average_ipu_activity.to_vec())
        }

        fn get_average_core_c0_activity(&self) -> Option<Vec<u16>> {
            Some(self.average_core_c0_activity.to_vec())
        }

        fn get_average_dram_reads(&self) -> Option<u16> {
            Some(self.average_dram_reads)
        }

        fn get_average_dram_writes(&self) -> Option<u16> {
            Some(self.average_dram_writes)
        }

        fn get_average_ipu_reads(&self) -> Option<u16> {
            Some(self.average_ipu_reads)
        }

        fn get_average_ipu_writes(&self) -> Option<u16> {
            Some(self.average_ipu_writes)
        }

        fn get_system_clock_counter(&self) -> Option<u64> {
            Some(self.system_clock_counter)
        }

        fn get_average_socket_power(&self) -> Option<u32> {
            Some(self.average_socket_power)
        }

        fn get_average_gfx_power(&self) -> Option<u16> {
            None
        }

        fn get_average_gfx_power_u32(&self) -> Option<u32> {
            Some(self.average_gfx_power)
        }

        fn get_average_core_power(&self) -> Option<Vec<u16>> {
            Some(self.average_core_power.to_vec())
        }

        fn get_average_ipu_power(&self) -> Option<u16> {
            Some(self.average_ipu_power)
        }

        fn get_average_apu_power(&self) -> Option<u32> {
            Some(self.average_apu_power)
        }

        fn get_average_dgpu_power(&self) -> Option<u32> {
            Some(self.average_dgpu_power)
        }

        fn get_average_all_core_power(&self) -> Option<u32> {
            Some(self.average_all_core_power)
        }

        fn get_average_sys_power(&self) -> Option<u16> {
            Some(self.average_sys_power)
        }

        fn get_stapm_power_limit(&self) -> Option<u16> {
            Some(self.stapm_power_limit)
        }

        fn get_current_stapm_power_limit(&self) -> Option<u16> {
            Some(self.current_stapm_power_limit)
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

        fn get_average_vpeclk_frequency(&self) -> Option<u16> {
            Some(self.average_vpeclk_frequency)
        }

        fn get_average_ipuclk_frequency(&self) -> Option<u16> {
            Some(self.average_ipuclk_frequency)
        }

        fn get_average_mpipu_frequency(&self) -> Option<u16> {
            Some(self.average_mpipu_frequency)
        }

        fn get_current_coreclk(&self) -> Option<Vec<u16>> {
            Some(self.current_coreclk.to_vec())
        }

        fn get_current_core_maxfreq(&self) -> Option<u16> {
            Some(self.current_core_maxfreq)
        }

        fn get_current_gfx_maxfreq(&self) -> Option<u16> {
            Some(self.current_gfx_maxfreq)
        }

        fn get_throttle_residency_prochot(&self) -> Option<u32> {
            Some(self.throttle_residency_prochot)
        }

        fn get_throttle_residency_spl(&self) -> Option<u32> {
            Some(self.throttle_residency_spl)
        }

        fn get_throttle_residency_fppt(&self) -> Option<u32> {
            Some(self.throttle_residency_fppt)
        }

        fn get_throttle_residency_sppt(&self) -> Option<u32> {
            Some(self.throttle_residency_sppt)
        }

        fn get_throttle_residency_thm_core(&self) -> Option<u32> {
            Some(self.throttle_residency_thm_core)
        }

        fn get_throttle_residency_thm_gfx(&self) -> Option<u32> {
            Some(self.throttle_residency_thm_gfx)
        }

        fn get_throttle_residency_thm_soc(&self) -> Option<u32> {
            Some(self.throttle_residency_thm_soc)
        }

        fn get_temperature_edge(&self) -> Option<u16> { None }
        fn get_temperature_hotspot(&self) -> Option<u16> { None }
        fn get_temperature_mem(&self) -> Option<u16> { None }
        fn get_temperature_vrgfx(&self) -> Option<u16> { None }
        fn get_temperature_vrsoc(&self) -> Option<u16> { None }
        fn get_temperature_vrmem(&self) -> Option<u16> { None }
        fn get_temperature_hbm(&self) -> Option<[u16; NUM_HBM_INSTANCES as usize]> { None }
        fn get_temperature_l3(&self) -> Option<Vec<u16>> { None }
        fn get_average_umc_activity(&self) -> Option<u16> { None }
        fn get_average_cpu_power(&self) -> Option<u16> { None }
        fn get_average_soc_power(&self) -> Option<u16> { None }
        fn get_average_dclk_frequency(&self) -> Option<u16> { None }
        fn get_average_vclk1_frequency(&self) -> Option<u16> { None }
        fn get_average_dclk1_frequency(&self) -> Option<u16> { None }
        fn get_current_l3clk(&self) -> Option<Vec<u16>> { None }
        fn get_current_gfxclk(&self) -> Option<u16> { None }
        fn get_current_socclk(&self) -> Option<u16> { None }
        fn get_current_fclk(&self) -> Option<u16> { None }
        fn get_current_uclk(&self) -> Option<u16> { None }
        fn get_current_vclk(&self) -> Option<u16> { None }
        fn get_current_dclk(&self) -> Option<u16> { None }
        fn get_current_vclk1(&self) -> Option<u16> { None }
        fn get_current_dclk1(&self) -> Option<u16> { None }
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
        fn get_throttle_status(&self) -> Option<u32> { None }
        fn get_indep_throttle_status(&self) -> Option<u64> { None }
        fn get_pcie_link_width(&self) -> Option<u16> { None }
        fn get_pcie_link_speed(&self) -> Option<u16> { None }
        fn get_pcie_bandwidth_acc(&self) -> Option<u64> { None }
        fn get_pcie_bandwidth_inst(&self) -> Option<u64> { None }
        fn get_xgmi_link_width(&self) -> Option<u16> { None }
        fn get_xgmi_link_speed(&self) -> Option<u16> { None }
        fn get_xgmi_read_data_acc(&self) -> Option<[u64; NUM_XGMI_LINKS as usize]> { None }
        fn get_xgmi_write_data_acc(&self) -> Option<[u64; NUM_XGMI_LINKS as usize]> { None }
        fn get_gfx_activity_acc(&self) -> Option<u32> { None }
        fn get_mem_activity_acc(&self) -> Option<u32> { None }
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

// SMU 14.0.0
impl MetricsInfo for gpu_metrics_v3_0 {
    v3_impl!();
}
