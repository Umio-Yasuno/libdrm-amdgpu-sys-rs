/// ref: drivers/gpu/drm/amd/include/kgd_pp_interface.h

mod v1;
mod v1_4_5;
mod v2_v3;

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
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_temperature_skin(&self) -> Option<u16>;

    fn get_average_gfx_activity(&self) -> Option<u16>;
    fn get_average_umc_activity(&self) -> Option<u16>;
    fn get_average_mm_activity(&self) -> Option<u16>;
    /// time filtered IPU per-column busy % [0-100],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_ipu_activity(&self) -> Option<Vec<u16>>;
    /// time filtered per-core C0 residency % [0-100],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_core_c0_activity(&self) -> Option<Vec<u16>>;

    /// time filtered DRAM read bandwidth [MB/sec],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_dram_reads(&self) -> Option<u16>;
    /// time filtered DRAM write bandwidth [MB/sec],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_dram_writes(&self) -> Option<u16>;
    /// time filtered IPU read bandwidth [MB/sec],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_ipu_reads(&self) -> Option<u16>;
    /// time filtered IPU write bandwidth [MB/sec],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_ipu_writes(&self) -> Option<u16>;

    fn get_system_clock_counter(&self) -> Option<u64>;
    /// Watts
    fn get_average_socket_power(&self) -> Option<u32>;
    /// Watts
    fn get_average_cpu_power(&self) -> Option<u16>;
    /// Watts
    fn get_average_soc_power(&self) -> Option<u16>;
    /// Watts
    fn get_average_gfx_power(&self) -> Option<u16>;
    /// Watts
    fn get_average_gfx_power_u32(&self) -> Option<u32>;
    /// Watts,  
    /// For VanGogh APU, only the first half is a valid value.  
    /// ref: `drivers/gpu/drm/amd/pm/swsmu/smu11/vangogh_ppt.c`
    fn get_average_core_power(&self) -> Option<Vec<u16>>;

    /// time filtered IPU power \[mW\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_ipu_power(&self) -> Option<u16>;
    /// time filtered APU power \[mW\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_apu_power(&self) -> Option<u32>;
    /// time filtered dGPU power \[mW\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_dgpu_power(&self) -> Option<u32>;
    /// time filtered sum of core power across all cores in the socket \[mW\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_all_core_power(&self) -> Option<u32>;
    /// time filtered total system power \[mW\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_sys_power(&self) -> Option<u16>;
    /// maximum IRM defined STAPM power limit \[mW\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_stapm_power_limit(&self) -> Option<u16>;
    /// time filtered STAPM power limit \[mW\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_current_stapm_power_limit(&self) -> Option<u16>;

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

    /// time filtered clocks \[MHz\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_vpeclk_frequency(&self) -> Option<u16>;
    /// time filtered clocks \[MHz\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_ipuclk_frequency(&self) -> Option<u16>;
    /// time filtered clocks \[MHz\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_average_mpipu_frequency(&self) -> Option<u16>;

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

    /// CCLK frequency limit enforced on classic cores \[MHz\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_current_core_maxfreq(&self) -> Option<u16>;
    /// GFXCLK frequency limit enforced on GFX \[MHz\],
    /// SMU v14.0.0 with [gpu_metrics_v3_0] supports it.
    fn get_current_gfx_maxfreq(&self) -> Option<u16>;

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
