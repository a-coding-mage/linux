/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from the Linux UAPI rtnetlink header. External Linux types and
// netlink macros referenced here are supplied by other translated headers.

pub const RTNL_FAMILY_IPMR: u8 = 128;
pub const RTNL_FAMILY_IP6MR: u8 = 129;
pub const RTNL_FAMILY_MAX: u8 = 129;

pub const RTM_BASE: u32 = 16;
pub const RTM_NEWLINK: u32 = 16; pub const RTM_DELLINK: u32 = 17; pub const RTM_GETLINK: u32 = 18; pub const RTM_SETLINK: u32 = 19;
pub const RTM_NEWADDR: u32 = 20; pub const RTM_DELADDR: u32 = 21; pub const RTM_GETADDR: u32 = 22;
pub const RTM_NEWROUTE: u32 = 24; pub const RTM_DELROUTE: u32 = 25; pub const RTM_GETROUTE: u32 = 26;
pub const RTM_NEWNEIGH: u32 = 28; pub const RTM_DELNEIGH: u32 = 29; pub const RTM_GETNEIGH: u32 = 30;
pub const RTM_NEWRULE: u32 = 32; pub const RTM_DELRULE: u32 = 33; pub const RTM_GETRULE: u32 = 34;
pub const RTM_NEWQDISC: u32 = 36; pub const RTM_DELQDISC: u32 = 37; pub const RTM_GETQDISC: u32 = 38;
pub const RTM_NEWTCLASS: u32 = 40; pub const RTM_DELTCLASS: u32 = 41; pub const RTM_GETTCLASS: u32 = 42;
pub const RTM_NEWTFILTER: u32 = 44; pub const RTM_DELTFILTER: u32 = 45; pub const RTM_GETTFILTER: u32 = 46;
pub const RTM_NEWACTION: u32 = 48; pub const RTM_DELACTION: u32 = 49; pub const RTM_GETACTION: u32 = 50;
pub const RTM_NEWPREFIX: u32 = 52;
pub const RTM_NEWMULTICAST: u32 = 56; pub const RTM_DELMULTICAST: u32 = 57; pub const RTM_GETMULTICAST: u32 = 58;
pub const RTM_NEWANYCAST: u32 = 60; pub const RTM_DELANYCAST: u32 = 61; pub const RTM_GETANYCAST: u32 = 62;
pub const RTM_NEWNEIGHTBL: u32 = 64; pub const RTM_GETNEIGHTBL: u32 = 66; pub const RTM_SETNEIGHTBL: u32 = 67;
pub const RTM_NEWNDUSEROPT: u32 = 68;
pub const RTM_NEWADDRLABEL: u32 = 72; pub const RTM_DELADDRLABEL: u32 = 73; pub const RTM_GETADDRLABEL: u32 = 74;
pub const RTM_GETDCB: u32 = 78; pub const RTM_SETDCB: u32 = 79;
pub const RTM_NEWNETCONF: u32 = 80; pub const RTM_DELNETCONF: u32 = 81; pub const RTM_GETNETCONF: u32 = 82;
pub const RTM_NEWMDB: u32 = 84; pub const RTM_DELMDB: u32 = 85; pub const RTM_GETMDB: u32 = 86;
pub const RTM_NEWNSID: u32 = 88; pub const RTM_DELNSID: u32 = 89; pub const RTM_GETNSID: u32 = 90;
pub const RTM_NEWSTATS: u32 = 92; pub const RTM_GETSTATS: u32 = 94; pub const RTM_SETSTATS: u32 = 95;
pub const RTM_NEWCACHEREPORT: u32 = 96;
pub const RTM_NEWCHAIN: u32 = 100; pub const RTM_DELCHAIN: u32 = 101; pub const RTM_GETCHAIN: u32 = 102;
pub const RTM_NEWNEXTHOP: u32 = 104; pub const RTM_DELNEXTHOP: u32 = 105; pub const RTM_GETNEXTHOP: u32 = 106;
pub const RTM_NEWLINKPROP: u32 = 108; pub const RTM_DELLINKPROP: u32 = 109; pub const RTM_GETLINKPROP: u32 = 110;
pub const RTM_NEWVLAN: u32 = 112; pub const RTM_DELVLAN: u32 = 113; pub const RTM_GETVLAN: u32 = 114;
pub const RTM_NEWNEXTHOPBUCKET: u32 = 116; pub const RTM_DELNEXTHOPBUCKET: u32 = 117; pub const RTM_GETNEXTHOPBUCKET: u32 = 118;
pub const RTM_NEWTUNNEL: u32 = 120; pub const RTM_DELTUNNEL: u32 = 121; pub const RTM_GETTUNNEL: u32 = 122;
pub const __RTM_MAX: u32 = 123;
pub const RTM_MAX: u32 = ((__RTM_MAX + 3) & !3) - 1;
pub const RTM_NR_MSGTYPES: u32 = RTM_MAX + 1 - RTM_BASE;
pub const RTM_NR_FAMILIES: u32 = RTM_NR_MSGTYPES >> 2;
#[inline] pub const fn RTM_FAM(cmd: u32) -> u32 { (cmd - RTM_BASE) >> 2 }

#[repr(C)] #[derive(Copy, Clone)] pub struct rtattr { pub rta_len: u16, pub rta_type: u16 }
pub const RTA_ALIGNTO: usize = 4;
#[inline] pub const fn RTA_ALIGN(len: usize) -> usize { (len + 3) & !3 }
#[inline] pub const fn RTA_LENGTH(len: usize) -> usize { RTA_ALIGN(core::mem::size_of::<rtattr>()) + len }
#[inline] pub const fn RTA_SPACE(len: usize) -> usize { RTA_ALIGN(RTA_LENGTH(len)) }
#[inline] pub unsafe fn RTA_OK(rta: *const rtattr, len: isize) -> bool { len >= core::mem::size_of::<rtattr>() as isize && (*rta).rta_len as usize >= core::mem::size_of::<rtattr>() && (*rta).rta_len as isize <= len }
#[inline] pub unsafe fn RTA_NEXT(rta: *mut rtattr, attrlen: &mut isize) -> *mut rtattr { *attrlen -= RTA_ALIGN((*rta).rta_len as usize) as isize; (rta as *mut u8).add(RTA_ALIGN((*rta).rta_len as usize)) as *mut rtattr }
#[inline] pub unsafe fn RTA_DATA(rta: *mut rtattr) -> *mut core::ffi::c_void { (rta as *mut u8).add(RTA_LENGTH(0)) as *mut _ }
#[inline] pub unsafe fn RTA_PAYLOAD(rta: *const rtattr) -> isize { (*rta).rta_len as isize - RTA_LENGTH(0) as isize }

#[repr(C)] #[derive(Copy, Clone)] pub struct rtmsg { pub rtm_family:u8, pub rtm_dst_len:u8, pub rtm_src_len:u8, pub rtm_tos:u8, pub rtm_table:u8, pub rtm_protocol:u8, pub rtm_scope:u8, pub rtm_type:u8, pub rtm_flags:u32 }
pub const RTN_UNSPEC:u32=0; pub const RTN_UNICAST:u32=1; pub const RTN_LOCAL:u32=2; pub const RTN_BROADCAST:u32=3; pub const RTN_ANYCAST:u32=4; pub const RTN_MULTICAST:u32=5; pub const RTN_BLACKHOLE:u32=6; pub const RTN_UNREACHABLE:u32=7; pub const RTN_PROHIBIT:u32=8; pub const RTN_THROW:u32=9; pub const RTN_NAT:u32=10; pub const RTN_XRESOLVE:u32=11; pub const __RTN_MAX:u32=12; pub const RTN_MAX:u32=11;
pub const RTPROT_UNSPEC:u32=0; pub const RTPROT_REDIRECT:u32=1; pub const RTPROT_KERNEL:u32=2; pub const RTPROT_BOOT:u32=3; pub const RTPROT_STATIC:u32=4; pub const RTPROT_GATED:u32=8; pub const RTPROT_RA:u32=9; pub const RTPROT_MRT:u32=10; pub const RTPROT_ZEBRA:u32=11; pub const RTPROT_BIRD:u32=12; pub const RTPROT_DNROUTED:u32=13; pub const RTPROT_XORP:u32=14; pub const RTPROT_NTK:u32=15; pub const RTPROT_DHCP:u32=16; pub const RTPROT_MROUTED:u32=17; pub const RTPROT_KEEPALIVED:u32=18; pub const RTPROT_BABEL:u32=42; pub const RTPROT_OVN:u32=84; pub const RTPROT_OPENR:u32=99; pub const RTPROT_BGP:u32=186; pub const RTPROT_ISIS:u32=187; pub const RTPROT_OSPF:u32=188; pub const RTPROT_RIP:u32=189; pub const RTPROT_EIGRP:u32=192;
pub const RT_SCOPE_UNIVERSE:u8=0; pub const RT_SCOPE_SITE:u8=200; pub const RT_SCOPE_LINK:u8=253; pub const RT_SCOPE_HOST:u8=254; pub const RT_SCOPE_NOWHERE:u8=255;
pub const RTM_F_NOTIFY:u32=0x100; pub const RTM_F_CLONED:u32=0x200; pub const RTM_F_EQUALIZE:u32=0x400; pub const RTM_F_PREFIX:u32=0x800; pub const RTM_F_LOOKUP_TABLE:u32=0x1000; pub const RTM_F_FIB_MATCH:u32=0x2000; pub const RTM_F_OFFLOAD:u32=0x4000; pub const RTM_F_TRAP:u32=0x8000; pub const RTM_F_OFFLOAD_FAILED:u32=0x20000000;
pub const RT_TABLE_UNSPEC:u32=0; pub const RT_TABLE_COMPAT:u32=252; pub const RT_TABLE_DEFAULT:u32=253; pub const RT_TABLE_MAIN:u32=254; pub const RT_TABLE_LOCAL:u32=255; pub const RT_TABLE_MAX:u32=0xffffffff;

pub const RTA_UNSPEC:u32=0; pub const RTA_DST:u32=1; pub const RTA_SRC:u32=2; pub const RTA_IIF:u32=3; pub const RTA_OIF:u32=4; pub const RTA_GATEWAY:u32=5; pub const RTA_PRIORITY:u32=6; pub const RTA_PREFSRC:u32=7; pub const RTA_METRICS:u32=8; pub const RTA_MULTIPATH:u32=9; pub const RTA_PROTOINFO:u32=10; pub const RTA_FLOW:u32=11; pub const RTA_CACHEINFO:u32=12; pub const RTA_SESSION:u32=13; pub const RTA_MP_ALGO:u32=14; pub const RTA_TABLE:u32=15; pub const RTA_MARK:u32=16; pub const RTA_MFC_STATS:u32=17; pub const RTA_VIA:u32=18; pub const RTA_NEWDST:u32=19; pub const RTA_PREF:u32=20; pub const RTA_ENCAP_TYPE:u32=21; pub const RTA_ENCAP:u32=22; pub const RTA_EXPIRES:u32=23; pub const RTA_PAD:u32=24; pub const RTA_UID:u32=25; pub const RTA_TTL_PROPAGATE:u32=26; pub const RTA_IP_PROTO:u32=27; pub const RTA_SPORT:u32=28; pub const RTA_DPORT:u32=29; pub const RTA_NH_ID:u32=30; pub const RTA_FLOWLABEL:u32=31; pub const RTA_DEL_REASON:u32=32; pub const __RTA_MAX:u32=33; pub const RTA_MAX:u32=32;
#[inline] pub unsafe fn RTM_RTA(r: *mut rtmsg) -> *mut rtattr { (r as *mut u8).add(NLMSG_ALIGN(core::mem::size_of::<rtmsg>())) as *mut _ }
#[inline] pub unsafe fn RTM_PAYLOAD(n: *const core::ffi::c_void) -> usize { NLMSG_PAYLOAD(n, core::mem::size_of::<rtmsg>()) }

pub const RT_DEL_REASON_UNSPEC:u32=0; pub const RT_DEL_REASON_EXPIRED:u32=1; pub const RT_DEL_REASON_RA_WITHDRAWN:u32=2; pub const __RT_DEL_REASON_MAX:u32=3; pub const RT_DEL_REASON_MAX:u32=2;
#[repr(C)] #[derive(Copy,Clone)] pub struct rtnexthop { pub rtnh_len:u16, pub rtnh_flags:u8, pub rtnh_hops:u8, pub rtnh_ifindex:i32 }
pub const RTNH_F_DEAD:u32=1; pub const RTNH_F_PERVASIVE:u32=2; pub const RTNH_F_ONLINK:u32=4; pub const RTNH_F_OFFLOAD:u32=8; pub const RTNH_F_LINKDOWN:u32=16; pub const RTNH_F_UNRESOLVED:u32=32; pub const RTNH_F_TRAP:u32=64; pub const RTNH_COMPARE_MASK:u32=1|16|8|64;
#[inline] pub const fn RTNH_ALIGN(len:usize)->usize {(len+3)&!3} #[inline] pub const fn RTNH_LENGTH(len:usize)->usize {RTNH_ALIGN(core::mem::size_of::<rtnexthop>())+len} #[inline] pub const fn RTNH_SPACE(len:usize)->usize {RTNH_ALIGN(RTNH_LENGTH(len))}
#[inline] pub unsafe fn RTNH_OK(r:*const rtnexthop,len:isize)->bool {(*r).rtnh_len as usize>=core::mem::size_of::<rtnexthop>() && (*r).rtnh_len as isize<=len} #[inline] pub unsafe fn RTNH_NEXT(r:*mut rtnexthop)->*mut rtnexthop {(r as *mut u8).add(RTNH_ALIGN((*r).rtnh_len as usize)) as *mut _} #[inline] pub unsafe fn RTNH_DATA(r:*mut rtnexthop)->*mut rtattr {(r as *mut u8).add(RTNH_LENGTH(0)) as *mut _}

#[repr(C)] pub struct rtvia { pub rtvia_family: u16, pub rtvia_addr: [u8;0] }
#[repr(C)] #[derive(Copy,Clone)] pub struct rta_cacheinfo { pub rta_clntref:u32,pub rta_lastuse:u32,pub rta_expires:i32,pub rta_error:u32,pub rta_used:u32,pub rta_id:u32,pub rta_ts:u32,pub rta_tsage:u32 }
pub const RTNETLINK_HAVE_PEERINFO:u32=1;
pub const RTAX_UNSPEC:u32=0; pub const RTAX_LOCK:u32=1; pub const RTAX_MTU:u32=2; pub const RTAX_WINDOW:u32=3; pub const RTAX_RTT:u32=4; pub const RTAX_RTTVAR:u32=5; pub const RTAX_SSTHRESH:u32=6; pub const RTAX_CWND:u32=7; pub const RTAX_ADVMSS:u32=8; pub const RTAX_REORDERING:u32=9; pub const RTAX_HOPLIMIT:u32=10; pub const RTAX_INITCWND:u32=11; pub const RTAX_FEATURES:u32=12; pub const RTAX_RTO_MIN:u32=13; pub const RTAX_INITRWND:u32=14; pub const RTAX_QUICKACK:u32=15; pub const RTAX_CC_ALGO:u32=16; pub const RTAX_FASTOPEN_NO_COOKIE:u32=17; pub const __RTAX_MAX:u32=18; pub const RTAX_MAX:u32=17;
pub const RTAX_FEATURE_ECN:u32=1; pub const RTAX_FEATURE_SACK:u32=2; pub const RTAX_FEATURE_TIMESTAMP:u32=4; pub const RTAX_FEATURE_ALLFRAG:u32=8; pub const RTAX_FEATURE_TCP_USEC_TS:u32=16; pub const RTAX_FEATURE_MASK:u32=31;
#[repr(C)] pub union rta_session_u { pub ports: rta_session_ports, pub icmpt: rta_session_icmpt, pub spi:u32 } #[repr(C)] #[derive(Copy,Clone)] pub struct rta_session_ports {pub sport:u16,pub dport:u16} #[repr(C)] #[derive(Copy,Clone)] pub struct rta_session_icmpt {pub r#type:u8,pub code:u8,pub ident:u16} #[repr(C)] pub struct rta_session {pub proto:u8,pub pad1:u8,pub pad2:u16,pub u:rta_session_u} #[repr(C)] pub struct rta_mfc_stats {pub mfcs_packets:u64,pub mfcs_bytes:u64,pub mfcs_wrong_if:u64}

#[repr(C)] pub struct rtgenmsg {pub rtgen_family:u8} #[repr(C)] pub struct ifinfomsg {pub ifi_family:u8,pub __ifi_pad:u8,pub ifi_type:u16,pub ifi_index:i32,pub ifi_flags:u32,pub ifi_change:u32}
#[repr(C)] pub struct prefixmsg {pub prefix_family:u8,pub prefix_pad1:u8,pub prefix_pad2:u16,pub prefix_ifindex:i32,pub prefix_type:u8,pub prefix_len:u8,pub prefix_flags:u8,pub prefix_pad3:u8} pub const PREFIX_UNSPEC:u32=0;pub const PREFIX_ADDRESS:u32=1;pub const PREFIX_CACHEINFO:u32=2;pub const __PREFIX_MAX:u32=3;pub const PREFIX_MAX:u32=2; #[repr(C)] pub struct prefix_cacheinfo {pub preferred_time:u32,pub valid_time:u32}
#[repr(C)] pub struct tcmsg {pub tcm_family:u8,pub tcm__pad1:u8,pub tcm__pad2:u16,pub tcm_ifindex:i32,pub tcm_handle:u32,pub tcm_parent:u32,pub tcm_info:u32} pub const TCM_IFINDEX_MAGIC_BLOCK:u32=0xffffffff;
pub const TCA_UNSPEC:u32=0;pub const TCA_KIND:u32=1;pub const TCA_OPTIONS:u32=2;pub const TCA_STATS:u32=3;pub const TCA_XSTATS:u32=4;pub const TCA_RATE:u32=5;pub const TCA_FCNT:u32=6;pub const TCA_STATS2:u32=7;pub const TCA_STAB:u32=8;pub const TCA_PAD:u32=9;pub const TCA_DUMP_INVISIBLE:u32=10;pub const TCA_CHAIN:u32=11;pub const TCA_HW_OFFLOAD:u32=12;pub const TCA_INGRESS_BLOCK:u32=13;pub const TCA_EGRESS_BLOCK:u32=14;pub const TCA_DUMP_FLAGS:u32=15;pub const TCA_EXT_WARN_MSG:u32=16;pub const __TCA_MAX:u32=17;pub const TCA_MAX:u32=16;pub const TCA_DUMP_FLAGS_TERSE:u32=1;
#[repr(C)] pub struct nduseroptmsg {pub nduseropt_family:u8,pub nduseropt_pad1:u8,pub nduseropt_opts_len:u16,pub nduseropt_ifindex:i32,pub nduseropt_icmp_type:u8,pub nduseropt_icmp_code:u8,pub nduseropt_pad2:u16,pub nduseropt_pad3:u32} pub const NDUSEROPT_UNSPEC:u32=0;pub const NDUSEROPT_SRCADDR:u32=1;pub const __NDUSEROPT_MAX:u32=2;pub const NDUSEROPT_MAX:u32=1;
#[repr(C)] pub struct tcamsg {pub tca_family:u8,pub tca__pad1:u8,pub tca__pad2:u16} pub const TCA_ROOT_UNSPEC:u32=0;pub const TCA_ROOT_TAB:u32=1;pub const TCA_ACT_TAB:u32=1;pub const TCAA_MAX:u32=1;pub const TCA_ROOT_FLAGS:u32=2;pub const TCA_ROOT_COUNT:u32=3;pub const TCA_ROOT_TIME_DELTA:u32=4;pub const TCA_ROOT_EXT_WARN_MSG:u32=5;pub const __TCA_ROOT_MAX:u32=6;pub const TCA_ROOT_MAX:u32=5;pub const TCA_FLAG_LARGE_DUMP_ON:u32=1;pub const TCA_ACT_FLAG_LARGE_DUMP_ON:u32=1;pub const TCA_ACT_FLAG_TERSE_DUMP:u32=2;
pub const RTEXT_FILTER_VF:u32=1;pub const RTEXT_FILTER_BRVLAN:u32=2;pub const RTEXT_FILTER_BRVLAN_COMPRESSED:u32=4;pub const RTEXT_FILTER_SKIP_STATS:u32=8;pub const RTEXT_FILTER_MRP:u32=16;pub const RTEXT_FILTER_CFM_CONFIG:u32=32;pub const RTEXT_FILTER_CFM_STATUS:u32=64;pub const RTEXT_FILTER_MST:u32=128;pub const RTEXT_FILTER_NAME_ONLY:u32=256;

// RTnetlink multicast groups (userspace compatibility names).
pub const RTMGRP_LINK:u32=1; pub const RTMGRP_NOTIFY:u32=2; pub const RTMGRP_NEIGH:u32=4; pub const RTMGRP_TC:u32=8; pub const RTMGRP_IPV4_IFADDR:u32=0x10; pub const RTMGRP_IPV4_MROUTE:u32=0x20; pub const RTMGRP_IPV4_ROUTE:u32=0x40; pub const RTMGRP_IPV4_RULE:u32=0x80; pub const RTMGRP_IPV6_IFADDR:u32=0x100; pub const RTMGRP_IPV6_MROUTE:u32=0x200; pub const RTMGRP_IPV6_ROUTE:u32=0x400; pub const RTMGRP_IPV6_IFINFO:u32=0x800; pub const RTMGRP_DECnet_IFADDR:u32=0x1000; pub const RTMGRP_DECnet_ROUTE:u32=0x4000; pub const RTMGRP_IPV6_PREFIX:u32=0x20000;
pub const RTNLGRP_NONE:u32=0; pub const RTNLGRP_LINK:u32=1; pub const RTNLGRP_NOTIFY:u32=2; pub const RTNLGRP_NEIGH:u32=3; pub const RTNLGRP_TC:u32=4; pub const RTNLGRP_IPV4_IFADDR:u32=5; pub const RTNLGRP_IPV4_MROUTE:u32=6; pub const RTNLGRP_IPV4_ROUTE:u32=7; pub const RTNLGRP_IPV4_RULE:u32=8; pub const RTNLGRP_IPV6_IFADDR:u32=9; pub const RTNLGRP_IPV6_MROUTE:u32=10; pub const RTNLGRP_IPV6_ROUTE:u32=11; pub const RTNLGRP_IPV6_IFINFO:u32=12; pub const RTNLGRP_DECnet_IFADDR:u32=13; pub const RTNLGRP_NOP2:u32=14; pub const RTNLGRP_DECnet_ROUTE:u32=15; pub const RTNLGRP_DECnet_RULE:u32=16; pub const RTNLGRP_NOP4:u32=17; pub const RTNLGRP_IPV6_PREFIX:u32=18; pub const RTNLGRP_IPV6_RULE:u32=19; pub const RTNLGRP_ND_USEROPT:u32=20; pub const RTNLGRP_PHONET_IFADDR:u32=21; pub const RTNLGRP_PHONET_ROUTE:u32=22; pub const RTNLGRP_DCB:u32=23; pub const RTNLGRP_IPV4_NETCONF:u32=24; pub const RTNLGRP_IPV6_NETCONF:u32=25; pub const RTNLGRP_MDB:u32=26; pub const RTNLGRP_MPLS_ROUTE:u32=27; pub const RTNLGRP_NSID:u32=28; pub const RTNLGRP_MPLS_NETCONF:u32=29; pub const RTNLGRP_IPV4_MROUTE_R:u32=30; pub const RTNLGRP_IPV6_MROUTE_R:u32=31; pub const RTNLGRP_NEXTHOP:u32=32; pub const RTNLGRP_BRVLAN:u32=33; pub const RTNLGRP_MCTP_IFADDR:u32=34; pub const RTNLGRP_TUNNEL:u32=35; pub const RTNLGRP_STATS:u32=36; pub const RTNLGRP_IPV4_MCADDR:u32=37; pub const RTNLGRP_IPV6_MCADDR:u32=38; pub const RTNLGRP_IPV6_ACADDR:u32=39; pub const __RTNLGRP_MAX:u32=40; pub const RTNLGRP_MAX:u32=39;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
