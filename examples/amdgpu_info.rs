use libdrm_amdgpu_sys::*;

use std::fs::File;
use std::os::unix::io::IntoRawFd;

fn main() {
    let v = File::open("/dev/dri/renderD128").unwrap();
    let fd = v.into_raw_fd();

    use libdrm_amdgpu_sys::AMDGPU::HANDLE;

    let amdgpu_dev = AMDGPU::DEVICE_HANDLE::init(fd).unwrap();

    let gpu_info = amdgpu_dev.query_gpu_info().unwrap();
    let _ext_info = amdgpu_dev.device_info().unwrap();

    /*
    println!("{gpu_info:?}");
    println!();
    println!("{ext_info:?}");
    */

    {
        let mark_name = amdgpu_dev.get_marketing_name().unwrap();

        println!();
        println!("Marketing Name: [{mark_name}]");
        println!("DeviceID.RevID: {:#0X}.{:#0X}", gpu_info.asic_id, gpu_info.pci_rev_id);

        let family = gpu_info.get_family_name();
        let asic_name = gpu_info.get_asic_name();
        let chip_class = gpu_info.get_chip_class();

        println!();
        println!("Family: {family}");
        println!("ASIC Name: {asic_name}");
        println!("Chip class: {chip_class}");

        let vram_type = gpu_info.get_vram_type();
        let peak_bw = gpu_info.peak_memory_bw_gb();

        println!();
        println!("VRAM Type: {vram_type}");
        println!("VRAM Bit Width: {}-bit", gpu_info.vram_bit_width);
        println!("Peak Memory BW: {peak_bw} GB/s");
    }

    {
        use libdrm_amdgpu_sys::AMDGPU::HW_IP::*;

        let ip_list = [
            HW_IP_TYPE::GFX,
            HW_IP_TYPE::COMPUTE,
            HW_IP_TYPE::DMA,
            HW_IP_TYPE::UVD,
            HW_IP_TYPE::VCE,
            HW_IP_TYPE::UVD_ENC,
            HW_IP_TYPE::VCN_DEC,
            HW_IP_TYPE::VCN_ENC,
            HW_IP_TYPE::VCN_JPEG,
        ];

        println!();
        for ip_type in &ip_list {
            let ip_info = amdgpu_dev.query_hw_ip_info(*ip_type, 0).unwrap();

            let (major, minor) = ip_info.version();
            let queues = ip_info.num_queues();

            if major == 0 || queues == 0 { continue; }

            println!("{ip_type} IP: ver {major}.{minor}, queues: {queues}", );
        }
    }
    
    {
        use libdrm_amdgpu_sys::AMDGPU::FW_VERSION::*;

        let fw_list = [
            FW_TYPE::VCE,
            FW_TYPE::UVD,
            FW_TYPE::GMC,
            FW_TYPE::GFX_ME,
            FW_TYPE::GFX_PFP,
            FW_TYPE::GFX_CE,
            FW_TYPE::GFX_RLC,
            FW_TYPE::GFX_MEC,
            FW_TYPE::SMC,
            FW_TYPE::SDMA,
            FW_TYPE::SOS,
            FW_TYPE::ASD,
            FW_TYPE::VCN,
            FW_TYPE::GFX_RLC_RESTORE_LIST_CNTL,
            FW_TYPE::GFX_RLC_RESTORE_LIST_GPM_MEM,
            FW_TYPE::GFX_RLC_RESTORE_LIST_SRM_MEM,
            FW_TYPE::DMCU,
            FW_TYPE::TA,
            FW_TYPE::DMCUB,
            FW_TYPE::TOC,
        ];

        println!();
        for fw_type in &fw_list {
            let query = amdgpu_dev.query_firmware_version(*fw_type, 0, 0);

            let fw_info = match query {
                Ok(v) => v,
                Err(_) => continue,
            };

            let (ver, ftr) = (fw_info.version, fw_info.feature);

            if ver == 0 { continue; }

            println!("{fw_type} FW ver: {ver}, feature: {ftr}");
        }
    }

    {
        use libdrm_amdgpu_sys::AMDGPU::VIDEO_CAPS::*;

        let codec_list = [
            CODEC::MPEG2,
            CODEC::MPEG4,
            CODEC::VC1,
            CODEC::MPEG4_AVC,
            CODEC::HEVC,
            CODEC::JPEG,
            CODEC::VP9,
            CODEC::AV1,
        ];

        let dec = amdgpu_dev.get_video_caps(CAP_TYPE::DECODE).unwrap();
        let enc = amdgpu_dev.get_video_caps(CAP_TYPE::ENCODE).unwrap();

        println!();
        for codec in &codec_list {
            let dec_cap = dec.get_codec_info(*codec).is_supported();
            let enc_cap = enc.get_codec_info(*codec).is_supported();

            println!("{codec} decode: {dec_cap}, encode: {enc_cap}");
        }
    }

    {
        let bus_info = PCI::BUS_INFO::drm_get_device2(fd).unwrap();
        println!();
        println!("PCI: {bus_info}");
        println!("{:?}", bus_info.get_link_info(PCI::STATUS::Max));
    }

    unsafe {
        use libdrm_amdgpu_sys::AMDGPU::VBIOS::*;

        let vbios = amdgpu_dev.vbios_info(fd).unwrap();

        let [name, pn, ver_str, date] = [
            vbios.name.to_vec(),
            vbios.vbios_pn.to_vec(),
            vbios.vbios_ver_str.to_vec(),
            vbios.date.to_vec(),
        ].map(|v| v.null_ctrl_to_space());

        println!();
        // println!("{:?}", vbios);
        println!("VBIOS info");
        println!("name: {}", String::from_utf8(name).unwrap());
        println!("pn: {}", String::from_utf8(pn).unwrap());
        println!("ver_str: {}", String::from_utf8(ver_str).unwrap());
        println!("date: {}", String::from_utf8(date).unwrap());

        let vbios_size = amdgpu_dev.vbios_size(fd).unwrap();
        println!("vbios size: {vbios_size}");
    }

    amdgpu_dev.deinit().unwrap();
}
