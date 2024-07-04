use libdrm_amdgpu_sys::*;
use AMDGPU::VBIOS::VbiosParser;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap();
    let vbios_image = std::fs::read(path).unwrap();
    let vbios_parser = VbiosParser::new(vbios_image);

    if let Some(name) = vbios_parser.get_vbios_name() {
        println!("name: {name:?}");
    }

    let rom_header = vbios_parser.get_atom_rom_header().unwrap();
    println!("{rom_header:#X?}");

    let data_table = vbios_parser.get_atom_data_table(&rom_header).unwrap();
    println!("{data_table:#X?}");

    let firmware_info = vbios_parser.get_atom_firmware_info(&data_table).unwrap();
    println!("{firmware_info:#?}");

    if let Some(ppt_bytes) = vbios_parser.get_powerplay_table_bytes(&data_table) {
        // for Navi31
        let ppt = AMDGPU::PPTable::decode_with_smu_version(&ppt_bytes, (13, 0, 0));
        println!("{ppt:#?}");
    }
}
