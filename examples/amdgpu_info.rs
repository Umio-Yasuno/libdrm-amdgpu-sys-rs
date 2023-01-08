use libdrm_amdgpu_sys::*;

fn main() {
    let fd = {
        use std::fs::File;
        use std::os::unix::io::IntoRawFd;

        let v = File::open("/dev/dri/renderD128").unwrap();

        v.into_raw_fd()
    };

    let amdgpu_dev = AMDGPU::DeviceHandle::init(fd).unwrap();

    if let Ok(ext_info) = amdgpu_dev.device_info() {
        use AMDGPU::GPU_INFO;

        println!("\n{ext_info:#?}\n");

        if let Ok(mark_name) = ext_info.parse_amdgpu_ids() {
            println!("Marketing Name: [{mark_name}]");
        }

        println!(
            "DeviceID.RevID: {:#0X}.{:#0X}",
            ext_info.device_id(),
            ext_info.pci_rev_id()
        );

        println!();
        println!("Family: {}", ext_info.get_family_name());
        println!("ASIC Name: {}", ext_info.get_asic_name());
        println!("Chip class: {}", ext_info.get_chip_class());

        println!();
        println!("CU: {}", ext_info.cu_active_number());
        println!("Max Engine Clock: {} MHz", ext_info.max_engine_clock() / 1000);
        println!("Peak FP32: {} GFLOPS", ext_info.peak_gflops());

        println!();
        println!("VRAM Type: {}", ext_info.get_vram_type());
        println!("VRAM Bit Width: {}-bit", ext_info.vram_bit_width);
        println!("Peak Memory BW: {} GB/s", ext_info.peak_memory_bw_gb());
        println!("L2cache: {} KiB", ext_info.calc_l2_cache_size() / 1024);
    }

    if let Ok(memory_info) = amdgpu_dev.memory_info() {
        let vram_size_mb = memory_info.vram.total_heap_size / 1024 / 1024;
        println!("VRAM size: {vram_size_mb} MiB");
    }

    {
        use AMDGPU::HW_IP::*;

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
            let ip_info = match amdgpu_dev.query_hw_ip_info(*ip_type, 0) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let (major, minor) = ip_info.version();
            let queues = ip_info.num_queues();

            if queues == 0 {
                continue;
            }

            println!(
                "{:8} IP ver: {major:2}.{minor}, queues: {queues}",
                ip_type.to_string()
            );
        }
    }

    {
        use AMDGPU::FW_VERSION::*;

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
            let fw_info = match amdgpu_dev.query_firmware_version(*fw_type, 0, 0) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let (ver, ftr) = (fw_info.version, fw_info.feature);

            if ver == 0 {
                continue;
            }

            println!("{fw_type} FW:\n   ver: {ver:>#10X}, feature: {ftr:>3}");
        }
    }

    if let [Ok(dec), Ok(enc)] = [
        amdgpu_dev.get_video_caps(AMDGPU::VIDEO_CAPS::CAP_TYPE::DECODE),
        amdgpu_dev.get_video_caps(AMDGPU::VIDEO_CAPS::CAP_TYPE::ENCODE),
    ] {
        use AMDGPU::VIDEO_CAPS::*;

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

        println!();
        println!("Video caps:");

        for codec in &codec_list {
            let [dec_cap, enc_cap] = [dec, enc].map(|type_| type_.get_codec_info(*codec));

            println!("{codec}:");
            println!("    Decode: w {:>5}, h {:>5}", dec_cap.max_width, dec_cap.max_height);
            println!("    Encode: w {:>5}, h {:>5}", enc_cap.max_width, enc_cap.max_height);
        }
    }

    if let Ok(bus_info) = PCI::BUS_INFO::drm_get_device2(fd) {
        println!();
        println!("PCI: {bus_info}");
        println!("{:?}", bus_info.get_link_info(PCI::STATUS::Max));
    }

    if let Ok(vbios) = unsafe { amdgpu_dev.vbios_info(fd) } {
        let [name, pn, ver_str, date] = [
            vbios.name.to_vec(),
            vbios.vbios_pn.to_vec(),
            vbios.vbios_ver_str.to_vec(),
            vbios.date.to_vec(),
        ]
        .map(|v| {
            let vec = v.null_ctrl_to_space();
            let tmp = String::from_utf8(vec).unwrap();

            tmp.trim_end().to_string()
        });

        println!();
        // println!("{:?}", vbios);
        println!("VBIOS info");
        println!("name: [{name}]");
        println!("pn: [{pn}]");
        println!("ver_str: [{ver_str}]");
        println!("date: [{date}]");
    }

    if let Ok(vbios_size) = unsafe { amdgpu_dev.vbios_size(fd) } {
        println!("vbios size: {vbios_size}");
    }

    {
        use AMDGPU::SENSOR_INFO::*;

        let sensors = [
            SENSOR_TYPE::GFX_SCLK,
            SENSOR_TYPE::GFX_MCLK,
            SENSOR_TYPE::GPU_TEMP,
            SENSOR_TYPE::GPU_LOAD,
            SENSOR_TYPE::GPU_AVG_POWER,
            SENSOR_TYPE::VDDNB,
            SENSOR_TYPE::VDDGFX,
            SENSOR_TYPE::STABLE_PSTATE_GFX_SCLK,
            SENSOR_TYPE::STABLE_PSTATE_GFX_MCLK,
        ];

        println!();

        for s in &sensors {
            if let Ok(val) = amdgpu_dev.sensor_info(*s) {
                println!("{s:?}: {val}");
            } else {
                println!("{s:?}: not supported");
            }
        }
    }

    amdgpu_dev.deinit().unwrap();
}
