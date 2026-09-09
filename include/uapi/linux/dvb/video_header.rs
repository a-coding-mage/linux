/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/*
 * video.h - DEPRECATED MPEG-TS video decoder API
 *
 * NOTE: should not be used on future drivers
 *
 * Copyright (C) 2000 Marcus Metzler <marcus@convergence.de>
 *                  & Ralph  Metzler <ralph@convergence.de>
 *                    for convergence integrated media GmbH
 */

use core::ffi::{c_char, c_long};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum video_format_t {
    VIDEO_FORMAT_4_3 = 0,
    VIDEO_FORMAT_16_9 = 1,
    VIDEO_FORMAT_221_1 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum video_displayformat_t {
    VIDEO_PAN_SCAN = 0,
    VIDEO_LETTER_BOX = 1,
    VIDEO_CENTER_CUT_OUT = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_size_t {
    pub w: i32,
    pub h: i32,
    pub aspect_ratio: video_format_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum video_stream_source_t {
    VIDEO_SOURCE_DEMUX = 0,
    VIDEO_SOURCE_MEMORY = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum video_play_state_t {
    VIDEO_STOPPED = 0,
    VIDEO_PLAYING = 1,
    VIDEO_FREEZED = 2,
}

pub const VIDEO_CMD_PLAY: u32 = 0;
pub const VIDEO_CMD_STOP: u32 = 1;
pub const VIDEO_CMD_FREEZE: u32 = 2;
pub const VIDEO_CMD_CONTINUE: u32 = 3;
pub const VIDEO_CMD_FREEZE_TO_BLACK: u32 = 1 << 0;
pub const VIDEO_CMD_STOP_TO_BLACK: u32 = 1 << 0;
pub const VIDEO_CMD_STOP_IMMEDIATELY: u32 = 1 << 1;
pub const VIDEO_PLAY_FMT_NONE: u32 = 0;
pub const VIDEO_PLAY_FMT_GOP: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_command_stop {
    pub pts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_command_play {
    pub speed: i32,
    pub format: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_command_raw {
    pub data: [u32; 16],
}

#[repr(C)]
pub union video_command_u {
    pub stop: video_command_stop,
    pub play: video_command_play,
    pub raw: video_command_raw,
}

#[repr(C)]
pub struct video_command {
    pub cmd: u32,
    pub flags: u32,
    pub u: video_command_u,
}

pub const VIDEO_VSYNC_FIELD_UNKNOWN: u32 = 0;
pub const VIDEO_VSYNC_FIELD_ODD: u32 = 1;
pub const VIDEO_VSYNC_FIELD_EVEN: u32 = 2;
pub const VIDEO_VSYNC_FIELD_PROGRESSIVE: u32 = 3;

pub const VIDEO_EVENT_SIZE_CHANGED: i32 = 1;
pub const VIDEO_EVENT_FRAME_RATE_CHANGED: i32 = 2;
pub const VIDEO_EVENT_DECODER_STOPPED: i32 = 3;
pub const VIDEO_EVENT_VSYNC: i32 = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub union video_event_u {
    pub size: video_size_t,
    pub frame_rate: u32,
    pub vsync_field: u8,
}

#[repr(C)]
pub struct video_event {
    pub type_: i32,
    pub timestamp: c_long,
    pub u: video_event_u,
}

#[repr(C)]
pub struct video_status {
    pub video_blank: i32,
    pub play_state: video_play_state_t,
    pub stream_source: video_stream_source_t,
    pub video_format: video_format_t,
    pub display_format: video_displayformat_t,
}

#[repr(C)]
pub struct video_still_picture {
    pub iFrame: *mut c_char,
    pub size: i32,
}

pub type video_attributes_t = u16;

pub const VIDEO_CAP_MPEG1: u32 = 1;
pub const VIDEO_CAP_MPEG2: u32 = 2;
pub const VIDEO_CAP_SYS: u32 = 4;
pub const VIDEO_CAP_PROG: u32 = 8;
pub const VIDEO_CAP_SPU: u32 = 16;
pub const VIDEO_CAP_NAVI: u32 = 32;
pub const VIDEO_CAP_CSS: u32 = 64;

/* ioctl constants use the platform-provided _IO, _IOR, _IOW, and _IOWR macros. */
pub const VIDEO_STOP: _ = _IO(b'o', 21);
pub const VIDEO_PLAY: _ = _IO(b'o', 22);
pub const VIDEO_FREEZE: _ = _IO(b'o', 23);
pub const VIDEO_CONTINUE: _ = _IO(b'o', 24);
pub const VIDEO_SELECT_SOURCE: _ = _IO(b'o', 25);
pub const VIDEO_SET_BLANK: _ = _IO(b'o', 26);
pub const VIDEO_GET_STATUS: _ = _IOR(b'o', 27, video_status);
pub const VIDEO_GET_EVENT: _ = _IOR(b'o', 28, video_event);
pub const VIDEO_SET_DISPLAY_FORMAT: _ = _IO(b'o', 29);
pub const VIDEO_STILLPICTURE: _ = _IOW(b'o', 30, video_still_picture);
pub const VIDEO_FAST_FORWARD: _ = _IO(b'o', 31);
pub const VIDEO_SLOWMOTION: _ = _IO(b'o', 32);
pub const VIDEO_GET_CAPABILITIES: _ = _IOR(b'o', 33, u32);
pub const VIDEO_CLEAR_BUFFER: _ = _IO(b'o', 34);
pub const VIDEO_SET_STREAMTYPE: _ = _IO(b'o', 36);
pub const VIDEO_SET_FORMAT: _ = _IO(b'o', 37);
pub const VIDEO_GET_SIZE: _ = _IOR(b'o', 55, video_size_t);
pub const VIDEO_GET_PTS: _ = _IOR(b'o', 57, u64);
pub const VIDEO_GET_FRAME_COUNT: _ = _IOR(b'o', 58, u64);
pub const VIDEO_COMMAND: _ = _IOWR(b'o', 59, video_command);
pub const VIDEO_TRY_COMMAND: _ = _IOWR(b'o', 60, video_command);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
