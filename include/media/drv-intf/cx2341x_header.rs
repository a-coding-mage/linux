/* SPDX-License-Identifier: GPL-2.0-or-later */
/* cx23415/6/8 header containing common defines. */

/* Dependency: media/v4l2-ctrls.h */

#[repr(i32)]
pub enum cx2341x_port {
    CX2341X_PORT_MEMORY = 0,
    CX2341X_PORT_STREAMING = 1,
    CX2341X_PORT_SERIAL = 2,
}

pub const CX2341X_CAP_HAS_SLICED_VBI: u32 = 1 << 0;
pub const CX2341X_CAP_HAS_TS: u32 = 1 << 1;
pub const CX2341X_CAP_HAS_AC3: u32 = 1 << 2;

#[repr(C)]
pub struct cx2341x_mpeg_params {
    pub capabilities: u32,
    pub port: cx2341x_port,
    pub width: u16,
    pub height: u16,
    pub is_50hz: u16,
    pub stream_type: v4l2_mpeg_stream_type,
    pub stream_vbi_fmt: v4l2_mpeg_stream_vbi_fmt,
    pub stream_insert_nav_packets: u16,
    pub audio_sampling_freq: v4l2_mpeg_audio_sampling_freq,
    pub audio_encoding: v4l2_mpeg_audio_encoding,
    pub audio_l2_bitrate: v4l2_mpeg_audio_l2_bitrate,
    pub audio_ac3_bitrate: v4l2_mpeg_audio_ac3_bitrate,
    pub audio_mode: v4l2_mpeg_audio_mode,
    pub audio_mode_extension: v4l2_mpeg_audio_mode_extension,
    pub audio_emphasis: v4l2_mpeg_audio_emphasis,
    pub audio_crc: v4l2_mpeg_audio_crc,
    pub audio_properties: u32,
    pub audio_mute: u16,
    pub video_encoding: v4l2_mpeg_video_encoding,
    pub video_aspect: v4l2_mpeg_video_aspect,
    pub video_b_frames: u16,
    pub video_gop_size: u16,
    pub video_gop_closure: u16,
    pub video_bitrate_mode: v4l2_mpeg_video_bitrate_mode,
    pub video_bitrate: u32,
    pub video_bitrate_peak: u32,
    pub video_temporal_decimation: u16,
    pub video_mute: u16,
    pub video_mute_yuv: u32,
    pub video_spatial_filter_mode: v4l2_mpeg_cx2341x_video_spatial_filter_mode,
    pub video_spatial_filter: u16,
    pub video_luma_spatial_filter_type: v4l2_mpeg_cx2341x_video_luma_spatial_filter_type,
    pub video_chroma_spatial_filter_type: v4l2_mpeg_cx2341x_video_chroma_spatial_filter_type,
    pub video_temporal_filter_mode: v4l2_mpeg_cx2341x_video_temporal_filter_mode,
    pub video_temporal_filter: u16,
    pub video_median_filter_type: v4l2_mpeg_cx2341x_video_median_filter_type,
    pub video_luma_median_filter_top: u16,
    pub video_luma_median_filter_bottom: u16,
    pub video_chroma_median_filter_top: u16,
    pub video_chroma_median_filter_bottom: u16,
}

pub const CX2341X_MBOX_MAX_DATA: usize = 16;

extern "C" {
    pub static cx2341x_mpeg_ctrls: u32;
}

pub type cx2341x_mbox_func = unsafe extern "C" fn(
    priv_: *mut core::ffi::c_void, cmd: u32, input: i32, out: i32,
    data: *mut u32,
) -> i32;

extern "C" {
    pub fn cx2341x_update(priv_: *mut core::ffi::c_void, func: cx2341x_mbox_func,
        old: *const cx2341x_mpeg_params, new: *const cx2341x_mpeg_params) -> i32;
    pub fn cx2341x_ctrl_query(params: *const cx2341x_mpeg_params, qctrl: *mut v4l2_queryctrl) -> i32;
    pub fn cx2341x_ctrl_get_menu(p: *const cx2341x_mpeg_params, id: u32) -> *const *const core::ffi::c_char;
    pub fn cx2341x_ext_ctrls(params: *mut cx2341x_mpeg_params, busy: i32,
        ctrls: *mut v4l2_ext_controls, cmd: u32) -> i32;
    pub fn cx2341x_fill_defaults(p: *mut cx2341x_mpeg_params);
    pub fn cx2341x_log_status(p: *const cx2341x_mpeg_params, prefix: *const core::ffi::c_char);
}

pub enum cx2341x_handler {}

#[repr(C)]
pub struct cx2341x_handler_ops {
    pub s_audio_sampling_freq: Option<unsafe extern "C" fn(*mut cx2341x_handler, u32) -> i32>,
    pub s_audio_mode: Option<unsafe extern "C" fn(*mut cx2341x_handler, u32) -> i32>,
    pub s_video_encoding: Option<unsafe extern "C" fn(*mut cx2341x_handler, u32) -> i32>,
    pub s_stream_vbi_fmt: Option<unsafe extern "C" fn(*mut cx2341x_handler, u32) -> i32>,
}

#[repr(C)]
pub struct cx2341x_handler {
    pub capabilities: u32,
    pub port: cx2341x_port,
    pub width: u16,
    pub height: u16,
    pub is_50hz: u16,
    pub audio_properties: u32,
    pub hdl: v4l2_ctrl_handler,
    pub priv_: *mut core::ffi::c_void,
    pub func: cx2341x_mbox_func,
    pub ops: *const cx2341x_handler_ops,
    pub stream_vbi_fmt: *mut v4l2_ctrl,
    pub audio_sampling_freq: *mut v4l2_ctrl,
    pub audio_encoding: *mut v4l2_ctrl,
    pub audio_l2_bitrate: *mut v4l2_ctrl,
    pub audio_mode: *mut v4l2_ctrl,
    pub audio_mode_extension: *mut v4l2_ctrl,
    pub audio_emphasis: *mut v4l2_ctrl,
    pub audio_crc: *mut v4l2_ctrl,
    pub audio_ac3_bitrate: *mut v4l2_ctrl,
    pub video_b_frames: *mut v4l2_ctrl,
    pub video_gop_size: *mut v4l2_ctrl,
    pub stream_type: *mut v4l2_ctrl,
    pub video_encoding: *mut v4l2_ctrl,
    pub video_bitrate_mode: *mut v4l2_ctrl,
    pub video_bitrate: *mut v4l2_ctrl,
    pub video_bitrate_peak: *mut v4l2_ctrl,
    pub video_mute: *mut v4l2_ctrl,
    pub video_mute_yuv: *mut v4l2_ctrl,
    pub video_spatial_filter_mode: *mut v4l2_ctrl,
    pub video_temporal_filter_mode: *mut v4l2_ctrl,
    pub video_median_filter_type: *mut v4l2_ctrl,
    pub video_luma_spatial_filter_type: *mut v4l2_ctrl,
    pub video_chroma_spatial_filter_type: *mut v4l2_ctrl,
    pub video_spatial_filter: *mut v4l2_ctrl,
    pub video_temporal_filter: *mut v4l2_ctrl,
    pub video_luma_median_filter_top: *mut v4l2_ctrl,
    pub video_luma_median_filter_bottom: *mut v4l2_ctrl,
    pub video_chroma_median_filter_top: *mut v4l2_ctrl,
    pub video_chroma_median_filter_bottom: *mut v4l2_ctrl,
}

extern "C" {
    pub fn cx2341x_handler_init(cxhdl: *mut cx2341x_handler, nr_of_controls_hint: u32) -> i32;
    pub fn cx2341x_handler_set_50hz(cxhdl: *mut cx2341x_handler, is_50hz: i32);
    pub fn cx2341x_handler_setup(cxhdl: *mut cx2341x_handler) -> i32;
    pub fn cx2341x_handler_set_busy(cxhdl: *mut cx2341x_handler, busy: i32);
}

pub const CX2341X_FIRM_ENC_FILENAME: &str = "v4l-cx2341x-enc.fw";
pub const CX2341X_FIRM_DEC_FILENAME: &str = "v4l-cx2341x-dec.fw";

pub const CX2341X_DEC_PING_FW: u32 = 0x00;
pub const CX2341X_DEC_START_PLAYBACK: u32 = 0x01;
pub const CX2341X_DEC_STOP_PLAYBACK: u32 = 0x02;
pub const CX2341X_DEC_SET_PLAYBACK_SPEED: u32 = 0x03;
pub const CX2341X_DEC_STEP_VIDEO: u32 = 0x05;
pub const CX2341X_DEC_SET_DMA_BLOCK_SIZE: u32 = 0x08;
pub const CX2341X_DEC_GET_XFER_INFO: u32 = 0x09;
pub const CX2341X_DEC_GET_DMA_STATUS: u32 = 0x0a;
pub const CX2341X_DEC_SCHED_DMA_FROM_HOST: u32 = 0x0b;
pub const CX2341X_DEC_PAUSE_PLAYBACK: u32 = 0x0d;
pub const CX2341X_DEC_HALT_FW: u32 = 0x0e;
pub const CX2341X_DEC_SET_STANDARD: u32 = 0x10;
pub const CX2341X_DEC_GET_VERSION: u32 = 0x11;
pub const CX2341X_DEC_SET_STREAM_INPUT: u32 = 0x14;
pub const CX2341X_DEC_GET_TIMING_INFO: u32 = 0x15;
pub const CX2341X_DEC_SET_AUDIO_MODE: u32 = 0x16;
pub const CX2341X_DEC_SET_EVENT_NOTIFICATION: u32 = 0x17;
pub const CX2341X_DEC_SET_DISPLAY_BUFFERS: u32 = 0x18;
pub const CX2341X_DEC_EXTRACT_VBI: u32 = 0x19;
pub const CX2341X_DEC_SET_DECODER_SOURCE: u32 = 0x1a;
pub const CX2341X_DEC_SET_PREBUFFERING: u32 = 0x1e;

pub const CX2341X_ENC_PING_FW: u32 = 0x80;
pub const CX2341X_ENC_START_CAPTURE: u32 = 0x81;
pub const CX2341X_ENC_STOP_CAPTURE: u32 = 0x82;
pub const CX2341X_ENC_SET_AUDIO_ID: u32 = 0x89;
pub const CX2341X_ENC_SET_VIDEO_ID: u32 = 0x8b;
pub const CX2341X_ENC_SET_PCR_ID: u32 = 0x8d;
pub const CX2341X_ENC_SET_FRAME_RATE: u32 = 0x8f;
pub const CX2341X_ENC_SET_FRAME_SIZE: u32 = 0x91;
pub const CX2341X_ENC_SET_BIT_RATE: u32 = 0x95;
pub const CX2341X_ENC_SET_GOP_PROPERTIES: u32 = 0x97;
pub const CX2341X_ENC_SET_ASPECT_RATIO: u32 = 0x99;
pub const CX2341X_ENC_SET_DNR_FILTER_MODE: u32 = 0x9b;
pub const CX2341X_ENC_SET_DNR_FILTER_PROPS: u32 = 0x9d;
pub const CX2341X_ENC_SET_CORING_LEVELS: u32 = 0x9f;
pub const CX2341X_ENC_SET_SPATIAL_FILTER_TYPE: u32 = 0xa1;
pub const CX2341X_ENC_SET_VBI_LINE: u32 = 0xb7;
pub const CX2341X_ENC_SET_STREAM_TYPE: u32 = 0xb9;
pub const CX2341X_ENC_SET_OUTPUT_PORT: u32 = 0xbb;
pub const CX2341X_ENC_SET_AUDIO_PROPERTIES: u32 = 0xbd;
pub const CX2341X_ENC_HALT_FW: u32 = 0xc3;
pub const CX2341X_ENC_GET_VERSION: u32 = 0xc4;
pub const CX2341X_ENC_SET_GOP_CLOSURE: u32 = 0xc5;
pub const CX2341X_ENC_GET_SEQ_END: u32 = 0xc6;
pub const CX2341X_ENC_SET_PGM_INDEX_INFO: u32 = 0xc7;
pub const CX2341X_ENC_SET_VBI_CONFIG: u32 = 0xc8;
pub const CX2341X_ENC_SET_DMA_BLOCK_SIZE: u32 = 0xc9;
pub const CX2341X_ENC_GET_PREV_DMA_INFO_MB_10: u32 = 0xca;
pub const CX2341X_ENC_GET_PREV_DMA_INFO_MB_9: u32 = 0xcb;
pub const CX2341X_ENC_SCHED_DMA_TO_HOST: u32 = 0xcc;
pub const CX2341X_ENC_INITIALIZE_INPUT: u32 = 0xcd;
pub const CX2341X_ENC_SET_FRAME_DROP_RATE: u32 = 0xd0;
pub const CX2341X_ENC_PAUSE_ENCODER: u32 = 0xd2;
pub const CX2341X_ENC_REFRESH_INPUT: u32 = 0xd3;
pub const CX2341X_ENC_SET_COPYRIGHT: u32 = 0xd4;
pub const CX2341X_ENC_SET_EVENT_NOTIFICATION: u32 = 0xd5;
pub const CX2341X_ENC_SET_NUM_VSYNC_LINES: u32 = 0xd6;
pub const CX2341X_ENC_SET_PLACEHOLDER: u32 = 0xd7;
pub const CX2341X_ENC_MUTE_VIDEO: u32 = 0xd9;
pub const CX2341X_ENC_MUTE_AUDIO: u32 = 0xda;
pub const CX2341X_ENC_SET_VERT_CROP_LINE: u32 = 0xdb;
pub const CX2341X_ENC_MISC: u32 = 0xdc;

pub const CX2341X_OSD_GET_FRAMEBUFFER: u32 = 0x41;
pub const CX2341X_OSD_GET_PIXEL_FORMAT: u32 = 0x42;
pub const CX2341X_OSD_SET_PIXEL_FORMAT: u32 = 0x43;
pub const CX2341X_OSD_GET_STATE: u32 = 0x44;
pub const CX2341X_OSD_SET_STATE: u32 = 0x45;
pub const CX2341X_OSD_GET_OSD_COORDS: u32 = 0x46;
pub const CX2341X_OSD_SET_OSD_COORDS: u32 = 0x47;
pub const CX2341X_OSD_GET_SCREEN_COORDS: u32 = 0x48;
pub const CX2341X_OSD_SET_SCREEN_COORDS: u32 = 0x49;
pub const CX2341X_OSD_GET_GLOBAL_ALPHA: u32 = 0x4a;
pub const CX2341X_OSD_SET_GLOBAL_ALPHA: u32 = 0x4b;
pub const CX2341X_OSD_SET_BLEND_COORDS: u32 = 0x4c;
pub const CX2341X_OSD_GET_FLICKER_STATE: u32 = 0x4f;
pub const CX2341X_OSD_SET_FLICKER_STATE: u32 = 0x50;
pub const CX2341X_OSD_BLT_COPY: u32 = 0x52;
pub const CX2341X_OSD_BLT_FILL: u32 = 0x53;
pub const CX2341X_OSD_BLT_TEXT: u32 = 0x54;
pub const CX2341X_OSD_SET_FRAMEBUFFER_WINDOW: u32 = 0x56;
pub const CX2341X_OSD_SET_CHROMA_KEY: u32 = 0x60;
pub const CX2341X_OSD_GET_ALPHA_CONTENT_INDEX: u32 = 0x61;
pub const CX2341X_OSD_SET_ALPHA_CONTENT_INDEX: u32 = 0x62;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
