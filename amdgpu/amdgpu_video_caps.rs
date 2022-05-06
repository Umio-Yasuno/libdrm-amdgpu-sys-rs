use crate::*;

use bindings::{
};

#[repr(u32)]
enum VIDEO_CAPS {
    DECODE,
    ENCODE,
}

#[repr(u32)]
enum VIDEO_CODEC {
    MPEG2,
    MPEG4,
    VC1,
    MPEG4_AVC,
    HEVC,
    JPEG,
    VP9,
    AV1,
}
