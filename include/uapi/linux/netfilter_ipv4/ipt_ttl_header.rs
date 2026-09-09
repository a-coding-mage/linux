/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* IP tables module for matching the value of the TTL
 * (C) 2000 by Harald Welte <laforge@gnumonks.org> */

// Dependency: __u8 is supplied by <linux/types.h>.

pub const IPT_TTL_EQ: i32 = 0; // equals
pub const IPT_TTL_NE: i32 = 1; // not equals
pub const IPT_TTL_LT: i32 = 2; // less than
pub const IPT_TTL_GT: i32 = 3; // greater than

#[repr(C)]
pub struct ipt_ttl_info {
	pub mode: u8,
	pub ttl: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
