use crate::*;
use crate::bindings::amdgpu_gpu_info;

impl amdgpu_gpu_info {
    pub fn get_family_name(&self) -> AMDGPU::FAMILY_NAME {
        AMDGPU::FAMILY_NAME::from_id(self.family_id)
    }
    pub fn get_asic_name(&self) -> AMDGPU::ASIC_NAME {
        self.get_family_name().asic_name(self.chip_external_rev)
    }
    pub fn get_chip_class(&self) -> AMDGPU::CHIP_CLASS {
        self.get_asic_name().chip_class()
    }
    pub fn get_vram_type(&self) -> AMDGPU::VRAM_TYPE {
        AMDGPU::VRAM_TYPE::from_type_id(self.vram_type)
    }
    pub fn is_apu(&self) -> bool {
        use crate::bindings::{
            AMDGPU_IDS_FLAGS_FUSION,
            // AMDGPU_IDS_FLAGS_PREEMPTION,
            // AMDGPU_IDS_FLAGS_TMZ,
        };

        return (self.ids_flags & AMDGPU_IDS_FLAGS_FUSION as u64) != 0;
    }
    pub fn calc_peak_bw(&self) -> u64 {
        let vram_type = self.get_vram_type();

        vram_type.peak_bw(self.max_memory_clk, self.vram_bit_width)
    }
    pub fn calc_peak_bw_gb(&self) -> u64 {
        self.calc_peak_bw() / 1000
    }
    pub fn calc_rop_count(&self) -> u64 {
        let asic_name = self.get_asic_name();
        let rop_per_rb = if asic_name.rbplus_allowed() {
            8
        } else {
            4
        };

        return (self.rb_pipes as u64) * rop_per_rb;
    }
}
