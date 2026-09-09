/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (c) 2026, Broadcom Inc
 */

// Translated from the Linux UAPI header. The original dependency on
// <linux/types.h> supplies the fixed-width integer types used here.

#[repr(i32)]
pub enum fwctl_bnxt_commands {
    FWCTL_BNXT_INLINE_COMMANDS = 0,
    FWCTL_BNXT_QUERY_COMMANDS,
    FWCTL_BNXT_SEND_COMMANDS,
    FWCTL_BNXT_DMA_COMMANDS,
}

/**
 * struct fwctl_info_bnxt - ioctl(FWCTL_INFO) out_device_data
 * @uctx_caps: The command capabilities driver accepts.
 *
 * Return basic information about the FW interface available.
 */
#[repr(C)]
pub struct fwctl_info_bnxt {
    pub uctx_caps: u32,
}

pub const FWCTL_BNXT_MAX_DMABUF: u32 = 0x10000; /* 64 KiB */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
