/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Definitions of the Internet Protocol.
 *
 * Version:	@(#)in.h	1.0.1	04/21/93
 *
 * Authors:	Original taken from the GNU Project <netinet/in.h> file.
 *		Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *
 *		This program is free software; you can redistribute it and/or
 *		modify it under the terms of the GNU General Public License
 *		as published by the Free Software Foundation; either version
 *		2 of the License, or (at your option) any later version.
 */

/* Depends on linux/types.h, linux/stddef.h, linux/libc-compat.h, linux/socket.h. */

/* Original C guarded this block with __UAPI_DEF_IN_IPPROTO. */
/* Standard well-defined IP protocols. */
pub const IPPROTO_IP: u32 = 0; /* Dummy protocol for TCP */
pub const IPPROTO_ICMP: u32 = 1; /* Internet Control Message Protocol */
pub const IPPROTO_IGMP: u32 = 2; /* Internet Group Management Protocol */
pub const IPPROTO_IPIP: u32 = 4; /* IPIP tunnels (older KA9Q tunnels use 94) */
pub const IPPROTO_TCP: u32 = 6; /* Transmission Control Protocol */
pub const IPPROTO_EGP: u32 = 8; /* Exterior Gateway Protocol */
pub const IPPROTO_PUP: u32 = 12; /* PUP protocol */
pub const IPPROTO_UDP: u32 = 17; /* User Datagram Protocol */
pub const IPPROTO_IDP: u32 = 22; /* XNS IDP protocol */
pub const IPPROTO_TP: u32 = 29; /* SO Transport Protocol Class 4 */
pub const IPPROTO_DCCP: u32 = 33; /* Datagram Congestion Control Protocol */
pub const IPPROTO_IPV6: u32 = 41; /* IPv6-in-IPv4 tunnelling */
pub const IPPROTO_RSVP: u32 = 46; /* RSVP Protocol */
pub const IPPROTO_GRE: u32 = 47; /* Cisco GRE tunnels (rfc 1701,1702) */
pub const IPPROTO_ESP: u32 = 50; /* Encapsulation Security Payload protocol */
pub const IPPROTO_AH: u32 = 51; /* Authentication Header protocol */
pub const IPPROTO_MTP: u32 = 92; /* Multicast Transport Protocol */
pub const IPPROTO_BEETPH: u32 = 94; /* IP option pseudo header for BEET */
pub const IPPROTO_ENCAP: u32 = 98; /* Encapsulation Header */
pub const IPPROTO_PIM: u32 = 103; /* Protocol Independent Multicast */
pub const IPPROTO_COMP: u32 = 108; /* Compression Header Protocol */
pub const IPPROTO_L2TP: u32 = 115; /* Layer 2 Tunnelling Protocol */
pub const IPPROTO_SCTP: u32 = 132; /* Stream Control Transport Protocol */
pub const IPPROTO_UDPLITE: u32 = 136; /* UDP-Lite (RFC 3828) */
pub const IPPROTO_MPLS: u32 = 137; /* MPLS in IP (RFC 4023) */
pub const IPPROTO_ETHERNET: u32 = 143; /* Ethernet-within-IPv6 Encapsulation */
pub const IPPROTO_AGGFRAG: u32 = 144; /* AGGFRAG in ESP (RFC 9347) */
pub const IPPROTO_RAW: u32 = 255; /* Raw IP packets */
pub const IPPROTO_SMC: u32 = 256; /* Shared Memory Communications */
pub const IPPROTO_MPTCP: u32 = 262; /* Multipath TCP connection */
pub const IPPROTO_MAX: u32 = 263;

/* Original C guarded this block with __UAPI_DEF_IN_ADDR. */
/* Internet address. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: __be32,
}

pub const IP_TOS: u32 = 1;
pub const IP_TTL: u32 = 2;
pub const IP_HDRINCL: u32 = 3;
pub const IP_OPTIONS: u32 = 4;
pub const IP_ROUTER_ALERT: u32 = 5;
pub const IP_RECVOPTS: u32 = 6;
pub const IP_RETOPTS: u32 = 7;
pub const IP_PKTINFO: u32 = 8;
pub const IP_PKTOPTIONS: u32 = 9;
pub const IP_MTU_DISCOVER: u32 = 10;
pub const IP_RECVERR: u32 = 11;
pub const IP_RECVTTL: u32 = 12;
pub const IP_RECVTOS: u32 = 13;
pub const IP_MTU: u32 = 14;
pub const IP_FREEBIND: u32 = 15;
pub const IP_IPSEC_POLICY: u32 = 16;
pub const IP_XFRM_POLICY: u32 = 17;
pub const IP_PASSSEC: u32 = 18;
pub const IP_TRANSPARENT: u32 = 19;

/* BSD compatibility */
pub const IP_RECVRETOPTS: u32 = IP_RETOPTS;

/* TProxy original addresses */
pub const IP_ORIGDSTADDR: u32 = 20;
pub const IP_RECVORIGDSTADDR: u32 = IP_ORIGDSTADDR;

pub const IP_MINTTL: u32 = 21;
pub const IP_NODEFRAG: u32 = 22;
pub const IP_CHECKSUM: u32 = 23;
pub const IP_BIND_ADDRESS_NO_PORT: u32 = 24;
pub const IP_RECVFRAGSIZE: u32 = 25;
pub const IP_RECVERR_RFC4884: u32 = 26;

/* IP_MTU_DISCOVER values */
pub const IP_PMTUDISC_DONT: u32 = 0; /* Never send DF frames */
pub const IP_PMTUDISC_WANT: u32 = 1; /* Use per route hints */
pub const IP_PMTUDISC_DO: u32 = 2; /* Always DF */
pub const IP_PMTUDISC_PROBE: u32 = 3; /* Ignore dst pmtu */
/* Always use interface mtu (ignores dst pmtu) but don't set DF flag.
 * Also incoming ICMP frag_needed notifications will be ignored on
 * this socket to prevent accepting spoofed ones.
 */
pub const IP_PMTUDISC_INTERFACE: u32 = 4;
/* weaker version of IP_PMTUDISC_INTERFACE, which allows packets to get
 * fragmented if they exceed the interface mtu
 */
pub const IP_PMTUDISC_OMIT: u32 = 5;

pub const IP_MULTICAST_IF: u32 = 32;
pub const IP_MULTICAST_TTL: u32 = 33;
pub const IP_MULTICAST_LOOP: u32 = 34;
pub const IP_ADD_MEMBERSHIP: u32 = 35;
pub const IP_DROP_MEMBERSHIP: u32 = 36;
pub const IP_UNBLOCK_SOURCE: u32 = 37;
pub const IP_BLOCK_SOURCE: u32 = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP: u32 = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP: u32 = 40;
pub const IP_MSFILTER: u32 = 41;
pub const MCAST_JOIN_GROUP: u32 = 42;
pub const MCAST_BLOCK_SOURCE: u32 = 43;
pub const MCAST_UNBLOCK_SOURCE: u32 = 44;
pub const MCAST_LEAVE_GROUP: u32 = 45;
pub const MCAST_JOIN_SOURCE_GROUP: u32 = 46;
pub const MCAST_LEAVE_SOURCE_GROUP: u32 = 47;
pub const MCAST_MSFILTER: u32 = 48;
pub const IP_MULTICAST_ALL: u32 = 49;
pub const IP_UNICAST_IF: u32 = 50;
pub const IP_LOCAL_PORT_RANGE: u32 = 51;
pub const IP_PROTOCOL: u32 = 52;

pub const MCAST_EXCLUDE: u32 = 0;
pub const MCAST_INCLUDE: u32 = 1;

/* These need to appear somewhere around here */
pub const IP_DEFAULT_MULTICAST_TTL: u32 = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: u32 = 1;

/* Request struct for multicast socket ops */

/* Original C guarded this block with __UAPI_DEF_IP_MREQ. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr, /* IP multicast address of group */
    pub imr_interface: in_addr, /* local IP address of interface */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_mreqn {
    pub imr_multiaddr: in_addr, /* IP multicast address of group */
    pub imr_address: in_addr,   /* local IP address of interface */
    pub imr_ifindex: i32,       /* Interface index */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_mreq_source {
    pub imr_multiaddr: __be32,
    pub imr_interface: __be32,
    pub imr_sourceaddr: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_msfilter {
    pub imsf_multiaddr: __be32,
    pub imsf_interface: __be32,
    pub imsf_fmode: __u32,
    pub imsf_numsrc: __u32,
    pub u: ip_msfilter_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ip_msfilter_union {
    pub imsf_slist: [__be32; 1],
    pub imsf_slist_flex: [__be32; 0],
}

pub const fn IP_MSFILTER_SIZE(numsrc: usize) -> usize {
    core::mem::size_of::<ip_msfilter>() - core::mem::size_of::<__u32>()
        + numsrc * core::mem::size_of::<__u32>()
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_req {
    pub gr_interface: __u32,                         /* interface index */
    pub gr_group: __kernel_sockaddr_storage,         /* group address */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_source_req {
    pub gsr_interface: __u32,                /* interface index */
    pub gsr_group: __kernel_sockaddr_storage, /* group address */
    pub gsr_source: __kernel_sockaddr_storage, /* source address */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_filter {
    pub u: group_filter_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union group_filter_union {
    pub aux: group_filter_aux,
    pub flex: group_filter_flex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_filter_aux {
    pub gf_interface_aux: __u32,                 /* interface index */
    pub gf_group_aux: __kernel_sockaddr_storage, /* multicast address */
    pub gf_fmode_aux: __u32,                     /* filter mode */
    pub gf_numsrc_aux: __u32,                    /* number of sources */
    pub gf_slist: [__kernel_sockaddr_storage; 1], /* interface index */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_filter_flex {
    pub gf_interface: __u32,                      /* interface index */
    pub gf_group: __kernel_sockaddr_storage,      /* multicast address */
    pub gf_fmode: __u32,                          /* filter mode */
    pub gf_numsrc: __u32,                         /* number of sources */
    pub gf_slist_flex: [__kernel_sockaddr_storage; 0], /* interface index */
}

pub const fn GROUP_FILTER_SIZE(numsrc: usize) -> usize {
    core::mem::size_of::<group_filter>() - core::mem::size_of::<__kernel_sockaddr_storage>()
        + numsrc * core::mem::size_of::<__kernel_sockaddr_storage>()
}

/* Original C guarded this block with __UAPI_DEF_IN_PKTINFO. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_pktinfo {
    pub ipi_ifindex: i32,
    pub ipi_spec_dst: in_addr,
    pub ipi_addr: in_addr,
}

/* Structure describing an Internet (IP) socket address. */
/* Original C guarded this block with __UAPI_DEF_SOCKADDR_IN. */
pub const __SOCK_SIZE__: usize = 16; /* sizeof(struct sockaddr) */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: __kernel_sa_family_t, /* Address family */
    pub sin_port: __be16,                 /* Port number */
    pub sin_addr: in_addr,                /* Internet address */

    /* Pad to size of `struct sockaddr'. */
    pub __pad: [u8; __SOCK_SIZE__
        - core::mem::size_of::<i16>()
        - core::mem::size_of::<u16>()
        - core::mem::size_of::<in_addr>()],
}

/* sin_zero was a C macro alias for sockaddr_in.__pad, for BSD UNIX comp. -FvK. */

/* Original C guarded this block with __UAPI_DEF_IN_CLASS. */
/*
 * Definitions of the bits in an Internet address integer.
 * On subnets, host and network parts are found according
 * to the subnet mask, not these masks.
 */
pub const fn IN_CLASSA(a: u32) -> bool {
    (a & 0x80000000) == 0
}
pub const IN_CLASSA_NET: u32 = 0xff000000;
pub const IN_CLASSA_NSHIFT: u32 = 24;
pub const IN_CLASSA_HOST: u32 = 0xffffffff & !IN_CLASSA_NET;
pub const IN_CLASSA_MAX: u32 = 128;

pub const fn IN_CLASSB(a: u32) -> bool {
    (a & 0xc0000000) == 0x80000000
}
pub const IN_CLASSB_NET: u32 = 0xffff0000;
pub const IN_CLASSB_NSHIFT: u32 = 16;
pub const IN_CLASSB_HOST: u32 = 0xffffffff & !IN_CLASSB_NET;
pub const IN_CLASSB_MAX: u32 = 65536;

pub const fn IN_CLASSC(a: u32) -> bool {
    (a & 0xe0000000) == 0xc0000000
}
pub const IN_CLASSC_NET: u32 = 0xffffff00;
pub const IN_CLASSC_NSHIFT: u32 = 8;
pub const IN_CLASSC_HOST: u32 = 0xffffffff & !IN_CLASSC_NET;

pub const fn IN_CLASSD(a: u32) -> bool {
    (a & 0xf0000000) == 0xe0000000
}
pub const fn IN_MULTICAST(a: u32) -> bool {
    IN_CLASSD(a)
}
pub const IN_MULTICAST_NET: u32 = 0xe0000000;

pub const fn IN_BADCLASS(a: u32) -> bool {
    a == 0xffffffff
}
pub const fn IN_EXPERIMENTAL(a: u32) -> bool {
    IN_BADCLASS(a)
}

pub const fn IN_CLASSE(a: u32) -> bool {
    (a & 0xf0000000) == 0xf0000000
}
pub const IN_CLASSE_NET: u32 = 0xffffffff;
pub const IN_CLASSE_NSHIFT: u32 = 0;

/* Address to accept any incoming messages. */
pub const INADDR_ANY: usize = 0x00000000;

/* Address to send to all hosts. */
pub const INADDR_BROADCAST: usize = 0xffffffff;

/* Address indicating an error return. */
pub const INADDR_NONE: usize = 0xffffffff;

/* Dummy address for src of ICMP replies if no real address is set (RFC7600). */
pub const INADDR_DUMMY: usize = 0xc0000008;

/* Network number for local host loopback. */
pub const IN_LOOPBACKNET: u32 = 127;

/* Address to loopback in software to local host. */
pub const INADDR_LOOPBACK: u32 = 0x7f000001; /* 127.0.0.1 */
pub const fn IN_LOOPBACK(a: u32) -> bool {
    (a & 0xff000000) == 0x7f000000
}

/* Defines for Multicast INADDR */
pub const INADDR_UNSPEC_GROUP: u32 = 0xe0000000; /* 224.0.0.0 */
pub const INADDR_ALLHOSTS_GROUP: u32 = 0xe0000001; /* 224.0.0.1 */
pub const INADDR_ALLRTRS_GROUP: u32 = 0xe0000002; /* 224.0.0.2 */
pub const INADDR_ALLSNOOPERS_GROUP: u32 = 0xe000006a; /* 224.0.0.106 */
pub const INADDR_MAX_LOCAL_GROUP: u32 = 0xe00000ff; /* 224.0.0.255 */

/* asm/byteorder.h contains the htonl type stuff. */
