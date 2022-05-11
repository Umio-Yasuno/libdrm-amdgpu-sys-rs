/* TODO: WIP */

use crate::*;
use crate::AMDGPU::*;
// use super::*;
use std::mem::{MaybeUninit, size_of};

use bindings::{
    drm_amdgpu_info_video_caps,
    drm_amdgpu_info_video_codec_info,
    AMDGPU_INFO_VIDEO_CAPS_DECODE,
    AMDGPU_INFO_VIDEO_CAPS_ENCODE,

    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG2,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VC1,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4_AVC,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_HEVC,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_JPEG,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VP9,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_AV1,
};

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum CAP_TYPE {
    DECODE = AMDGPU_INFO_VIDEO_CAPS_DECODE,
    ENCODE = AMDGPU_INFO_VIDEO_CAPS_ENCODE,
}

pub trait VIDEO_CAPS {
    fn get_video_caps(self, type_: CAP_TYPE) -> Result<drm_amdgpu_info_video_caps, i32>;
}

impl VIDEO_CAPS for DEVICE_HANDLE {
    fn get_video_caps(self, type_: CAP_TYPE) -> Result<drm_amdgpu_info_video_caps, i32> {
        unsafe {
            let mut video_caps: MaybeUninit<drm_amdgpu_info_video_caps> = MaybeUninit::uninit();

            let r = bindings::amdgpu_query_video_caps_info(
                self,
                type_ as u32,
                size_of::<drm_amdgpu_info_video_caps> as u32,
                video_caps.as_mut_ptr() as *mut ::std::os::raw::c_void,
            );

            query_error!(r);

            let video_caps = video_caps.assume_init();

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

use std::fmt;
impl fmt::Display for CODEC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MPEG2 => write!(f, "MPEG2"),
            Self::MPEG4 => write!(f, "MPEG4"),
            Self::VC1 => write!(f, "VC1"),
            Self::MPEG4_AVC => write!(f, "MPEG4_AVC"),
            Self::HEVC => write!(f, "HEVC"),
            Self::JPEG => write!(f, "JPEG"),
            Self::VP9 => write!(f, "VP9"),
            Self::AV1 => write!(f, "AV1"),
        }
    }
}

impl drm_amdgpu_info_video_codec_info {
    pub fn is_supported(&self) -> bool {
        self.valid != 0
    }
}
