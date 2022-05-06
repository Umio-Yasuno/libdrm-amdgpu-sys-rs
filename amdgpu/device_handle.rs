use crate::*;

use std::mem::{MaybeUninit, size_of};
use std::ffi::CStr;
use bindings::{
    amdgpu_device_handle,
    amdgpu_device_initialize,
    amdgpu_gpu_info,
    amdgpu_gds_resource_info,
    amdgpu_heap_info,
    drm_amdgpu_info_device,
    drm_amdgpu_memory_info,
    drm_amdgpu_info_gds,
    drm_amdgpu_info_hw_ip,
};
use bindings::{
    AMDGPU_INFO_DEV_INFO,
    AMDGPU_INFO_MEMORY,
    AMDGPU_INFO_VRAM_USAGE,
    AMDGPU_INFO_GDS_CONFIG,
};

pub trait HANDLE {
    fn init(fd: ::std::os::raw::c_int) -> Result<Self, i32> where Self: Sized;
    fn get_marketing_name(self) -> Result<String, std::str::Utf8Error>;
    fn query_gpu_info(self) -> Result<amdgpu_gpu_info, i32>;
    fn query_gds_info(self) -> Result<amdgpu_gds_resource_info, i32>;
    // fn query_heap_info(self) -> Result<amdgpu_heap_info, i32>;

    fn query_hw_ip_info(self,
        type_: ::std::os::raw::c_uint,
        ip_instance: ::std::os::raw::c_uint,
    ) -> Result<drm_amdgpu_info_hw_ip, i32>;
    fn query_firmware_version(
        self,
        fw_type: ::std::os::raw::c_uint,
        ip_instance: ::std::os::raw::c_uint,
        index: ::std::os::raw::c_uint,
    ) -> Result<(u32, u32), i32>;

    fn device_info(self) -> Result<drm_amdgpu_info_device, i32>;
    fn memory_info(self) -> Result<drm_amdgpu_memory_info, i32>;
    fn vram_usage_info(self) -> Result<u64, i32>;
    fn gds_info(self) -> Result<drm_amdgpu_info_gds, i32>;

    #[doc(hidden)]
    fn query<T>(self, info_id: ::std::os::raw::c_uint) -> Result<T, i32>;

    #[doc(hidden)]
    unsafe fn query_vbios<T>(
        self,
        fd: ::std::os::raw::c_int,
        info_id: ::std::os::raw::c_uint
    ) -> Result<T, i32>;
    unsafe fn vbios_info(
        self,
        fd: ::std::os::raw::c_int,
    ) -> Result<bindings::drm_amdgpu_info_vbios, i32>;
    unsafe fn vbios_size(
        self,
        fd: ::std::os::raw::c_int,
    ) -> Result<u32, i32>;
}

pub type DEVICE = bindings::amdgpu_device;
pub type DEVICE_HANDLE = bindings::amdgpu_device_handle;

impl HANDLE for DEVICE_HANDLE {
    fn init(fd: ::std::os::raw::c_int) -> Result<Self, i32> {
        unsafe {
            let mut amdgpu_dev: MaybeUninit<amdgpu_device_handle> = MaybeUninit::uninit();
            let mut _major: MaybeUninit<u32> = MaybeUninit::zeroed();
            let mut _minor: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = amdgpu_device_initialize(
                fd,
                _major.as_mut_ptr(),
                _minor.as_mut_ptr(),
                amdgpu_dev.as_mut_ptr(),
            );

            query_error!(r);

            let [_major, _minor] = [_major, _minor].map(
                |v| v.assume_init()
            );

            return Ok(amdgpu_dev.assume_init());
        }
    }
    fn get_marketing_name(self) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let c_str = CStr::from_ptr(bindings::amdgpu_get_marketing_name(self));

            match c_str.to_str() {
                Ok(v) => Ok(v.to_string()),
                Err(e) => Err(e),
            }
        }
    }
    fn query_gpu_info(self) -> Result<amdgpu_gpu_info, i32> {
        unsafe {
            let mut gpu_info: MaybeUninit<amdgpu_gpu_info> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_gpu_info(
                self,
                gpu_info.as_mut_ptr()
            );

            query_error!(r);

            return Ok(gpu_info.assume_init());
        }
    }
    fn query_gds_info(self) -> Result<amdgpu_gds_resource_info, i32> {
        unsafe {
            let mut gds_info: MaybeUninit<amdgpu_gds_resource_info> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_gds_info(
                self,
                gds_info.as_mut_ptr(),
            );

            query_error!(r);

            return Ok(gds_info.assume_init());
        }
    }

    fn query_hw_ip_info(
        self,
        type_: ::std::os::raw::c_uint,
        ip_instance: ::std::os::raw::c_uint,
    ) -> Result<drm_amdgpu_info_hw_ip, i32> {
        unsafe {
            let mut hw_ip_info: MaybeUninit<drm_amdgpu_info_hw_ip> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_hw_ip_info(
                self,
                type_,
                ip_instance,
                hw_ip_info.as_mut_ptr(),
            );

            query_error!(r);

            return Ok(hw_ip_info.assume_init());
        }
    }
    fn query_firmware_version(
        self,
        fw_type: ::std::os::raw::c_uint,
        ip_instance: ::std::os::raw::c_uint,
        index: ::std::os::raw::c_uint,
    ) -> Result<(u32, u32), i32> {
        unsafe {
            let mut version: MaybeUninit<u32> = MaybeUninit::zeroed();
            let mut feature: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_firmware_version(
                self,
                fw_type,
                ip_instance,
                index,
                version.as_mut_ptr(),
                feature.as_mut_ptr(),
            );

            query_error!(r);

            return Ok((version.assume_init(), feature.assume_init()));
        }
    }
    /*
    fn query_heap_info(self) -> Result<amdgpu_heap_info, i32> {
        unsafe {
            let mut heap_info: MaybeUninit<amdgpu_heap_info> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_query_heap_info(
                self,
                heap_info.as_mut_ptr(),
            );

            query_error!(r);

            return Ok(heap_info.assume_init());
        }
    }
    */
    fn query<T>(self, info_id: ::std::os::raw::c_uint) -> Result<T, i32> {
        unsafe {
            let mut device_info: MaybeUninit<T> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_info(
                self,
                info_id,
                size_of::<T>() as u32,
                device_info.as_mut_ptr() as *mut ::std::os::raw::c_void,
            );

            query_error!(r);

            return Ok(device_info.assume_init());
        }
    }
    fn device_info(self) -> Result<drm_amdgpu_info_device, i32> {
        Self::query(self, AMDGPU_INFO_DEV_INFO)
    }
    fn memory_info(self) -> Result<drm_amdgpu_memory_info, i32> {
        Self::query(self, AMDGPU_INFO_MEMORY)
    }
    fn vram_usage_info(self) -> Result<u64, i32> {
        Self::query(self, AMDGPU_INFO_VRAM_USAGE)
    }
    fn gds_info(self) -> Result<drm_amdgpu_info_gds, i32> {
        Self::query(self, AMDGPU_INFO_GDS_CONFIG)
    }
    unsafe fn query_vbios<T>(
        self,
        fd: ::std::os::raw::c_int,
        info_id: ::std::os::raw::c_uint,
    ) -> Result<T, i32> {
        use bindings::{
            drmCommandWrite,
            drm_amdgpu_info,
            AMDGPU_INFO_VBIOS,
        };

        let mut vbios: MaybeUninit<T> = MaybeUninit::uninit();

        // std::ptr::write_bytes(device_info.as_mut_ptr(), 0x0, 1);
        let mut device_info: drm_amdgpu_info = std::mem::zeroed();

        device_info.return_pointer = vbios.as_mut_ptr() as u64;
        device_info.return_size = size_of::<T>() as u32;
        device_info.query = AMDGPU_INFO_VBIOS;

        device_info.__bindgen_anon_1.vbios_info.type_ = info_id;

        // println!("vbios type: {}", device_info.__bindgen_anon_1.vbios_info.type_);

        let mut device_info = MaybeUninit::new(device_info);

        let r = drmCommandWrite(
            fd,
            bindings::DRM_AMDGPU_INFO as u64,
            device_info.as_mut_ptr() as *mut ::std::os::raw::c_void,
            size_of::<drm_amdgpu_info> as u64, 
        );

        query_error!(r);

        let _ = device_info.assume_init();
        let vbios = vbios.assume_init();

        return Ok(vbios);
    }
    unsafe fn vbios_info(
        self,
        fd: ::std::os::raw::c_int,
    ) -> Result<bindings::drm_amdgpu_info_vbios, i32> {
        use bindings::{
            drm_amdgpu_info_vbios,
            AMDGPU_INFO_VBIOS_INFO,
        };

        let vbios: drm_amdgpu_info_vbios = Self::query_vbios(
            self,
            fd,
            AMDGPU_INFO_VBIOS_INFO
        )?;

        return Ok(vbios);
    }
    unsafe fn vbios_size(
        self,
        fd: ::std::os::raw::c_int,
    ) -> Result<u32, i32> {
        use bindings::{
            AMDGPU_INFO_VBIOS_SIZE,
        };

        let vbios_size: u32 = Self::query_vbios(
            self,
            fd,
            AMDGPU_INFO_VBIOS_SIZE
        )?;

        return Ok(vbios_size);
    }
}

pub mod FW {
    pub use crate::bindings::{
        AMDGPU_INFO_FW_VCE as VCE,
        AMDGPU_INFO_FW_UVD as UVD,
        AMDGPU_INFO_FW_GMC as GMC,
        AMDGPU_INFO_FW_GFX_ME as GFX_ME,
        AMDGPU_INFO_FW_GFX_PFP as GFX_PFP,
        AMDGPU_INFO_FW_GFX_CE as GFX_CE,
        AMDGPU_INFO_FW_GFX_RLC as GFX_RLC,
        AMDGPU_INFO_FW_GFX_MEC as GFX_MEC,
        AMDGPU_INFO_FW_SMC as SMC,
        AMDGPU_INFO_FW_SDMA as SDMA,
        AMDGPU_INFO_FW_SOS as SOS,
        AMDGPU_INFO_FW_ASD as ASD,
        AMDGPU_INFO_FW_VCN as VCN,
        AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_CNTL as GFX_RLC_RESTORE_LIST_CNTL,
        AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_GPM_MEM as GFX_RLC_RESTORE_LIST_GPM_MEM,
        AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_SRM_MEM as GFX_RLC_RESTORE_LIST_SRM_MEM,
        AMDGPU_INFO_FW_DMCU as DMCU,
        AMDGPU_INFO_FW_TA as TA,
        AMDGPU_INFO_FW_DMCUB as DMCUB,
        AMDGPU_INFO_FW_TOC as TOC,
    };
}

pub mod HW_IP {
    pub use crate::bindings::{
        AMDGPU_HW_IP_GFX as GFX,
        AMDGPU_HW_IP_COMPUTE as COMPUTE,
        AMDGPU_HW_IP_DMA as DMA,
        AMDGPU_HW_IP_UVD as UVD,
        AMDGPU_HW_IP_VCE as VCE,
        AMDGPU_HW_IP_UVD_ENC as UVD_ENC,
        AMDGPU_HW_IP_VCN_DEC as VCN_DEC,
        AMDGPU_HW_IP_VCN_ENC as VCN_ENC,
        AMDGPU_HW_IP_VCN_JPEG as VCN_JPEG,
        // AMDGPU_HW_IP_NUM as NUM,
        // AMDGPU_HW_IP_INSTANCE_MAX_COUNT as INSTANCE_MAX_COUNT,
    };
}

pub mod GEM {
    pub mod DOMAIN {
        pub use crate::bindings::{
            AMDGPU_GEM_DOMAIN_CPU as CPU,
            AMDGPU_GEM_DOMAIN_GTT as GTT,
            AMDGPU_GEM_DOMAIN_VRAM as VRAM,
            AMDGPU_GEM_DOMAIN_GDS as GDS,
            AMDGPU_GEM_DOMAIN_GWS as GWS,
            AMDGPU_GEM_DOMAIN_OA as OA,
            AMDGPU_GEM_DOMAIN_MASK as MASK,
        };
    }
    pub mod CREATE {
        pub use crate::bindings::{
            AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED as CPU_ACCESS_REQUIRED,
            AMDGPU_GEM_CREATE_NO_CPU_ACCESS as NO_CPU_ACCESS,
            AMDGPU_GEM_CREATE_CPU_GTT_USWC as CPU_GTT_USWC,
            AMDGPU_GEM_CREATE_VRAM_CLEARED as VRAM_CLEARED,
            AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS as VRAM_CONTIGUOUS,
            AMDGPU_GEM_CREATE_VM_ALWAYS_VALID as VM_ALWAYS_VALID,
            AMDGPU_GEM_CREATE_EXPLICIT_SYNC as EXPLICIT_SYNC,
            AMDGPU_GEM_CREATE_CP_MQD_GFX9 as CP_MQD_GFX9,
            AMDGPU_GEM_CREATE_VRAM_WIPE_ON_RELEASE as VRAM_WIPE_ON_RELEASE,
            AMDGPU_GEM_CREATE_ENCRYPTED as ENCRYPTED,
            AMDGPU_GEM_CREATE_PREEMPTIBLE as PREEMPTIBLE,
        };
    }
}
