/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Definitions of the Internet Protocol. */

/* The declarations below are conditional in the C header on the corresponding
 * __UAPI_DEF_* libc-compatibility configuration symbols. */

#[cfg(__UAPI_DEF_IN_IPPROTO)]
#[repr(i32)]
pub enum IpProto {
    IPPROTO_IP = 0,
    IPPROTO_ICMP = 1,
    IPPROTO_IGMP = 2,
    IPPROTO_IPIP = 4,
    IPPROTO_TCP = 6,
    IPPROTO_EGP = 8,
    IPPROTO_PUP = 12,
    IPPROTO_UDP = 17,
    IPPROTO_IDP = 22,
    IPPROTO_TP = 29,
    IPPROTO_DCCP = 33,
    IPPROTO_IPV6 = 41,
    IPPROTO_RSVP = 46,
    IPPROTO_GRE = 47,
    IPPROTO_ESP = 50,
    IPPROTO_AH = 51,
    IPPROTO_MTP = 92,
    IPPROTO_BEETPH = 94,
    IPPROTO_ENCAP = 98,
    IPPROTO_PIM = 103,
    IPPROTO_COMP = 108,
    IPPROTO_L2TP = 115,
    IPPROTO_SCTP = 132,
    IPPROTO_UDPLITE = 136,
    IPPROTO_MPLS = 137,
    IPPROTO_ETHERNET = 143,
    IPPROTO_AGGFRAG = 144,
    IPPROTO_RAW = 255,
    IPPROTO_SMC = 256,
    IPPROTO_MPTCP = 262,
    IPPROTO_MAX,
}

#[cfg(__UAPI_DEF_IN_ADDR)]
#[repr(C)]
pub struct in_addr { pub s_addr: __be32 }

pub const IP_TOS: i32 = 1;
pub const IP_TTL: i32 = 2;
pub const IP_HDRINCL: i32 = 3;
pub const IP_OPTIONS: i32 = 4;
pub const IP_ROUTER_ALERT: i32 = 5;
pub const IP_RECVOPTS: i32 = 6;
pub const IP_RETOPTS: i32 = 7;
pub const IP_PKTINFO: i32 = 8;
pub const IP_PKTOPTIONS: i32 = 9;
pub const IP_MTU_DISCOVER: i32 = 10;
pub const IP_RECVERR: i32 = 11;
pub const IP_RECVTTL: i32 = 12;
pub const IP_RECVTOS: i32 = 13;
pub const IP_MTU: i32 = 14;
pub const IP_FREEBIND: i32 = 15;
pub const IP_IPSEC_POLICY: i32 = 16;
pub const IP_XFRM_POLICY: i32 = 17;
pub const IP_PASSSEC: i32 = 18;
pub const IP_TRANSPARENT: i32 = 19;
pub const IP_RECVRETOPTS: i32 = IP_RETOPTS;
pub const IP_ORIGDSTADDR: i32 = 20;
pub const IP_RECVORIGDSTADDR: i32 = IP_ORIGDSTADDR;
pub const IP_MINTTL: i32 = 21;
pub const IP_NODEFRAG: i32 = 22;
pub const IP_CHECKSUM: i32 = 23;
pub const IP_BIND_ADDRESS_NO_PORT: i32 = 24;
pub const IP_RECVFRAGSIZE: i32 = 25;
pub const IP_RECVERR_RFC4884: i32 = 26;

pub const IP_PMTUDISC_DONT: i32 = 0;
pub const IP_PMTUDISC_WANT: i32 = 1;
pub const IP_PMTUDISC_DO: i32 = 2;
pub const IP_PMTUDISC_PROBE: i32 = 3;
pub const IP_PMTUDISC_INTERFACE: i32 = 4;
pub const IP_PMTUDISC_OMIT: i32 = 5;

pub const IP_MULTICAST_IF: i32 = 32;
pub const IP_MULTICAST_TTL: i32 = 33;
pub const IP_MULTICAST_LOOP: i32 = 34;
pub const IP_ADD_MEMBERSHIP: i32 = 35;
pub const IP_DROP_MEMBERSHIP: i32 = 36;
pub const IP_UNBLOCK_SOURCE: i32 = 37;
pub const IP_BLOCK_SOURCE: i32 = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP: i32 = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP: i32 = 40;
pub const IP_MSFILTER: i32 = 41;
pub const MCAST_JOIN_GROUP: i32 = 42;
pub const MCAST_BLOCK_SOURCE: i32 = 43;
pub const MCAST_UNBLOCK_SOURCE: i32 = 44;
pub const MCAST_LEAVE_GROUP: i32 = 45;
pub const MCAST_JOIN_SOURCE_GROUP: i32 = 46;
pub const MCAST_LEAVE_SOURCE_GROUP: i32 = 47;
pub const MCAST_MSFILTER: i32 = 48;
pub const IP_MULTICAST_ALL: i32 = 49;
pub const IP_UNICAST_IF: i32 = 50;
pub const IP_LOCAL_PORT_RANGE: i32 = 51;
pub const IP_PROTOCOL: i32 = 52;
pub const MCAST_EXCLUDE: i32 = 0;
pub const MCAST_INCLUDE: i32 = 1;
pub const IP_DEFAULT_MULTICAST_TTL: i32 = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: i32 = 1;

#[cfg(__UAPI_DEF_IP_MREQ)]
#[repr(C)]
pub struct ip_mreq { pub imr_multiaddr: in_addr, pub imr_interface: in_addr }
#[cfg(__UAPI_DEF_IP_MREQ)]
#[repr(C)]
pub struct ip_mreqn { pub imr_multiaddr: in_addr, pub imr_address: in_addr, pub imr_ifindex: i32 }
#[cfg(__UAPI_DEF_IP_MREQ)]
#[repr(C)]
pub struct ip_mreq_source { pub imr_multiaddr: __be32, pub imr_interface: __be32, pub imr_sourceaddr: __be32 }
#[cfg(__UAPI_DEF_IP_MREQ)]
#[repr(C)]
pub union ip_msfilter_slist { pub imsf_slist: [__be32; 1], pub imsf_slist_flex: [__be32; 0] }
#[cfg(__UAPI_DEF_IP_MREQ)]
#[repr(C)]
pub struct ip_msfilter { pub imsf_multiaddr: __be32, pub imsf_interface: __be32, pub imsf_fmode: __u32, pub imsf_numsrc: __u32, pub __bindgen_anon_1: ip_msfilter_slist }
#[inline]
pub const fn IP_MSFILTER_SIZE(numsrc: usize) -> usize {
    core::mem::size_of::<ip_msfilter>() - core::mem::size_of::<__u32>() + numsrc * core::mem::size_of::<__u32>()
}

#[cfg(__UAPI_DEF_IP_MREQ)]
#[repr(C)]
pub struct group_req { pub gr_interface: __u32, pub gr_group: __kernel_sockaddr_storage }
#[cfg(__UAPI_DEF_IP_MREQ)]
#[repr(C)]
pub struct group_source_req {
    pub gsr_interface: __u32,
    pub gsr_group: __kernel_sockaddr_storage,
    pub gsr_source: __kernel_sockaddr_storage,
}
#[cfg(__UAPI_DEF_IP_MREQ)]
#[repr(C)]
pub union group_filter {
    pub aux: group_filter_aux,
    pub flexible: group_filter_flexible,
}
#[cfg(__UAPI_DEF_IP_MREQ)]
#[repr(C)]
pub struct group_filter_aux {
    pub gf_interface_aux: __u32,
    pub gf_group_aux: __kernel_sockaddr_storage,
    pub gf_fmode_aux: __u32,
    pub gf_numsrc_aux: __u32,
    pub gf_slist: [__kernel_sockaddr_storage; 1],
}
#[cfg(__UAPI_DEF_IP_MREQ)]
#[repr(C)]
pub struct group_filter_flexible {
    pub gf_interface: __u32,
    pub gf_group: __kernel_sockaddr_storage,
    pub gf_fmode: __u32,
    pub gf_numsrc: __u32,
    pub gf_slist_flex: [__kernel_sockaddr_storage; 0],
}
#[inline]
pub const fn GROUP_FILTER_SIZE(numsrc: usize) -> usize {
    core::mem::size_of::<group_filter>() - core::mem::size_of::<__kernel_sockaddr_storage>() + numsrc * core::mem::size_of::<__kernel_sockaddr_storage>()
}

#[cfg(__UAPI_DEF_IN_PKTINFO)]
#[repr(C)]
pub struct in_pktinfo { pub ipi_ifindex: i32, pub ipi_spec_dst: in_addr, pub ipi_addr: in_addr }

#[cfg(__UAPI_DEF_SOCKADDR_IN)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: __kernel_sa_family_t,
    pub sin_port: __be16,
    pub sin_addr: in_addr,
    pub __pad: [u8; 8],
}

#[inline]
pub const fn IN_CLASSA(a: u32) -> bool { (a & 0x80000000) == 0 }
pub const IN_CLASSA_NET: u32 = 0xff000000;
pub const IN_CLASSA_NSHIFT: i32 = 24;
pub const IN_CLASSA_HOST: u32 = 0x00ffffff;
pub const IN_CLASSA_MAX: i32 = 128;
#[inline]
pub const fn IN_CLASSB(a: u32) -> bool { (a & 0xc0000000) == 0x80000000 }
pub const IN_CLASSB_NET: u32 = 0xffff0000;
pub const IN_CLASSB_NSHIFT: i32 = 16;
pub const IN_CLASSB_HOST: u32 = 0x0000ffff;
pub const IN_CLASSB_MAX: i32 = 65536;
#[inline]
pub const fn IN_CLASSC(a: u32) -> bool { (a & 0xe0000000) == 0xc0000000 }
pub const IN_CLASSC_NET: u32 = 0xffffff00;
pub const IN_CLASSC_NSHIFT: i32 = 8;
pub const IN_CLASSC_HOST: u32 = 0x000000ff;
#[inline]
pub const fn IN_CLASSD(a: u32) -> bool { (a & 0xf0000000) == 0xe0000000 }
pub const fn IN_MULTICAST(a: u32) -> bool { IN_CLASSD(a) }
pub const IN_MULTICAST_NET: u32 = 0xe0000000;
pub const fn IN_BADCLASS(a: u32) -> bool { a == 0xffffffff }
pub const fn IN_EXPERIMENTAL(a: u32) -> bool { IN_BADCLASS(a) }
pub const fn IN_CLASSE(a: u32) -> bool { (a & 0xf0000000) == 0xf0000000 }
pub const IN_CLASSE_NET: u32 = 0xffffffff;
pub const IN_CLASSE_NSHIFT: i32 = 0;
pub const INADDR_ANY: u64 = 0;
pub const INADDR_BROADCAST: u64 = 0xffffffff;
pub const INADDR_NONE: u64 = 0xffffffff;
pub const INADDR_DUMMY: u64 = 0xc0000008;
pub const IN_LOOPBACKNET: i32 = 127;
pub const INADDR_LOOPBACK: u32 = 0x7f000001;
pub const fn IN_LOOPBACK(a: u32) -> bool { (a & 0xff000000) == 0x7f000000 }
pub const INADDR_UNSPEC_GROUP: u32 = 0xe0000000;
pub const INADDR_ALLHOSTS_GROUP: u32 = 0xe0000001;
pub const INADDR_ALLRTRS_GROUP: u32 = 0xe0000002;
pub const INADDR_ALLSNOOPERS_GROUP: u32 = 0xe000006a;
pub const INADDR_MAX_LOCAL_GROUP: u32 = 0xe00000ff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
