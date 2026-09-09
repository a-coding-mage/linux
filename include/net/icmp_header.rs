/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET	An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Definitions for the ICMP module.
 *
 * Version:	@(#)icmp.h	1.0.4	05/13/93
 *
 * Authors:	Ross Biro
 *		Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 */

// Dependencies supplied by the corresponding Linux networking headers:
// linux/icmp.h, net/inet_sock.h, net/snmp.h, and net/ip.h.

#[repr(C)]
pub struct icmp_err {
    pub errno: i32,
    pub fatal: u32,
}

extern "C" {
    pub static icmp_err_convert: [icmp_err; 0];
}

macro_rules! ICMP_INC_STATS {
    ($net:expr, $field:expr) => {
        SNMP_INC_STATS!((*$net).mib.icmp_statistics, $field)
    };
}

macro_rules! __ICMP_INC_STATS {
    ($net:expr, $field:expr) => {
        __SNMP_INC_STATS!((*$net).mib.icmp_statistics, $field)
    };
}

macro_rules! ICMPMSGOUT_INC_STATS {
    ($net:expr, $field:expr) => {
        SNMP_INC_STATS_ATOMIC_LONG!((*$net).mib.icmpmsg_statistics, ($field) + 256)
    };
}

macro_rules! ICMPMSGIN_INC_STATS {
    ($net:expr, $field:expr) => {
        SNMP_INC_STATS_ATOMIC_LONG!((*$net).mib.icmpmsg_statistics, $field)
    };
}

#[repr(C)]
pub struct dst_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_proto_family {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

extern "C" {
    pub fn __icmp_send(
        skb_in: *mut sk_buff,
        type_: i32,
        code: i32,
        info: __be32,
        parm: *const inet_skb_parm,
    );
}

#[inline]
pub unsafe fn icmp_send(skb_in: *mut sk_buff, type_: i32, code: i32, info: __be32) {
    __icmp_send(skb_in, type_, code, info, IPCB!(skb_in));
}

// The CONFIG_NF_NAT condition is preserved from the source build configuration.
#[cfg(feature = "CONFIG_NF_NAT")]
extern "C" {
    pub fn icmp_ndo_send(skb_in: *mut sk_buff, type_: i32, code: i32, info: __be32);
}

#[cfg(not(feature = "CONFIG_NF_NAT"))]
#[inline]
pub unsafe fn icmp_ndo_send(skb_in: *mut sk_buff, type_: i32, code: i32, info: __be32) {
    let mut parm: inet_skb_parm = core::mem::zeroed();
    __icmp_send(skb_in, type_, code, info, &parm);
}

extern "C" {
    pub fn icmp_rcv(skb: *mut sk_buff) -> i32;
    pub fn icmp_err(skb: *mut sk_buff, info: u32) -> i32;
    pub fn icmp_init() -> i32;
    pub fn icmp_out_count(net: *mut net, type_: u8);
    pub fn icmp_build_probe(skb: *mut sk_buff, icmphdr: *mut icmphdr) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
