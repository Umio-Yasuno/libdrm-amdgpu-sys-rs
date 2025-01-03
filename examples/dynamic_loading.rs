use libdrm_amdgpu_sys::*;
use AMDGPU::GPU_INFO;

fn main() {
    let drm_amdgpu = LibDrmAmdgpu::new().unwrap();
    let fd = {
        use std::fs;
        use std::os::fd::IntoRawFd;

        let f = fs::OpenOptions::new().read(true).write(true).open("/dev/dri/renderD128").unwrap();

        f.into_raw_fd()
    };
    let (amdgpu_dev, _major, _minor) = drm_amdgpu.init_device_handle(fd).unwrap();
    let device_info = amdgpu_dev.device_info().unwrap();
    let device_name = device_info.find_device_name_or_default();

    println!("{device_info:#?}");
    println!("Device Name: [{device_name}]");
}
