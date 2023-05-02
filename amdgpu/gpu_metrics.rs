use crate::AMDGPU::*;
pub use crate::bindings::{
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
    V2_0(gpu_metrics_v2_0),
    V2_1(gpu_metrics_v2_1),
    V2_2(gpu_metrics_v2_2),
    V2_3(gpu_metrics_v2_3),
}

macro_rules! impl_metrics {
    ($name: tt, $type: ty) => {
        fn $name(&self) -> $type {
            match self {
                Self::V1_0(table) => table.$name(),
                Self::V1_1(table) => table.$name(),
                Self::V1_2(table) => table.$name(),
                Self::V1_3(table) => table.$name(),
                Self::V2_0(table) => table.$name(),
                Self::V2_1(table) => table.$name(),
                Self::V2_2(table) => table.$name(),
                Self::V2_3(table) => table.$name(),
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
    impl_metrics!(get_temperature_core, Option<[u16; 8]>);
    impl_metrics!(get_temperature_l3, Option<[u16; 2]>);
    impl_metrics!(get_average_gfx_activity, Option<u16>);
    impl_metrics!(get_average_umc_activity, Option<u16>);
    impl_metrics!(get_average_mm_activity, Option<u16>);
    impl_metrics!(get_system_clock_counter, Option<u64>);
    impl_metrics!(get_average_socket_power, Option<u16>);
    impl_metrics!(get_average_cpu_power, Option<u16>);
    impl_metrics!(get_average_soc_power, Option<u16>);
    impl_metrics!(get_average_gfx_power, Option<u16>);
    impl_metrics!(get_average_core_power, Option<[u16; 8]>);
    impl_metrics!(get_average_gfxclk_frequency, Option<u16>);
    impl_metrics!(get_average_socclk_frequency, Option<u16>);
    impl_metrics!(get_average_uclk_frequency, Option<u16>);
    impl_metrics!(get_average_fclk_frequency, Option<u16>);
    impl_metrics!(get_average_vclk_frequency, Option<u16>);
    impl_metrics!(get_average_dclk_frequency, Option<u16>);
    impl_metrics!(get_average_vclk1_frequency, Option<u16>);
    impl_metrics!(get_average_dclk1_frequency, Option<u16>);
    impl_metrics!(get_current_gfxclk, Option<u16>);
    impl_metrics!(get_current_socclk, Option<u16>);
    impl_metrics!(get_current_uclk, Option<u16>);
    impl_metrics!(get_current_fclk, Option<u16>);
    impl_metrics!(get_current_vclk, Option<u16>);
    impl_metrics!(get_current_dclk, Option<u16>);
    impl_metrics!(get_current_vclk1, Option<u16>);
    impl_metrics!(get_current_dclk1, Option<u16>);
    impl_metrics!(get_current_coreclk, Option<[u16; 8]>);
    impl_metrics!(get_current_l3clk, Option<[u16; 2]>);
    impl_metrics!(get_throttle_status, Option<u32>);
    impl_metrics!(get_current_fan_speed, Option<u16>);
    impl_metrics!(get_fan_pwm, Option<u16>);
    impl_metrics!(get_pcie_link_width, Option<u16>);
    impl_metrics!(get_pcie_link_spped, Option<u16>);
    impl_metrics!(get_gfx_activity_acc, Option<u32>);
    impl_metrics!(get_mem_activity_acc, Option<u32>);
    impl_metrics!(get_temperature_hbm, Option<[u16; NUM_HBM_INSTANCES as usize]>);
    impl_metrics!(get_voltage_soc, Option<u16>);
    impl_metrics!(get_voltage_gfx, Option<u16>);
    impl_metrics!(get_voltage_mem, Option<u16>);
}

impl DeviceHandle {
    pub fn get_gpu_metrics_from_sysfs_path<P: Into<PathBuf>>(
        &self,
        path: P
    ) -> io::Result<GpuMetrics> {
        let mut f = File::open(path.into().join("gpu_metrics"))?;
        let mut buf: Vec<u8> = Vec::with_capacity(256);
        f.read_to_end(&mut buf)?;

        let header = metrics_table_header::from_slice(&buf);

        let metrics = match (header.format_revision, header.content_revision) {
            (1, 0) => GpuMetrics::V1_0(Self::get_metrics_table(&buf)),
            (1, 1) => GpuMetrics::V1_1(Self::get_metrics_table(&buf)),
            (1, 2) => GpuMetrics::V1_2(Self::get_metrics_table(&buf)),
            (1, 3) => GpuMetrics::V1_3(Self::get_metrics_table(&buf)),
            (2, 0) => GpuMetrics::V2_0(Self::get_metrics_table(&buf)),
            (2, 1) => GpuMetrics::V2_1(Self::get_metrics_table(&buf)),
            (2, 2) => GpuMetrics::V2_2(Self::get_metrics_table(&buf)),
            (2, 3) => GpuMetrics::V2_3(Self::get_metrics_table(&buf)),
            _ => GpuMetrics::Unknown,
        };

        Ok(metrics)
    }

    pub fn get_gpu_metrics(&self) -> io::Result<GpuMetrics> {
        let sysfs_path = self.get_sysfs_path().unwrap();
        self.get_gpu_metrics_from_sysfs_path(sysfs_path)
    }

    fn get_metrics_table<T>(bytes: &[u8]) -> T {
        unsafe {
            let mut metrics: MaybeUninit<T> = MaybeUninit::zeroed();
            let metrics_ptr = metrics.as_mut_ptr();
            let structure_size = size_of::<T>();

            ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                metrics_ptr as *mut u8,
                structure_size,
            );

            metrics.assume_init()
        }
    }
}
