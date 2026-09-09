/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Copyright (C) 2002,2004 MARA Systems AB <https://www.marasystems.com>
 * by Henrik Nordstrom <hno@marasystems.com>
 */

// Dependency intent: the C header includes <linux/types.h> for __u32 and __u8.

pub const XT_CONNMARK_SET: u32 = 0;
pub const XT_CONNMARK_SAVE: u32 = 1;
pub const XT_CONNMARK_RESTORE: u32 = 2;

pub const D_SHIFT_LEFT: u32 = 0;
pub const D_SHIFT_RIGHT: u32 = 1;

#[repr(C)]
pub struct xt_connmark_tginfo1 {
    pub ctmark: u32,
    pub ctmask: u32,
    pub nfmask: u32,
    pub mode: u8,
}

#[repr(C)]
pub struct xt_connmark_tginfo2 {
    pub ctmark: u32,
    pub ctmask: u32,
    pub nfmask: u32,
    pub shift_dir: u8,
    pub shift_bits: u8,
    pub mode: u8,
}

#[repr(C)]
pub struct xt_connmark_mtinfo1 {
    pub mark: u32,
    pub mask: u32,
    pub invert: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
