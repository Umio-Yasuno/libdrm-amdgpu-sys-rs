use libdrm_amdgpu_sys::AMDGPU::{self, MetricsInfo};
use std::fs::File;

fn main() {
    let (amdgpu_dev, _, _) = {
        use std::os::fd::IntoRawFd;

        let f = File::open("/dev/dri/renderD129").unwrap();

        AMDGPU::DeviceHandle::init(f.into_raw_fd()).unwrap()
    };

    let path = amdgpu_dev.get_sysfs_path().unwrap();
    let metrics = amdgpu_dev.get_gpu_metrics_from_sysfs_path(&path).unwrap();

    println!("{:#?}", metrics);
    if let Some(socket_power) = metrics.get_average_socket_power() {
        println!("Average Socket Power: {socket_power} W");
    }
}
