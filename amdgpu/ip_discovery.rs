use std::path::PathBuf;

/* ref: drivers/gpu/drm/amd/amdgpu/amdgpu_discovery.c */
#[derive(Debug, Clone)]
pub struct IpHwInstance {
    pub hw_id: HwId,
    pub num_instance: u8,
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
    pub harvest: u8,
    pub num_base_addresses: isize,
    pub base_address: Vec<u32>,
}

impl IpHwInstance {
    /// `ip_discovery/die/#die/#hw_id/#instance/`
    pub fn get_from_instance_sysfs<P: Into<PathBuf>>(sysfs_path: P) -> Option<Self> {
        let path = sysfs_path.into();
        let hw_id = {
            let tmp = Self::parse_file::<isize>(path.join("hw_id"))?;
            HwId::from(tmp)
        };
        let num_instance = Self::parse_file::<u8>(path.join("num_instance"))?;
        let major = Self::parse_file::<u8>(path.join("major"))?;
        let minor = Self::parse_file::<u8>(path.join("minor"))?;
        let revision = Self::parse_file::<u8>(path.join("revision"))?;
        let harvest = Self::parse_harvest_file(&path)?;
        let num_base_addresses = Self::parse_file::<isize>(path.join("num_base_addresses"))?;
        let base_address = Self::parse_base_address_file(&path);

        Some(Self {
            hw_id,
            num_instance,
            major,
            minor,
            revision,
            harvest,
            num_base_addresses,
            base_address,
        })
    }

    pub fn parse_harvest_file(sysfs_path: &PathBuf) -> Option<u8> {
        let s = std::fs::read_to_string(sysfs_path.join("harvest")).ok()?;
        let len = s.len();

        if len < 2 { return None }

        u8::from_str_radix(&s[2..len-1], 16).ok() // "0x0\n"
    }

    pub fn parse_base_address_file(sysfs_path: &PathBuf) -> Vec<u32> {
        let mut base_addr = Vec::with_capacity(8);
        let Ok(s) = std::fs::read_to_string(sysfs_path.join("base_addr")) else { return base_addr };
        let lines = s.lines();

        for l in lines {
            if l.len() < 2 { return base_addr }

            if let Ok(addr) = u32::from_str_radix(&l[2..], 16) { // "0x0000"
                base_addr.push(addr);
            }
        }

        base_addr
    }

    pub fn parse_file<T: std::str::FromStr>(path: PathBuf) -> Option<T> {
        let s = std::fs::read_to_string(path).ok()?;

        if s.is_empty() { return None }

        let len = s.len();

        s[..len-1].parse::<T>().ok()
    }
}

#[derive(Debug, Clone)]
pub struct IpHwId {
    pub hw_id: HwId,
    pub instances: Vec<IpHwInstance>,
}

impl IpHwId {
    /// `ip_discovery/die/#die/#hw_id/`
    pub fn get_from_ip_hw_sysfs(hw_id: HwId, ip_hw_path: &PathBuf) -> Result<Self, std::io::Error> {
        let inst_count = std::fs::read_dir(&ip_hw_path)?.count(); // use count for the order

        Ok(IpHwId {
            hw_id,
            instances: (0..inst_count).filter_map(|i| {
                let path = ip_hw_path.join(i.to_string());
                IpHwInstance::get_from_instance_sysfs(&path)
            }).collect(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IpDieEntry {
    pub die_id: usize,
    pub ip_hw_ids: Vec<IpHwId>,
}

impl IpDieEntry {
    pub fn get_all_entries_from_sysfs<P: Into<PathBuf>>(sysfs_path: P) -> Vec<Self> {
        let path = sysfs_path.into().join("ip_discovery/die/");
        let Ok(dies) = std::fs::read_dir(&path) else { return Vec::new() };
        let die_count = dies.count(); // use count for the order

        (0..die_count).filter_map(|die_id| {
            Self::get_from_sysfs_with_die_id(die_id, &path)
        }).collect()
    }

    /// `ip_discovery/die/#die/`
    pub fn get_from_die_sysfs(sysfs_path: &PathBuf) -> Option<Self> {
        let die_id: usize = sysfs_path.file_name()?.to_str()?.parse().ok()?;
        Self::get_from_sysfs_with_die_id(die_id, &sysfs_path.join("../"))
    }

    pub fn get_from_sysfs_with_die_id(die_id: usize, sysfs_path: &PathBuf) -> Option<Self> {
        let die_path = sysfs_path.join(die_id.to_string());
        let die = std::fs::read_dir(&die_path).ok()?;
        let mut ip_hw: Vec<isize> = die.filter_map(|ip_hw| {
            ip_hw.ok()?.file_name().to_str()?.parse().ok()
        }).collect();

        ip_hw.sort();

        let ip_hw_ids = ip_hw.iter().filter_map(|hw_id| {
            let ip_hw_path = die_path.join(hw_id.to_string());
            IpHwId::get_from_ip_hw_sysfs(HwId::from(*hw_id), &ip_hw_path).ok()
        }).collect();

        Some(Self{ die_id, ip_hw_ids })
    }
}

/* ref: drivers/gpu/drm/amd/include/soc15_hw_ip.h */
const MP1_HWID: isize = 1;
const MP2_HWID: isize = 2;
const THM_HWID: isize = 3;
const SMUIO_HWID: isize = 4;
const FUSE_HWID: isize = 5;
const CLKA_HWID: isize = 6;
const PWR_HWID: isize = 10;
const GC_HWID: isize = 11;
const UVD_HWID: isize = 12;
// const VCN_HWID: isize = UVD_HWID;
const AUDIO_AZ_HWID: isize = 13;
const ACP_HWID: isize = 14;
const DCI_HWID: isize = 15;
const DMU_HWID: isize = 271;
const DCO_HWID: isize = 16;
const DIO_HWID: isize = 272;
const XDMA_HWID: isize = 17;
const DCEAZ_HWID: isize = 18;
const DAZ_HWID: isize = 274;
const SDPMUX_HWID: isize = 19;
const NTB_HWID: isize = 20;
const IOHC_HWID: isize = 24;
const L2IMU_HWID: isize = 28;
const VCE_HWID: isize = 32;
const MMHUB_HWID: isize = 34;
const ATHUB_HWID: isize = 35;
const DBGU_NBIO_HWID: isize = 36;
const DFX_HWID: isize = 37;
const DBGU0_HWID: isize = 38;
const DBGU1_HWID: isize = 39;
const OSSSYS_HWID: isize = 40;
const HDP_HWID: isize = 41;
const SDMA0_HWID: isize = 42;
const SDMA1_HWID: isize = 43;
const ISP_HWID: isize = 44;
const DBGU_IO_HWID: isize = 45;
const DF_HWID: isize = 46;
const CLKB_HWID: isize = 47;
const FCH_HWID: isize = 48;
const DFX_DAP_HWID: isize = 49;
const L1IMU_PCIE_HWID: isize = 50;
const L1IMU_NBIF_HWID: isize = 51;
const L1IMU_IOAGR_HWID: isize = 52;
const L1IMU3_HWID: isize = 53;
const L1IMU4_HWID: isize = 54;
const L1IMU5_HWID: isize = 55;
const L1IMU6_HWID: isize = 56;
const L1IMU7_HWID: isize = 57;
const L1IMU8_HWID: isize = 58;
const L1IMU9_HWID: isize = 59;
const L1IMU10_HWID: isize = 60;
const L1IMU11_HWID: isize = 61;
const L1IMU12_HWID: isize = 62;
const L1IMU13_HWID: isize = 63;
const L1IMU14_HWID: isize = 64;
const L1IMU15_HWID: isize = 65;
const WAFLC_HWID: isize = 66;
const FCH_USB_PD_HWID: isize = 67;
const SDMA2_HWID: isize = 68;
const SDMA3_HWID: isize = 69;
const PCIE_HWID: isize = 70;
const PCS_HWID: isize = 80;
const DDCL_HWID: isize = 89;
const SST_HWID: isize = 90;
const LSDMA_HWID: isize = 91;
const IOAGR_HWID: isize = 100;
const NBIF_HWID: isize = 108;
const IOAPIC_HWID: isize = 124;
const SYSTEMHUB_HWID: isize = 128;
const NTBCCP_HWID: isize = 144;
const UMC_HWID: isize = 150;
const SATA_HWID: isize = 168;
const USB_HWID: isize = 170;
const CCXSEC_HWID: isize = 176;
const XGMI_HWID: isize = 200;
const XGBE_HWID: isize = 216;
const MP0_HWID: isize = 255;

#[derive(Debug, Clone, Copy)]
#[repr(isize)]
pub enum HwId {
    MP1 = self::MP1_HWID,
    MP2 = self::MP2_HWID,
    THM = self::THM_HWID,
    SMUIO = self::SMUIO_HWID,
    FUSE = self::FUSE_HWID,
    CLKA = self::CLKA_HWID,
    PWR = self::PWR_HWID,
    GC = self::GC_HWID,
    UVD = self::UVD_HWID,
    // VCN = self::VCN_HWID,
    AUDIO_AZ = self::AUDIO_AZ_HWID,
    ACP = self::ACP_HWID,
    DCI = self::DCI_HWID,
    DMU = self::DMU_HWID,
    DCO = self::DCO_HWID,
    DIO = self::DIO_HWID,
    XDMA = self::XDMA_HWID,
    DCEAZ = self::DCEAZ_HWID,
    DAZ = self::DAZ_HWID,
    SDPMUX = self::SDPMUX_HWID,
    NTB = self::NTB_HWID,
    IOHC = self::IOHC_HWID,
    L2IMU = self::L2IMU_HWID,
    VCE = self::VCE_HWID,
    MMHUB = self::MMHUB_HWID,
    ATHUB = self::ATHUB_HWID,
    DBGU_NBIO = self::DBGU_NBIO_HWID,
    DFX = self::DFX_HWID,
    DBGU0 = self::DBGU0_HWID,
    DBGU1 = self::DBGU1_HWID,
    OSSSYS = self::OSSSYS_HWID,
    HDP = self::HDP_HWID,
    SDMA0 = self::SDMA0_HWID,
    SDMA1 = self::SDMA1_HWID,
    ISP = self::ISP_HWID,
    DBGU_IO = self::DBGU_IO_HWID,
    DF = self::DF_HWID,
    CLKB = self::CLKB_HWID,
    FCH = self::FCH_HWID,
    DFX_DAP = self::DFX_DAP_HWID,
    L1IMU_PCIE = self::L1IMU_PCIE_HWID,
    L1IMU_NBIF = self::L1IMU_NBIF_HWID,
    L1IMU_IOAGR = self::L1IMU_IOAGR_HWID,
    L1IMU3 = self::L1IMU3_HWID,
    L1IMU4 = self::L1IMU4_HWID,
    L1IMU5 = self::L1IMU5_HWID,
    L1IMU6 = self::L1IMU6_HWID,
    L1IMU7 = self::L1IMU7_HWID,
    L1IMU8 = self::L1IMU8_HWID,
    L1IMU9 = self::L1IMU9_HWID,
    L1IMU10 = self::L1IMU10_HWID,
    L1IMU11 = self::L1IMU11_HWID,
    L1IMU12 = self::L1IMU12_HWID,
    L1IMU13 = self::L1IMU13_HWID,
    L1IMU14 = self::L1IMU14_HWID,
    L1IMU15 = self::L1IMU15_HWID,
    WAFLC = self::WAFLC_HWID,
    FCH_USB_PD = self::FCH_USB_PD_HWID,
    SDMA2 = self::SDMA2_HWID,
    SDMA3 = self::SDMA3_HWID,
    PCIE = self::PCIE_HWID,
    PCS = self::PCS_HWID,
    DDCL = self::DDCL_HWID,
    SST = self::SST_HWID,
    LSDMA = self::LSDMA_HWID,
    IOAGR = self::IOAGR_HWID,
    NBIF = self::NBIF_HWID,
    IOAPIC = self::IOAPIC_HWID,
    SYSTEMHUB = self::SYSTEMHUB_HWID,
    NTBCCP = self::NTBCCP_HWID,
    UMC = self::UMC_HWID,
    SATA = self::SATA_HWID,
    USB = self::USB_HWID,
    CCXSEC = self::CCXSEC_HWID,
    XGMI = self::XGMI_HWID,
    XGBE = self::XGBE_HWID,
    MP0 = self::MP0_HWID,
    Unknown(isize),
}

impl From<isize> for HwId {
    fn from(hw_id: isize) -> Self {
        match hw_id {
            self::MP1_HWID => Self::MP1,
            self::MP2_HWID => Self::MP2,
            self::THM_HWID => Self::THM,
            self::SMUIO_HWID => Self::SMUIO,
            self::FUSE_HWID => Self::FUSE,
            self::CLKA_HWID => Self::CLKA,
            self::PWR_HWID => Self::PWR,
            self::GC_HWID => Self::GC,
            self::UVD_HWID => Self::UVD,
            self::AUDIO_AZ_HWID => Self::AUDIO_AZ,
            self::ACP_HWID => Self::ACP,
            self::DCI_HWID => Self::DCI,
            self::DMU_HWID => Self::DMU,
            self::DCO_HWID => Self::DCO,
            self::DIO_HWID => Self::DIO,
            self::XDMA_HWID => Self::XDMA,
            self::DCEAZ_HWID => Self::DCEAZ,
            self::DAZ_HWID => Self::DAZ,
            self::SDPMUX_HWID => Self::SDPMUX,
            self::NTB_HWID => Self::NTB,
            self::IOHC_HWID => Self::IOHC,
            self::L2IMU_HWID => Self::L2IMU,
            self::VCE_HWID => Self::VCE,
            self::MMHUB_HWID => Self::MMHUB,
            self::ATHUB_HWID => Self::ATHUB,
            self::DBGU_NBIO_HWID => Self::DBGU_NBIO,
            self::DFX_HWID => Self::DFX,
            self::DBGU0_HWID => Self::DBGU0,
            self::DBGU1_HWID => Self::DBGU1,
            self::OSSSYS_HWID => Self::OSSSYS,
            self::HDP_HWID => Self::HDP,
            self::SDMA0_HWID => Self::SDMA0,
            self::SDMA1_HWID => Self::SDMA1,
            self::ISP_HWID => Self::ISP,
            self::DBGU_IO_HWID => Self::DBGU_IO,
            self::DF_HWID => Self::DF,
            self::CLKB_HWID => Self::CLKB,
            self::FCH_HWID => Self::FCH,
            self::DFX_DAP_HWID => Self::DFX_DAP,
            self::L1IMU_PCIE_HWID => Self::L1IMU_PCIE,
            self::L1IMU_NBIF_HWID => Self::L1IMU_NBIF,
            self::L1IMU_IOAGR_HWID => Self::L1IMU_IOAGR,
            self::L1IMU3_HWID => Self::L1IMU3,
            self::L1IMU4_HWID => Self::L1IMU4,
            self::L1IMU5_HWID => Self::L1IMU5,
            self::L1IMU6_HWID => Self::L1IMU6,
            self::L1IMU7_HWID => Self::L1IMU7,
            self::L1IMU8_HWID => Self::L1IMU8,
            self::L1IMU9_HWID => Self::L1IMU9,
            self::L1IMU10_HWID => Self::L1IMU10,
            self::L1IMU11_HWID => Self::L1IMU11,
            self::L1IMU12_HWID => Self::L1IMU12,
            self::L1IMU13_HWID => Self::L1IMU13,
            self::L1IMU14_HWID => Self::L1IMU14,
            self::L1IMU15_HWID => Self::L1IMU15,
            self::WAFLC_HWID => Self::WAFLC,
            self::FCH_USB_PD_HWID => Self::FCH_USB_PD,
            self::SDMA2_HWID => Self::SDMA2,
            self::SDMA3_HWID => Self::SDMA3,
            self::PCIE_HWID => Self::PCIE,
            self::PCS_HWID => Self::PCS,
            self::DDCL_HWID => Self::DDCL,
            self::SST_HWID => Self::SST,
            self::LSDMA_HWID => Self::LSDMA,
            self::IOAGR_HWID => Self::IOAGR,
            self::NBIF_HWID => Self::NBIF,
            self::IOAPIC_HWID => Self::IOAPIC,
            self::SYSTEMHUB_HWID => Self::SYSTEMHUB,
            self::NTBCCP_HWID => Self::NTBCCP,
            self::UMC_HWID => Self::UMC,
            self::SATA_HWID => Self::SATA,
            self::USB_HWID => Self::USB,
            self::CCXSEC_HWID => Self::CCXSEC,
            self::XGMI_HWID => Self::XGMI,
            self::XGBE_HWID => Self::XGBE,
            self::MP0_HWID => Self::MP0,
            _ => Self::Unknown(hw_id),
        }
    }
}

use std::fmt;
impl fmt::Display for HwId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
