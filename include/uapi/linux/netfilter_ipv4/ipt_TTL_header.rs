/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* TTL modification module for IP tables
 * (C) 2000 by Harald Welte <laforge@netfilter.org> */

// Translated from the C header; the original include and header guard are omitted.

pub const IPT_TTL_SET: i32 = 0;
pub const IPT_TTL_INC: i32 = 1;
pub const IPT_TTL_DEC: i32 = 2;

pub const IPT_TTL_MAXMODE: i32 = IPT_TTL_DEC;

#[repr(C)]
pub struct ipt_TTL_info {
    pub mode: u8,
    pub ttl: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
