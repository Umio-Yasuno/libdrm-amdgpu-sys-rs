pub mod PCI {
    #[derive(Debug, Clone, Copy)]
    pub struct BUS_INFO {
        pub domain: u16,
        pub bus: u8,
        pub dev: u8,
        pub func: u8,
    }
    pub enum STATUS {
        Current,
        Max,
    }
    #[derive(Debug, Clone)]
    pub struct LINK {
        pub gen: u8,
        pub gts: String,
        pub width: u8,
    }
}

impl PCI::BUS_INFO {
    pub fn drm_get_device2(
        fd: ::std::os::raw::c_int,
        //  flags: u32,
    ) -> Result<Self, i32> {
        unsafe {
            let dev_info = __drmGetDevice2(fd, 0)?;

            let bus_info = PCI::BUS_INFO {
                domain: (*(*dev_info).businfo.pci).domain,
                bus: (*(*dev_info).businfo.pci).bus,
                dev: (*(*dev_info).businfo.pci).dev,
                func: (*(*dev_info).businfo.pci).func,
            };

            return Ok(bus_info);
        }
    }
    pub fn get_link_info(&self, status: PCI::STATUS) -> PCI::LINK {
        /* TODO: use buffer */
        let (speed, width) = {
            use std::fs;
            use std::path::PathBuf;

            let status = match status {
                PCI::STATUS::Current => "current",
                PCI::STATUS::Max => "max",
            };
            let path = PathBuf::from(format!("/sys/bus/pci/devices/{}/", self));

            let file_name = [
                &format!("{status}_link_speed"),
                &format!("{status}_link_width"),
            ];

            (
                fs::read_to_string(path.join(file_name[0])).unwrap(),
                fs::read_to_string(path.join(file_name[1])).unwrap(),
            )
        };

        let speed = speed.trim();
        let width: u8 = width.trim().parse().unwrap();

        let gen = Self::speed_to_gen(speed);
        let gts = speed.to_string();

        return PCI::LINK {
            gen,
            gts,
            width,
        };
    }
    fn speed_to_gen(speed: &str) -> u8 {
        match speed {
            "2.5 GT/s PCIe" => 1,
            "5.0 GT/s PCIe" => 2,
            "8.0 GT/s PCIe" => 3,
            "16.0 GT/s PCIe" => 4,
            "32.0 GT/s PCIe" => 5,
            "64.0 GT/s PCIe" => 6,
            _ => 0,
        }
    }
}

use std::fmt;

impl fmt::Display for PCI::BUS_INFO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}:{:02}:{:02}.{:01}",
            self.domain, self.bus, self.dev, self.func)
    }
}

use std::mem::MaybeUninit;
use crate::{
    bindings::{
        drmDevicePtr,
        drmGetDevice2,
    },
    query_error,
};

unsafe fn __drmGetDevice2(
    fd: ::std::os::raw::c_int,
    flags: u32,
) -> Result<drmDevicePtr, i32> {
    let mut drm_dev_info: MaybeUninit<drmDevicePtr> = MaybeUninit::uninit();
    
    let r = drmGetDevice2(
        fd,
        flags,
        drm_dev_info.as_mut_ptr(),
    );

    query_error!(r);

    return Ok(drm_dev_info.assume_init());
}
