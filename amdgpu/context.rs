use crate::AMDGPU::DeviceHandle;
use crate::query_error;
use crate::bindings::{self, amdgpu_context_handle};
use core::mem::MaybeUninit;

#[cfg(feature = "dynamic_loading")]
use std::sync::Arc;
#[cfg(feature = "dynamic_loading")]
use crate::DynLibDrmAmdgpu;

pub struct ContextHandle {
    pub(crate) ctx_handle: amdgpu_context_handle,
    #[cfg(feature = "dynamic_loading")]
    pub(crate) libdrm_amdgpu: Arc<DynLibDrmAmdgpu>,
}

impl DeviceHandle {
    pub fn create_context(&self) -> Result<ContextHandle, i32> {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::amdgpu_cs_ctx_create;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm_amdgpu.amdgpu_cs_ctx_create;

        unsafe {
            let mut ctx_handle: MaybeUninit<amdgpu_context_handle> = MaybeUninit::zeroed();

            let r = func(self.amdgpu_dev, ctx_handle.as_mut_ptr());

            let ctx_handle = ContextHandle::new(&self, ctx_handle.assume_init());

            query_error!(r);

            Ok(ctx_handle)
        }
    }
}

impl ContextHandle {
    pub fn new(_amdgpu_dev: &DeviceHandle, ctx_handle: amdgpu_context_handle) -> Self {
        Self {
            ctx_handle,
            #[cfg(feature = "dynamic_loading")]
            libdrm_amdgpu: _amdgpu_dev.libdrm_amdgpu.clone(),
        }
    }

    unsafe fn free(&self) -> Result<(), i32> { unsafe {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::amdgpu_cs_ctx_free;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm_amdgpu.amdgpu_cs_ctx_free;

        let r = func(self.ctx_handle);

        query_error!(r);

        Ok(())
    }}

    fn stable_pstate(
        &self,
        op: u32,
        pstate_flag: StablePstateFlag,
    ) -> Result<StablePstateFlag, i32> {
        #[cfg(not(feature = "dynamic_loading"))]
        let func = bindings::amdgpu_cs_ctx_stable_pstate;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm_amdgpu.amdgpu_cs_ctx_stable_pstate;

        unsafe {
            let mut out_flags: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = func(
                self.ctx_handle,
                op,
                pstate_flag as u32,
                out_flags.as_mut_ptr(),
            );

            let out_flags = out_flags.assume_init();

            query_error!(r);

            Ok(out_flags.into())
        }
    }

    pub fn set_stable_pstate(
        &self,
        pstate_flag: StablePstateFlag,
    ) -> Result<StablePstateFlag, i32> {
        self.stable_pstate(bindings::AMDGPU_CTX_OP_SET_STABLE_PSTATE, pstate_flag)
    }

    pub fn get_stable_pstate(&self) -> Result<StablePstateFlag, i32> {
        self.stable_pstate(bindings::AMDGPU_CTX_OP_GET_STABLE_PSTATE, StablePstateFlag::NONE)
    }
}

impl Drop for ContextHandle {
    fn drop(&mut self) {
        unsafe { self.free().unwrap(); }
    }
}

use crate::bindings::{
    // AMDGPU_CTX_STABLE_PSTATE_FLAGS_MASK,
    AMDGPU_CTX_STABLE_PSTATE_NONE,
    AMDGPU_CTX_STABLE_PSTATE_STANDARD,
    AMDGPU_CTX_STABLE_PSTATE_MIN_SCLK,
    AMDGPU_CTX_STABLE_PSTATE_MIN_MCLK,
    AMDGPU_CTX_STABLE_PSTATE_PEAK,
};

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StablePstateFlag {
    NONE = AMDGPU_CTX_STABLE_PSTATE_NONE,
    STANDARD = AMDGPU_CTX_STABLE_PSTATE_STANDARD,
    MIN_SCLK = AMDGPU_CTX_STABLE_PSTATE_MIN_SCLK,
    MIN_MCLK = AMDGPU_CTX_STABLE_PSTATE_MIN_MCLK,
    PEAK = AMDGPU_CTX_STABLE_PSTATE_PEAK,
}

impl From<u32> for StablePstateFlag {
    fn from(val: u32) -> Self {
        match val {
            AMDGPU_CTX_STABLE_PSTATE_NONE => Self::NONE,
            AMDGPU_CTX_STABLE_PSTATE_STANDARD => Self::STANDARD,
            AMDGPU_CTX_STABLE_PSTATE_MIN_SCLK => Self::MIN_SCLK,
            AMDGPU_CTX_STABLE_PSTATE_MIN_MCLK => Self::MIN_MCLK,
            AMDGPU_CTX_STABLE_PSTATE_PEAK => Self::PEAK,
            _ => Self::NONE,
        }
    }
}
