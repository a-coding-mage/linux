/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  ebt_ip6
 *
 *	Authors:
 * Kuo-Lang Tseng <kuo-lang.tseng@intel.com>
 * Manohar Castelino <manohar.r.castelino@intel.com>
 *
 *  Jan 11, 2008
 *
 */

// Dependencies corresponding to <linux/types.h> and <linux/in6.h> are
// supplied externally.

pub const EBT_IP6_SOURCE: u32 = 0x01;
pub const EBT_IP6_DEST: u32 = 0x02;
pub const EBT_IP6_TCLASS: u32 = 0x04;
pub const EBT_IP6_PROTO: u32 = 0x08;
pub const EBT_IP6_SPORT: u32 = 0x10;
pub const EBT_IP6_DPORT: u32 = 0x20;
pub const EBT_IP6_ICMP6: u32 = 0x40;

pub const EBT_IP6_MASK: u32 = EBT_IP6_SOURCE
    | EBT_IP6_DEST
    | EBT_IP6_TCLASS
    | EBT_IP6_PROTO
    | EBT_IP6_SPORT
    | EBT_IP6_DPORT
    | EBT_IP6_ICMP6;
pub const EBT_IP6_MATCH: &str = "ip6";

/* the same values are used for the invflags */
#[repr(C)]
pub union ebt_ip6_info_sport_icmpv6_type {
    pub sport: [u16; 2],
    pub icmpv6_type: [u8; 2],
}

#[repr(C)]
pub union ebt_ip6_info_dport_icmpv6_code {
    pub dport: [u16; 2],
    pub icmpv6_code: [u8; 2],
}

#[repr(C)]
pub struct ebt_ip6_info {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
    pub smsk: in6_addr,
    pub dmsk: in6_addr,
    pub tclass: u8,
    pub protocol: u8,
    pub bitmask: u8,
    pub invflags: u8,
    pub sport_or_icmpv6_type: ebt_ip6_info_sport_icmpv6_type,
    pub dport_or_icmpv6_code: ebt_ip6_info_dport_icmpv6_code,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
