use libdrm_amdgpu_sys::*;
use AMDGPU::{IpHwId, HwId};
use std::fs::File;

fn main() {
    let libdrm_amdgpu = LibDrmAmdgpu::new().unwrap();
    let device_path = std::env::var("AMDGPU_PATH").unwrap_or("/dev/dri/renderD128".to_string());
    let (amdgpu_dev, _, _) = {
        use std::os::fd::IntoRawFd;

        let f = File::open(device_path).unwrap();

        libdrm_amdgpu.init_device_handle(f.into_raw_fd()).unwrap()
    };

    let sysfs = amdgpu_dev.get_sysfs_path().unwrap();
    let smu = IpHwId::get_from_die_id_sysfs(HwId::MP1, &sysfs.join("ip_discovery/die/0/")).ok().and_then(|smu| smu.instances.get(0).map(|v| v.clone()));

    if let Some(smu) = &smu {
        println!("SMU (MP1) version: {}.{}.{}", smu.major, smu.minor, smu.revision);
    }

    let pp_table_bytes_sysfs = std::fs::read(&sysfs.join("pp_table")).ok();
    let pp_table_bytes_vbios = amdgpu_dev.get_vbios_image().ok().and_then(|vbios_image| {
        use AMDGPU::VBIOS::VbiosParser;

        let vbios_parser = VbiosParser::new(vbios_image);
        let rom_header = vbios_parser.get_atom_rom_header()?;
        let data_table = vbios_parser.get_atom_data_table(&rom_header)?;

        Some(vbios_parser.get_powerplay_table_bytes(&data_table)?.to_vec())
    });

    for (bytes, src) in [
        (pp_table_bytes_sysfs, "sysfs"),
        (pp_table_bytes_vbios, "VBIOS"),
    ] {
        let Some(bytes) = bytes else { continue };
        let pp_table = if let Some(smu) = &smu {
            AMDGPU::PPTable::decode_with_smu_version(&bytes, smu.version())
        } else {
            AMDGPU::PPTable::decode(&bytes)
        };

        println!("from {src}: {pp_table:#?}");
    }
}
