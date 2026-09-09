/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of v4l2-ctrls.h. Included Linux/kernel types are external. */

#[repr(C)]
pub union v4l2_ctrl_ptr {
    pub p_s32: *mut i32, pub p_s64: *mut i64, pub p_u8: *mut u8,
    pub p_u16: *mut u16, pub p_u32: *mut u32, pub p_char: *mut core::ffi::c_char,
    pub p_mpeg2_sequence: *mut v4l2_ctrl_mpeg2_sequence,
    pub p_mpeg2_picture: *mut v4l2_ctrl_mpeg2_picture,
    pub p_mpeg2_quantisation: *mut v4l2_ctrl_mpeg2_quantisation,
    pub p_fwht_params: *mut v4l2_ctrl_fwht_params,
    pub p_h264_sps: *mut v4l2_ctrl_h264_sps, pub p_h264_pps: *mut v4l2_ctrl_h264_pps,
    pub p_h264_scaling_matrix: *mut v4l2_ctrl_h264_scaling_matrix,
    pub p_h264_slice_params: *mut v4l2_ctrl_h264_slice_params,
    pub p_h264_decode_params: *mut v4l2_ctrl_h264_decode_params,
    pub p_h264_pred_weights: *mut v4l2_ctrl_h264_pred_weights,
    pub p_vp8_frame: *mut v4l2_ctrl_vp8_frame,
    pub p_hevc_sps: *mut v4l2_ctrl_hevc_sps, pub p_hevc_pps: *mut v4l2_ctrl_hevc_pps,
    pub p_hevc_slice_params: *mut v4l2_ctrl_hevc_slice_params,
    pub p_vp9_compressed_hdr_probs: *mut v4l2_ctrl_vp9_compressed_hdr,
    pub p_vp9_frame: *mut v4l2_ctrl_vp9_frame,
    pub p_hdr10_cll: *mut v4l2_ctrl_hdr10_cll_info,
    pub p_hdr10_mastering: *mut v4l2_ctrl_hdr10_mastering_display,
    pub p_area: *mut v4l2_area, pub p_av1_sequence: *mut v4l2_ctrl_av1_sequence,
    pub p_av1_tile_group_entry: *mut v4l2_ctrl_av1_tile_group_entry,
    pub p_av1_frame: *mut v4l2_ctrl_av1_frame, pub p_av1_film_grain: *mut v4l2_ctrl_av1_film_grain,
    pub p_rect: *mut v4l2_rect, pub p: *mut core::ffi::c_void,
    pub p_const: *const core::ffi::c_void,
}

#[inline]
pub unsafe fn v4l2_ctrl_ptr_create(ptr: *mut core::ffi::c_void) -> v4l2_ctrl_ptr {
    v4l2_ctrl_ptr { p: ptr }
}

#[repr(C)] pub struct v4l2_ctrl_ops { pub g_volatile_ctrl: Option<unsafe extern "C" fn(*mut v4l2_ctrl) -> i32>, pub try_ctrl: Option<unsafe extern "C" fn(*mut v4l2_ctrl) -> i32>, pub s_ctrl: Option<unsafe extern "C" fn(*mut v4l2_ctrl) -> i32> }
#[repr(C)] pub struct v4l2_ctrl_type_ops {
    pub equal: Option<unsafe extern "C" fn(*const v4l2_ctrl, v4l2_ctrl_ptr, v4l2_ctrl_ptr) -> bool>,
    pub init: Option<unsafe extern "C" fn(*const v4l2_ctrl, u32, v4l2_ctrl_ptr)>,
    pub minimum: Option<unsafe extern "C" fn(*const v4l2_ctrl, u32, v4l2_ctrl_ptr)>,
    pub maximum: Option<unsafe extern "C" fn(*const v4l2_ctrl, u32, v4l2_ctrl_ptr)>,
    pub log: Option<unsafe extern "C" fn(*const v4l2_ctrl)>,
    pub validate: Option<unsafe extern "C" fn(*const v4l2_ctrl, v4l2_ctrl_ptr) -> i32>,
}
pub type v4l2_ctrl_notify_fnc = unsafe extern "C" fn(*mut v4l2_ctrl, *mut core::ffi::c_void);

#[repr(C)] pub struct v4l2_ctrl {
    pub node: list_head, pub ev_subs: list_head, pub handler: *mut v4l2_ctrl_handler,
    pub cluster: *mut *mut v4l2_ctrl, pub ncontrols: u32,
    /* C bitfields, represented as their storage unit. */ pub flags_bits: u32,
    pub ops: *const v4l2_ctrl_ops, pub type_ops: *const v4l2_ctrl_type_ops,
    pub id: u32, pub name: *const core::ffi::c_char, pub type_: v4l2_ctrl_type,
    pub minimum: i64, pub maximum: i64, pub default_value: i64, pub elems: u32,
    pub elem_size: u32, pub new_elems: u32, pub dims: [u32; V4L2_CTRL_MAX_DIMS], pub nr_of_dims: u32,
    pub step_or_menu_skip_mask: u64, pub qmenu_or_qmenu_int: *const core::ffi::c_void,
    pub flags: core::ffi::c_ulong, pub priv_: *mut core::ffi::c_void, pub p_array: *mut core::ffi::c_void,
    pub p_array_alloc_elems: u32, pub val: i32, pub cur: v4l2_ctrl_cur,
    pub p_def: v4l2_ctrl_ptr, pub p_min: v4l2_ctrl_ptr, pub p_max: v4l2_ctrl_ptr,
    pub p_new: v4l2_ctrl_ptr, pub p_cur: v4l2_ctrl_ptr,
}
#[repr(C)] pub struct v4l2_ctrl_cur { pub val: i32 }
#[repr(C)] pub struct v4l2_ctrl_ref { pub node: list_head, pub next: *mut v4l2_ctrl_ref, pub ctrl: *mut v4l2_ctrl, pub helper: *mut v4l2_ctrl_helper, pub from_other_dev: bool, pub req_done: bool, pub p_req_valid: bool, pub p_req_array_enomem: bool, pub p_req_array_alloc_elems: u32, pub p_req_elems: u32, pub p_req: v4l2_ctrl_ptr }
#[repr(C)] pub struct v4l2_ctrl_handler { pub _lock: mutex, pub lock: *mut mutex, pub ctrls: list_head, pub ctrl_refs: list_head, pub cached: *mut v4l2_ctrl_ref, pub buckets: *mut *mut v4l2_ctrl_ref, pub notify: Option<v4l2_ctrl_notify_fnc>, pub notify_priv: *mut core::ffi::c_void, pub nr_of_buckets: u16, pub error: i32, pub request_is_queued: bool, pub requests: list_head, pub requests_queued: list_head, pub req_obj: media_request_object }
#[repr(C)] pub struct v4l2_ctrl_config { pub ops: *const v4l2_ctrl_ops, pub type_ops: *const v4l2_ctrl_type_ops, pub id: u32, pub name: *const core::ffi::c_char, pub type_: v4l2_ctrl_type, pub min: i64, pub max: i64, pub step: u64, pub def: i64, pub p_def: v4l2_ctrl_ptr, pub p_min: v4l2_ctrl_ptr, pub p_max: v4l2_ctrl_ptr, pub dims: [u32; V4L2_CTRL_MAX_DIMS], pub elem_size: u32, pub flags: u32, pub menu_skip_mask: u64, pub qmenu: *const *const core::ffi::c_char, pub qmenu_int: *const i64, pub is_private: u32 }

extern "C" {
    pub fn v4l2_ctrl_fill(id: u32, name: *mut *const core::ffi::c_char, type_: *mut v4l2_ctrl_type, min: *mut i64, max: *mut i64, step: *mut u64, def: *mut i64, flags: *mut u32);
    pub fn v4l2_ctrl_handler_init_class(hdl: *mut v4l2_ctrl_handler, nr: u32, key: *mut lock_class_key, name: *const core::ffi::c_char) -> i32;
    pub fn v4l2_ctrl_handler_free(hdl: *mut v4l2_ctrl_handler) -> i32;
    pub fn __v4l2_ctrl_handler_setup(hdl: *mut v4l2_ctrl_handler) -> i32;
    pub fn v4l2_ctrl_handler_setup(hdl: *mut v4l2_ctrl_handler) -> i32;
    pub fn v4l2_ctrl_handler_log_status(hdl: *mut v4l2_ctrl_handler, prefix: *const core::ffi::c_char);
    pub fn v4l2_ctrl_new_custom(hdl: *mut v4l2_ctrl_handler, cfg: *const v4l2_ctrl_config, priv_: *mut core::ffi::c_void) -> *mut v4l2_ctrl;
    pub fn v4l2_ctrl_new_std(hdl: *mut v4l2_ctrl_handler, ops: *const v4l2_ctrl_ops, id: u32, min: i64, max: i64, step: u64, def: i64) -> *mut v4l2_ctrl;
    pub fn v4l2_ctrl_find(hdl: *mut v4l2_ctrl_handler, id: u32) -> *mut v4l2_ctrl;
    pub fn v4l2_ctrl_activate(ctrl: *mut v4l2_ctrl, active: bool);
    pub fn __v4l2_ctrl_grab(ctrl: *mut v4l2_ctrl, grabbed: bool);
    pub fn __v4l2_ctrl_modify_range(ctrl: *mut v4l2_ctrl, min: i64, max: i64, step: u64, def: i64) -> i32;
    pub fn __v4l2_ctrl_modify_dimensions(ctrl: *mut v4l2_ctrl, dims: *mut u32) -> i32;
    pub fn v4l2_ctrl_notify(ctrl: *mut v4l2_ctrl, notify: Option<v4l2_ctrl_notify_fnc>, priv_: *mut core::ffi::c_void);
    pub fn v4l2_ctrl_get_name(id: u32) -> *const core::ffi::c_char;
    pub fn v4l2_ctrl_g_ctrl(ctrl: *mut v4l2_ctrl) -> i32;
    pub fn __v4l2_ctrl_s_ctrl(ctrl: *mut v4l2_ctrl, val: i32) -> i32;
    pub fn v4l2_ctrl_g_ctrl_int64(ctrl: *mut v4l2_ctrl) -> i64;
    pub fn __v4l2_ctrl_s_ctrl_int64(ctrl: *mut v4l2_ctrl, val: i64) -> i32;
    pub fn __v4l2_ctrl_s_ctrl_string(ctrl: *mut v4l2_ctrl, s: *const core::ffi::c_char) -> i32;
    pub fn __v4l2_ctrl_s_ctrl_compound(ctrl: *mut v4l2_ctrl, type_: v4l2_ctrl_type, p: *const core::ffi::c_void) -> i32;
    pub fn v4l2_ctrl_cluster(ncontrols: u32, controls: *mut *mut v4l2_ctrl);
    pub fn v4l2_ctrl_auto_cluster(ncontrols: u32, controls: *mut *mut v4l2_ctrl, manual_val: u8, set_volatile: bool);
}

/* External kernel declarations and the remaining ioctl/request helpers. */
extern "C" {
    pub fn v4l2_ctrl_add_handler(hdl: *mut v4l2_ctrl_handler, add: *mut v4l2_ctrl_handler, filter: Option<unsafe extern "C" fn(*const v4l2_ctrl) -> bool>, from_other_dev: bool) -> i32;
    pub fn v4l2_ctrl_radio_filter(ctrl: *const v4l2_ctrl) -> bool;
    pub fn v4l2_ctrl_get_menu(id: u32) -> *const *const core::ffi::c_char;
    pub fn v4l2_ctrl_get_int_menu(id: u32, len: *mut u32) -> *const i64;
    pub fn v4l2_queryctrl(hdl: *mut v4l2_ctrl_handler, qc: *mut v4l2_queryctrl) -> i32;
    pub fn v4l2_querymenu(hdl: *mut v4l2_ctrl_handler, qm: *mut v4l2_querymenu) -> i32;
    pub fn v4l2_g_ctrl(hdl: *mut v4l2_ctrl_handler, ctrl: *mut v4l2_control) -> i32;
    pub fn v4l2_s_ctrl(fh: *mut v4l2_fh, hdl: *mut v4l2_ctrl_handler, ctrl: *mut v4l2_control) -> i32;
    pub fn v4l2_ctrl_subdev_log_status(sd: *mut v4l2_subdev) -> i32;
    pub fn v4l2_ctrl_type_op_equal(ctrl: *const v4l2_ctrl, ptr1: v4l2_ctrl_ptr, ptr2: v4l2_ctrl_ptr) -> bool;
    pub fn v4l2_ctrl_type_op_init(ctrl: *const v4l2_ctrl, from_idx: u32, ptr: v4l2_ctrl_ptr);
    pub fn v4l2_ctrl_type_op_log(ctrl: *const v4l2_ctrl);
    pub fn v4l2_ctrl_type_op_validate(ctrl: *const v4l2_ctrl, ptr: v4l2_ctrl_ptr) -> i32;
}

/* Names supplied by the Linux V4L2 headers. */
pub type v4l2_ctrl_type = u32; pub type list_head = core::ffi::c_void; pub type mutex = core::ffi::c_void; pub type lock_class_key = core::ffi::c_void; pub type media_request_object = core::ffi::c_void;
pub type v4l2_ctrl_helper = core::ffi::c_void; pub type v4l2_fh = core::ffi::c_void; pub type v4l2_subdev = core::ffi::c_void; pub type v4l2_queryctrl = core::ffi::c_void; pub type v4l2_querymenu = core::ffi::c_void; pub type v4l2_control = core::ffi::c_void;
pub const V4L2_CTRL_MAX_DIMS: usize = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
