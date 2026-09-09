/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright 2019 IBM Corp.
 */

// Dependency equivalent of <linux/types.h> and <asm/ioctl.h> is supplied externally.

pub const VAS_MAGIC: u8 = b'v';

#[repr(C)]
pub struct vas_tx_win_open_attr {
    pub version: u32,
    pub vas_id: i16, /* specific instance of vas or -1 for default */
    pub reserved1: u16,
    pub flags: u64,
    pub reserved2: [u64; 6],
}

// VAS_TX_WIN_OPEN = _IOW(VAS_MAGIC, 0x20, struct vas_tx_win_open_attr)
// The ioctl encoding macro is provided by the external asm/ioctl dependency.
pub const VAS_TX_WIN_OPEN: u32 = ioctl_iow!(VAS_MAGIC, 0x20, vas_tx_win_open_attr);

/* Flags to VAS TX open window ioctl */
/* To allocate a window with QoS credit, otherwise use default credit */
pub const VAS_TX_WIN_FLAG_QOS_CREDIT: u64 = 0x0000000000000001;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
