use crate::AMDGPU::DEVICE_HANDLE;
use crate::*;

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

pub struct DeviceHandle(pub(crate) DEVICE_HANDLE);

unsafe impl Send for DeviceHandle {}
unsafe impl Sync for DeviceHandle {}

#[cfg(feature = "std")]
use std::path::{Path, PathBuf};

impl DeviceHandle {
    /// Initialization.
    /// Example of `fd`: `/dev/dri/renderD128`, `/dev/dri/by-path/pci-{[PCI::BUS]}-render`
    pub fn init(fd: i32) -> Result<(Self, u32, u32), i32> {
        unsafe {
            let mut amdgpu_dev: MaybeUninit<amdgpu_device_handle> = MaybeUninit::uninit();
            let mut major: MaybeUninit<u32> = MaybeUninit::zeroed();
            let mut minor: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_device_initialize(
                fd,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
                amdgpu_dev.as_mut_ptr(),
            );

            let [major, minor] = [major.assume_init(), minor.assume_init()];
            let amdgpu_dev = Self(amdgpu_dev.assume_init());

            query_error!(r);

            Ok((amdgpu_dev, major, minor))
        }
    }

    fn deinit(&self) -> Result<i32, i32> {
        let r = unsafe { bindings::amdgpu_device_deinitialize(self.0) };

        query_error!(r);

        Ok(r)
    }

    pub fn get_fd(&self) -> i32 {
        unsafe { bindings::amdgpu_device_get_fd(self.0) }
    }

    /// (`major`, `minor`, `patchlevel`)
    #[deprecated(since = "0.1.3", note = "superseded by `get_drm_version_struct`")]
    pub fn get_drm_version(&self) -> Result<(i32, i32, i32), ()> {
        let fd = self.get_fd();
        let drm_ver_ptr = unsafe { bindings::drmGetVersion(fd) };

        if drm_ver_ptr.is_null() {
            return Err(());
        }

        let ver = unsafe { (
            (*drm_ver_ptr).version_major,
            (*drm_ver_ptr).version_minor,
            (*drm_ver_ptr).version_patchlevel,
        ) };

        unsafe { bindings::drmFreeVersion(drm_ver_ptr) }

        Ok(ver)
    }

    #[cfg(feature = "std")]
    pub fn get_drm_version_struct(&self) -> Result<drmVersion, i32> {
        drmVersion::get(self.get_fd())
    }

    /// Returns the result of reading the register at the specified offset.
    /// If the offset is not allowed, returns `Err(i32)`.
    pub fn read_mm_registers(&self, offset: u32) -> Result<u32, i32> {
        unsafe {
            let mut out: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_read_mm_registers(
                self.0,
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

    /// From libdrm-2.4.114, it returns the default name ("AMD Radeon Graphics")
    ///  if there is no name that matches amdgpu.ids  
    /// https://gitlab.freedesktop.org/mesa/drm/-/commit/a81b9ab8f3fb6840b36f732c1dd25fe5e0d68d0a
    #[cfg(feature = "std")]
    #[deprecated(since = "0.1.3",  note = "superseded by `get_marketing_name_or_default`")]
    pub fn get_marketing_name(&self) -> Result<String, std::str::Utf8Error> {
        use core::ffi::CStr;

        let mark_name = unsafe { bindings::amdgpu_get_marketing_name(self.0) };

        if mark_name.is_null() {
            eprintln!("libdrm_amdgpu_sys: ASIC not found in amdgpu.ids");
            return Ok("".to_string());
        }

        let c_str = unsafe { CStr::from_ptr(mark_name) };

        Ok(c_str.to_str()?.to_string())
    }

    /// Returns the default marketing name ("AMD Radeon Graphics") 
    /// when the device name is not available.
    #[cfg(feature = "std")]
    pub fn get_marketing_name_or_default(&self) -> String {
        use core::ffi::CStr;

        let mark_name_ptr = unsafe { bindings::amdgpu_get_marketing_name(self.0) };

        if mark_name_ptr.is_null() {
            return AMDGPU::DEFAULT_DEVICE_NAME.to_string();
        }

        let c_str = unsafe { CStr::from_ptr(mark_name_ptr) };

        match c_str.to_str() {
            Ok(name) => name,
            Err(_) => AMDGPU::DEFAULT_DEVICE_NAME,
        }.to_string()
    }

    pub fn query_gpu_info(&self) -> Result<amdgpu_gpu_info, i32> {
        unsafe {
            let mut gpu_info: MaybeUninit<amdgpu_gpu_info> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_gpu_info(self.0, gpu_info.as_mut_ptr());

            let gpu_info = gpu_info.assume_init();

            query_error!(r);

            Ok(gpu_info)
        }
    }

    pub fn query_gds_info(&self) -> Result<amdgpu_gds_resource_info, i32> {
        unsafe {
            let mut gds_info: MaybeUninit<amdgpu_gds_resource_info> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_gds_info(self.0, gds_info.as_mut_ptr());

            let gds_info = gds_info.assume_init();

            query_error!(r);

            Ok(gds_info)
        }
    }

    pub fn query_sw_info(&self, info: amdgpu_sw_info) -> Result<u32, i32> {
        unsafe {
            let mut val: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_sw_info(
                self.0,
                info as u32,
                val.as_mut_ptr() as *mut ::core::ffi::c_void,
            );

            let val = val.assume_init();

            query_error!(r);

            Ok(val)
        }
    }

    fn query<T>(&self, info_id: ::core::ffi::c_uint) -> Result<T, i32> {
        unsafe {
            let mut dev: MaybeUninit<T> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_info(
                self.0,
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
        // return 
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

    pub fn num_vram_cpu_page_failts(&self) -> Result<u64, i32> {
        Self::query(self, AMDGPU_INFO_NUM_VRAM_CPU_PAGE_FAULTS)
    }

    pub fn num_bytes_moved(&self) -> Result<u64, i32> {
        Self::query(self, AMDGPU_INFO_NUM_BYTES_MOVED)
    }

    /// Get [PCI::BUS_INFO]
    pub fn get_pci_bus_info(&self) -> Result<PCI::BUS_INFO, i32> {
        PCI::BUS_INFO::drm_get_device2(self.get_fd())
    }

    #[cfg(feature = "std")]
    fn get_first_line<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let f = File::open(path)?;
        let mut first_line = String::new();
        let mut buf = BufReader::new(f);
        buf.read_line(&mut first_line)?;

        Ok(first_line)
    }

    #[cfg(feature = "std")]
    fn trim_dpm_clk(line: &str) -> Result<u64, std::num::ParseIntError> {
        const MHZ: &str = "Mhz";
        let mut tmp = String::new();

        /* 0: 214Mhz */
        for s in line.split(' ') {
            if s.ends_with(MHZ) {
                tmp = s.trim_end_matches(MHZ).to_string();
            }
        }

        tmp.parse::<u64>()
    }

    #[cfg(feature = "std")]
    fn get_min_clock(&self, pci: &PCI::BUS_INFO, file_name: &str) -> Option<u64> {
        let path = pci.get_sysfs_path().join(file_name);

        if let Ok(line) = Self::get_first_line(path) {
            if let Ok(clk) = Self::trim_dpm_clk(&line) {
                return Some(clk);
            }
        }

        None
    }

    /// Get the minimum gpu core clock (MHz) from sysfs (`pp_dpm_sclk`).  
    /// Recommend [DeviceHandle::get_min_max_gpu_clock]
    #[cfg(feature = "std")]
    #[deprecated(since = "0.1.2", note = "superseded by `get_min_max_gpu_clock_from_sysfs`")]
    pub fn get_min_gpu_clock_from_sysfs(&self, pci: &PCI::BUS_INFO) -> Option<u64> {
        Self::get_min_clock(self, pci, "pp_dpm_sclk")
    }

    /// Get the minimum memory clock (MHz) from sysfs (`pp_dpm_mclk`).  
    /// Recommend [DeviceHandle::get_min_max_memory_clock]
    #[cfg(feature = "std")]
    #[deprecated(since = "0.1.2", note = "superseded by `get_min_max_memory_clock_from_sysfs`")]
    pub fn get_min_memory_clock_from_sysfs(&self, pci: &PCI::BUS_INFO) -> Option<u64> {
        Self::get_min_clock(self, pci, "pp_dpm_mclk")
    }

    #[cfg(feature = "std")]
    fn get_min_max_clock_from_sysfs<P: Into<PathBuf>>(
        &self,
        path: P,
    ) -> Option<(u32, u32)> {
        const MHZ: usize = "Mhz".len();
        let parse_line = |s: &str| -> Option<u32> {
            let mut split = s.split(' ').skip(1);

            split.next().and_then(|mhz| mhz[..mhz.len()-MHZ].parse::<u32>().ok())
        };

        let s = std::fs::read_to_string(path.into()).ok()?;
        let mut lines = s.lines();

        let min_clk = parse_line(lines.next()?)?;
        let max_clk = if let Some(last) = lines.last() {
            parse_line(last)?
        } else {
            min_clk
        };

        Some((min_clk, max_clk))
    }

    /// Get the min/max gpu core clock (MHz) from sysfs (`pp_dpm_mclk`)
    #[cfg(feature = "std")]
    pub fn get_min_max_memory_clock_from_sysfs<P: Into<PathBuf>>(
        &self,
        path: P
    ) -> Option<(u32, u32)> {
        self.get_min_max_clock_from_sysfs(path.into().join("pp_dpm_mclk"))
    }

    /// Get the min/max gpu core clock (MHz) from sysfs (`pp_dpm_mclk`)
    #[cfg(feature = "std")]
    pub fn get_min_max_memory_clock(&self) -> Option<(u32, u32)> {
        let sysfs_path = self.get_sysfs_path().ok()?;
        self.get_min_max_memory_clock_from_sysfs(sysfs_path)
    }

    /// Get the min/max gpu core clock (MHz) from sysfs (`pp_dpm_sclk`)
    #[cfg(feature = "std")]
    pub fn get_min_max_gpu_clock_from_sysfs<P: Into<PathBuf>>(
        &self,
        path: P
    ) -> Option<(u32, u32)> {
        self.get_min_max_clock_from_sysfs(path.into().join("pp_dpm_sclk"))
    }

    /// Get the min/max gpu core clock (MHz) from sysfs (`pp_dpm_sclk`)
    #[cfg(feature = "std")]
    pub fn get_min_max_gpu_clock(&self) -> Option<(u32, u32)> {
        let sysfs_path = self.get_sysfs_path().ok()?;
        self.get_min_max_gpu_clock_from_sysfs(sysfs_path)
    }

    /// 
    #[cfg(feature = "std")]
    pub fn get_sysfs_path(&self) -> Result<PathBuf, i32> {
        let path = self.get_pci_bus_info()?.get_sysfs_path();

        Ok(path)
    }

    /// 
    #[cfg(feature = "std")]
    pub fn get_hwmon_path(&self) -> Option<PathBuf> {
        self.get_pci_bus_info().ok().and_then(|pci| pci.get_hwmon_path())
    }
}

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        self.deinit().unwrap();
    }
}

impl drm_amdgpu_memory_info {
    pub fn check_resizable_bar(&self) -> bool {
        // The AMDGPU driver allocates part of VRAM to pre-OS buffer (vbios, frame buffer)
        // if VRAM is larger than 8GiB
        // ref: drivers/gpu/drm/amd/amdgpu/amdgpu_gmc.c
        // ref: https://gitlab.freedesktop.org/mesa/mesa/blob/main/src/amd/common/ac_gpu_info.c
        (self.vram.total_heap_size * 9 / 10) <= self.cpu_accessible_vram.total_heap_size
    }
}

#[repr(u32)]
pub enum amdgpu_sw_info {
    address32_hi = 0,
}
