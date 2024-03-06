use libdrm_amdgpu_sys::*;
use std::fs::File;
use std::io;

fn dump(image: &[u8], vbios_name: String) -> io::Result<()> {
    use std::io::Write;

    let path = format!("./{vbios_name}.bin");
    let mut f = File::create(&path)?;

    f.write_all(&image)?;

    println!("\nDumped to {path}");
    Ok(())
}

fn main() {
    let device_path = std::env::var("AMDGPU_PATH").unwrap_or("/dev/dri/renderD128".to_string());
    let (amdgpu_dev, _, _) = {
        use std::os::fd::IntoRawFd;

        let f = File::open(device_path).unwrap();

        AMDGPU::DeviceHandle::init(f.into_raw_fd()).unwrap()
    };

    if let Ok(vbios) = amdgpu_dev.get_vbios_info() {
        println!("\nVBIOS info:");
        println!("name: [{}]", vbios.name);
        println!("pn: [{}]", vbios.pn);
        println!("ver: [{}]", vbios.ver);
        println!("date: [{}]", vbios.date);
        println!("vbios size: {}", vbios.size);

        let args: Vec<String> = std::env::args().collect();

        if args.contains(&"-d".to_string()) || args.contains(&"--dump".to_string()) {
            // if let Ok(vbios_image) = unsafe { amdgpu_dev.get_vbios_image_with_size(vbios.size) } {
            if let Ok(vbios_image) = amdgpu_dev.get_vbios_image() {
                let name = vbios.name.replace(' ', "_").replace('/', "_");
                dump(&vbios_image, name).unwrap();
            }
        } else {
            if let Ok(vbios_image) = amdgpu_dev.get_vbios_image() {
                use AMDGPU::VBIOS::VbiosParser;

                let vbios_parser = VbiosParser::new(vbios_image);
                
                let rom_header = vbios_parser.get_atom_rom_header().unwrap();
                println!("{rom_header:#X?}");

                let data_table = vbios_parser.get_atom_data_table(&rom_header).unwrap();
                println!("{data_table:#X?}");

                let firmware_info = vbios_parser.get_atom_firmware_info(&data_table).unwrap();
                println!("firmwareinfo: {firmware_info:#?}");

                if !vbios_parser.valid_vbios() {
                    panic!();
                }

                return;
            }
            println!("If you need a VBIOS image, add \"-d\" or \"--dump\" as an argument and run.");
        }
    }
}
