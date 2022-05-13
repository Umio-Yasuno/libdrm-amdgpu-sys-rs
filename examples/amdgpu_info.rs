use libdrm_amdgpu_sys::*;

use std::fs::File;
use std::os::unix::io::IntoRawFd;

fn main() {
    let v = File::open("/dev/dri/renderD128").unwrap();
    let fd = v.into_raw_fd();

    use libdrm_amdgpu_sys::AMDGPU::HANDLE;

    let amdgpu_dev = AMDGPU::DEVICE_HANDLE::init(fd).unwrap();

    let gpu_info = amdgpu_dev.query_gpu_info().unwrap();
    println!("{gpu_info:?}");

    let mark_name = amdgpu_dev.get_marketing_name().unwrap();

    println!();
    println!("Marketing Name: [{mark_name}]");
    println!("DeviceID.RevID: {:#0X}.{:#0X}", gpu_info.asic_id, gpu_info.pci_rev_id);

    let family = AMDGPU::FAMILY_NAME::from_id(gpu_info.family_id);
    let asic_name = AMDGPU::ASIC_NAME::get(family, gpu_info.chip_external_rev);
    let chip_class = asic_name.chip_class();

    println!();
    println!("Family: {family}");
    println!("ASIC Name: {asic_name}");
    println!("Chip class: {chip_class}");

    let vram_type = AMDGPU::VRAM_TYPE::from_type_id(gpu_info.vram_type);
    let peak_bw = vram_type.peak_bw_gb(gpu_info.max_memory_clk, gpu_info.vram_bit_width);

    println!();
    println!("VRAM Type: {vram_type}");
    println!("VRAM Bit Width: {}-bit", gpu_info.vram_bit_width);
    println!("Peak Memory BW: {peak_bw} GB/s");

    let info = amdgpu_dev.device_info().unwrap();
    // let info = AMDGPU::INFO::device_info(amdgpu_dev).unwrap();
    // println!("{:?}", info);

    use libdrm_amdgpu_sys::AMDGPU::HW_IP::*;

    let hw_ip = amdgpu_dev.query_hw_ip_info(HW_IP_TYPE::GFX, 0).unwrap();
    println!("GFX: {hw_ip:?}");
    
    use libdrm_amdgpu_sys::AMDGPU::FW_VERSION::*;
    let fw_ver = amdgpu_dev.query_firmware_version(FW_TYPE::VCE, 0, 0).unwrap();
    println!("VCE FW: {:X}", fw_ver.version);

    unsafe {
        let bus_info = PCI::BUS_INFO::drm_get_device2(fd).unwrap();
        println!();
        println!("{:?}", bus_info);
        println!("{:?}", bus_info.get_link_info(PCI::STATUS::Max));

        use libdrm_amdgpu_sys::AMDGPU::VBIOS::*;
        use libdrm_amdgpu_sys::AMDGPU::VIDEO_CAPS::*;

        println!();
        let dec_caps = amdgpu_dev.get_video_caps(CAP_TYPE::DECODE).unwrap();
        let dec_mpeg4 = dec_caps.get_codec_info(CODEC::MPEG4).is_supported();
        println!("MPEG4 Decode: {}", dec_mpeg4);

        println!();
        let vbios = amdgpu_dev.vbios_info(fd).unwrap();
        let vbios_size = amdgpu_dev.vbios_size(fd).unwrap();
        let ver_str = null_control_to_space(vbios.vbios_ver_str.to_vec());

        // println!("{:?}", vbios);
        println!("name: {}", String::from_utf8(vbios.name.to_vec()).unwrap());
        println!("pn: {}", String::from_utf8(vbios.vbios_pn.to_vec()).unwrap());
        println!("ver_str: {}", String::from_utf8(ver_str.to_vec()).unwrap());
        println!("date: {}", String::from_utf8(vbios.date.to_vec()).unwrap());

        println!("vbios size: {vbios_size}");
    }

    amdgpu_dev.deinit().unwrap();
}
