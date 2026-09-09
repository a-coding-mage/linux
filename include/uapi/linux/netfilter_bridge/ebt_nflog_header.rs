/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const EBT_NFLOG_MASK: u32 = 0x0;

pub const EBT_NFLOG_PREFIX_SIZE: usize = 64;
pub const EBT_NFLOG_WATCHER: &str = "nflog";

pub const EBT_NFLOG_DEFAULT_GROUP: u16 = 0x1;
pub const EBT_NFLOG_DEFAULT_THRESHOLD: u16 = 1;

#[repr(C)]
pub struct ebt_nflog_info {
    pub len: u32,
    pub group: u16,
    pub threshold: u16,
    pub flags: u16,
    pub pad: u16,
    pub prefix: [i8; EBT_NFLOG_PREFIX_SIZE],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
