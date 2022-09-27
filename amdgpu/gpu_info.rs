use crate::*;
use crate::bindings::{amdgpu_gpu_info, drm_amdgpu_info_device};

pub trait GPU_INFO {
    fn family_id(&self) -> u32;
    fn chip_external_rev(&self) -> u32;
    fn device_id(&self) -> u32;
    fn pci_rev_id(&self) -> u32;
    fn vram_type(&self) -> u32;
    fn vram_bit_width(&self) -> u32;
    fn max_memory_clock(&self) -> u64;
    fn max_engine_clock(&self) -> u64;
    fn ids_flags(&self) -> u64;
    fn rb_pipes(&self) -> u32;
    fn cu_active_number(&self) -> u32;

    fn get_family_name(&self) -> AMDGPU::FAMILY_NAME {
        AMDGPU::FAMILY_NAME::from_id(self.family_id())
    }
    fn get_asic_name(&self) -> AMDGPU::ASIC_NAME {
        self.get_family_name().asic_name(self.chip_external_rev())
    }
    fn get_chip_class(&self) -> AMDGPU::CHIP_CLASS {
        self.get_asic_name().chip_class()
    }
    fn get_vram_type(&self) -> AMDGPU::VRAM_TYPE {
        AMDGPU::VRAM_TYPE::from_type_id(self.vram_type())
    }
    fn is_apu(&self) -> bool {
        use crate::bindings::{
            AMDGPU_IDS_FLAGS_FUSION,
            // AMDGPU_IDS_FLAGS_PREEMPTION,
            // AMDGPU_IDS_FLAGS_TMZ,
        };

        return (self.ids_flags() & AMDGPU_IDS_FLAGS_FUSION as u64) != 0;
    }
    fn peak_memory_bw(&self) -> u64 {
        let vram_type = self.get_vram_type();

        vram_type.peak_bw(self.max_memory_clock(), self.vram_bit_width())
    }
    fn peak_memory_bw_gb(&self) -> u64 {
        self.peak_memory_bw() / 1000
    }
    fn calc_rop_count(&self) -> u32 {
        let rop_per_rb = if self.get_asic_name().rbplus_allowed() {
            8
        } else {
            4
        };

        return self.rb_pipes() * rop_per_rb;
    }
    fn peak_gflops(&self) -> u32 {
        /* [CU] * 64 [Lane] * 2 [ops] * [GHz] */
        (self.cu_active_number() as u64 * 64 * 2 * (self.max_engine_clock() / 1000) / 1000) as u32
    }
    fn parse_amdgpu_ids(&self, ids_path: &str) -> std::io::Result<String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let ids = File::open(ids_path)?;
        let reader = BufReader::new(ids);

        let did = self.device_id();
        let rid = self.pci_rev_id();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            let s: Vec<&str> = line.splitn(3, ",\t").collect();

            if s.len() < 3 {
                continue;
            }

            let s_did = match u32::from_str_radix(s[0], 16) {
                Ok(did) => did,
                Err(_) => continue,
            };
            let s_rid = u32::from_str_radix(s[1], 16).expect("Parse error: {s:?}");

            if did == s_did && rid == s_rid {
                return Ok(s[2].to_string());
            }
        }

        Ok("AMD Radeon Graphics".to_string())
    }
}

impl GPU_INFO for amdgpu_gpu_info {
    fn family_id(&self) -> u32 {
        self.family_id
    }
    fn chip_external_rev(&self) -> u32 {
        self.chip_external_rev
    }
    fn device_id(&self) -> u32 {
        self.asic_id
    }
    fn pci_rev_id(&self) -> u32 {
        self.pci_rev_id
    }
    fn vram_type(&self) -> u32 {
        self.vram_type
    }
    fn vram_bit_width(&self) -> u32 {
        self.vram_bit_width
    }
    fn max_memory_clock(&self) -> u64 {
        self.max_memory_clk
    }
    fn max_engine_clock(&self) -> u64 {
        self.max_engine_clk
    }
    fn ids_flags(&self) -> u64 {
        self.ids_flags
    }
    fn rb_pipes(&self) -> u32 {
        self.rb_pipes
    }
    fn cu_active_number(&self) -> u32 {
        self.cu_active_number
    }
}

impl GPU_INFO for drm_amdgpu_info_device {
    fn family_id(&self) -> u32 {
        self.family
    }
    fn chip_external_rev(&self) -> u32 {
        self.external_rev
    }
    fn device_id(&self) -> u32 {
        self.device_id
    }
    fn pci_rev_id(&self) -> u32 {
        self.pci_rev
    }
    fn vram_type(&self) -> u32 {
        self.vram_type
    }
    fn vram_bit_width(&self) -> u32 {
        self.vram_bit_width
    }
    fn max_memory_clock(&self) -> u64 {
        self.max_memory_clock
    }
    fn max_engine_clock(&self) -> u64 {
        self.max_engine_clock
    }
    fn ids_flags(&self) -> u64 {
        self.ids_flags
    }
    fn rb_pipes(&self) -> u32 {
        self.num_rb_pipes
    }
    fn cu_active_number(&self) -> u32 {
        self.cu_active_number
    }
}

impl drm_amdgpu_info_device {
    pub fn calc_l2_cache_size(&self) -> u32 {
        self.num_tcc_blocks * self.get_asic_name().l2_cache_size_per_block()
    }
}
