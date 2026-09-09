/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent from <linux/types.h> and <linux/in6.h> is preserved by
// using the corresponding Rust integer types and the external `in6_addr` type.

pub const IP6T_RT_HOPS: usize = 16;

#[repr(C)]
pub struct ip6t_rt {
    pub rt_type: u32,              /* Routing Type */
    pub segsleft: [u32; 2],        /* Segments Left */
    pub hdrlen: u32,               /* Header Length */
    pub flags: u8,                 /*  */
    pub invflags: u8,              /* Inverse flags */
    pub addrs: [in6_addr; IP6T_RT_HOPS], /* Hops */
    pub addrnr: u8,                /* Nr of Addresses */
}

pub const IP6T_RT_TYP: u8 = 0x01;
pub const IP6T_RT_SGS: u8 = 0x02;
pub const IP6T_RT_LEN: u8 = 0x04;
pub const IP6T_RT_RES: u8 = 0x08;
pub const IP6T_RT_FST_MASK: u8 = 0x30;
pub const IP6T_RT_FST: u8 = 0x10;
pub const IP6T_RT_FST_NSTRICT: u8 = 0x20;

/* Values for "invflags" field in struct ip6t_rt. */
pub const IP6T_RT_INV_TYP: u8 = 0x01; /* Invert the sense of type. */
pub const IP6T_RT_INV_SGS: u8 = 0x02; /* Invert the sense of Segments. */
pub const IP6T_RT_INV_LEN: u8 = 0x04; /* Invert the sense of length. */
pub const IP6T_RT_INV_MASK: u8 = 0x07; /* All possible flags. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
