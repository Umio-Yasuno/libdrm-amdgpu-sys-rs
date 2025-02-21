use libdrm_amdgpu_sys::LibDrmAmdgpu;
use libdrm_amdgpu_sys::AMDGPU::{self, GPU_INFO, MetricsInfo};
use std::fs::File;

fn main() {
    let libdrm_amdgpu = LibDrmAmdgpu::new().unwrap();
    let device_path = std::env::var("AMDGPU_PATH").unwrap_or("/dev/dri/renderD128".to_string());
    let (amdgpu_dev, _, _) = {
        use std::os::fd::IntoRawFd;

        let f = File::open(device_path).unwrap();

        libdrm_amdgpu.init_device_handle(f.into_raw_fd()).unwrap()
    };

    let path = amdgpu_dev.get_sysfs_path().unwrap();

    match amdgpu_dev.get_gpu_metrics_from_sysfs_path(&path) { Ok(metrics) => {
        println!("{:#?}", metrics);

        if let Some(socket_power) = metrics.get_average_socket_power() {
            println!("Average Socket Power: {socket_power} W");
        }

        if let Some(thr) = metrics.get_throttle_status_info() {
            println!("Throttle Status: {:?}", thr.get_all_throttler());
        }
    } _ => {
        let ext_info = amdgpu_dev.device_info().unwrap();
        let asic_name = ext_info.get_asic_name();

        println!("{asic_name} dose not support GPU metrics.");
        println!("Vega12 (dGPU) or later, Renoir (APU) or later supports GPU metrics.")
    }}
}
