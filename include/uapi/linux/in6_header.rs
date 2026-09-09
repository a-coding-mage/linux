/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Types and definitions for AF_INET6 (translated from linux/in6.h). */

// C dependencies: linux/types.h and linux/libc-compat.h.
// __UAPI_DEF_* preprocessor conditions are preserved here as source intent;
// this translation exposes the corresponding declarations directly.

#[repr(C)]
pub union In6AddrUnion {
    pub u6_addr8: [u8; 16],
    pub u6_addr16: [u16; 8],
    pub u6_addr32: [u32; 4],
}

#[repr(C)]
pub struct in6_addr {
    pub in6_u: In6AddrUnion,
}

// C aliases: s6_addr = in6_u.u6_addr8, s6_addr16 = in6_u.u6_addr16,
// and s6_addr32 = in6_u.u6_addr32.

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,   // AF_INET6
    pub sin6_port: u16,     // Transport layer port # (__be16)
    pub sin6_flowinfo: u32, // IPv6 flow information (__be32)
    pub sin6_addr: in6_addr, // IPv6 address
    pub sin6_scope_id: u32, // scope id (new in RFC2553)
}

#[repr(C)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr, // IPv6 multicast address of group
    pub ipv6mr_ifindex: i32,        // local IPv6 address of interface
}

// #define ipv6mr_acaddr ipv6mr_multiaddr

#[repr(C)]
pub struct in6_flowlabel_req {
    pub flr_dst: in6_addr,
    pub flr_label: u32,
    pub flr_action: u8,
    pub flr_share: u8,
    pub flr_flags: u16,
    pub flr_expires: u16,
    pub flr_linger: u16,
    pub __flr_pad: u32,
    // Options in format of IPV6_PKTOPTIONS
}

pub const IPV6_FL_A_GET: u32 = 0;
pub const IPV6_FL_A_PUT: u32 = 1;
pub const IPV6_FL_A_RENEW: u32 = 2;
pub const IPV6_FL_F_CREATE: u32 = 1;
pub const IPV6_FL_F_EXCL: u32 = 2;
pub const IPV6_FL_F_REFLECT: u32 = 4;
pub const IPV6_FL_F_REMOTE: u32 = 8;
pub const IPV6_FL_S_NONE: u32 = 0;
pub const IPV6_FL_S_EXCL: u32 = 1;
pub const IPV6_FL_S_PROCESS: u32 = 2;
pub const IPV6_FL_S_USER: u32 = 3;
pub const IPV6_FL_S_ANY: u32 = 255;

pub const IPV6_FLOWINFO_FLOWLABEL: u32 = 0x000fffff;
pub const IPV6_FLOWINFO_PRIORITY: u32 = 0x0ff00000;

pub const IPV6_PRIORITY_UNCHARACTERIZED: u32 = 0x0000;
pub const IPV6_PRIORITY_FILLER: u32 = 0x0100;
pub const IPV6_PRIORITY_UNATTENDED: u32 = 0x0200;
pub const IPV6_PRIORITY_RESERVED1: u32 = 0x0300;
pub const IPV6_PRIORITY_BULK: u32 = 0x0400;
pub const IPV6_PRIORITY_RESERVED2: u32 = 0x0500;
pub const IPV6_PRIORITY_INTERACTIVE: u32 = 0x0600;
pub const IPV6_PRIORITY_CONTROL: u32 = 0x0700;
pub const IPV6_PRIORITY_8: u32 = 0x0800;
pub const IPV6_PRIORITY_9: u32 = 0x0900;
pub const IPV6_PRIORITY_10: u32 = 0x0a00;
pub const IPV6_PRIORITY_11: u32 = 0x0b00;
pub const IPV6_PRIORITY_12: u32 = 0x0c00;
pub const IPV6_PRIORITY_13: u32 = 0x0d00;
pub const IPV6_PRIORITY_14: u32 = 0x0e00;
pub const IPV6_PRIORITY_15: u32 = 0x0f00;

pub const IPPROTO_HOPOPTS: u32 = 0;
pub const IPPROTO_ROUTING: u32 = 43;
pub const IPPROTO_FRAGMENT: u32 = 44;
pub const IPPROTO_ICMPV6: u32 = 58;
pub const IPPROTO_NONE: u32 = 59;
pub const IPPROTO_DSTOPTS: u32 = 60;
pub const IPPROTO_MH: u32 = 135;

pub const IPV6_TLV_PAD1: u32 = 0;
pub const IPV6_TLV_PADN: u32 = 1;
pub const IPV6_TLV_ROUTERALERT: u32 = 5;
pub const IPV6_TLV_CALIPSO: u32 = 7;
pub const IPV6_TLV_IOAM: u32 = 49;
pub const IPV6_TLV_JUMBO: u32 = 194;
pub const IPV6_TLV_HAO: u32 = 201;

pub const IPV6_ADDRFORM: u32 = 1;
pub const IPV6_2292PKTINFO: u32 = 2;
pub const IPV6_2292HOPOPTS: u32 = 3;
pub const IPV6_2292DSTOPTS: u32 = 4;
pub const IPV6_2292RTHDR: u32 = 5;
pub const IPV6_2292PKTOPTIONS: u32 = 6;
pub const IPV6_CHECKSUM: u32 = 7;
pub const IPV6_2292HOPLIMIT: u32 = 8;
pub const IPV6_NEXTHOP: u32 = 9;
pub const IPV6_AUTHHDR: u32 = 10;
pub const IPV6_FLOWINFO: u32 = 11;
pub const IPV6_UNICAST_HOPS: u32 = 16;
pub const IPV6_MULTICAST_IF: u32 = 17;
pub const IPV6_MULTICAST_HOPS: u32 = 18;
pub const IPV6_MULTICAST_LOOP: u32 = 19;
pub const IPV6_ADD_MEMBERSHIP: u32 = 20;
pub const IPV6_DROP_MEMBERSHIP: u32 = 21;
pub const IPV6_ROUTER_ALERT: u32 = 22;
pub const IPV6_MTU_DISCOVER: u32 = 23;
pub const IPV6_MTU: u32 = 24;
pub const IPV6_RECVERR: u32 = 25;
pub const IPV6_V6ONLY: u32 = 26;
pub const IPV6_JOIN_ANYCAST: u32 = 27;
pub const IPV6_LEAVE_ANYCAST: u32 = 28;
pub const IPV6_MULTICAST_ALL: u32 = 29;
pub const IPV6_ROUTER_ALERT_ISOLATE: u32 = 30;
pub const IPV6_RECVERR_RFC4884: u32 = 31;

pub const IPV6_PMTUDISC_DONT: u32 = 0;
pub const IPV6_PMTUDISC_WANT: u32 = 1;
pub const IPV6_PMTUDISC_DO: u32 = 2;
pub const IPV6_PMTUDISC_PROBE: u32 = 3;
pub const IPV6_PMTUDISC_INTERFACE: u32 = 4;
pub const IPV6_PMTUDISC_OMIT: u32 = 5;
pub const IPV6_FLOWLABEL_MGR: u32 = 32;
pub const IPV6_FLOWINFO_SEND: u32 = 33;
pub const IPV6_IPSEC_POLICY: u32 = 34;
pub const IPV6_XFRM_POLICY: u32 = 35;
pub const IPV6_HDRINCL: u32 = 36;
pub const IPV6_RECVPKTINFO: u32 = 49;
pub const IPV6_PKTINFO: u32 = 50;
pub const IPV6_RECVHOPLIMIT: u32 = 51;
pub const IPV6_HOPLIMIT: u32 = 52;
pub const IPV6_RECVHOPOPTS: u32 = 53;
pub const IPV6_HOPOPTS: u32 = 54;
pub const IPV6_RTHDRDSTOPTS: u32 = 55;
pub const IPV6_RECVRTHDR: u32 = 56;
pub const IPV6_RTHDR: u32 = 57;
pub const IPV6_RECVDSTOPTS: u32 = 58;
pub const IPV6_DSTOPTS: u32 = 59;
pub const IPV6_RECVPATHMTU: u32 = 60;
pub const IPV6_PATHMTU: u32 = 61;
pub const IPV6_DONTFRAG: u32 = 62;
pub const IPV6_RECVTCLASS: u32 = 66;
pub const IPV6_TCLASS: u32 = 67;
pub const IPV6_AUTOFLOWLABEL: u32 = 70;
pub const IPV6_ADDR_PREFERENCES: u32 = 72;
pub const IPV6_PREFER_SRC_TMP: u32 = 0x0001;
pub const IPV6_PREFER_SRC_PUBLIC: u32 = 0x0002;
pub const IPV6_PREFER_SRC_PUBTMP_DEFAULT: u32 = 0x0100;
pub const IPV6_PREFER_SRC_COA: u32 = 0x0004;
pub const IPV6_PREFER_SRC_HOME: u32 = 0x0400;
pub const IPV6_PREFER_SRC_CGA: u32 = 0x0008;
pub const IPV6_PREFER_SRC_NONCGA: u32 = 0x0800;
pub const IPV6_MINHOPCOUNT: u32 = 73;
pub const IPV6_ORIGDSTADDR: u32 = 74;
pub const IPV6_RECVORIGDSTADDR: u32 = IPV6_ORIGDSTADDR;
pub const IPV6_TRANSPARENT: u32 = 75;
pub const IPV6_UNICAST_IF: u32 = 76;
pub const IPV6_RECVFRAGSIZE: u32 = 77;
pub const IPV6_FREEBIND: u32 = 78;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
