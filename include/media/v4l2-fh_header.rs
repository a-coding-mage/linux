/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * v4l2-fh.h
 *
 * V4L2 file handle. Store per file handle data for the V4L2
 * framework. Using file handles is mandatory for the drivers.
 *
 * Copyright (C) 2009--2010 Nokia Corporation.
 *
 * Contact: Sakari Ailus <sakari.ailus@iki.fi>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external: linux/fs.h, linux/kconfig.h, linux/list.h, linux/videodev2.h.

#[repr(C)]
pub struct v4l2_fh {
    pub list: list_head,
    pub vdev: *mut video_device,
    pub ctrl_handler: *mut v4l2_ctrl_handler,
    pub prio: v4l2_priority,

    /* Events */
    pub wait: wait_queue_head_t,
    pub subscribe_lock: mutex,
    pub subscribed: list_head,
    pub available: list_head,
    pub navailable: ::core::ffi::c_uint,
    pub sequence: u32,

    pub m2m_ctx: *mut v4l2_m2m_ctx,
}

pub type video_device = ::core::ffi::c_void;
pub type v4l2_ctrl_handler = ::core::ffi::c_void;
pub type v4l2_m2m_ctx = ::core::ffi::c_void;
pub type list_head = ::core::ffi::c_void;
pub type wait_queue_head_t = ::core::ffi::c_void;
pub type mutex = ::core::ffi::c_void;
pub type v4l2_priority = ::core::ffi::c_int;
pub type file = ::core::ffi::c_void;

/**
 * file_to_v4l2_fh - Return the v4l2_fh associated with a struct file
 */
#[inline]
pub unsafe fn file_to_v4l2_fh(filp: *mut file) -> *mut v4l2_fh {
    *(filp as *mut *mut v4l2_fh)
}

extern "C" {
    pub fn v4l2_fh_init(fh: *mut v4l2_fh, vdev: *mut video_device);
    pub fn v4l2_fh_add(fh: *mut v4l2_fh, filp: *mut file);
    pub fn v4l2_fh_open(filp: *mut file) -> ::core::ffi::c_int;
    pub fn v4l2_fh_del(fh: *mut v4l2_fh, filp: *mut file);
    pub fn v4l2_fh_exit(fh: *mut v4l2_fh);
    pub fn v4l2_fh_release(filp: *mut file) -> ::core::ffi::c_int;
    pub fn v4l2_fh_is_singular(fh: *mut v4l2_fh) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn v4l2_fh_is_singular_file(filp: *mut file) -> ::core::ffi::c_int {
    v4l2_fh_is_singular(file_to_v4l2_fh(filp))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
