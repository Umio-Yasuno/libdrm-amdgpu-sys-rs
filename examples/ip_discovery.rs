use libdrm_amdgpu_sys::AMDGPU::IpDieEntry;
use libdrm_amdgpu_sys::LibDrmAmdgpu;
use std::fs::File;
use std::os::fd::AsRawFd;

fn main() {
    let libdrm_amdgpu = LibDrmAmdgpu::new().unwrap();
    let device_path = std::env::var("AMDGPU_PATH").unwrap_or("/dev/dri/renderD128".to_string());
    let f = File::open(device_path).unwrap();
    let (amdgpu_dev, _, _) = libdrm_amdgpu.init_device_handle(f.as_raw_fd()).unwrap();

    let path = amdgpu_dev.get_sysfs_path().unwrap();

    let ip_die_entries = IpDieEntry::get_all_entries_from_sysfs(&path);

    for entry in &ip_die_entries {
        println!("\ndie_id: {:>2}", entry.die_id);

        for ip_hw_id in &entry.ip_hw_ids {
            println!("{ip_hw_id:#X?}");
        }
    }
}
