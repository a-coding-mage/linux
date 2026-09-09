/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Platform Security Processor (PSP) Dynamic Boost Control support
 *
 * Copyright (C) 2023 Advanced Micro Devices, Inc.
 *
 * Author: Mario Limonciello <mario.limonciello@amd.com>
 */

// Dependency intent from the original header:
// <uapi/linux/psp-dbc.h>, <linux/device.h>, <linux/miscdevice.h>,
// <linux/psp-platform-access.h>, and "psp-dev.h" provide the referenced
// types and symbols.

#[repr(C)]
pub union dbc_buffer {
    pub pa_req: std::mem::ManuallyDrop<psp_request>,
    pub ext_req: std::mem::ManuallyDrop<psp_ext_request>,
}

#[repr(C)]
pub struct psp_dbc_device {
    pub dev: *mut device,
    pub psp: *mut psp_device,

    pub mbox: *mut dbc_buffer,

    pub ioctl_mutex: mutex,

    pub char_dev: miscdevice,

    /* used to abstract communication path */
    pub use_ext: bool,
    pub header_size: u32,
    pub payload_size: *mut u32,
    pub result: *mut u32,
    pub payload: *mut std::ffi::c_void,
}

unsafe extern "C" {
    pub fn dbc_dev_destroy(psp: *mut psp_device);
    pub fn dbc_dev_init(psp: *mut psp_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
