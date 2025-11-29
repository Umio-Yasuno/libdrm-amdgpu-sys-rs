mod metrics_table;
pub use metrics_table::*;

use crate::AMDGPU::{DeviceHandle, ThrottleStatus};
pub use crate::bindings::{
    NUM_HBM_INSTANCES,
    NUM_VCN,
    NUM_JPEG_ENG,
    NUM_XGMI_LINKS,
    MAX_CLKS,
    MAX_GFX_CLKS,
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
};

use core::mem::{size_of, MaybeUninit};
use core::ptr;

use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

/// AMD GPU metrics data available from `"{sysfs_path}/gpu_metrics"`.  
/// Vega12 (dGPU) or later, Renoir (APU) or later supports GPU metrics.  
/// [DeviceHandle::get_gpu_metrics]
#[derive(Clone, Debug)]
pub enum GpuMetrics {
    Unknown,
    V1_0(gpu_metrics_v1_0),
    V1_1(gpu_metrics_v1_1),
    V1_2(gpu_metrics_v1_2),
    V1_3(gpu_metrics_v1_3),
    V1_4(gpu_metrics_v1_4),
    V1_5(gpu_metrics_v1_5),
    V2_0(gpu_metrics_v2_0),
    V2_1(gpu_metrics_v2_1),
    V2_2(gpu_metrics_v2_2),
    V2_3(gpu_metrics_v2_3),
    V2_4(gpu_metrics_v2_4),
    V3_0(gpu_metrics_v3_0),
}

macro_rules! impl_metrics {
    ($name: tt, $type: ty) => {
        fn $name(&self) -> $type {
            match self {
                Self::V1_0(table) => table.$name(),
                Self::V1_1(table) => table.$name(),
                Self::V1_2(table) => table.$name(),
                Self::V1_3(table) => table.$name(),
                Self::V1_4(table) => table.$name(),
                Self::V1_5(table) => table.$name(),
                Self::V2_0(table) => table.$name(),
                Self::V2_1(table) => table.$name(),
                Self::V2_2(table) => table.$name(),
                Self::V2_3(table) => table.$name(),
                Self::V2_4(table) => table.$name(),
                Self::V3_0(table) => table.$name(),
                Self::Unknown => None,
            }
        }
    }
}

impl MetricsInfo for GpuMetrics {
    impl_metrics!(get_header, Option<metrics_table_header>);
    impl_metrics!(get_temperature_edge, Option<u16>);
    impl_metrics!(get_temperature_hotspot, Option<u16>);
    impl_metrics!(get_temperature_mem, Option<u16>);
    impl_metrics!(get_temperature_vrgfx, Option<u16>);
    impl_metrics!(get_temperature_vrsoc, Option<u16>);
    impl_metrics!(get_temperature_vrmem, Option<u16>);
    impl_metrics!(get_temperature_gfx, Option<u16>);
    impl_metrics!(get_temperature_soc, Option<u16>);
    impl_metrics!(get_temperature_core, Option<Vec<u16>>);
    impl_metrics!(get_temperature_l3, Option<Vec<u16>>);
    impl_metrics!(get_temperature_skin, Option<u16>);
    impl_metrics!(get_average_gfx_activity, Option<u16>);
    impl_metrics!(get_average_umc_activity, Option<u16>);
    impl_metrics!(get_average_mm_activity, Option<u16>);
    impl_metrics!(get_average_ipu_activity, Option<Vec<u16>>);
    impl_metrics!(get_average_core_c0_activity, Option<Vec<u16>>);
    impl_metrics!(get_average_dram_reads, Option<u16>);
    impl_metrics!(get_average_dram_writes, Option<u16>);
    impl_metrics!(get_average_ipu_reads, Option<u16>);
    impl_metrics!(get_average_ipu_writes, Option<u16>);
    impl_metrics!(get_system_clock_counter, Option<u64>);
    impl_metrics!(get_average_socket_power, Option<u32>);
    impl_metrics!(get_average_cpu_power, Option<u16>);
    impl_metrics!(get_average_soc_power, Option<u16>);
    impl_metrics!(get_average_gfx_power, Option<u16>);
    impl_metrics!(get_average_gfx_power_u32, Option<u32>);
    impl_metrics!(get_average_core_power, Option<Vec<u16>>);
    impl_metrics!(get_average_ipu_power, Option<u16>);
    impl_metrics!(get_average_apu_power, Option<u32>);
    impl_metrics!(get_average_dgpu_power, Option<u32>);
    impl_metrics!(get_average_all_core_power, Option<u32>);
    impl_metrics!(get_average_sys_power, Option<u16>);
    impl_metrics!(get_stapm_power_limit, Option<u16>);
    impl_metrics!(get_current_stapm_power_limit, Option<u16>);
    impl_metrics!(get_average_gfxclk_frequency, Option<u16>);
    impl_metrics!(get_average_socclk_frequency, Option<u16>);
    impl_metrics!(get_average_uclk_frequency, Option<u16>);
    impl_metrics!(get_average_fclk_frequency, Option<u16>);
    impl_metrics!(get_average_vclk_frequency, Option<u16>);
    impl_metrics!(get_average_dclk_frequency, Option<u16>);
    impl_metrics!(get_average_vclk1_frequency, Option<u16>);
    impl_metrics!(get_average_dclk1_frequency, Option<u16>);
    impl_metrics!(get_average_vpeclk_frequency, Option<u16>);
    impl_metrics!(get_average_ipuclk_frequency, Option<u16>);
    impl_metrics!(get_average_mpipu_frequency, Option<u16>);
    impl_metrics!(get_current_gfxclk, Option<u16>);
    impl_metrics!(get_current_socclk, Option<u16>);
    impl_metrics!(get_current_uclk, Option<u16>);
    impl_metrics!(get_current_fclk, Option<u16>);
    impl_metrics!(get_current_vclk, Option<u16>);
    impl_metrics!(get_current_dclk, Option<u16>);
    impl_metrics!(get_current_vclk1, Option<u16>);
    impl_metrics!(get_current_dclk1, Option<u16>);
    impl_metrics!(get_current_coreclk, Option<Vec<u16>>);
    impl_metrics!(get_current_l3clk, Option<Vec<u16>>);
    impl_metrics!(get_current_core_maxfreq, Option<u16>);
    impl_metrics!(get_current_gfx_maxfreq, Option<u16>);
    impl_metrics!(get_throttle_status, Option<u32>);
    impl_metrics!(get_indep_throttle_status, Option<u64>);
    impl_metrics!(get_indep_throttle_status_without_check, Option<u64>);
    impl_metrics!(get_throttle_status_info, Option<ThrottleStatus>);
    impl_metrics!(get_current_fan_speed, Option<u16>);
    impl_metrics!(get_fan_pwm, Option<u16>);
    impl_metrics!(get_pcie_link_width, Option<u16>);
    impl_metrics!(get_pcie_link_speed, Option<u16>);
    impl_metrics!(get_pcie_bandwidth_acc, Option<u64>);
    impl_metrics!(get_pcie_bandwidth_inst, Option<u64>);
    impl_metrics!(get_xgmi_link_width, Option<u16>);
    impl_metrics!(get_xgmi_link_speed, Option<u16>);
    impl_metrics!(get_xgmi_read_data_acc, Option<[u64; NUM_XGMI_LINKS as usize]>);
    impl_metrics!(get_xgmi_write_data_acc, Option<[u64; NUM_XGMI_LINKS as usize]>);
    impl_metrics!(get_gfx_activity_acc, Option<u32>);
    impl_metrics!(get_mem_activity_acc, Option<u32>);
    impl_metrics!(get_temperature_hbm, Option<[u16; NUM_HBM_INSTANCES as usize]>);
    impl_metrics!(get_voltage_soc, Option<u16>);
    impl_metrics!(get_voltage_gfx, Option<u16>);
    impl_metrics!(get_voltage_mem, Option<u16>);
    impl_metrics!(get_average_temperature_gfx, Option<u16>);
    impl_metrics!(get_average_temperature_soc, Option<u16>);
    impl_metrics!(get_average_temperature_core, Option<Vec<u16>>);
    impl_metrics!(get_average_temperature_l3, Option<Vec<u16>>);
    impl_metrics!(get_average_cpu_voltage, Option<u16>);
    impl_metrics!(get_average_soc_voltage, Option<u16>);
    impl_metrics!(get_average_gfx_voltage, Option<u16>);
    impl_metrics!(get_average_cpu_current, Option<u16>);
    impl_metrics!(get_average_soc_current, Option<u16>);
    impl_metrics!(get_average_gfx_current, Option<u16>);
    impl_metrics!(get_gfxclk_lock_status, Option<u32>);
    impl_metrics!(get_current_socket_power, Option<u16>);
    impl_metrics!(get_all_instances_current_gfxclk, Option<[u16; MAX_GFX_CLKS as usize]>);
    impl_metrics!(get_all_instances_current_socclk, Option<[u16; MAX_CLKS as usize]>);
    impl_metrics!(get_all_instances_current_vclk0, Option<[u16; MAX_CLKS as usize]>);
    impl_metrics!(get_all_instances_current_dclk0, Option<[u16; MAX_CLKS as usize]>);
    impl_metrics!(get_all_vcn_activity, Option<[u16; NUM_VCN as usize]>);
    impl_metrics!(get_all_jpeg_activity, Option<[u16; NUM_JPEG_ENG as usize]>);
    impl_metrics!(get_throttle_residency_prochot, Option<u32>);
    impl_metrics!(get_throttle_residency_spl, Option<u32>);
    impl_metrics!(get_throttle_residency_fppt, Option<u32>);
    impl_metrics!(get_throttle_residency_sppt, Option<u32>);
    impl_metrics!(get_throttle_residency_thm_core, Option<u32>);
    impl_metrics!(get_throttle_residency_thm_gfx, Option<u32>);
    impl_metrics!(get_throttle_residency_thm_soc, Option<u32>);
}

impl DeviceHandle {
    pub fn get_gpu_metrics_from_sysfs_path<P: Into<PathBuf>>(
        &self,
        path: P,
    ) -> io::Result<GpuMetrics> {
        GpuMetrics::get_from_sysfs_path(path)
    }

    pub fn get_gpu_metrics(&self) -> io::Result<GpuMetrics> {
        let sysfs_path = self.get_sysfs_path_io()?;
        GpuMetrics::get_from_sysfs_path(sysfs_path)
    }

    pub fn get_raw_gpu_metrics(&self) -> io::Result<Vec<u8>> {
        let sysfs_path = self.get_sysfs_path_io()?;
        GpuMetrics::get_raw_from_sysfs_path(sysfs_path)
    }
/*
    pub fn get_gpu_metrics_with_buffer<P: Into<PathBuf>>(
        &self,
        buf: &mut Vec<u8>,
        sysfs_path: P,
    ) -> io::Result<GpuMetrics> {
        GpuMetrics::read_file_with_buffer(buf, sysfs_path.into().join("gpu_metrics"))
    }
*/
}

impl GpuMetrics {
    pub fn get_from_sysfs_path<P: Into<PathBuf>>(sysfs_path: P) -> io::Result<Self> {
        let raw = Self::get_raw_from_sysfs_path(sysfs_path.into())?;

        Ok(Self::from_raw(&raw))
    }

    pub fn get_raw_from_sysfs_path<P: Into<PathBuf>>(sysfs_path: P) -> io::Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::with_capacity(256);

        let mut f = File::open(sysfs_path.into().join("gpu_metrics"))?;
        f.read_to_end(&mut buf)?;

        Ok(buf)
    }

    fn from_bytes<T>(bytes: &[u8]) -> T {
        unsafe {
            let mut metrics: MaybeUninit<T> = MaybeUninit::zeroed();
            let metrics_ptr = metrics.as_mut_ptr();
            let size = std::cmp::min(bytes.len(), size_of::<T>());

            ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                metrics_ptr as *mut u8,
                size,
            );

            metrics.assume_init()
        }
    }

    pub fn from_raw(raw: &[u8]) -> Self {
        let header = metrics_table_header::from_bytes(raw);

        match (header.format_revision, header.content_revision) {
            (1, 0) => GpuMetrics::V1_0(Self::from_bytes(raw)),
            (1, 1) => GpuMetrics::V1_1(Self::from_bytes(raw)),
            (1, 2) => GpuMetrics::V1_2(Self::from_bytes(raw)),
            (1, 3) => GpuMetrics::V1_3(Self::from_bytes(raw)),
            (1, 4) => GpuMetrics::V1_4(Self::from_bytes(raw)),
            (1, 5) => GpuMetrics::V1_5(Self::from_bytes(raw)),
            (2, 0) => GpuMetrics::V2_0(Self::from_bytes(raw)),
            (2, 1) => GpuMetrics::V2_1(Self::from_bytes(raw)),
            (2, 2) => GpuMetrics::V2_2(Self::from_bytes(raw)),
            (2, 3) => GpuMetrics::V2_3(Self::from_bytes(raw)),
            (2, 4) => GpuMetrics::V2_4(Self::from_bytes(raw)),
            (3, 0) => GpuMetrics::V3_0(Self::from_bytes(raw)),
            _ => GpuMetrics::Unknown,
        }
    }

    pub fn read_file_with_buffer<P: Into<PathBuf>>(buf: &mut Vec<u8>, path: P) -> io::Result<Self> {
        let mut f = File::open(path.into())?;
        f.read_to_end(buf)?;

        Ok(Self::from_raw(buf))
    }
}
