use libdrm_amdgpu_sys::*;
use libdrm_amdgpu_sys::AMDGPU::HANDLE;

use std::fs::File;
use std::os::unix::io::IntoRawFd;

fn main() {
    let v = File::open("/dev/dri/renderD128").unwrap();
    let fd = v.into_raw_fd();

    // let amdgpu_dev = AMDGPU::device_initialize(fd).unwrap();
    let amdgpu_dev = AMDGPU::DEVICE_HANDLE::init(fd).unwrap();

    // let gpu_info = AMDGPU::query_gpu_info(amdgpu_dev).unwrap();
    let gpu_info = amdgpu_dev.query_gpu_info().unwrap();
    println!("{gpu_info:?}");

    // let mark_name = AMDGPU::get_marketing_name(amdgpu_dev).unwrap();
    let mark_name = amdgpu_dev.get_marketing_name().unwrap();

    println!();
    println!("Marketing Name: [{mark_name}]");
    println!("DeviceID.RevID: {:#0X}.{:#0X}", gpu_info.asic_id, gpu_info.pci_rev_id);

    let family = AMDGPU::FAMILY_NAME::get(gpu_info.family_id);
    let asic_name = AMDGPU::ASIC_NAME::get(family, gpu_info.chip_external_rev);
    let chip_class = asic_name.chip_class();

    println!();
    println!("Family: {family}");
    println!("ASIC Name: {asic_name}");
    println!("Chip class: {chip_class}");

    let vram_type = AMDGPU::VRAM_TYPE::get(gpu_info.vram_type);
    let peak_bw = vram_type.peak_bw_gb(gpu_info.max_memory_clk, gpu_info.vram_bit_width);

    println!();
    println!("VRAM Type: {vram_type}");
    println!("VRAM Bit Width: {}-bit", gpu_info.vram_bit_width);
    println!("Peak Memory BW: {peak_bw} GB/s");

    let info = amdgpu_dev.device_info().unwrap();
    // let info = AMDGPU::INFO::device_info(amdgpu_dev).unwrap();
    // println!("{:?}", info);

    unsafe {
        println!();
        let vbios = amdgpu_dev.vbios_info(fd).unwrap();
        let vbios_size = amdgpu_dev.vbios_size(fd).unwrap();

        // println!("{:?}", vbios);
        println!("name: {}", String::from_utf8(vbios.name.to_vec()).unwrap());
        println!("pn: {}", String::from_utf8(vbios.vbios_pn.to_vec()).unwrap());
        println!("ver: {}", String::from_utf8(vbios.vbios_ver_str.to_vec()).unwrap());
        println!("date: {}", String::from_utf8(vbios.date.to_vec()).unwrap());

        println!("vbios size: {vbios_size}");
    }
}
