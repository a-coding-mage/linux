/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* ip6tables module for matching the Hop Limit value
 * Maciej Soltysiak <solt@dns.toxicfilms.tv>
 * Based on HW's ttl module */

// Dependency: `__u8` from <linux/types.h> is represented as `u8` in Rust.

pub const IP6T_HL_EQ: u8 = 0; /* equals */
pub const IP6T_HL_NE: u8 = 1; /* not equals */
pub const IP6T_HL_LT: u8 = 2; /* less than */
pub const IP6T_HL_GT: u8 = 3; /* greater than */

#[repr(C)]
pub struct ip6t_hl_info {
    pub mode: u8,
    pub hop_limit: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
