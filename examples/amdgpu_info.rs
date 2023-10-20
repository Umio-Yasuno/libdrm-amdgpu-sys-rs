use libdrm_amdgpu_sys::*;

fn info(pci_bus: &PCI::BUS_INFO) {
    let Ok(device_path) = pci_bus.get_drm_render_path() else { return };
    let (amdgpu_dev, _major, _minor) = {
        use std::fs::File;
        use std::os::fd::IntoRawFd;

        let fd = File::open(device_path).unwrap();

        AMDGPU::DeviceHandle::init(fd.into_raw_fd()).unwrap()
    };

    if let Ok(drm_ver) = amdgpu_dev.get_drm_version_struct() {
        println!("{drm_ver:#?}");
    }

    if let Ok(ext_info) = amdgpu_dev.device_info() {
        use AMDGPU::GPU_INFO;

        println!("Marketing Name: [{}]", ext_info.find_device_name_or_default());
        // println!("\n{ext_info:#X?}\n");
        let gpu_type = if ext_info.is_apu() { "APU" } else { "dGPU" };
        let asic = ext_info.get_asic_name();

        println!(
            "DeviceID.RevID: {:#0X}.{:#0X}",
            ext_info.device_id(),
            ext_info.pci_rev_id()
        );

        println!();
        println!("Family:\t\t{}", ext_info.get_family_name());
        println!("ASIC Name:\t{asic}");
        println!("Chip class:\t{}", ext_info.get_chip_class());
        println!("GPU Type:\t{gpu_type}");

        if let Some(gfx_ver) = ext_info.get_gfx_target_version() {
            println!("gfx_target_version: {gfx_ver}");
        }

        let max_good_cu_per_sa = ext_info.get_max_good_cu_per_sa();
        let min_good_cu_per_sa = ext_info.get_min_good_cu_per_sa();

        println!();
        println!("Shader Engine (SE):\t\t{:3}", ext_info.max_se());
        println!("Shader Array (SA/SH) per SE:\t{:3}", ext_info.max_sa_per_se());
        if max_good_cu_per_sa != min_good_cu_per_sa {
            println!("CU per SA[0]:\t\t\t{:3}", max_good_cu_per_sa);
            println!("CU per SA[1]:\t\t\t{:3}", min_good_cu_per_sa);
        } else {
            println!("CU per SA:\t\t\t{:3}", max_good_cu_per_sa);
        }
        println!("Total Compute Unit:\t\t{:3}", ext_info.cu_active_number());

        if let Some((min, max)) = amdgpu_dev.get_min_max_gpu_clock() {
            println!("Engine Clock:\t\t{min}-{max} MHz");
        }

        println!("Peak FP32:\t\t{} GFLOPS", ext_info.peak_gflops());

        println!();
        println!("VRAM Type:\t\t{}", ext_info.get_vram_type());
        println!("VRAM Bit Width:\t\t{}-bit", ext_info.vram_bit_width);

        if let Some((min, max)) = amdgpu_dev.get_min_max_memory_clock() {
            println!("Memory Clock:\t\t{min}-{max} MHz");
        }

        println!("Peak Memory BW:\t\t{} GB/s", ext_info.peak_memory_bw_gb());

        println!();
        println!("L1cache (per CU):\t{:4} KiB", ext_info.get_l1_cache_size() >> 10);
        if 0 < ext_info.sqc_data_cache_size {
            println!("SQC Data Cache:\t\t{:4} KiB", ext_info.sqc_data_cache_size);
        }
        if 0 < ext_info.sqc_inst_cache_size {
            println!("SQC Inst Cache:\t\t{:4} KiB", ext_info.sqc_inst_cache_size);
        }
        let gl1_cache_size = ext_info.get_gl1_cache_size();
        let l3_cache_size = ext_info.calc_l3_cache_size_mb();
        if 0 < gl1_cache_size {
            println!("GL1cache (per SA/SH):\t{:4} KiB", gl1_cache_size >> 10);
        }
        if 0 < ext_info.gl1c_cache_size {
            println!("Total GL1cache:\t\t{gl1_cache_size:4} KiB");
        }
        println!(
            "L2cache:\t\t{:4} KiB ({} Banks)",
            ext_info.calc_l2_cache_size() >> 10,
            ext_info.get_actual_num_tcc_blocks(),
        );
        if 0 < l3_cache_size {
            println!("L3cache:\t\t{l3_cache_size:4} MiB");
        }
    }

    if let Ok(info) = amdgpu_dev.memory_info() {
        println!();
        println!(
            "VRAM Usage:\t\t\t{usage}/{total} MiB",
            usage = info.vram.heap_usage >> 20,
            total = info.vram.total_heap_size >> 20,
        );
        println!(
            "CPU Accessible VRAM Usage:\t{usage}/{total} MiB",
            usage = info.cpu_accessible_vram.heap_usage >> 20,
            total = info.cpu_accessible_vram.total_heap_size >> 20,
        );
        println!(
            "GTT Usage:\t\t\t{usage}/{total} MiB",
            usage = info.gtt.heap_usage >> 20,
            total = info.gtt.total_heap_size >> 20,
        );
        let re_bar = if info.check_resizable_bar() { "Enabled" } else { "Disabled" };
        println!("ResizableBAR:\t\t\t{re_bar}");

        if let Ok(moved) = amdgpu_dev.num_bytes_moved() {
            println!("Number of bytes moved for TTM migration: {moved}");
        }

        if let Ok(fault_count) = amdgpu_dev.num_vram_cpu_page_faults() {
            println!("Number of VRAM page faults on CPU access: {fault_count}");
        }

        if let Ok(e) = amdgpu_dev.num_evictions() {
            println!("Number of TTM buffer evictions: {e}");
        }

        if let Ok(lost) = amdgpu_dev.vram_lost_counter() {
            println!("VRAM lost counter: {lost}");
        }

        if let Ok(ras) = amdgpu_dev.ras_enabled_features() {
            use AMDGPU::RasBlock;

            println!("ECC Memory: {}", if ras.is_supported(RasBlock::UMC) {
                "supported"
            } else {
                "not supported"
            });
        }
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
            HW_IP_TYPE::VPE,
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
                    "{ip_type:8} count: {ip_count}, ver: {major:2}.{minor}, queues: {queues}",
                    ip_type = ip_type.to_string(),
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
            FW_TYPE::VPE,
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

            println!(
                "{fw_type:<8} ver: {ver:>#10X}, feature: {ftr:>3}",
                fw_type = fw_type.to_string(),
            );
        }
    }

    {
        use AMDGPU::VIDEO_CAPS::CAP_TYPE;

        println!("\nVideo caps:");
        if let Ok(codec_info) = amdgpu_dev.get_video_caps_info(CAP_TYPE::DECODE) {
            println!("{codec_info:#?}");
        }
        if let Ok(codec_info) = amdgpu_dev.get_video_caps_info(CAP_TYPE::ENCODE) {
            println!("{codec_info:#?}");
        }
    }

    if let Ok(bus_info) = amdgpu_dev.get_pci_bus_info() {
        println!("\nPCI (domain:bus:dev.func): {bus_info}");

        if let Ok(render) = bus_info.get_drm_render_path() {
            println!("Render: {render:?}");
        }
        if let Ok(card) = bus_info.get_drm_card_path() {
            println!("Card: {card:?}");
        }
    }

    if let Some([min, max]) = amdgpu_dev.get_min_max_link_info_from_dpm() {
        println!(
            "PCIe Link Speed     (DPM)    : Gen{}x{} - Gen{}x{}",
            min.gen,
            min.width,
            max.gen,
            max.width,
        );

        if let Some(max_gpu_link) = amdgpu_dev.get_max_gpu_link() {
            println!(
                "PCIe Link Speed (GPU, Max)   : Gen{}x{}",
                max_gpu_link.gen,
                max_gpu_link.width,
            );
        }

        if let Some(max_system_link) = amdgpu_dev.get_max_system_link() {
            println!(
                "PCIe Link Speed (System, Max): Gen{}x{}",
                max_system_link.gen,
                max_system_link.width,
            );
        }
    } else {
        println!("PCIe Link Speed     (DPM)    : None");
    }

    if let Ok(vbios) = amdgpu_dev.get_vbios_info() {
        println!("\nVBIOS info:");
        println!("name: [{}]", vbios.name);
        println!("pn: [{}]", vbios.pn);
        println!("ver: [{}]", vbios.ver);
        println!("date: [{}]", vbios.date);
    }

/*
    if let Ok(vce_clock) = amdgpu_dev.vce_clock_info() {
        println!("\n{vce_clock:#?}");
    }
*/

    {
        use AMDGPU::SENSOR_INFO::*;

        let sensors = [
            SENSOR_TYPE::GFX_SCLK,
            SENSOR_TYPE::GFX_MCLK,
            SENSOR_TYPE::GPU_TEMP,
            SENSOR_TYPE::GPU_LOAD,
            SENSOR_TYPE::GPU_AVG_POWER,
            SENSOR_TYPE::GPU_INPUT_POWER,
            SENSOR_TYPE::VDDNB,
            SENSOR_TYPE::VDDGFX,
            SENSOR_TYPE::STABLE_PSTATE_GFX_SCLK,
            SENSOR_TYPE::STABLE_PSTATE_GFX_MCLK,
            SENSOR_TYPE::PEAK_PSTATE_GFX_SCLK,
            SENSOR_TYPE::PEAK_PSTATE_GFX_MCLK,
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

    if let Ok(sysfs) = amdgpu_dev.get_sysfs_path() {
        println!("sysfs: {sysfs:?}");

        use AMDGPU::{DpmForcedLevel, PowerProfile};

        let profiles: Vec<String> = PowerProfile::get_all_supported_profiles_from_sysfs(&sysfs)
            .iter()
            .map(|p| p.to_string())
            .collect();

        println!("Supported Power Profiles: {profiles:?}");

        if let Some(profiles) = PowerProfile::get_current_profile_from_sysfs(&sysfs) {
            println!("Current Power Profiles: {profiles}");
        }

        if let Ok(level) = DpmForcedLevel::get_from_sysfs(&sysfs) {
            println!("power_dpm_force_performance_level: {level:?}");
        }

        use AMDGPU::{RasErrorCount, RasBlock};

        if let Ok(cnt) = RasErrorCount::get_from_sysfs_with_ras_block(&sysfs, RasBlock::UMC) {
            println!(
                "Memory Error Count: uncorrected {}, corrected {}",
                cnt.uncorrected,
                cnt.corrected,
            );
        }
    }

    if let Some(hwmon) = amdgpu_dev.get_hwmon_path() {
        println!("hwmon: {hwmon:?}");

        use AMDGPU::{HwmonTemp, HwmonTempType, PowerCap};
        if let Some(power_cap) = PowerCap::from_hwmon_path(&hwmon) {
            let PowerCap { type_, current, default, min, max } = power_cap;
            println!("PowerCap ({type_}): {current} W (Current), {default} W (Default), {min}-{max} W (Range)");
        }
        if let Some(edge_temp) = HwmonTemp::from_hwmon_path(&hwmon, HwmonTempType::Edge) {
            println!("{edge_temp:?}");
        }
        if let Some(junction_temp) = HwmonTemp::from_hwmon_path(&hwmon, HwmonTempType::Junction) {
            println!("{junction_temp:?}");
        }
        if let Some(mem_temp) = HwmonTemp::from_hwmon_path(&hwmon, HwmonTempType::Memory) {
            println!("{mem_temp:?}");
        }
    }

    println!();
}

fn main() {
    let pci_devs = AMDGPU::get_all_amdgpu_pci_bus();

    if pci_devs.is_empty() {
        panic!("No AMDGPU devices.");
    }

    for pci_bus in &pci_devs {
        info(pci_bus);
    }
}
