use libdrm_amdgpu_sys::*;

fn main() {
    let fd = {
        use std::fs::File;
        use std::os::unix::io::IntoRawFd;

        let v = File::open("/dev/dri/renderD128").unwrap();

        v.into_raw_fd()
    };

    let amdgpu_dev = AMDGPU::DeviceHandle::init(fd).unwrap();

    if let Ok(mark_name) = amdgpu_dev.get_marketing_name() {
        println!("Marketing Name: [{mark_name}]");
    }

    if let Ok(ext_info) = amdgpu_dev.device_info() {
        use AMDGPU::GPU_INFO;

        // println!("\n{ext_info:#?}\n");

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

    if let Ok(info) = amdgpu_dev.memory_info() {
        println!(
            "VRAM Usage:\t\t\t{usage}/{total} MiB",
            usage = info.vram.heap_usage / 1024 / 1024,
            total = info.vram.total_heap_size / 1024 / 1024,
        );
        println!(
            "CPU Accessible VRAM Usage:\t{usage}/{total} MiB",
            usage = info.cpu_accessible_vram.heap_usage / 1024 / 1024,
            total = info.cpu_accessible_vram.total_heap_size / 1024 / 1024,
        );
        println!(
            "GTT Usage:\t\t\t{usage}/{total} MiB",
            usage = info.gtt.heap_usage / 1024 / 1024,
            total = info.gtt.total_heap_size / 1024 / 1024,
        );
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

        println!("\nHardware IP info:");

        for ip_type in &ip_list {
            if let (Ok(ip_info), Ok(ip_count)) = (
                amdgpu_dev.query_hw_ip_info(*ip_type, 0),
                amdgpu_dev.query_hw_ip_count(*ip_type),
            ) {
                let (major, minor) = ip_info.version();
                let queues = ip_info.num_queues();

                if queues == 0 {
                    continue;
                }

                println!(
                    "{:8} count: {ip_count}, ver: {major:2}.{minor}, queues: {queues}",
                    ip_type.to_string()
                );
            }
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

        println!("\nFirmware info:");

        for fw_type in &fw_list {
            let fw_info = match amdgpu_dev.query_firmware_version(*fw_type, 0, 0) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let (ver, ftr) = (fw_info.version, fw_info.feature);

            if ver == 0 {
                continue;
            }

            println!("{fw_type}:\n   ver: {ver:>#10X}, feature: {ftr:>3}");
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
        let cur = bus_info.get_link_info(PCI::STATUS::Current);
        let max = bus_info.get_link_info(PCI::STATUS::Max);

        println!("\nPCI (domain:bus:dev.func): {bus_info}");
        println!("Current Link: Gen{}x{}", cur.gen, cur.width);
        println!("Max     Link: Gen{}x{}", max.gen, max.width);
    }

    if let Ok(vbios) = unsafe { amdgpu_dev.vbios_info() } {
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
    }

    if let Ok(vce_clock) = amdgpu_dev.vce_clock_info() {
        println!("\n{vce_clock:#?}");
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

        println!("\nSensors:");

        for s in &sensors {
            if let Ok(val) = amdgpu_dev.sensor_info(*s) {
                println!("{s:?}: {val}");
            } else {
                println!("{s:?}: not supported");
            }
        }
    }
}
