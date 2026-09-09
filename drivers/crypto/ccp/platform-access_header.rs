/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Platform Security Processor (PSP) Platform Access interface
 *
 * Copyright (C) 2023 Advanced Micro Devices, Inc.
 *
 * Author: Mario Limonciello <mario.limonciello@amd.com>
 */

// Dependencies supplied by the surrounding kernel translation.
pub struct device;
pub struct miscdevice;
pub struct mutex {
    _private: [u8; 0],
}
pub struct psp_device;
pub struct platform_access_vdata;

#[repr(C)]
pub struct psp_platform_access_device {
    pub dev: *mut device,
    pub psp: *mut psp_device,

    pub vdata: *mut platform_access_vdata,

    pub mailbox_mutex: mutex,
    pub doorbell_mutex: mutex,

    pub platform_access_data: *mut core::ffi::c_void,
}

unsafe extern "C" {
    pub fn platform_access_dev_destroy(psp: *mut psp_device);
    pub fn platform_access_dev_init(psp: *mut psp_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
