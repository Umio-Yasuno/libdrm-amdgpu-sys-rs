use libdrm_amdgpu_sys::*;

fn main() {
    let fd = {
        use std::fs::File;
        use std::os::unix::io::IntoRawFd;

        let v = File::open("/dev/dri/renderD128").unwrap();

        v.into_raw_fd()
    };

    let amdgpu_dev = AMDGPU::DeviceHandle::init(fd).unwrap();

    {
        // let gpu_info = amdgpu_dev.query_gpu_info().unwrap();
        let ext_info = amdgpu_dev.device_info().unwrap();

        // println!("{gpu_info:?}");
        println!();
        println!("{ext_info:#?}");

        use libdrm_amdgpu_sys::AMDGPU::GPU_INFO;

        let mark_name = ext_info.parse_amdgpu_ids().unwrap();

        println!();
        println!("Marketing Name: [{mark_name}]");
        println!(
            "DeviceID.RevID: {:#0X}.{:#0X}",
            ext_info.device_id(),
            ext_info.pci_rev_id()
        );

        let family = ext_info.get_family_name();
        let asic_name = ext_info.get_asic_name();
        let chip_class = ext_info.get_chip_class();

        println!();
        println!("Family: {family}");
        println!("ASIC Name: {asic_name}");
        println!("Chip class: {chip_class}");

        let cu = ext_info.cu_active_number();
        let max_engine_clock = ext_info.max_engine_clock();
        let peak_gflops = ext_info.peak_gflops();

        println!();
        println!("CU: {cu}");
        println!("Max Engine Clock: {} MHz", max_engine_clock / 1000);
        println!("Peak FP32: {peak_gflops} GFLOPS");

        let vram_type = ext_info.get_vram_type();
        let peak_bw = ext_info.peak_memory_bw_gb();
        let l2c_size = ext_info.calc_l2_cache_size() / 1024;

        println!();
        println!("VRAM Type: {vram_type}");
        println!("VRAM Bit Width: {}-bit", ext_info.vram_bit_width);
        println!("Peak Memory BW: {peak_bw} GB/s");
        println!("L2cache: {l2c_size} KiB");
    }

    {
        let memory_info = amdgpu_dev.memory_info().unwrap();
        let vram_size_mb = memory_info.vram.total_heap_size / 1024 / 1024;
        println!("VRAM size: {vram_size_mb} MiB");
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

            if ver == 0 {
                continue;
            }

            println!("{fw_type} FW:\n   ver: {ver:>#10X}, feature: {ftr:>3}");
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

        let [dec, enc] = [CAP_TYPE::DECODE, CAP_TYPE::ENCODE]
            .map(|type_| amdgpu_dev.get_video_caps(type_).unwrap());

        println!();

        for codec in &codec_list {
            let [dec_cap, enc_cap] =
                [dec, enc].map(|type_| type_.get_codec_info(*codec).is_supported());

            println!(
                "{:<12} decode: {dec_cap:>5}, encode: {enc_cap:>5}",
                codec.to_string()
            );
        }
    }

    {
        let bus_info = PCI::BUS_INFO::drm_get_device2(fd).unwrap();
        println!();
        println!("PCI: {bus_info}");
        println!("{:?}", bus_info.get_link_info(PCI::STATUS::Max));
    }

    {
        let vbios = unsafe { amdgpu_dev.vbios_info(fd).unwrap() };

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

        let vbios_size = unsafe { amdgpu_dev.vbios_size(fd).unwrap() };
        println!("vbios size: {vbios_size}");
    }

    {
        use libdrm_amdgpu_sys::AMDGPU::SENSOR_INFO::SENSOR_TYPE::*;

        let sensors = [
            GFX_SCLK,
            GFX_MCLK,
            GPU_TEMP,
            GPU_LOAD,
            GPU_AVG_POWER,
            VDDNB,
            VDDGFX,
            STABLE_PSTATE_GFX_SCLK,
            STABLE_PSTATE_GFX_MCLK,
        ];

        println!();

        for s in &sensors {
            let val = match amdgpu_dev.sensor_info(*s) {
                Ok(val) => val,
                Err(_) => {
                    println!("{s:?}: not supported");
                    continue;
                },
            };

            println!("{s:?}: {val}");
        }
    }

    amdgpu_dev.deinit().unwrap();
}
