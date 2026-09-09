/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header. The C `__u32` type is represented by
// Rust `u32` here.

#[repr(u32)]
pub enum xt_devgroup_flags {
    XT_DEVGROUP_MATCH_SRC = 0x1,
    XT_DEVGROUP_INVERT_SRC = 0x2,
    XT_DEVGROUP_MATCH_DST = 0x4,
    XT_DEVGROUP_INVERT_DST = 0x8,
}

#[repr(C)]
pub struct xt_devgroup_info {
    pub flags: u32,
    pub src_group: u32,
    pub src_mask: u32,
    pub dst_group: u32,
    pub dst_mask: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
