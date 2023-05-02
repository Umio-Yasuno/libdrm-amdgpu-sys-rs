use crate::*;
use crate::AMDGPU::*;
use core::mem::{size_of, MaybeUninit};
pub use bindings::{drm_amdgpu_info_video_caps, drm_amdgpu_info_video_codec_info};

#[derive(Debug, Clone, Copy)]
pub struct VideoCapsInfo {
    pub cap_type: CAP_TYPE,
    pub mpeg2: Option<drm_amdgpu_info_video_codec_info>,
    pub mpeg4: Option<drm_amdgpu_info_video_codec_info>,
    pub vc1: Option<drm_amdgpu_info_video_codec_info>,
    pub mpeg4_avc: Option<drm_amdgpu_info_video_codec_info>,
    pub hevc: Option<drm_amdgpu_info_video_codec_info>,
    pub jpeg: Option<drm_amdgpu_info_video_codec_info>,
    pub vp9: Option<drm_amdgpu_info_video_codec_info>,
    pub av1: Option<drm_amdgpu_info_video_codec_info>,
}

impl From<(&CAP_TYPE, &drm_amdgpu_info_video_caps)> for VideoCapsInfo {
    fn from(caps: (&CAP_TYPE, &drm_amdgpu_info_video_caps)) -> Self {
        let (cap_type, video_caps) = caps;
        let [mpeg2, mpeg4, vc1, mpeg4_avc, hevc, jpeg, vp9, av1] = CODEC::LIST
            .map(|codec| {
                let info = video_caps.get_codec_info(codec);

                (info.valid != 0).then_some(info)
            });

        Self {
            cap_type: *cap_type,
            mpeg2,
            mpeg4,
            vc1,
            mpeg4_avc,
            hevc,
            jpeg,
            vp9,
            av1,
        }
    }
}

impl DeviceHandle {
    pub fn get_video_caps_info(&self, cap_type: CAP_TYPE) -> Result<VideoCapsInfo, i32> {
        let cap = self.get_video_caps(cap_type)?;

        Ok(VideoCapsInfo::from((&cap_type, &cap)))
    }
}

use bindings::{
    AMDGPU_INFO_VIDEO_CAPS_DECODE,
    AMDGPU_INFO_VIDEO_CAPS_ENCODE,
};

/// Used for [DeviceHandle::get_video_caps]
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
                size_of::<drm_amdgpu_info_video_caps>() as u32,
                video_caps.as_mut_ptr() as *mut ::core::ffi::c_void,
            );

            let video_caps = video_caps.assume_init();

            query_error!(r);

            Ok(video_caps)
        }
    }
}

impl drm_amdgpu_info_video_caps {
    pub fn get_codec_info(&self, codec: CODEC) -> drm_amdgpu_info_video_codec_info {
        self.codec_info[codec as usize]
    }
}

use bindings::{
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG2,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VC1,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4_AVC,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_HEVC,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_JPEG,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VP9,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_AV1,
    AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_COUNT,
};

/// Used for [drm_amdgpu_info_video_caps::get_codec_info]
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

impl CODEC {
    pub const LIST: [Self; AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_COUNT as usize] = [
        Self::MPEG2,
        Self::MPEG4,
        Self::VC1,
        Self::MPEG4_AVC,
        Self::HEVC,
        Self::JPEG,
        Self::VP9,
        Self::AV1,
    ];
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
