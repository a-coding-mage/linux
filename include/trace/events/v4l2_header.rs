/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/v4l2.h.
// The Linux tracepoint and videobuf2 definitions are supplied by external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Enums exported to userspace for trace parsing. */
pub const SHOW_TYPE: &[(u32, &str)] = &[
    (V4L2_BUF_TYPE_VIDEO_CAPTURE, "VIDEO_CAPTURE"),
    (V4L2_BUF_TYPE_VIDEO_OUTPUT, "VIDEO_OUTPUT"),
    (V4L2_BUF_TYPE_VIDEO_OVERLAY, "VIDEO_OVERLAY"),
    (V4L2_BUF_TYPE_VBI_CAPTURE, "VBI_CAPTURE"),
    (V4L2_BUF_TYPE_VBI_OUTPUT, "VBI_OUTPUT"),
    (V4L2_BUF_TYPE_SLICED_VBI_CAPTURE, "SLICED_VBI_CAPTURE"),
    (V4L2_BUF_TYPE_SLICED_VBI_OUTPUT, "SLICED_VBI_OUTPUT"),
    (V4L2_BUF_TYPE_VIDEO_OUTPUT_OVERLAY, "VIDEO_OUTPUT_OVERLAY"),
    (V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE, "VIDEO_CAPTURE_MPLANE"),
    (V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE, "VIDEO_OUTPUT_MPLANE"),
    (V4L2_BUF_TYPE_SDR_CAPTURE, "SDR_CAPTURE"),
    (V4L2_BUF_TYPE_SDR_OUTPUT, "SDR_OUTPUT"),
    (V4L2_BUF_TYPE_META_CAPTURE, "META_CAPTURE"),
    (V4L2_BUF_TYPE_PRIVATE, "PRIVATE"),
];

pub const SHOW_FIELD: &[(u32, &str)] = &[
    (V4L2_FIELD_ANY, "ANY"), (V4L2_FIELD_NONE, "NONE"),
    (V4L2_FIELD_TOP, "TOP"), (V4L2_FIELD_BOTTOM, "BOTTOM"),
    (V4L2_FIELD_INTERLACED, "INTERLACED"), (V4L2_FIELD_SEQ_TB, "SEQ_TB"),
    (V4L2_FIELD_SEQ_BT, "SEQ_BT"), (V4L2_FIELD_ALTERNATE, "ALTERNATE"),
    (V4L2_FIELD_INTERLACED_TB, "INTERLACED_TB"),
    (V4L2_FIELD_INTERLACED_BT, "INTERLACED_BT"),
];

pub const SHOW_TIMECODE_TYPE: &[(u32, &str)] = &[
    (V4L2_TC_TYPE_24FPS, "24FPS"), (V4L2_TC_TYPE_25FPS, "25FPS"),
    (V4L2_TC_TYPE_30FPS, "30FPS"), (V4L2_TC_TYPE_50FPS, "50FPS"),
    (V4L2_TC_TYPE_60FPS, "60FPS"),
];

pub const SHOW_FLAGS: &[(u32, &str)] = &[
    (V4L2_BUF_FLAG_MAPPED, "MAPPED"), (V4L2_BUF_FLAG_QUEUED, "QUEUED"),
    (V4L2_BUF_FLAG_DONE, "DONE"), (V4L2_BUF_FLAG_KEYFRAME, "KEYFRAME"),
    (V4L2_BUF_FLAG_PFRAME, "PFRAME"), (V4L2_BUF_FLAG_BFRAME, "BFRAME"),
    (V4L2_BUF_FLAG_ERROR, "ERROR"), (V4L2_BUF_FLAG_TIMECODE, "TIMECODE"),
    (V4L2_BUF_FLAG_PREPARED, "PREPARED"),
    (V4L2_BUF_FLAG_NO_CACHE_INVALIDATE, "NO_CACHE_INVALIDATE"),
    (V4L2_BUF_FLAG_NO_CACHE_CLEAN, "NO_CACHE_CLEAN"),
    (V4L2_BUF_FLAG_TIMESTAMP_MASK, "TIMESTAMP_MASK"),
    (V4L2_BUF_FLAG_TIMESTAMP_UNKNOWN, "TIMESTAMP_UNKNOWN"),
    (V4L2_BUF_FLAG_TIMESTAMP_MONOTONIC, "TIMESTAMP_MONOTONIC"),
    (V4L2_BUF_FLAG_TIMESTAMP_COPY, "TIMESTAMP_COPY"),
    (V4L2_BUF_FLAG_LAST, "LAST"),
];

pub const SHOW_TIMECODE_FLAGS: &[(u32, &str)] = &[
    (V4L2_TC_FLAG_DROPFRAME, "DROPFRAME"),
    (V4L2_TC_FLAG_COLORFRAME, "COLORFRAME"),
    (V4L2_TC_USERBITS_USERDEFINED, "USERBITS_USERDEFINED"),
    (V4L2_TC_USERBITS_8BITCHARS, "USERBITS_8BITCHARS"),
];

/* TP_STRUCT__entry fields for v4l2_event_class. */
#[repr(C)]
pub struct v4l2_event_entry {
    pub minor: i32, pub index: u32, pub type_: u32, pub bytesused: u32,
    pub flags: u32, pub field: u32, pub timestamp: i64,
    pub timecode_type: u32, pub timecode_flags: u32,
    pub timecode_frames: u8, pub timecode_seconds: u8,
    pub timecode_minutes: u8, pub timecode_hours: u8,
    pub timecode_userbits0: u8, pub timecode_userbits1: u8,
    pub timecode_userbits2: u8, pub timecode_userbits3: u8,
    pub sequence: u32,
}

/* TP_STRUCT__entry fields for vb2_v4l2_event_class. */
#[repr(C)]
pub struct vb2_v4l2_event_entry {
    pub minor: i32, pub flags: u32, pub field: u32, pub timestamp: u64,
    pub timecode_type: u32, pub timecode_flags: u32,
    pub timecode_frames: u8, pub timecode_seconds: u8,
    pub timecode_minutes: u8, pub timecode_hours: u8,
    pub timecode_userbits0: u8, pub timecode_userbits1: u8,
    pub timecode_userbits2: u8, pub timecode_userbits3: u8,
    pub sequence: u32,
}

/* DEFINE_EVENT declarations: implementation is provided by the tracepoint subsystem. */
extern "C" {
    pub fn v4l2_dqbuf(minor: i32, buf: *mut v4l2_buffer);
    pub fn v4l2_qbuf(minor: i32, buf: *mut v4l2_buffer);
    pub fn vb2_v4l2_buf_done(q: *mut vb2_queue, vb: *mut vb2_buffer);
    pub fn vb2_v4l2_buf_queue(q: *mut vb2_queue, vb: *mut vb2_buffer);
    pub fn vb2_v4l2_dqbuf(q: *mut vb2_queue, vb: *mut vb2_buffer);
    pub fn vb2_v4l2_qbuf(q: *mut vb2_queue, vb: *mut vb2_buffer);
}

/* v4l2_buffer, vb2_queue, and vb2_buffer are supplied by external headers. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
