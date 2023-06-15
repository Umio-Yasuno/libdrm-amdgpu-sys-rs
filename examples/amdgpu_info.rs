use libdrm_amdgpu_sys::*;

fn main() {
    let device_path = std::env::var("AMDGPU_PATH").unwrap_or("/dev/dri/renderD128".to_string());
    let (amdgpu_dev, _major, _minor) = {
        use std::fs::File;
        use std::os::fd::IntoRawFd;

        let fd = File::open(device_path).unwrap();

        AMDGPU::DeviceHandle::init(fd.into_raw_fd()).unwrap()
    };

    if let Ok(drm_ver) = amdgpu_dev.get_drm_version_struct() {
        println!("{drm_ver:#?}");
    }

    println!("Marketing Name: [{}]", amdgpu_dev.get_marketing_name_or_default());

    if let Ok(ext_info) = amdgpu_dev.device_info() {
        use AMDGPU::GPU_INFO;

        // println!("\n{ext_info:#X?}\n");
        let gpu_type = if ext_info.is_apu() { "APU" } else { "dGPU" };

        println!(
            "DeviceID.RevID: {:#0X}.{:#0X}",
            ext_info.device_id(),
            ext_info.pci_rev_id()
        );

        println!();
        println!("Family:\t\t{}", ext_info.get_family_name());
        println!("ASIC Name:\t{}", ext_info.get_asic_name());
        println!("Chip class:\t{}", ext_info.get_chip_class());
        println!("GPU Type:\t{gpu_type}");

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
        let gl1_cache_size = ext_info.get_gl1_cache_size();
        let l3_cache_size = ext_info.calc_l3_cache_size_mb();
        if 0 < gl1_cache_size {
            println!("GL1cache (per SA/SH):\t{gl1_cache_size:4} KiB");
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
        if let Some(cur) = bus_info.get_current_link_info_from_dpm() {
            println!("PCI Link Speed (Current) : Gen{}x{}", cur.gen, cur.width);
        }
        if let Some([min, max]) = bus_info.get_min_max_link_info_from_dpm() {
            println!(
                "PCI Link Speed           : Gen{}x{} - Gen{}x{}",
                min.gen,
                min.width,
                max.gen,
                max.width,
            );
        }
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

    if let Ok(sysfs) = amdgpu_dev.get_sysfs_path() {
        println!("sysfs: {sysfs:?}");
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
}
