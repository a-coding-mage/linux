/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* IPv4 routing cache flags */

pub const RTCF_DEAD: u32 = RTNH_F_DEAD;
pub const RTCF_ONLINK: u32 = RTNH_F_ONLINK;

/* Obsolete flag. About to be deleted */
pub const RTCF_NOPMTUDISC: u32 = RTM_F_NOPMTUDISC;

pub const RTCF_NOTIFY: u32 = 0x00010000;
pub const RTCF_DIRECTDST: u32 = 0x00020000; /* unused */
pub const RTCF_REDIRECTED: u32 = 0x00040000;
pub const RTCF_TPROXY: u32 = 0x00080000; /* unused */

pub const RTCF_FAST: u32 = 0x00200000; /* unused */
pub const RTCF_MASQ: u32 = 0x00400000; /* unused */
pub const RTCF_SNAT: u32 = 0x00800000; /* unused */
pub const RTCF_DOREDIRECT: u32 = 0x01000000;
pub const RTCF_DIRECTSRC: u32 = 0x04000000;
pub const RTCF_DNAT: u32 = 0x08000000;
pub const RTCF_BROADCAST: u32 = 0x10000000;
pub const RTCF_MULTICAST: u32 = 0x20000000;
pub const RTCF_REJECT: u32 = 0x40000000; /* unused */
pub const RTCF_LOCAL: u32 = 0x80000000;

pub const RTCF_NAT: u32 = RTCF_DNAT | RTCF_SNAT;

#[allow(non_snake_case)]
pub const fn RT_TOS(tos: u32) -> u32 {
    tos & IPTOS_TOS_MASK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
