/* WIP */
use crate::*;
use crate::AMDGPU::*;
// use super::*;
use std::mem::MaybeUninit;

use bindings::{
    amdgpu_context_handle,
};

pub trait CS {
    fn ctx_create2(
        self,
        priority: CTX_PRIORITY,
    ) -> Result<amdgpu_context_handle, i32>;
    fn ctx_create(self) -> Result<amdgpu_context_handle, i32>;
}

impl CS for DEVICE_HANDLE {
    fn ctx_create2(
        self,
        priority: CTX_PRIORITY,
    ) -> Result<amdgpu_context_handle, i32> {
        unsafe {
            let mut ctx_handle: MaybeUninit<amdgpu_context_handle> = MaybeUninit::uninit();

            let r = bindings::amdgpu_cs_ctx_create2(
                self,
                priority as u32,
                ctx_handle.as_mut_ptr(),
            );

            query_error!(r);

            return Ok(ctx_handle.assume_init());
        }
    }

    fn ctx_create(self) -> Result<amdgpu_context_handle, i32> {
        unsafe {
            let mut ctx_handle: MaybeUninit<amdgpu_context_handle> = MaybeUninit::uninit();

            let r = bindings::amdgpu_cs_ctx_create(
                self,
                ctx_handle.as_mut_ptr(),
            );

            query_error!(r);

            return Ok(ctx_handle.assume_init());
        }
    }
}

use bindings::{
    AMDGPU_CTX_PRIORITY_UNSET,
    AMDGPU_CTX_PRIORITY_VERY_LOW,
    AMDGPU_CTX_PRIORITY_LOW,
    AMDGPU_CTX_PRIORITY_NORMAL,
    AMDGPU_CTX_PRIORITY_HIGH,
    AMDGPU_CTX_PRIORITY_VERY_HIGH,
};

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum CTX_PRIORITY {
    UNSET = AMDGPU_CTX_PRIORITY_UNSET as u32,
    VERY_LOW = AMDGPU_CTX_PRIORITY_VERY_LOW as u32,
    LOW = AMDGPU_CTX_PRIORITY_LOW as u32,
    NORMAL = AMDGPU_CTX_PRIORITY_NORMAL as u32,
    HIGH = AMDGPU_CTX_PRIORITY_HIGH as u32,
    VERY_HIGH = AMDGPU_CTX_PRIORITY_VERY_HIGH as u32,
}

pub trait CS_CTX {
    unsafe fn free(self) -> Result<i32, i32>;
}

impl CS_CTX for amdgpu_context_handle {
    unsafe fn free(self) -> Result<i32, i32> {
        let r = bindings::amdgpu_cs_ctx_free(self);

        query_error!(r);

        return Ok(r);
    }
}
