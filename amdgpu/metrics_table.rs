/// ref: drivers/gpu/drm/amd/include/kgd_pp_interface.h

use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
pub use crate::bindings::{
    metrics_table_header,
    gpu_metrics_v1_0,
    gpu_metrics_v1_1,
    gpu_metrics_v1_2,
    gpu_metrics_v1_3,
    gpu_metrics_v1_4,
    gpu_metrics_v1_5,
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
use crate::AMDGPU::ThrottleStatus;

impl metrics_table_header {
    pub(crate) fn from_bytes(buf: &[u8]) -> Self {
        let [structer_size_0, structer_size_1, format_revision, content_revision] = {
            if let Some(tmp) = buf.get(0..4).and_then(|v| v.try_into().ok()) {
                tmp
            } else {
                return Self { structure_size: 0, format_revision: 0, content_revision: 0 };
            }
        };

        Self {
            structure_size: u16::from_le_bytes([structer_size_0, structer_size_1]),
            format_revision,
            content_revision,
        }
    }

    pub fn from_buf(buf: [u8; 4]) -> Self {
        Self::from_bytes(&buf)
    }

    #[cfg(feature = "std")]
    pub fn from_sysfs_path<P: Into<PathBuf>>(path: P) -> io::Result<Self> {
        let mut f = File::open(path.into())?;
        let mut buf = [0u8; 4];

        f.read_exact(&mut buf)?;

        Ok(Self::from_buf(buf))
    }
}

/// The actual unsupported value will be 0xFFFF.
pub trait MetricsInfo {
    fn get_header(&self) -> Option<metrics_table_header>;
    /// Celsius
    fn get_temperature_edge(&self) -> Option<u16>;
    /// Celsius
    fn get_temperature_hotspot(&self) -> Option<u16>;
    /// Celsius
    fn get_temperature_mem(&self) -> Option<u16>;
    /// Celsius
    fn get_temperature_vrgfx(&self) -> Option<u16>;
    /// Celsius
    fn get_temperature_vrsoc(&self) -> Option<u16>;
    /// Celsius
    fn get_temperature_vrmem(&self) -> Option<u16>;
    /// millidegrees Celsius
    fn get_temperature_gfx(&self) -> Option<u16>;
    /// millidegrees Celsius
    fn get_temperature_soc(&self) -> Option<u16>;
    /// millidegrees Celsius,  
    /// For VanGogh APU, only the first half is a valid value.  
    /// ref: `drivers/gpu/drm/amd/pm/swsmu/smu11/vangogh_ppt.c`
    fn get_temperature_core(&self) -> Option<Vec<u16>>;
    /// millidegrees Celsius,  
    /// For VanGogh APU, only the first half is a valid value.  
    /// ref: `drivers/gpu/drm/amd/pm/swsmu/smu11/vangogh_ppt.c`
    fn get_temperature_l3(&self) -> Option<Vec<u16>>;
    fn get_average_gfx_activity(&self) -> Option<u16>;
    fn get_average_umc_activity(&self) -> Option<u16>;
    fn get_average_mm_activity(&self) -> Option<u16>;
    fn get_system_clock_counter(&self) -> Option<u64>;
    /// Watts
    fn get_average_socket_power(&self) -> Option<u32>;
    /// Watts
    fn get_average_cpu_power(&self) -> Option<u16>;
    /// Watts
    fn get_average_soc_power(&self) -> Option<u16>;
    /// Watts
    fn get_average_gfx_power(&self) -> Option<u16>;
    /// Watts,  
    /// For VanGogh APU, only the first half is a valid value.  
    /// ref: `drivers/gpu/drm/amd/pm/swsmu/smu11/vangogh_ppt.c`
    fn get_average_core_power(&self) -> Option<Vec<u16>>;
    /// MHz
    fn get_average_gfxclk_frequency(&self) -> Option<u16>;
    /// MHz
    fn get_average_socclk_frequency(&self) -> Option<u16>;
    /// UMC Clock, MHz
    fn get_average_uclk_frequency(&self) -> Option<u16>;
    /// MHz
    fn get_average_fclk_frequency(&self) -> Option<u16>;
    /// MHz
    fn get_average_vclk_frequency(&self) -> Option<u16>;
    /// MHz
    fn get_average_dclk_frequency(&self) -> Option<u16>;
    /// MHz
    fn get_average_vclk1_frequency(&self) -> Option<u16>;
    /// MHz
    fn get_average_dclk1_frequency(&self) -> Option<u16>;
    /// MHz
    fn get_current_gfxclk(&self) -> Option<u16>;
    /// MHz
    fn get_current_socclk(&self) -> Option<u16>;
    /// MHz
    fn get_current_uclk(&self) -> Option<u16>;
    /// MHz
    fn get_current_fclk(&self) -> Option<u16>;
    /// MHz
    fn get_current_vclk(&self) -> Option<u16>;
    /// MHz
    fn get_current_dclk(&self) -> Option<u16>;
    /// MHz
    fn get_current_vclk1(&self) -> Option<u16>;
    /// MHz
    fn get_current_dclk1(&self) -> Option<u16>;
    /// MHz,  
    /// For VanGogh APU, only the first half is a valid value.  
    /// ref: `drivers/gpu/drm/amd/pm/swsmu/smu11/vangogh_ppt.c`
    fn get_current_coreclk(&self) -> Option<Vec<u16>>;
    /// MHz,  
    /// For VanGogh APU, only the first half is a valid value.  
    /// ref: `drivers/gpu/drm/amd/pm/swsmu/smu11/vangogh_ppt.c`
    fn get_current_l3clk(&self) -> Option<Vec<u16>>;
    fn get_throttle_status(&self) -> Option<u32>;
    fn get_indep_throttle_status(&self) -> Option<u64>;
    fn get_current_fan_speed(&self) -> Option<u16>;
    fn get_fan_pwm(&self) -> Option<u16>;

    /// Clock Lock Status. Each bit corresponds to clock instance
    fn get_pcie_link_width(&self) -> Option<u16>;
    /// Clock Lock Status. Each bit corresponds to clock instance
    fn get_pcie_link_speed(&self) -> Option<u16>;
    /// PCIE accumulated bandwidth (GB/sec),
    /// only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_pcie_bandwidth_acc(&self) -> Option<u64>;
    /// PCIE instantaneous bandwidth (GB/sec)
    /// only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_pcie_bandwidth_inst(&self) -> Option<u64>;

    /// XGMI bus width and bitrate (in Gbps)
    /// only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_xgmi_link_width(&self) -> Option<u16>;
    /// XGMI bus width and bitrate (in Gbps)
    /// only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_xgmi_link_speed(&self) -> Option<u16>;
    /// XGMI accumulated data transfer size(KiloBytes),
    /// only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_xgmi_read_data_acc(&self) -> Option<[u64; NUM_XGMI_LINKS as usize]>;
    /// XGMI accumulated data transfer size(KiloBytes),
    /// only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_xgmi_write_data_acc(&self) -> Option<[u64; NUM_XGMI_LINKS as usize]>;

    fn get_gfx_activity_acc(&self) -> Option<u32>;
    fn get_mem_activity_acc(&self) -> Option<u32>;

    /// Only Aldebaran (MI200) supports it.
    fn get_temperature_hbm(&self) -> Option<[u16; NUM_HBM_INSTANCES as usize]>;

    /// mV
    fn get_voltage_soc(&self) -> Option<u16>;
    /// mV
    fn get_voltage_gfx(&self) -> Option<u16>;
    /// mV
    fn get_voltage_mem(&self) -> Option<u16>;

    /// Average Temperature (unit: centi-Celsius)
    fn get_average_temperature_gfx(&self) -> Option<u16>;
    /// Average Temperature (unit: centi-Celsius)
    fn get_average_temperature_soc(&self) -> Option<u16>;
    /// Average Temperature (unit: centi-Celsius)
    fn get_average_temperature_core(&self) -> Option<Vec<u16>>;
    /// Average Temperature (unit: centi-Celsius)
    fn get_average_temperature_l3(&self) -> Option<Vec<u16>>;

    /// Power/Voltage (unit: mV)
    /// only Vangogh with [gpu_metrics_v2_4] supports it.
    fn get_average_cpu_voltage(&self) -> Option<u16>;
    /// Power/Voltage (unit: mV)
    /// only Vangogh with [gpu_metrics_v2_4] supports it.
    fn get_average_soc_voltage(&self) -> Option<u16>;
    /// Power/Voltage (unit: mV)
    /// only Vangogh with [gpu_metrics_v2_4] supports it.
    fn get_average_gfx_voltage(&self) -> Option<u16>;

    /// Power/Current (unit: mA),
    /// only Vangogh with [gpu_metrics_v2_4] supports it.
    fn get_average_cpu_current(&self) -> Option<u16>;
    /// Power/Voltage (unit: mV)
    /// only Vangogh with [gpu_metrics_v2_4] supports it.
    fn get_average_soc_current(&self) -> Option<u16>;
    /// Power/Voltage (unit: mV)
    /// only Vangogh with [gpu_metrics_v2_4] supports it.
    fn get_average_gfx_current(&self) -> Option<u16>;

    /// Clock Lock Status. Each bit corresponds to clock instance,
    /// only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_gfxclk_lock_status(&self) -> Option<u32>;
    /// Only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_current_socket_power(&self) -> Option<u16>;

    /// All instances (XCC) current gfx clock,
    /// only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_all_instances_current_gfxclk(&self) -> Option<[u16; MAX_GFX_CLKS as usize]>;
    /// All instances current soc clock,
    /// only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_all_instances_current_socclk(&self) -> Option<[u16; MAX_CLKS as usize]>;
    /// Only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_all_instances_current_vclk0(&self) -> Option<[u16; MAX_CLKS as usize]>;
    /// Only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_all_instances_current_dclk0(&self) -> Option<[u16; MAX_CLKS as usize]>;

    /// Utilization (%), only MI300 with [gpu_metrics_v1_4] or [gpu_metrics_v1_5] supports it.
    fn get_all_vcn_activity(&self) -> Option<[u16; NUM_VCN as usize]>;
    /// Utilization (%), only MI300 with [gpu_metrics_v1_5] supports it.
    fn get_all_jpeg_activity(&self) -> Option<[u16; NUM_JPEG_ENG as usize]>;

    fn get_throttle_status_info(&self) -> Option<ThrottleStatus> {
        self.get_indep_throttle_status().map(|thr| ThrottleStatus::new(thr))
    }
}

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

        fn get_average_cpu_power(&self) -> Option<u16> { None }
        fn get_average_soc_power(&self) -> Option<u16> { None }
        fn get_average_gfx_power(&self) -> Option<u16> { None }
        fn get_average_core_power(&self) -> Option<Vec<u16>> { None }

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
        fn get_temperature_edge(&self) -> Option<u16> { None }
        fn get_temperature_vrmem(&self) -> Option<u16> { None }
        fn get_temperature_vrgfx(&self) -> Option<u16> { None }
        fn get_temperature_hbm(&self) -> Option<[u16; NUM_HBM_INSTANCES as usize]> { None }
        fn get_average_mm_activity(&self) -> Option<u16> { None }
        fn get_average_socket_power(&self) -> Option<u32> { None }
        fn get_average_cpu_power(&self) -> Option<u16> { None }
        fn get_average_soc_power(&self) -> Option<u16> { None }
        fn get_average_gfx_power(&self) -> Option<u16> { None }
        fn get_average_core_power(&self) -> Option<Vec<u16>> { None }
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

/*
        fn get_average_gfx_power(&self) -> Option<u16> {
            Some(self.average_gfx_power)
        }
*/
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
    }
}

impl MetricsInfo for gpu_metrics_v2_0 {
    v2_impl!();

    fn get_average_gfx_power(&self) -> Option<u16> { None }
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

// Renoir, Lucienne, Cezanne (Green Sardine), Barcelo, Cyan Skillfish, VanGogh (legacy SMU)
impl MetricsInfo for gpu_metrics_v2_2 {
    v2_impl!();

    /// Renoir, Lucienne, Cezanne (Green Sardine), Barcelo APU dose not support `average_gfx_power`. always returns 65535 (0xFFFF).
    fn get_average_gfx_power(&self) -> Option<u16> {
        Some(self.average_gfx_power)
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

        fn get_temperature_core(&self) -> Option<Vec<u16>> {
            Some(self.temperature_core.to_vec())
        }

        fn get_average_gfx_activity(&self) -> Option<u16> {
            Some(self.average_gfx_activity)
        }

        fn get_average_mm_activity(&self) -> Option<u16> {
            Some(self.average_vcn_activity)
        }

        fn get_system_clock_counter(&self) -> Option<u64> {
            Some(self.system_clock_counter)
        }

        fn get_average_socket_power(&self) -> Option<u32> {
            Some(self.average_socket_power)
        }

        fn get_average_gfx_power(&self) -> Option<u16> {
            None
        /*
            TODO: u32
            Some(self.average_gfx_power)
        */
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
        fn get_current_coreclk(&self) -> Option<Vec<u16>> { None }
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
