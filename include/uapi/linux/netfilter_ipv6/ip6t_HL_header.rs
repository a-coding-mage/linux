/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Hop Limit modification module for ip6tables
 * Maciej Soltysiak <solt@dns.toxicfilms.tv>
 * Based on HW's TTL module */

// Dependency intent: `__u8` is supplied by <linux/types.h>.

pub const IP6T_HL_SET: i32 = 0;
pub const IP6T_HL_INC: i32 = 1;
pub const IP6T_HL_DEC: i32 = 2;

pub const IP6T_HL_MAXMODE: i32 = IP6T_HL_DEC;

#[repr(C)]
pub struct ip6t_HL_info {
    pub mode: u8,
    pub hop_limit: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
