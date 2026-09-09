/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/netfilter_ipv6/ip6t_frag.h>.
// Dependency intent: the C header includes <linux/types.h> for __u32 and __u8.

#[repr(C)]
pub struct ip6t_frag {
    pub ids: [u32; 2], // Identification range
    pub hdrlen: u32,   // Header Length
    pub flags: u8,     // Flags
    pub invflags: u8,  // Inverse flags
}

pub const IP6T_FRAG_IDS: u32 = 0x01;
pub const IP6T_FRAG_LEN: u32 = 0x02;
pub const IP6T_FRAG_RES: u32 = 0x04;
pub const IP6T_FRAG_FST: u32 = 0x08;
pub const IP6T_FRAG_MF: u32 = 0x10;
pub const IP6T_FRAG_NMF: u32 = 0x20;

// Values for "invflags" field in struct ip6t_frag.
pub const IP6T_FRAG_INV_IDS: u32 = 0x01; // Invert the sense of ids.
pub const IP6T_FRAG_INV_LEN: u32 = 0x02; // Invert the sense of length.
pub const IP6T_FRAG_INV_MASK: u32 = 0x03; // All possible flags.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
