/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  ebt_ip
 *
 *	Authors:
 *	Bart De Schuymer <bart.de.schuymer@pandora.be>
 *
 *  April, 2002
 *
 *  Changes:
 *    added ip-sport and ip-dport
 *    Innominate Security Technologies AG <mhopf@innominate.com>
 *    September, 2002
 */

// Translated from the Linux UAPI header <linux/netfilter_bridge/ebt_ip.h>.
// The original dependency <linux/types.h> supplies the corresponding integer types.

pub const EBT_IP_SOURCE: u32 = 0x01;
pub const EBT_IP_DEST: u32 = 0x02;
pub const EBT_IP_TOS: u32 = 0x04;
pub const EBT_IP_PROTO: u32 = 0x08;
pub const EBT_IP_SPORT: u32 = 0x10;
pub const EBT_IP_DPORT: u32 = 0x20;
pub const EBT_IP_ICMP: u32 = 0x40;
pub const EBT_IP_IGMP: u32 = 0x80;
pub const EBT_IP_MASK: u32 = EBT_IP_SOURCE
    | EBT_IP_DEST
    | EBT_IP_TOS
    | EBT_IP_PROTO
    | EBT_IP_SPORT
    | EBT_IP_DPORT
    | EBT_IP_ICMP
    | EBT_IP_IGMP;
pub const EBT_IP_MATCH: &str = "ip";

#[repr(C)]
pub union ebt_ip_info__bindgen_ty_1 {
    pub sport: [u16; 2],
    pub icmp_type: [u8; 2],
    pub igmp_type: [u8; 2],
}

#[repr(C)]
pub union ebt_ip_info__bindgen_ty_2 {
    pub dport: [u16; 2],
    pub icmp_code: [u8; 2],
}

// The same values are used for the invflags.
#[repr(C)]
pub struct ebt_ip_info {
    pub saddr: u32,
    pub daddr: u32,
    pub smsk: u32,
    pub dmsk: u32,
    pub tos: u8,
    pub protocol: u8,
    pub bitmask: u8,
    pub invflags: u8,
    pub _bindgen_anon_1: ebt_ip_info__bindgen_ty_1,
    pub _bindgen_anon_2: ebt_ip_info__bindgen_ty_2,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
