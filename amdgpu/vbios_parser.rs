use core::mem::{size_of, MaybeUninit};
use core::ptr;
pub use crate::bindings::{atom_common_table_header, atom_rom_header_v2_2, atom_master_data_table_v2_1, atom_firmware_info_v3_4};

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

    pub fn valid_vbios(&self) -> bool {
        let Some(p) = self.0.get(..2) else { return false };
        let Some(sig) = self.0.get(SIGNATURE_OFFSET..SIGNATURE_END) else { return false };

        p == VALID_VBIOS && sig == SIGNATURE
    }

    pub fn length(&self) -> usize {
        let Some(length) = self.0.get(2) else { return 0 };

        usize::from(*length) << 9
    }

    pub fn get_rom_table_ptr(&self) -> Option<usize> {
        self.read_u16(ROM_TABLE_PTR).map(|v| v as usize)
    }

    fn read_u16(&self, offset: usize) -> Option<u16> {
        self.0.get(offset..offset+2)
            .and_then(|r| r.try_into().ok())
            .map(|arr| u16::from_le_bytes(arr))
    }

    pub fn read_header(&self, offset: usize) -> Option<atom_common_table_header> {
        if offset == 0 { return None }

        let size = size_of::<atom_common_table_header>();
        let range = offset..offset+size;

        let h = self.0.get(range)?;

        unsafe {
            let mut header = MaybeUninit::<atom_common_table_header>::zeroed();

            ptr::copy_nonoverlapping(
                h.as_ptr(),
                header.as_mut_ptr() as *mut u8,
                size,
            );

            Some(header.assume_init())
        }
    }

    pub fn read_table<T>(&self, offset: usize) -> Option<T> {
        if offset == 0 { return None }

        let size = {
            let header = self.read_header(offset)?;
            header.structuresize as usize
        };
        let range = offset..offset+size;

        let t = self.0.get(range)?;

        unsafe {
            let mut table = MaybeUninit::<T>::zeroed();

            ptr::copy_nonoverlapping(
                t.as_ptr(),
                table.as_mut_ptr() as *mut u8,
                size,
            );

            Some(table.assume_init())
        }
    }

    pub fn get_atom_rom_header(&self) -> Option<atom_rom_header_v2_2> {
        let offset = self.read_u16(ROM_TABLE_PTR)?;

        let rom_header = self.read_table::<atom_rom_header_v2_2>(offset as usize)?;

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
}
