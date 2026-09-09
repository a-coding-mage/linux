/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Platform Security Processor (PSP) Seamless Firmware (SFS) Support.
 *
 * Copyright (C) 2025 Advanced Micro Devices, Inc.
 *
 * Author: Ashish Kalra <ashish.kalra@amd.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// uapi/linux/psp-sfs.h, linux/device.h, linux/miscdevice.h,
// linux/psp-sev.h, linux/psp-platform-access.h, linux/set_memory.h,
// and psp-dev.h.

#[repr(C)]
pub struct sfs_misc_dev {
    pub refcount: kref,
    pub misc: miscdevice,
}

#[repr(C, packed)]
pub struct sfs_command {
    pub hdr: psp_ext_req_buffer_hdr,
    pub buf: [u8; PAGE_SIZE - core::mem::size_of::<psp_ext_req_buffer_hdr>()],
    pub sfs_buffer: [u8; 0],
}

#[repr(C)]
pub struct sfs_device {
    pub dev: *mut device,
    pub psp: *mut psp_device,

    pub page: *mut page,
    pub command_buf: *mut sfs_command,

    pub misc: *mut sfs_misc_dev,
}

extern "C" {
    pub fn sfs_dev_destroy(psp: *mut psp_device);
    pub fn sfs_dev_init(psp: *mut psp_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
