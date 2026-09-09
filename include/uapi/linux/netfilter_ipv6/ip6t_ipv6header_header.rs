/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* ipv6header match - matches IPv6 packets based
 * on whether they contain certain headers */

/* Original idea: Brad Chapman
 * Rewritten by: Andras Kis-Szabo <kisza@sch.bme.hu> */

/* Dependency: linux/types.h */

#[repr(C)]
pub struct ip6t_ipv6header_info {
    pub matchflags: __u8,
    pub invflags: __u8,
    pub modeflag: __u8,
}

pub const MASK_HOPOPTS: i32 = 128;
pub const MASK_DSTOPTS: i32 = 64;
pub const MASK_ROUTING: i32 = 32;
pub const MASK_FRAGMENT: i32 = 16;
pub const MASK_AH: i32 = 8;
pub const MASK_ESP: i32 = 4;
pub const MASK_NONE: i32 = 2;
pub const MASK_PROTO: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
