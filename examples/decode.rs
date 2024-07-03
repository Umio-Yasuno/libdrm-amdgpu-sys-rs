use libdrm_amdgpu_sys::*;
use std::fs::File;
use std::io;

fn main() {
    let vbios_image = std::fs::read("/tmp/vbios.rom").unwrap();

    use AMDGPU::VBIOS::VbiosParser;

    let vbios_parser = VbiosParser::new(vbios_image);

    if let Some(name) = vbios_parser.get_vbios_name() {
        println!("name: {name:?}");
    }

    let rom_header = vbios_parser.get_atom_rom_header().unwrap();
    println!("{rom_header:#?}");
    let data_table = vbios_parser.get_atom_data_table(&rom_header).unwrap();
    println!("{data_table:#?}");
    let firmware_info = vbios_parser.get_atom_firmware_info(&data_table).unwrap();
    println!("{firmware_info:#?}");
    let ppt_bytes = vbios_parser.get_powerplay_table_bytes(&data_table).unwrap();
    let ppt = AMDGPU::PPTable::decode_with_smu_version(&ppt_bytes, (13, 0, 0));

    println!("{ppt:#?}");
}
