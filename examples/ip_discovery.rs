use libdrm_amdgpu_sys::LibDrmAmdgpu;
use libdrm_amdgpu_sys::AMDGPU::{self, IpDieEntry};
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

    let ip_die_entries = IpDieEntry::get_all_entries_from_sysfs(&path);

    for entry in &ip_die_entries {
        println!("\ndie_id: {:>2}", entry.die_id);

        for ip_hw_id in &entry.ip_hw_ids {
            println!("{ip_hw_id:#X?}");
        }
    }
}
