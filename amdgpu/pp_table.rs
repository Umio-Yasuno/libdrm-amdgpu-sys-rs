use core::mem::{size_of, MaybeUninit};
use core::ptr;
use crate::bindings::atom_common_table_header;
pub use crate::bindings::ppt::{
    smu_v11_0_0_ppt::{smu_11_0_powerplay_table, PPTable_t as PPTable_smu_11_0_t},
    smu_v11_0_7_ppt::{smu_11_0_7_powerplay_table, PPTable_t as PPTable_smu_11_0_7_t, PPTable_beige_goby_t},
    smu_v13_0_0_ppt::{smu_13_0_0_powerplay_table, PPTable_t as PPTable_smu_13_0_0_t},
    smu_v13_0_7_ppt::{smu_13_0_7_powerplay_table, PPTable_t as PPTable_smu_13_0_7_t},
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PPTableDecodeError {
    SmallerThanHeader,
    SmallerThanStructureSizeInHeader,
}

#[derive(Debug, Clone)]
pub enum PPTable {
    V11_0_0(smu_11_0_powerplay_table),
    V11_0_7(smu_11_0_7_powerplay_table),
    V13_0_0(smu_13_0_0_powerplay_table),
    V13_0_7(smu_13_0_7_powerplay_table),
    Unknown(atom_common_table_header),
    Invalid,
}

impl PPTable {
    fn get_header(bytes: &[u8]) -> Option<atom_common_table_header> {
        const HEADER_SIZE: usize = size_of::<atom_common_table_header>();

        let Some(bin) = bytes.get(0..HEADER_SIZE) else {
            if cfg!(debug_assertions) {
                println!("The binary is smaller than header size.");
            }

            return None;
        };

        unsafe {
            let mut h = MaybeUninit::<atom_common_table_header>::zeroed();

            ptr::copy_nonoverlapping(
                bin.as_ptr(),
                h.as_mut_ptr() as *mut u8,
                HEADER_SIZE,
            );

            Some(h.assume_init())
        }
    }

    fn check_length(header: &atom_common_table_header, len: usize) -> bool {
        let b = header.structuresize as usize <= len;

        debug_assert!(
            b,
            "header.structuresize ({:?}) <= bytes ({})",
            header,
            len,
        );

        b
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, PPTableDecodeError> {
        let Some(header) = Self::get_header(bytes) else { return Err(PPTableDecodeError::SmallerThanHeader) };

        if !Self::check_length(&header, bytes.len()) {
            return Err(PPTableDecodeError::SmallerThanStructureSizeInHeader);
        }

        // ref: https://github.com/sibradzic/upp/blob/master/src/upp/decode.py
        let ppt = match header.format_revision {
            // Navi10: 12
            // Navi12: 14
            // Navi14: 12?
            12 | 14 => Self::V11_0_0(Self::to_pptable(&bytes)),
            // Navi21: 15
            // Navi22: 16?
            // Navi23: 18
            // Navi24: 19?
            15 | 16 | 18 | 19 => Self::V11_0_7(Self::to_pptable(&bytes)),
            // Navi31: 20
            // Navi32: ?
            // Navi33: ?
            20 => Self::V13_0_0(Self::to_pptable(&bytes)),
            _ => Self::Unknown(header),
        };

        Ok(ppt)
    }

    pub fn decode_with_smu_version(bytes: &[u8], smu_ver: (u8, u8, u8)) -> Result<Self, PPTableDecodeError> {
        let Some(header) = Self::get_header(bytes) else { return Err(PPTableDecodeError::SmallerThanHeader) };

        if !Self::check_length(&header, bytes.len()) {
            return Err(PPTableDecodeError::SmallerThanStructureSizeInHeader);
        }

        // ref: https://github.com/sibradzic/upp/blob/master/src/upp/decode.py
        let ppt = match smu_ver {
            (11, 0, 0) | /* Navi10 */
            (11, 0, 5) | /* Navi14 */
            (11, 0, 9) /* Navi12 */
                => Self::V11_0_0(Self::to_pptable(&bytes)),
            (11, 0, 7) | /* Navi21 */
            (11, 0, 11) | /* Navi22 */
            (11, 0, 12) | /* Navi23 */
            (11, 0, 13) /* Navi24 */
                => Self::V11_0_7(Self::to_pptable(&bytes)),
            (13, 0, 0) |
            (13, 0, 10) => Self::V13_0_0(Self::to_pptable(&bytes)),
            (13, 0, 7) => Self::V13_0_7(Self::to_pptable(&bytes)),
            _ => Self::Unknown(header),
        };

        Ok(ppt)
    }

    fn to_pptable<T>(bytes: &[u8]) -> T {
        unsafe {
            let mut t = MaybeUninit::<T>::zeroed();

            ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                t.as_mut_ptr() as *mut u8,
                size_of::<T>(),
            );

            t.assume_init()
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            Self::Unknown(_) |
            Self::Invalid => false,
            _ => true,
        }
    }
}
