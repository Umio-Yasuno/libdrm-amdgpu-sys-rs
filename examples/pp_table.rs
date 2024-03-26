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
    let bytes = if let Ok(bytes) = std::fs::read(&sysfs.join("pp_table")) {
        println!("from sysfs");

        bytes
    } else if let Ok(vbios_image) = amdgpu_dev.get_vbios_image() {
        use AMDGPU::VBIOS::VbiosParser;

        println!("from VBIOS");

        let vbios_parser = VbiosParser::new(vbios_image);
        let rom_header = vbios_parser.get_atom_rom_header().unwrap();
        let data_table = vbios_parser.get_atom_data_table(&rom_header).unwrap();

        vbios_parser.get_powerplay_table_bytes(&data_table).unwrap().to_vec()
    } else {
        return;
    };

    let pp_table = if let Some(smu) = IpHwId::get_from_die_id_sysfs(HwId::MP1, &sysfs.join("ip_discovery/die/0/")).ok().and_then(|smu| smu.instances.get(0).map(|v| v.clone())) {
        println!("SMU (MP1) version: {}.{}.{}", smu.major, smu.minor, smu.revision);

        AMDGPU::PPTable::decode_with_smu_version(&bytes, smu.version())
    } else {
        AMDGPU::PPTable::decode(&bytes)
    };

    println!("{pp_table:#?}");
}
