/// PCI information (Domain, Bus, Device, Function)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BUS_INFO {
    pub domain: u16,
    pub bus: u8,
    pub dev: u8,
    pub func: u8,
}

/// PCI link status
pub enum STATUS {
    Current,
    Max,
}

/// PCI link speed information
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct LINK {
    pub gen: u8,
    pub width: u8,
}

mod bus_info;
pub use bus_info::*;

mod status;
pub use status::*;

mod link;
pub use link::*;
