use crate::AMDGPU::DeviceHandle;
use crate::query_error;
use crate::bindings::{self, amdgpu_context_handle};
use core::mem::MaybeUninit;

pub struct ContextHandle(pub(crate) amdgpu_context_handle);

impl DeviceHandle {
    pub fn create_context(&self) -> Result<ContextHandle, i32> {
        unsafe {
            let mut ctx_handle: MaybeUninit<amdgpu_context_handle> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_cs_ctx_create(self.0, ctx_handle.as_mut_ptr());

            let ctx_handle = ContextHandle::new(ctx_handle.assume_init());

            query_error!(r);

            Ok(ctx_handle)
        }
    }
}

impl ContextHandle {
    pub fn new(ctx_handle: amdgpu_context_handle) -> Self {
        Self(ctx_handle)
    }

    unsafe fn free(&self) -> Result<(), i32> {
        let r = bindings::amdgpu_cs_ctx_free(self.0);

        query_error!(r);

        Ok(())
    }

    fn stable_pstate(
        &self,
        op: u32,
        pstate_flag: StablePstateFlag,
    ) -> Result<StablePstateFlag, i32> {
        unsafe {
            let mut out_flags: MaybeUninit<u32> = MaybeUninit::zeroed();

            let r = bindings::amdgpu_cs_ctx_stable_pstate(
                self.0,
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
