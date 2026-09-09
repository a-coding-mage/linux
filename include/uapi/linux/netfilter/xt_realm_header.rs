/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/types.h>; __u32 and __u8 map directly to u32 and u8.
#[repr(C)]
pub struct xt_realm_info {
    pub id: u32,
    pub mask: u32,
    pub invert: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
