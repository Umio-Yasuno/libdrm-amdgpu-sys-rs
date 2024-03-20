use core::mem::{size_of, MaybeUninit};
use core::ptr;
pub use crate::bindings::{atom_common_table_header, atom_rom_header_v2_2, atom_master_data_table_v2_1, atom_firmware_info_v3_4};

pub use crate::bindings::ppt::{
    smu_v11_0_0_ppt::{smu_11_0_powerplay_table, PPTable_t as PPTable_smu_11_0_t},
    smu_v11_0_7_ppt::{smu_11_0_7_powerplay_table, PPTable_t as PPTable_smu_11_0_7_t},
    smu_v13_0_0_ppt::{smu_13_0_0_powerplay_table, PPTable_t as PPTable_smu_13_0_0_t},
    smu_v13_0_7_ppt::{smu_13_0_7_powerplay_table, PPTable_t as PPTable_smu_13_0_7_t},
};

// ref: drivers/gpu/drm/amd/amdgpu/amdgpu_bios.c

const SIGNATURE: &[u8] = b" 761295520";
const SIGNATURE_OFFSET: usize = 0x30;
const SIGNATURE_END: usize = SIGNATURE_OFFSET + SIGNATURE.len();
const VALID_VBIOS: &[u8] = &[0x55, 0xAA];
const ROM_TABLE_PTR: usize = 0x48;

#[derive(Debug, Clone)]
pub struct VbiosParser(Vec<u8>);

impl VbiosParser {
    pub fn new(v: Vec<u8>) -> Self {
        Self(v)
    }

    pub fn vbios(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn length(&self) -> usize {
        let Some(length) = self.0.get(2) else { return 0 };

        usize::from(*length) << 9
    }

    pub fn valid_vbios(&self) -> bool {
        let Some(p) = self.0.get(..2) else { return false };
        let Some(sig) = self.0.get(SIGNATURE_OFFSET..SIGNATURE_END) else { return false };

        p == VALID_VBIOS && sig == SIGNATURE
    }

    pub fn check_length(&self) -> bool {
        self.length() == self.0.len()
    }

    fn to_struct<T>(bin: &[u8], size: usize) -> T {
        unsafe {
            let mut s = MaybeUninit::<T>::zeroed();

            ptr::copy_nonoverlapping(
                bin.as_ptr(),
                s.as_mut_ptr() as *mut u8,
                size,
            );

            s.assume_init()
        }
    }

    fn read_u16(&self, offset: usize) -> Option<u16> {
        self.0.get(offset..offset+2)
            .and_then(|r| r.try_into().ok())
            .map(|arr| u16::from_le_bytes(arr))
    }

    fn get_size_from_header(&self, offset: usize) -> Option<usize> {
        let size = self.read_header(offset)?.structuresize as usize;

        Some(size)
    }

    pub fn read_header(&self, offset: usize) -> Option<atom_common_table_header> {
        if offset == 0 { return None }

        let size = size_of::<atom_common_table_header>();
        let range = offset..offset+size;

        let h = self.0.get(range)?;

        Some(Self::to_struct(h, size))
    }

    fn read_table_unchecked_size<T>(&self, offset: usize) -> Option<T> {
        let size = self.get_size_from_header(offset)?;
        let t = self.0.get(offset..offset+size)?;

        Some(Self::to_struct(t, size))
    }

    pub fn read_table<T>(&self, offset: usize) -> Option<T> {
        let size = self.get_size_from_header(offset)?;

        if size != size_of::<T>() { return None }

        let t = self.0.get(offset..offset+size)?;

        Some(Self::to_struct(t, size))
    }

    pub fn get_atom_rom_header(&self) -> Option<atom_rom_header_v2_2> {
        let offset = self.read_u16(ROM_TABLE_PTR)?;

        /* only `atom_rom_header_v2_2` is defined */
        let rom_header = self.read_table_unchecked_size::<atom_rom_header_v2_2>(offset as usize)?;

        if &rom_header.atom_bios_string == b"ATOM" {
            Some(rom_header)
        } else {
            None
        }
    }

    pub fn get_atom_data_table(
        &self,
        rom_header: &atom_rom_header_v2_2,
    ) -> Option<atom_master_data_table_v2_1> {
        self.read_table(rom_header.masterdatatable_offset as usize)
    }

    pub fn get_atom_firmware_info(
        &self,
        data_table: &atom_master_data_table_v2_1,
    ) -> Option<atom_firmware_info_v3_4> {
        self.read_table(data_table.listOfdatatables.firmwareinfo as usize)
    }

    pub fn get_smu_11_0_7_powerplay_table(
        &self,
        data_table: &atom_master_data_table_v2_1,
    ) -> Option<smu_11_0_7_powerplay_table> {
        self.read_table(data_table.listOfdatatables.powerplayinfo as usize)
    }

    pub fn get_smu_13_0_0_powerplay_table(
        &self,
        data_table: &atom_master_data_table_v2_1,
    ) -> Option<smu_13_0_0_powerplay_table> {
        self.read_table(data_table.listOfdatatables.powerplayinfo as usize)
    }

    pub fn get_smu_13_0_7_powerplay_table(
        &self,
        data_table: &atom_master_data_table_v2_1,
    ) -> Option<smu_13_0_7_powerplay_table> {
        self.read_table(data_table.listOfdatatables.powerplayinfo as usize)
    }
}
