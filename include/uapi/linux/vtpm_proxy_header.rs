/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Definitions for the VTPM proxy driver
 * Copyright (c) 2015, 2016, IBM Corporation
 * Copyright (C) 2016 Intel Corporation
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms and conditions of the GNU General Public License,
 * version 2, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 */

// C dependencies: <linux/types.h>, <linux/ioctl.h>

/// Flags for the proxy TPM.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum vtpm_proxy_flags {
    /// The proxy TPM uses the TPM 2.0 protocol.
    VTPM_PROXY_FLAG_TPM2 = 1,
}

/// Parameter structure for the `VTPM_PROXY_IOC_NEW_DEV` ioctl.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct vtpm_proxy_new_dev {
    pub flags: u32,   // input
    pub tpm_num: u32, // output
    pub fd: u32,      // output
    pub major: u32,   // output
    pub minor: u32,   // output
}

// _IOWR(0xa1, 0x00, struct vtpm_proxy_new_dev)
// Linux ioctl encoding: direction = read|write, size = sizeof(struct vtpm_proxy_new_dev).
pub const VTPM_PROXY_IOC_NEW_DEV: u32 =
    (3u32 << 30) | ((core::mem::size_of::<vtpm_proxy_new_dev>() as u32) << 16) |
    (0xa1u32 << 8);

/// Vendor-specific command to set locality.
pub const TPM2_CC_SET_LOCALITY: u32 = 0x20001000;

/// Vendor-specific command to set locality.
pub const TPM_ORD_SET_LOCALITY: u32 = 0x20001000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
