use crate::AMDGPU::DEVICE_HANDLE;
use crate::*;
#[cfg(feature = "dynamic_loading")]
use std::sync::Arc;
#[cfg(feature = "dynamic_loading")]
use bindings::{DynLibDrm, DynLibDrmAmdgpu};

use crate::bindings::drmDevicePtr;
pub use bindings::{
    amdgpu_device_handle,
    // amdgpu_device_initialize,
    amdgpu_gds_resource_info,
    amdgpu_gpu_info,
    drm_amdgpu_heap_info,
    drm_amdgpu_info_device,
    drm_amdgpu_info_gds,
    drm_amdgpu_info_vram_gtt,
    drm_amdgpu_memory_info,
    drm_amdgpu_info_vce_clock_table,
};
use bindings::{
    AMDGPU_INFO_NUM_BYTES_MOVED,
    AMDGPU_INFO_NUM_EVICTIONS,
    AMDGPU_INFO_VRAM_LOST_COUNTER,
    AMDGPU_INFO_DEV_INFO,
    AMDGPU_INFO_GDS_CONFIG,
    AMDGPU_INFO_VRAM_GTT,
    AMDGPU_INFO_MEMORY,
    AMDGPU_INFO_VRAM_USAGE,
    AMDGPU_INFO_VIS_VRAM_USAGE,
    AMDGPU_INFO_GTT_USAGE,
    AMDGPU_INFO_VCE_CLOCK_TABLE,
    AMDGPU_INFO_NUM_VRAM_CPU_PAGE_FAULTS,
};
use core::mem::{size_of, MaybeUninit};

pub struct DeviceHandle {
    #[cfg(feature = "dynamic_loading")]
    pub(crate) libdrm: Arc<DynLibDrm>,
    #[cfg(feature = "dynamic_loading")]
    pub(crate) libdrm_amdgpu: Arc<DynLibDrmAmdgpu>,
    pub(crate) amdgpu_dev: DEVICE_HANDLE,
    pub(crate) fd: i32,
}

unsafe impl Send for DeviceHandle {}
unsafe impl Sync for DeviceHandle {}

use std::path::PathBuf;

impl LibDrmAmdgpu {
    pub fn init_device_handle(&self, fd: i32) -> Result<(DeviceHandle, u32, u32), i32> {
        #[cfg(not(feature = "dynamic_loading"))]
        let init = bindings::amdgpu_device_initialize;
        #[cfg(feature = "dynamic_loading")]
        let init = self.libdrm_amdgpu.amdgpu_device_initialize;

        unsafe {
            let mut amdgpu_dev: MaybeUninit<amdgpu_device_handle> = MaybeUninit::zeroed();
            let mut major: MaybeUninit<u32> = MaybeUninit::zeroed();
            let mut minor: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = init(
                fd,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
                amdgpu_dev.as_mut_ptr(),
            );

            let [major, minor] = [major.assume_init(), minor.assume_init()];
            let device_handle = DeviceHandle {
                #[cfg(feature = "dynamic_loading")]
                libdrm: self.libdrm.clone(),
                #[cfg(feature = "dynamic_loading")]
                libdrm_amdgpu: self.libdrm_amdgpu.clone(),
                amdgpu_dev: amdgpu_dev.assume_init(),
                fd,
            };

            query_error!(r);

            Ok((device_handle, major, minor))
        }
    }
}

impl DeviceHandle {
    /// Initialization.
    /// Example of `fd`: `/dev/dri/renderD128`, `/dev/dri/by-path/pci-{[PCI::BUS]}-render`  
    /// It may require a write option (`std::fs::OpenOptions::new().read(true).write(true)`)
    /// for GUI context.  
    /// ref: <https://gitlab.freedesktop.org/mesa/mesa/-/issues/2424>
    #[cfg(not(feature = "dynamic_loading"))]
    pub fn init(fd: i32) -> Result<(Self, u32, u32), i32> {
        unsafe {
            let mut amdgpu_dev: MaybeUninit<amdgpu_device_handle> = MaybeUninit::zeroed();
            let mut major: MaybeUninit<u32> = MaybeUninit::zeroed();
            let mut minor: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_device_initialize(
                fd,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
                amdgpu_dev.as_mut_ptr(),
            );

            let [major, minor] = [major.assume_init(), minor.assume_init()];
            let device_handle = Self {
                amdgpu_dev: amdgpu_dev.assume_init(),
                fd,
            };

            query_error!(r);

            Ok((device_handle, major, minor))
        }
    }

    fn deinit(&self) -> Result<i32, i32> {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::amdgpu_device_deinitialize;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm_amdgpu.amdgpu_device_deinitialize;

        let r = unsafe { func(self.amdgpu_dev) };

        query_error!(r);

        Ok(r)
    }

    pub fn get_fd(&self) -> i32 {
        self.fd
    }

    /// Returns the result of reading the register at the specified offset.
    /// If the offset is not allowed, returns `Err(i32)`.
    pub fn read_mm_registers(&self, offset: u32) -> Result<u32, i32> {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::amdgpu_read_mm_registers;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm_amdgpu.amdgpu_read_mm_registers;

        unsafe {
            let mut out: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = func(
                self.amdgpu_dev,
                offset, // DWORD offset
                1, // count
                0xFFFF_FFFF, // instance mask, full mask
                0, // flags
                out.as_mut_ptr(),
            );

            let out = out.assume_init();

            query_error!(r);

            Ok(out)
        }
    }

    pub fn query_gpu_info(&self) -> Result<amdgpu_gpu_info, i32> {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::amdgpu_query_gpu_info;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm_amdgpu.amdgpu_query_gpu_info;

        unsafe {
            let mut gpu_info: MaybeUninit<amdgpu_gpu_info> = MaybeUninit::zeroed();

            let r = func(self.amdgpu_dev, gpu_info.as_mut_ptr());

            let gpu_info = gpu_info.assume_init();

            query_error!(r);

            Ok(gpu_info)
        }
    }

    pub fn query_gds_info(&self) -> Result<amdgpu_gds_resource_info, i32> {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::amdgpu_query_gds_info;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm_amdgpu.amdgpu_query_gds_info;

        unsafe {
            let mut gds_info: MaybeUninit<amdgpu_gds_resource_info> = MaybeUninit::zeroed();

            let r = func(self.amdgpu_dev, gds_info.as_mut_ptr());

            let gds_info = gds_info.assume_init();

            query_error!(r);

            Ok(gds_info)
        }
    }

    pub fn query_sw_info(&self, info: amdgpu_sw_info) -> Result<u32, i32> {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::amdgpu_query_sw_info;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm_amdgpu.amdgpu_query_sw_info;

        unsafe {
            let mut val: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = func(
                self.amdgpu_dev,
                info as u32,
                val.as_mut_ptr() as *mut ::core::ffi::c_void,
            );

            let val = val.assume_init();

            query_error!(r);

            Ok(val)
        }
    }

    pub(crate) fn query<T>(&self, info_id: ::core::ffi::c_uint) -> Result<T, i32> {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::amdgpu_query_info;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm_amdgpu.amdgpu_query_info;

        unsafe {
            let mut dev: MaybeUninit<T> = MaybeUninit::zeroed();

            let r = func(
                self.amdgpu_dev,
                info_id,
                size_of::<T>() as u32,
                dev.as_mut_ptr() as *mut ::core::ffi::c_void,
            );

            let dev = dev.assume_init();

            query_error!(r);

            Ok(dev)
        }
    }

    pub fn device_info(&self) -> Result<drm_amdgpu_info_device, i32> {
        Self::query(self, AMDGPU_INFO_DEV_INFO)
    }

    /// Note: `usable_heap_size` equal `real_size - pin_size - reserved_size`, is not fixed.
    pub fn vram_gtt_info(&self) -> Result<drm_amdgpu_info_vram_gtt, i32> {
        Self::query(self, AMDGPU_INFO_VRAM_GTT)
    }

    pub fn memory_info(&self) -> Result<drm_amdgpu_memory_info, i32> {
        Self::query(self, AMDGPU_INFO_MEMORY)
    }

    pub fn vram_usage_info(&self) -> Result<u64, i32> {
        Self::query(self, AMDGPU_INFO_VRAM_USAGE)
    }

    pub fn vis_vram_usage_info(&self) -> Result<u64, i32> {
        Self::query(self, AMDGPU_INFO_VIS_VRAM_USAGE)
    }

    pub fn gtt_usage_info(&self) -> Result<u64, i32> {
        Self::query(self, AMDGPU_INFO_GTT_USAGE)
    }

    pub fn gds_info(&self) -> Result<drm_amdgpu_info_gds, i32> {
        Self::query(self, AMDGPU_INFO_GDS_CONFIG)
    }

    /// AMDGPU driver returns invalid [drm_amdgpu_info_vce_clock_table].
    /// ref: <https://gitlab.freedesktop.org/drm/amd/-/issues/2391>
    pub fn vce_clock_info(&self) -> Result<drm_amdgpu_info_vce_clock_table, i32> {
        Self::query(self, AMDGPU_INFO_VCE_CLOCK_TABLE)
    }

    /// Number of VRAM page faults on CPU access
    pub fn num_vram_cpu_page_faults(&self) -> Result<u64, i32> {
        Self::query(self, AMDGPU_INFO_NUM_VRAM_CPU_PAGE_FAULTS)
    }

    /// Number of bytes moved for TTM migration
    pub fn num_bytes_moved(&self) -> Result<u64, i32> {
        Self::query(self, AMDGPU_INFO_NUM_BYTES_MOVED)
    }

    /// Number of TTM buffer evictions
    pub fn num_evictions(&self) -> Result<u64, i32> {
        Self::query(self, AMDGPU_INFO_NUM_EVICTIONS)
    }

    pub fn vram_lost_counter(&self) -> Result<u32, i32> {
        Self::query(self, AMDGPU_INFO_VRAM_LOST_COUNTER)
    }

    /// Get [PCI::BUS_INFO]
    pub fn get_pci_bus_info(&self) -> Result<PCI::BUS_INFO, i32> {
        self.drm_get_device2()
    }

    fn drm_get_device2(&self) -> Result<PCI::BUS_INFO, i32> {
        let pci = unsafe {
            let mut dev_info = self.__drmGetDevice2(self.fd, 0)?;
            let pci = core::ptr::read((*dev_info).businfo.pci);
            self.__drmFreeDevice(&mut dev_info);

            pci
        };

        Ok(PCI::BUS_INFO {
            domain: pci.domain,
            bus: pci.bus,
            dev: pci.dev,
            func: pci.func,
        })
    }

    unsafe fn __drmGetDevice2(&self, fd: ::core::ffi::c_int, flags: u32) -> Result<drmDevicePtr, i32> { unsafe {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::drmGetDevice2;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm.drmGetDevice2;

        let mut drm_dev_info: MaybeUninit<drmDevicePtr> = MaybeUninit::uninit();

        let r = func(fd, flags, drm_dev_info.as_mut_ptr());

        let drm_dev_info = drm_dev_info.assume_init();

        if drm_dev_info.is_null() {
            return Err(r);
        }

        query_error!(r);

        Ok(drm_dev_info)
    }}

    unsafe fn __drmFreeDevice(&self, device: *mut drmDevicePtr) { unsafe {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::drmFreeDevice;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm.drmFreeDevice;

        func(device)
    }}

    fn get_min_max_clock_from_dpm<P: Into<PathBuf>>(
        &self,
        sysfs_path: P,
    ) -> Option<[u32; 2]> {
        let parse_line = |s: &str| -> Option<u32> {
            s.split(' ').nth(1)?.trim_end_matches("Mhz").parse::<u32>().ok()
        };

        AMDGPU::get_min_max_from_dpm(sysfs_path.into(), parse_line)
    }

    /// Get the min/max gpu core clock (MHz) from sysfs (`pp_dpm_mclk`)
    pub fn get_min_max_memory_clock_from_dpm<P: Into<PathBuf>>(
        &self,
        path: P
    ) -> Option<[u32; 2]> {
        self.get_min_max_clock_from_dpm(path.into().join("pp_dpm_mclk"))
    }

    /// Get the min/max gpu core clock (MHz) from sysfs (`pp_dpm_sclk`)
    pub fn get_min_max_gpu_clock_from_dpm<P: Into<PathBuf>>(
        &self,
        path: P
    ) -> Option<[u32; 2]> {
        self.get_min_max_clock_from_dpm(path.into().join("pp_dpm_sclk"))
    }

    /// Get the min/max gpu core clock (MHz) from sysfs (`pp_dpm_mclk`)
    pub fn get_min_max_memory_clock_from_sysfs<P: Into<PathBuf>>(
        &self,
        path: P
    ) -> Option<(u32, u32)> {
        let tmp = self.get_min_max_clock_from_dpm(path.into().join("pp_dpm_mclk"))?;

        Some((tmp[0], tmp[1]))
    }

    /// Get the min/max gpu core clock (MHz) from sysfs (`pp_dpm_mclk`)
    pub fn get_min_max_memory_clock(&self) -> Option<(u32, u32)> {
        let sysfs_path = self.get_sysfs_path().ok()?;
        self.get_min_max_memory_clock_from_sysfs(sysfs_path)
    }

    /// Get the min/max gpu core clock (MHz) from sysfs (`pp_dpm_sclk`)
    pub fn get_min_max_gpu_clock_from_sysfs<P: Into<PathBuf>>(
        &self,
        path: P
    ) -> Option<(u32, u32)> {
        let tmp = self.get_min_max_clock_from_dpm(path.into().join("pp_dpm_sclk"))?;

        Some((tmp[0], tmp[1]))
    }

    /// Get the min/max gpu core clock (MHz) from sysfs (`pp_dpm_sclk`)
    pub fn get_min_max_gpu_clock(&self) -> Option<(u32, u32)> {
        let sysfs_path = self.get_sysfs_path().ok()?;
        self.get_min_max_gpu_clock_from_sysfs(sysfs_path)
    }

    /// 
    pub fn get_sysfs_path(&self) -> Result<PathBuf, i32> {
        let path = self.get_pci_bus_info()?.get_sysfs_path();

        Ok(path)
    }

    pub(crate) fn get_sysfs_path_io(&self) -> std::io::Result<PathBuf> {
        let path = self
            .get_pci_bus_info()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed get_pci_bus_info"))?
            .get_sysfs_path();

        Ok(path)
    }

    /// 
    pub fn get_hwmon_path(&self) -> Option<PathBuf> {
        self.get_pci_bus_info().ok()?.get_hwmon_path()
    }

    /// ref: drivers/gpu/drm/amd/pm/swsmu/smu13/aldebaran_ppt.c
    /// ref: <https://github.com/RadeonOpenCompute/rocm_smi_lib/blob/master/python_smi_tools/rocm_smi.py>
    pub fn check_if_secondary_die(&self) -> bool {
        let Some(power_cap) = self.get_power_cap() else { return false };

        power_cap.check_if_secondary_die()
    }

    pub fn get_min_max_link_info_from_dpm(&self) -> Option<[PCI::LINK; 2]> {
        let pci_bus = self.get_pci_bus_info().ok()?;

        pci_bus.get_min_max_link_info_from_dpm()
    }

    /// [PCI::BUS_INFO::get_max_gpu_link]
    pub fn get_max_gpu_link(&self) -> Option<PCI::LINK> {
        let pci_bus = self.get_pci_bus_info().ok()?;

        pci_bus.get_max_gpu_link()
    }

    /// [PCI::BUS_INFO::get_max_system_link]
    pub fn get_max_system_link(&self) -> Option<PCI::LINK> {
        let pci_bus = self.get_pci_bus_info().ok()?;

        pci_bus.get_max_system_link()
    }
}

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        self.deinit().unwrap();
    }
}

impl drm_amdgpu_memory_info {
    /// The AMDGPU driver allocates part of VRAM to pre-OS buffer (vbios, frame buffer)
    /// if VRAM is larger than 8GiB
    /// ref: drivers/gpu/drm/amd/amdgpu/amdgpu_gmc.c  
    /// ref: <https://gitlab.freedesktop.org/mesa/mesa/blob/main/src/amd/common/ac_gpu_info.c>  
    pub fn check_resizable_bar(&self) -> bool {
        (self.vram.total_heap_size * 9 / 10) <= self.cpu_accessible_vram.total_heap_size
    }
}

#[repr(u32)]
pub enum amdgpu_sw_info {
    address32_hi = 0,
}
