use libdrm_amdgpu_sys::*;
use AMDGPU::VBIOS::VbiosParser;
use std::fs::File;
use std::io;

fn main() {
    let libdrm_amdgpu = LibDrmAmdgpu::new().unwrap();
    let device_path = std::env::var("AMDGPU_PATH").unwrap_or("/dev/dri/renderD128".to_string());
    let (amdgpu_dev, _, _) = {
        use std::os::fd::IntoRawFd;

        let f = File::open(device_path).unwrap();

        libdrm_amdgpu.init_device_handle(f.into_raw_fd()).unwrap()
    };

    let Ok(vbios_image) = amdgpu_dev.get_vbios_image() else { return };

    let vbios_parser = VbiosParser::new(vbios_image);

    if !vbios_parser.valid_vbios() || !vbios_parser.check_length() {
        panic!();
    }

    if let Some(name) = vbios_parser.get_vbios_name() {
        println!("name: {name:?}");
    }

    let rom_header = vbios_parser.get_atom_rom_header().unwrap();
    println!("{rom_header:#X?}");

    let data_table = vbios_parser.get_atom_data_table(&rom_header).unwrap();
    println!("{data_table:#X?}");

    if let Some(h) = vbios_parser.get_atom_firmware_info_header(&data_table) {
        println!("{h:#?}");
        match (h.format_revision, h.content_revision) {
            (3, 5) => {
                let firmware_info = vbios_parser.get_atom_firmware_info_v3_5(&data_table).unwrap();
                println!("firmwareinfo: {firmware_info:#?}");
            },
            _ => {
                let firmware_info = vbios_parser.get_atom_firmware_info(&data_table).unwrap();
                println!("firmwareinfo: {firmware_info:#?}");
            },
        }
    }
}
