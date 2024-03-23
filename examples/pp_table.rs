use libdrm_amdgpu_sys::*;
use std::fs::File;

fn main() {
    let device_path = std::env::var("AMDGPU_PATH").unwrap_or("/dev/dri/renderD128".to_string());
    let (amdgpu_dev, _, _) = {
        use std::os::fd::IntoRawFd;

        let f = File::open(device_path).unwrap();

        AMDGPU::DeviceHandle::init(f.into_raw_fd()).unwrap()
    };

    let f = std::fs::read(amdgpu_dev.get_sysfs_path().unwrap().join("pp_table")).unwrap();
    let pp_table = AMDGPU::PPTable::from_bytes(&f);

    println!("{pp_table:#?}");
}
