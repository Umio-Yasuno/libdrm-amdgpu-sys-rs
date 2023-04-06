use libdrm_amdgpu_sys::AMDGPU::{self, GPU_INFO, MetricsInfo};
use std::fs::File;

fn main() {
    let (amdgpu_dev, _, _) = {
        use std::os::fd::IntoRawFd;

        let f = File::open("/dev/dri/renderD128").unwrap();

        AMDGPU::DeviceHandle::init(f.into_raw_fd()).unwrap()
    };

    let path = amdgpu_dev.get_sysfs_path().unwrap();

    if let Ok(metrics) = amdgpu_dev.get_gpu_metrics_from_sysfs_path(&path) {
        println!("{:#?}", metrics);

        if let Some(socket_power) = metrics.get_average_socket_power() {
            println!("Average Socket Power: {socket_power} W");
        }
    } else {
        let ext_info = amdgpu_dev.device_info().unwrap();
        let asic_name = ext_info.get_asic_name();

        println!("{asic_name} dose not support GPU metrics.");
        println!("Vega12 (dGPU) or later, Renoir (APU) or later supports GPU metrics.")
    }
}
