use libdrm_amdgpu_sys::*;
use std::fs::File;
use std::io;

fn main() {
    let device_path = std::env::var("AMDGPU_PATH").unwrap_or("/dev/dri/renderD128".to_string());
    let (amdgpu_dev, _, _) = {
        use std::os::fd::IntoRawFd;

        let f = File::open(device_path).unwrap();

        AMDGPU::DeviceHandle::init(f.into_raw_fd()).unwrap()
    };

    if let Ok(vbios_image) = amdgpu_dev.get_vbios_image() {
        use AMDGPU::VBIOS::VbiosParser;

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

        let firmware_info = vbios_parser.get_atom_firmware_info(&data_table).unwrap();
        println!("firmwareinfo: {firmware_info:#?}");
    }
}
