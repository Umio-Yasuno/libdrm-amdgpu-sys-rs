/* TODO: WIP */

use crate::AMDGPU::*;
use crate::*;
// use super::*;
use core::mem::{size_of, MaybeUninit};

use bindings::{
    drm_amdgpu_info_video_caps, drm_amdgpu_info_video_codec_info,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_AV1, AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_HEVC,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_JPEG, AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG2,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4, AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4_AVC,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VC1, AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VP9,
    AMDGPU_INFO_VIDEO_CAPS_DECODE, AMDGPU_INFO_VIDEO_CAPS_ENCODE,
};

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum CAP_TYPE {
    DECODE = AMDGPU_INFO_VIDEO_CAPS_DECODE,
    ENCODE = AMDGPU_INFO_VIDEO_CAPS_ENCODE,
}

impl DeviceHandle {
    pub fn get_video_caps(&self, type_: CAP_TYPE) -> Result<drm_amdgpu_info_video_caps, i32> {
        unsafe {
            let mut video_caps: MaybeUninit<drm_amdgpu_info_video_caps> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_video_caps_info(
                self.0,
                type_ as u32,
                size_of::<drm_amdgpu_info_video_caps> as u32,
                video_caps.as_mut_ptr() as *mut ::core::ffi::c_void,
            );

            let video_caps = video_caps.assume_init();

            query_error!(r);

            return Ok(video_caps);
        }
    }
}

impl drm_amdgpu_info_video_caps {
    pub fn get_codec_info(&self, codec: CODEC) -> drm_amdgpu_info_video_codec_info {
        self.codec_info[codec as usize]
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum CODEC {
    MPEG2 = AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG2,
    MPEG4 = AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4,
    VC1 = AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VC1,
    MPEG4_AVC = AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4_AVC,
    HEVC = AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_HEVC,
    JPEG = AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_JPEG,
    VP9 = AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VP9,
    AV1 = AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_AV1,
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for CODEC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl drm_amdgpu_info_video_codec_info {
    pub fn is_supported(&self) -> bool {
        self.valid != 0
    }
}
