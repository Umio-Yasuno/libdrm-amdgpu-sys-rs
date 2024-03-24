use libdrm_amdgpu_sys::*;
use AMDGPU::{IpHwId, HwId};
use std::fs::File;

fn main() {
    let device_path = std::env::var("AMDGPU_PATH").unwrap_or("/dev/dri/renderD128".to_string());
    let (amdgpu_dev, _, _) = {
        use std::os::fd::IntoRawFd;

        let f = File::open(device_path).unwrap();

        AMDGPU::DeviceHandle::init(f.into_raw_fd()).unwrap()
    };

    let sysfs = amdgpu_dev.get_sysfs_path().unwrap();
    if let Ok(smu) = IpHwId::get_from_die_id_sysfs(HwId::MP0, &sysfs.join("ip_discovery/die/0/")) {
        if let Some(inst) = smu.instances.get(0) {
            println!("SMU (MP0) version: {}.{}.{}", inst.major, inst.minor, inst.revision);
        }
    };

    let pp_table;

    if let Ok(f) = std::fs::read(&sysfs.join("pp_table")) {
        pp_table = AMDGPU::PPTable::from_bytes(&f);
        println!("from sysfs");
    } else if let Ok(vbios_image) = amdgpu_dev.get_vbios_image() {
        use AMDGPU::VBIOS::VbiosParser;

        let vbios_parser = VbiosParser::new(vbios_image);
        let rom_header = vbios_parser.get_atom_rom_header().unwrap();
        let data_table = vbios_parser.get_atom_data_table(&rom_header).unwrap();

        pp_table = vbios_parser.get_powerplay_table(&data_table).unwrap();
        println!("from VBIOS");
    } else {
        return;
    }

    println!("{pp_table:#?}");
}
