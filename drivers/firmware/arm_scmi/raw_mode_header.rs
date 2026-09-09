/* SPDX-License-Identifier: GPL-2.0 */
/*
 * System Control and Management Interface (SCMI) Message Protocol
 * Raw mode support header.
 *
 * Copyright (C) 2022 ARM Ltd.
 */

// Dependency intent: declarations below are supplied by "common.h".

use core::ffi::c_void;

#[repr(C)]
pub struct scmi_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scmi_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scmi_xfer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scmi_chan_info {
    _private: [u8; 0],
}

pub const SCMI_RAW_REPLY_QUEUE: i32 = 0;
pub const SCMI_RAW_NOTIF_QUEUE: i32 = 1;
pub const SCMI_RAW_ERRS_QUEUE: i32 = 2;
pub const SCMI_RAW_MAX_QUEUE: i32 = 3;

unsafe extern "C" {
    pub fn scmi_raw_mode_init(
        handle: *const scmi_handle,
        top_dentry: *mut dentry,
        instance_id: i32,
        channels: *mut u8,
        num_chans: i32,
        desc: *const scmi_desc,
        tx_max_msg: i32,
    ) -> *mut c_void;

    pub fn scmi_raw_mode_cleanup(raw: *mut c_void);

    pub fn scmi_raw_message_report(
        raw: *mut c_void,
        xfer: *mut scmi_xfer,
        idx: u32,
        chan_id: u32,
    );

    pub fn scmi_raw_error_report(
        raw: *mut c_void,
        cinfo: *mut scmi_chan_info,
        msg_hdr: u32,
        priv_: *mut c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
