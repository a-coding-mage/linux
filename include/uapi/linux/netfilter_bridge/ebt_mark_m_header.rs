/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/types.h>; `c_ulong` preserves C `unsigned long` ABI width.
use ::core::ffi::c_ulong;

pub const EBT_MARK_AND: u32 = 0x01;
pub const EBT_MARK_OR: u32 = 0x02;
pub const EBT_MARK_MASK: u32 = EBT_MARK_AND | EBT_MARK_OR;

#[repr(C)]
pub struct ebt_mark_m_info {
    pub mark: c_ulong,
    pub mask: c_ulong,
    pub invert: u8,
    pub bitmask: u8,
}

pub const EBT_MARK_MATCH: &str = "mark_m";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
