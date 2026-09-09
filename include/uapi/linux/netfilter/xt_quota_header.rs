/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by <linux/types.h>.

#[repr(C)]
pub enum xt_quota_flags {
    XT_QUOTA_INVERT = 0x1,
}

pub const XT_QUOTA_MASK: u32 = 0x1;

pub struct xt_quota_priv;

#[repr(C)]
pub struct xt_quota_info {
    pub flags: u32,
    pub pad: u32,
    pub quota: u64,

    /* Used internally by the kernel */
    pub master: *mut xt_quota_priv,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
