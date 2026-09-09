/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 */

// Dependency intent from <linux/ioctl.h> and <linux/types.h> is represented
// using Rust's C-compatible layout and integer types below.

pub const NSM_MAGIC: u32 = 0x0A;

pub const NSM_REQUEST_MAX_SIZE: usize = 0x1000;
pub const NSM_RESPONSE_MAX_SIZE: usize = 0x3000;

#[repr(C)]
pub struct nsm_iovec {
    /// Virtual address of target buffer
    pub addr: u64,
    /// Length of target buffer
    pub len: u64,
}

/* Raw NSM message. Only available with CAP_SYS_ADMIN. */
#[repr(C)]
pub struct nsm_raw {
    /* Request from user */
    pub request: nsm_iovec,
    /* Response to user */
    pub response: nsm_iovec,
}

// _IOWR(NSM_MAGIC, 0x0, struct nsm_raw). The Linux ioctl encoding uses
// direction 3 and the C size of struct nsm_raw (32 bytes).
pub const NSM_IOCTL_RAW: u32 = 0xC0200A00;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
