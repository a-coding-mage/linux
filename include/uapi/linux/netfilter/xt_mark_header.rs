/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency equivalent of <linux/types.h> is supplied externally. */

#[repr(C)]
pub struct xt_mark_tginfo2 {
    pub mark: __u32,
    pub mask: __u32,
}

#[repr(C)]
pub struct xt_mark_mtinfo1 {
    pub mark: __u32,
    pub mask: __u32,
    pub invert: __u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
