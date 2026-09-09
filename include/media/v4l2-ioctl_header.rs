/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of v4l2-ioctl.h.  Kernel-provided types are external. */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)] pub struct file;
#[repr(C)] pub struct v4l2_fh;
#[repr(C)] pub struct v4l2_capability;
#[repr(C)] pub struct v4l2_fmtdesc;
#[repr(C)] pub struct v4l2_format;
#[repr(C)] pub struct v4l2_requestbuffers;
#[repr(C)] pub struct v4l2_buffer;
#[repr(C)] pub struct v4l2_exportbuffer;
#[repr(C)] pub struct v4l2_create_buffers;
#[repr(C)] pub struct v4l2_remove_buffers;
#[repr(C)] pub struct v4l2_framebuffer;
#[repr(C)] pub struct v4l2_input;
#[repr(C)] pub struct v4l2_output;
#[repr(C)] pub struct v4l2_query_ext_ctrl;
#[repr(C)] pub struct v4l2_ext_controls;
#[repr(C)] pub struct v4l2_querymenu;
#[repr(C)] pub struct v4l2_audio;
#[repr(C)] pub struct v4l2_audioout;
#[repr(C)] pub struct v4l2_modulator;
#[repr(C)] pub struct v4l2_fract;
#[repr(C)] pub struct v4l2_selection;
#[repr(C)] pub struct v4l2_jpegcompression;
#[repr(C)] pub struct v4l2_enc_idx;
#[repr(C)] pub struct v4l2_encoder_cmd;
#[repr(C)] pub struct v4l2_decoder_cmd;
#[repr(C)] pub struct v4l2_streamparm;
#[repr(C)] pub struct v4l2_tuner;
#[repr(C)] pub struct v4l2_frequency;
#[repr(C)] pub struct v4l2_frequency_band;
#[repr(C)] pub struct v4l2_sliced_vbi_cap;
#[repr(C)] pub struct v4l2_hw_freq_seek;
#[repr(C)] pub struct v4l2_dbg_register;
#[repr(C)] pub struct v4l2_dbg_chip_info;
#[repr(C)] pub struct v4l2_frmsizeenum;
#[repr(C)] pub struct v4l2_frmivalenum;
#[repr(C)] pub struct v4l2_dv_timings;
#[repr(C)] pub struct v4l2_enum_dv_timings;
#[repr(C)] pub struct v4l2_dv_timings_cap;
#[repr(C)] pub struct v4l2_edid;
#[repr(C)] pub struct v4l2_event_subscription;
#[repr(C)] pub struct v4l2_standard;
#[repr(C)] pub struct v4l2_event_vsync;
#[repr(C)] pub struct v4l2_event_ctrl;
#[repr(C)] pub struct v4l2_event_frame_sync;
#[repr(C)] pub struct v4l2_event_src_change;
#[repr(C)] pub struct v4l2_event_motion_det;
#[repr(C)] pub struct v4l2_timecode;
#[repr(C)] pub struct old_timespec32;
#[repr(C)] pub struct old_timeval32;
#[repr(C)] pub struct v4l2_plane;
pub type v4l2_std_id = u64;
pub type v4l2_buf_type = c_int;

type IoctlCb = Option<unsafe extern "C" fn(*mut file, *mut c_void, *mut c_void) -> c_int>;
type IoctlCbConst = Option<unsafe extern "C" fn(*mut file, *mut c_void, *const c_void) -> c_int>;

/* ioctl callbacks; callback signatures retain the C ABI and raw-pointer behavior. */
#[repr(C)]
pub struct v4l2_ioctl_ops {
    pub vidioc_querycap: IoctlCb,
    pub vidioc_enum_fmt_vid_cap: IoctlCb, pub vidioc_enum_fmt_vid_overlay: IoctlCb,
    pub vidioc_enum_fmt_vid_out: IoctlCb, pub vidioc_enum_fmt_sdr_cap: IoctlCb,
    pub vidioc_enum_fmt_sdr_out: IoctlCb, pub vidioc_enum_fmt_meta_cap: IoctlCb,
    pub vidioc_enum_fmt_meta_out: IoctlCb,
    pub vidioc_g_fmt_vid_cap: IoctlCb, pub vidioc_g_fmt_vid_overlay: IoctlCb,
    pub vidioc_g_fmt_vid_out: IoctlCb, pub vidioc_g_fmt_vid_out_overlay: IoctlCb,
    pub vidioc_g_fmt_vbi_cap: IoctlCb, pub vidioc_g_fmt_vbi_out: IoctlCb,
    pub vidioc_g_fmt_sliced_vbi_cap: IoctlCb, pub vidioc_g_fmt_sliced_vbi_out: IoctlCb,
    pub vidioc_g_fmt_vid_cap_mplane: IoctlCb, pub vidioc_g_fmt_vid_out_mplane: IoctlCb,
    pub vidioc_g_fmt_sdr_cap: IoctlCb, pub vidioc_g_fmt_sdr_out: IoctlCb,
    pub vidioc_g_fmt_meta_cap: IoctlCb, pub vidioc_g_fmt_meta_out: IoctlCb,
    pub vidioc_s_fmt_vid_cap: IoctlCb, pub vidioc_s_fmt_vid_overlay: IoctlCb,
    pub vidioc_s_fmt_vid_out: IoctlCb, pub vidioc_s_fmt_vid_out_overlay: IoctlCb,
    pub vidioc_s_fmt_vbi_cap: IoctlCb, pub vidioc_s_fmt_vbi_out: IoctlCb,
    pub vidioc_s_fmt_sliced_vbi_cap: IoctlCb, pub vidioc_s_fmt_sliced_vbi_out: IoctlCb,
    pub vidioc_s_fmt_vid_cap_mplane: IoctlCb, pub vidioc_s_fmt_vid_out_mplane: IoctlCb,
    pub vidioc_s_fmt_sdr_cap: IoctlCb, pub vidioc_s_fmt_sdr_out: IoctlCb,
    pub vidioc_s_fmt_meta_cap: IoctlCb, pub vidioc_s_fmt_meta_out: IoctlCb,
    pub vidioc_try_fmt_vid_cap: IoctlCb, pub vidioc_try_fmt_vid_overlay: IoctlCb,
    pub vidioc_try_fmt_vid_out: IoctlCb, pub vidioc_try_fmt_vid_out_overlay: IoctlCb,
    pub vidioc_try_fmt_vbi_cap: IoctlCb, pub vidioc_try_fmt_vbi_out: IoctlCb,
    pub vidioc_try_fmt_sliced_vbi_cap: IoctlCb, pub vidioc_try_fmt_sliced_vbi_out: IoctlCb,
    pub vidioc_try_fmt_vid_cap_mplane: IoctlCb, pub vidioc_try_fmt_vid_out_mplane: IoctlCb,
    pub vidioc_try_fmt_sdr_cap: IoctlCb, pub vidioc_try_fmt_sdr_out: IoctlCb,
    pub vidioc_try_fmt_meta_cap: IoctlCb, pub vidioc_try_fmt_meta_out: IoctlCb,
    pub vidioc_reqbufs: IoctlCb, pub vidioc_querybuf: IoctlCb, pub vidioc_qbuf: IoctlCb,
    pub vidioc_expbuf: IoctlCb, pub vidioc_dqbuf: IoctlCb, pub vidioc_create_bufs: IoctlCb,
    pub vidioc_prepare_buf: IoctlCb, pub vidioc_remove_bufs: IoctlCb,
    pub vidioc_overlay: IoctlCb, pub vidioc_g_fbuf: IoctlCb, pub vidioc_s_fbuf: IoctlCb,
    pub vidioc_streamon: IoctlCb, pub vidioc_streamoff: IoctlCb,
    pub vidioc_g_std: IoctlCb, pub vidioc_s_std: IoctlCb, pub vidioc_querystd: IoctlCb,
    pub vidioc_enum_input: IoctlCb, pub vidioc_g_input: IoctlCb, pub vidioc_s_input: IoctlCb,
    pub vidioc_enum_output: IoctlCb, pub vidioc_g_output: IoctlCb, pub vidioc_s_output: IoctlCb,
    pub vidioc_query_ext_ctrl: IoctlCb, pub vidioc_g_ext_ctrls: IoctlCb,
    pub vidioc_s_ext_ctrls: IoctlCb, pub vidioc_try_ext_ctrls: IoctlCb, pub vidioc_querymenu: IoctlCb,
    pub vidioc_enumaudio: IoctlCb, pub vidioc_g_audio: IoctlCb, pub vidioc_s_audio: IoctlCb,
    pub vidioc_enumaudout: IoctlCb, pub vidioc_g_audout: IoctlCb, pub vidioc_s_audout: IoctlCb,
    pub vidioc_g_modulator: IoctlCb, pub vidioc_s_modulator: IoctlCb,
    pub vidioc_g_pixelaspect: IoctlCb, pub vidioc_g_selection: IoctlCb, pub vidioc_s_selection: IoctlCb,
    pub vidioc_g_jpegcomp: IoctlCb, pub vidioc_s_jpegcomp: IoctlCb, pub vidioc_g_enc_index: IoctlCb,
    pub vidioc_encoder_cmd: IoctlCb, pub vidioc_try_encoder_cmd: IoctlCb,
    pub vidioc_decoder_cmd: IoctlCb, pub vidioc_try_decoder_cmd: IoctlCb,
    pub vidioc_g_parm: IoctlCb, pub vidioc_s_parm: IoctlCb, pub vidioc_g_tuner: IoctlCb,
    pub vidioc_s_tuner: IoctlCb, pub vidioc_g_frequency: IoctlCb, pub vidioc_s_frequency: IoctlCb,
    pub vidioc_enum_freq_bands: IoctlCb, pub vidioc_g_sliced_vbi_cap: IoctlCb,
    pub vidioc_log_status: IoctlCb, pub vidioc_s_hw_freq_seek: IoctlCb,
    pub vidioc_enum_framesizes: IoctlCb, pub vidioc_enum_frameintervals: IoctlCb,
    pub vidioc_s_dv_timings: IoctlCb, pub vidioc_g_dv_timings: IoctlCb,
    pub vidioc_query_dv_timings: IoctlCb, pub vidioc_enum_dv_timings: IoctlCb,
    pub vidioc_dv_timings_cap: IoctlCb, pub vidioc_g_edid: IoctlCb, pub vidioc_s_edid: IoctlCb,
    pub vidioc_subscribe_event: Option<unsafe extern "C" fn(*mut v4l2_fh, *const v4l2_event_subscription) -> c_int>,
    pub vidioc_unsubscribe_event: Option<unsafe extern "C" fn(*mut v4l2_fh, *const v4l2_event_subscription) -> c_int>,
    pub vidioc_default: Option<unsafe extern "C" fn(*mut file, *mut c_void, bool, u32, *mut c_void) -> c_long>,
}

pub const V4L2_DEV_DEBUG_IOCTL: u32 = 0x01;
pub const V4L2_DEV_DEBUG_IOCTL_ARG: u32 = 0x02;
pub const V4L2_DEV_DEBUG_FOP: u32 = 0x04;
pub const V4L2_DEV_DEBUG_STREAMING: u32 = 0x08;
pub const V4L2_DEV_DEBUG_POLL: u32 = 0x10;
pub const V4L2_DEV_DEBUG_CTRL: u32 = 0x20;

extern "C" {
    pub fn v4l2_norm_to_name(id: v4l2_std_id) -> *const c_char;
    pub fn v4l2_video_std_frame_period(id: c_int, frameperiod: *mut v4l2_fract);
    pub fn v4l2_video_std_construct(vs: *mut v4l2_standard, id: c_int, name: *const c_char) -> c_int;
    pub fn v4l_video_std_enumstd(vs: *mut v4l2_standard, id: v4l2_std_id) -> c_int;
    pub fn v4l_printk_ioctl(prefix: *const c_char, cmd: u32);
    pub static mut v4l2_field_names: *const *const c_char;
    pub static mut v4l2_type_names: *const *const c_char;
    pub fn v4l2_compat_translate_cmd(cmd: u32) -> u32;
    pub fn v4l2_translate_cmd(cmd: u32) -> u32;
    pub fn v4l2_compat_get_user(arg: *mut c_void, parg: *mut c_void, cmd: u32) -> c_int;
    pub fn v4l2_compat_put_user(arg: *mut c_void, parg: *mut c_void, cmd: u32) -> c_int;
    pub fn v4l2_compat_get_array_args(file: *mut file, mbuf: *mut c_void, user_ptr: *mut c_void, array_size: usize, cmd: u32, arg: *mut c_void) -> c_int;
    pub fn v4l2_compat_put_array_args(file: *mut file, user_ptr: *mut c_void, mbuf: *mut c_void, array_size: usize, cmd: u32, arg: *mut c_void) -> c_int;
    pub fn video_usercopy(file: *mut file, cmd: u32, arg: c_ulong, func: Option<unsafe extern "C" fn(*mut file, u32, *mut c_void) -> c_long>) -> c_long;
    pub fn video_ioctl2(file: *mut file, cmd: u32, arg: c_ulong) -> c_long;
}

pub type v4l2_kioctl = Option<unsafe extern "C" fn(*mut file, u32, *mut c_void) -> c_long>;

#[repr(C)]
pub union v4l2_event_time32_u { pub vsync: v4l2_event_vsync, pub ctrl: v4l2_event_ctrl, pub frame_sync: v4l2_event_frame_sync, pub src_change: v4l2_event_src_change, pub motion_det: v4l2_event_motion_det, pub data: [u8; 64] }
#[repr(C)] pub struct v4l2_event_time32 { pub type_: u32, pub u: v4l2_event_time32_u, pub pending: u32, pub sequence: u32, pub timestamp: old_timespec32, pub id: u32, pub reserved: [u32; 8] }
#[repr(C)] pub union v4l2_buffer_time32_m { pub offset: u32, pub userptr: c_ulong, pub planes: *mut v4l2_plane, pub fd: i32 }
#[repr(C)] pub union v4l2_buffer_time32_tail { pub request_fd: i32, pub reserved: u32 }
#[repr(C)] pub struct v4l2_buffer_time32 { pub index: u32, pub type_: u32, pub bytesused: u32, pub flags: u32, pub field: u32, pub timestamp: old_timeval32, pub timecode: v4l2_timecode, pub sequence: u32, pub memory: u32, pub m: v4l2_buffer_time32_m, pub length: u32, pub reserved2: u32, pub tail: v4l2_buffer_time32_tail }

/* _IOR('V', 89, struct v4l2_event_time32) */
pub const VIDIOC_DQEVENT_TIME32: u32 = 0;
/* _IOWR('V', 9/15/17/93, struct v4l2_buffer_time32); ioctl encodings are external. */
pub const VIDIOC_QUERYBUF_TIME32: u32 = 0;
pub const VIDIOC_QBUF_TIME32: u32 = 0;
pub const VIDIOC_DQBUF_TIME32: u32 = 0;
pub const VIDIOC_PREPARE_BUF_TIME32: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
