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
    let fd = {
        use std::os::unix::io::IntoRawFd;

        let v = File::open("/dev/dri/renderD128").unwrap();

        v.into_raw_fd()
    };

    let amdgpu_dev = AMDGPU::DeviceHandle::init(fd).unwrap();

    if let (Ok(vbios), Ok(vbios_size)) = unsafe {
        (amdgpu_dev.vbios_info(), amdgpu_dev.vbios_size())
    } {
        let [name, pn, ver_str, date] = [
            vbios.name.to_vec(),
            vbios.vbios_pn.to_vec(),
            vbios.vbios_ver_str.to_vec(),
            vbios.date.to_vec(),
        ]
        .map(|v| {
            let tmp = String::from_utf8(v).unwrap();

            tmp.trim_end_matches(|c: char| c.is_control() || c.is_whitespace()).to_string()
        });

        println!("\nVBIOS info:");
        println!("name: [{name}]");
        println!("pn: [{pn}]");
        println!("ver_str: [{ver_str}]");
        println!("date: [{date}]");

        println!("vbios size: {vbios_size}");

        let args: Vec<String> = std::env::args().collect();

        if args.contains(&"-d".to_string()) || args.contains(&"--dump".to_string()) {
            if let Ok(vbios_image) = unsafe { amdgpu_dev.vbios_image(vbios_size as usize) } {
                let name = name.replace(' ', "");
                dump(&vbios_image, name).unwrap();
            }
        } else {
            println!("If you need a VBIOS image, add \"-d\" or \"--dump\" as an argument and run.");
        }
    }
}
