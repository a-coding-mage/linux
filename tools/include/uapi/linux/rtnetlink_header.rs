/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies from the original header:
 * <linux/types.h>, <linux/netlink.h>, <linux/if_link.h>,
 * <linux/if_addr.h>, <linux/neighbour.h>
 */

use core::ffi::c_void;
use core::mem::size_of;

pub const RTNL_FAMILY_IPMR: u32 = 128;
pub const RTNL_FAMILY_IP6MR: u32 = 129;
pub const RTNL_FAMILY_MAX: u32 = 129;

/****
 *		Routing/neighbour discovery messages.
 ****/

/* Types of messages */

pub const RTM_BASE: u32 = 16;
pub const RTM_NEWLINK: u32 = 16;
pub const RTM_DELLINK: u32 = 17;
pub const RTM_GETLINK: u32 = 18;
pub const RTM_SETLINK: u32 = 19;
pub const RTM_NEWADDR: u32 = 20;
pub const RTM_DELADDR: u32 = 21;
pub const RTM_GETADDR: u32 = 22;
pub const RTM_NEWROUTE: u32 = 24;
pub const RTM_DELROUTE: u32 = 25;
pub const RTM_GETROUTE: u32 = 26;
pub const RTM_NEWNEIGH: u32 = 28;
pub const RTM_DELNEIGH: u32 = 29;
pub const RTM_GETNEIGH: u32 = 30;
pub const RTM_NEWRULE: u32 = 32;
pub const RTM_DELRULE: u32 = 33;
pub const RTM_GETRULE: u32 = 34;
pub const RTM_NEWQDISC: u32 = 36;
pub const RTM_DELQDISC: u32 = 37;
pub const RTM_GETQDISC: u32 = 38;
pub const RTM_NEWTCLASS: u32 = 40;
pub const RTM_DELTCLASS: u32 = 41;
pub const RTM_GETTCLASS: u32 = 42;
pub const RTM_NEWTFILTER: u32 = 44;
pub const RTM_DELTFILTER: u32 = 45;
pub const RTM_GETTFILTER: u32 = 46;
pub const RTM_NEWACTION: u32 = 48;
pub const RTM_DELACTION: u32 = 49;
pub const RTM_GETACTION: u32 = 50;
pub const RTM_NEWPREFIX: u32 = 52;
pub const RTM_NEWMULTICAST: u32 = 56;
pub const RTM_DELMULTICAST: u32 = 57;
pub const RTM_GETMULTICAST: u32 = 58;
pub const RTM_NEWANYCAST: u32 = 60;
pub const RTM_DELANYCAST: u32 = 61;
pub const RTM_GETANYCAST: u32 = 62;
pub const RTM_NEWNEIGHTBL: u32 = 64;
pub const RTM_GETNEIGHTBL: u32 = 66;
pub const RTM_SETNEIGHTBL: u32 = 67;
pub const RTM_NEWNDUSEROPT: u32 = 68;
pub const RTM_NEWADDRLABEL: u32 = 72;
pub const RTM_DELADDRLABEL: u32 = 73;
pub const RTM_GETADDRLABEL: u32 = 74;
pub const RTM_GETDCB: u32 = 78;
pub const RTM_SETDCB: u32 = 79;
pub const RTM_NEWNETCONF: u32 = 80;
pub const RTM_DELNETCONF: u32 = 81;
pub const RTM_GETNETCONF: u32 = 82;
pub const RTM_NEWMDB: u32 = 84;
pub const RTM_DELMDB: u32 = 85;
pub const RTM_GETMDB: u32 = 86;
pub const RTM_NEWNSID: u32 = 88;
pub const RTM_DELNSID: u32 = 89;
pub const RTM_GETNSID: u32 = 90;
pub const RTM_NEWSTATS: u32 = 92;
pub const RTM_GETSTATS: u32 = 94;
pub const RTM_SETSTATS: u32 = 95;
pub const RTM_NEWCACHEREPORT: u32 = 96;
pub const RTM_NEWCHAIN: u32 = 100;
pub const RTM_DELCHAIN: u32 = 101;
pub const RTM_GETCHAIN: u32 = 102;
pub const RTM_NEWNEXTHOP: u32 = 104;
pub const RTM_DELNEXTHOP: u32 = 105;
pub const RTM_GETNEXTHOP: u32 = 106;
pub const RTM_NEWLINKPROP: u32 = 108;
pub const RTM_DELLINKPROP: u32 = 109;
pub const RTM_GETLINKPROP: u32 = 110;
pub const RTM_NEWVLAN: u32 = 112;
pub const RTM_DELVLAN: u32 = 113;
pub const RTM_GETVLAN: u32 = 114;
pub const RTM_NEWNEXTHOPBUCKET: u32 = 116;
pub const RTM_DELNEXTHOPBUCKET: u32 = 117;
pub const RTM_GETNEXTHOPBUCKET: u32 = 118;
pub const RTM_NEWTUNNEL: u32 = 120;
pub const RTM_DELTUNNEL: u32 = 121;
pub const RTM_GETTUNNEL: u32 = 122;
pub const __RTM_MAX: u32 = 123;
pub const RTM_MAX: u32 = (((__RTM_MAX + 3) & !3) - 1);

pub const RTM_NR_MSGTYPES: u32 = RTM_MAX + 1 - RTM_BASE;
pub const RTM_NR_FAMILIES: u32 = RTM_NR_MSGTYPES >> 2;
pub const fn RTM_FAM(cmd: u32) -> u32 {
    (cmd - RTM_BASE) >> 2
}

/*
   Generic structure for encapsulation of optional route information.
   It is reminiscent of sockaddr, but with sa_family replaced
   with attribute type.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtattr {
    pub rta_len: u16,
    pub rta_type: u16,
}

/* Macros to handle rtattributes */

pub const RTA_ALIGNTO: usize = 4;
pub const fn RTA_ALIGN(len: usize) -> usize {
    (len + RTA_ALIGNTO - 1) & !(RTA_ALIGNTO - 1)
}
pub unsafe fn RTA_OK(rta: *const rtattr, len: i32) -> bool {
    len >= size_of::<rtattr>() as i32
        && unsafe { (*rta).rta_len as usize } >= size_of::<rtattr>()
        && unsafe { (*rta).rta_len as i32 } <= len
}
pub unsafe fn RTA_NEXT(rta: *mut rtattr, attrlen: *mut i32) -> *mut rtattr {
    let aligned = RTA_ALIGN(unsafe { (*rta).rta_len as usize }) as i32;
    unsafe {
        *attrlen -= aligned;
        (rta as *mut u8).add(aligned as usize) as *mut rtattr
    }
}
pub const fn RTA_LENGTH(len: usize) -> usize {
    RTA_ALIGN(size_of::<rtattr>()) + len
}
pub const fn RTA_SPACE(len: usize) -> usize {
    RTA_ALIGN(RTA_LENGTH(len))
}
pub unsafe fn RTA_DATA(rta: *mut rtattr) -> *mut c_void {
    unsafe { (rta as *mut u8).add(RTA_LENGTH(0)) as *mut c_void }
}
pub unsafe fn RTA_PAYLOAD(rta: *const rtattr) -> i32 {
    unsafe { (*rta).rta_len as i32 } - RTA_LENGTH(0) as i32
}

/******************************************************************************
 *		Definitions used in routing table administration.
 ****/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtmsg {
    pub rtm_family: u8,
    pub rtm_dst_len: u8,
    pub rtm_src_len: u8,
    pub rtm_tos: u8,
    pub rtm_table: u8,    /* Routing table id */
    pub rtm_protocol: u8, /* Routing protocol; see below	*/
    pub rtm_scope: u8,    /* See below */
    pub rtm_type: u8,     /* See below	*/
    pub rtm_flags: u32,
}

/* rtm_type */

pub const RTN_UNSPEC: u32 = 0;
pub const RTN_UNICAST: u32 = 1; /* Gateway or direct route	*/
pub const RTN_LOCAL: u32 = 2; /* Accept locally		*/
pub const RTN_BROADCAST: u32 = 3; /* Accept locally as broadcast,
                                   * send as broadcast */
pub const RTN_ANYCAST: u32 = 4; /* Accept locally as broadcast,
                                 * but send as unicast */
pub const RTN_MULTICAST: u32 = 5; /* Multicast route		*/
pub const RTN_BLACKHOLE: u32 = 6; /* Drop				*/
pub const RTN_UNREACHABLE: u32 = 7; /* Destination is unreachable   */
pub const RTN_PROHIBIT: u32 = 8; /* Administratively prohibited	*/
pub const RTN_THROW: u32 = 9; /* Not in this table		*/
pub const RTN_NAT: u32 = 10; /* Translate this address	*/
pub const RTN_XRESOLVE: u32 = 11; /* Use external resolver	*/
pub const __RTN_MAX: u32 = 12;
pub const RTN_MAX: u32 = __RTN_MAX - 1;

/* rtm_protocol */

pub const RTPROT_UNSPEC: u32 = 0;
pub const RTPROT_REDIRECT: u32 = 1; /* Route installed by ICMP redirects;
                                     * not used by current IPv4 */
pub const RTPROT_KERNEL: u32 = 2; /* Route installed by kernel		*/
pub const RTPROT_BOOT: u32 = 3; /* Route installed during boot		*/
pub const RTPROT_STATIC: u32 = 4; /* Route installed by administrator	*/

/* Values of protocol >= RTPROT_STATIC are not interpreted by kernel;
   they are just passed from user and back as is.
   It will be used by hypothetical multiple routing daemons.
   Note that protocol values should be standardized in order to
   avoid conflicts.
 */

pub const RTPROT_GATED: u32 = 8; /* Apparently, GateD */
pub const RTPROT_RA: u32 = 9; /* RDISC/ND router advertisements */
pub const RTPROT_MRT: u32 = 10; /* Merit MRT */
pub const RTPROT_ZEBRA: u32 = 11; /* Zebra */
pub const RTPROT_BIRD: u32 = 12; /* BIRD */
pub const RTPROT_DNROUTED: u32 = 13; /* DECnet routing daemon */
pub const RTPROT_XORP: u32 = 14; /* XORP */
pub const RTPROT_NTK: u32 = 15; /* Netsukuku */
pub const RTPROT_DHCP: u32 = 16; /* DHCP client */
pub const RTPROT_MROUTED: u32 = 17; /* Multicast daemon */
pub const RTPROT_KEEPALIVED: u32 = 18; /* Keepalived daemon */
pub const RTPROT_BABEL: u32 = 42; /* Babel daemon */
pub const RTPROT_OVN: u32 = 84; /* OVN daemon */
pub const RTPROT_OPENR: u32 = 99; /* Open Routing (Open/R) Routes */
pub const RTPROT_BGP: u32 = 186; /* BGP Routes */
pub const RTPROT_ISIS: u32 = 187; /* ISIS Routes */
pub const RTPROT_OSPF: u32 = 188; /* OSPF Routes */
pub const RTPROT_RIP: u32 = 189; /* RIP Routes */
pub const RTPROT_EIGRP: u32 = 192; /* EIGRP Routes */

/* rtm_scope

   Really it is not scope, but sort of distance to the destination.
   NOWHERE are reserved for not existing destinations, HOST is our
   local addresses, LINK are destinations, located on directly attached
   link and UNIVERSE is everywhere in the Universe.

   Intermediate values are also possible f.e. interior routes
   could be assigned a value between UNIVERSE and LINK.
*/

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt_scope_t {
    RT_SCOPE_UNIVERSE = 0,
    /* User defined values  */
    RT_SCOPE_SITE = 200,
    RT_SCOPE_LINK = 253,
    RT_SCOPE_HOST = 254,
    RT_SCOPE_NOWHERE = 255,
}

/* rtm_flags */

pub const RTM_F_NOTIFY: u32 = 0x100; /* Notify user of route change	*/
pub const RTM_F_CLONED: u32 = 0x200; /* This route is cloned		*/
pub const RTM_F_EQUALIZE: u32 = 0x400; /* Multipath equalizer: NI	*/
pub const RTM_F_PREFIX: u32 = 0x800; /* Prefix addresses		*/
pub const RTM_F_LOOKUP_TABLE: u32 = 0x1000; /* set rtm_table to FIB lookup result */
pub const RTM_F_FIB_MATCH: u32 = 0x2000; /* return full fib lookup match */
pub const RTM_F_OFFLOAD: u32 = 0x4000; /* route is offloaded */
pub const RTM_F_TRAP: u32 = 0x8000; /* route is trapping packets */
pub const RTM_F_OFFLOAD_FAILED: u32 = 0x20000000; /* route offload failed, this value
                                                   * is chosen to avoid conflicts with
                                                   * other flags defined in
                                                   * include/uapi/linux/ipv6_route.h
                                                   */

/* Reserved table identifiers */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt_class_t {
    RT_TABLE_UNSPEC = 0,
    /* User defined values */
    RT_TABLE_COMPAT = 252,
    RT_TABLE_DEFAULT = 253,
    RT_TABLE_MAIN = 254,
    RT_TABLE_LOCAL = 255,
    RT_TABLE_MAX = 0xFFFFFFFF,
}

/* Routing message attributes */

pub const RTA_UNSPEC: u32 = 0;
pub const RTA_DST: u32 = 1;
pub const RTA_SRC: u32 = 2;
pub const RTA_IIF: u32 = 3;
pub const RTA_OIF: u32 = 4;
pub const RTA_GATEWAY: u32 = 5;
pub const RTA_PRIORITY: u32 = 6;
pub const RTA_PREFSRC: u32 = 7;
pub const RTA_METRICS: u32 = 8;
pub const RTA_MULTIPATH: u32 = 9;
pub const RTA_PROTOINFO: u32 = 10; /* no longer used */
pub const RTA_FLOW: u32 = 11;
pub const RTA_CACHEINFO: u32 = 12;
pub const RTA_SESSION: u32 = 13; /* no longer used */
pub const RTA_MP_ALGO: u32 = 14; /* no longer used */
pub const RTA_TABLE: u32 = 15;
pub const RTA_MARK: u32 = 16;
pub const RTA_MFC_STATS: u32 = 17;
pub const RTA_VIA: u32 = 18;
pub const RTA_NEWDST: u32 = 19;
pub const RTA_PREF: u32 = 20;
pub const RTA_ENCAP_TYPE: u32 = 21;
pub const RTA_ENCAP: u32 = 22;
pub const RTA_EXPIRES: u32 = 23;
pub const RTA_PAD: u32 = 24;
pub const RTA_UID: u32 = 25;
pub const RTA_TTL_PROPAGATE: u32 = 26;
pub const RTA_IP_PROTO: u32 = 27;
pub const RTA_SPORT: u32 = 28;
pub const RTA_DPORT: u32 = 29;
pub const RTA_NH_ID: u32 = 30;
pub const RTA_FLOWLABEL: u32 = 31;
pub const __RTA_MAX: u32 = 32;
pub const RTA_MAX: u32 = __RTA_MAX - 1;

extern "C" {
    pub fn NLMSG_ALIGN(len: usize) -> usize;
    pub fn NLMSG_PAYLOAD(n: *const c_void, len: usize) -> i32;
}

pub unsafe fn RTM_RTA(r: *mut rtmsg) -> *mut rtattr {
    unsafe { (r as *mut u8).add(NLMSG_ALIGN(size_of::<rtmsg>())) as *mut rtattr }
}
pub unsafe fn RTM_PAYLOAD(n: *const c_void) -> i32 {
    unsafe { NLMSG_PAYLOAD(n, size_of::<rtmsg>()) }
}

/* RTM_MULTIPATH --- array of struct rtnexthop.
 *
 * "struct rtnexthop" describes all necessary nexthop information,
 * i.e. parameters of path to a destination via this nexthop.
 *
 * At the moment it is impossible to set different prefsrc, mtu, window
 * and rtt for different paths from multipath.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtnexthop {
    pub rtnh_len: u16,
    pub rtnh_flags: u8,
    pub rtnh_hops: u8,
    pub rtnh_ifindex: i32,
}

/* rtnh_flags */

pub const RTNH_F_DEAD: u32 = 1; /* Nexthop is dead (used by multipath)	*/
pub const RTNH_F_PERVASIVE: u32 = 2; /* Do recursive gateway lookup	*/
pub const RTNH_F_ONLINK: u32 = 4; /* Gateway is forced on link	*/
pub const RTNH_F_OFFLOAD: u32 = 8; /* Nexthop is offloaded */
pub const RTNH_F_LINKDOWN: u32 = 16; /* carrier-down on nexthop */
pub const RTNH_F_UNRESOLVED: u32 = 32; /* The entry is unresolved (ipmr) */
pub const RTNH_F_TRAP: u32 = 64; /* Nexthop is trapping packets */

pub const RTNH_COMPARE_MASK: u32 = RTNH_F_DEAD | RTNH_F_LINKDOWN | RTNH_F_OFFLOAD | RTNH_F_TRAP;

/* Macros to handle hexthops */

pub const RTNH_ALIGNTO: usize = 4;
pub const fn RTNH_ALIGN(len: usize) -> usize {
    (len + RTNH_ALIGNTO - 1) & !(RTNH_ALIGNTO - 1)
}
pub unsafe fn RTNH_OK(rtnh: *const rtnexthop, len: i32) -> bool {
    unsafe { (*rtnh).rtnh_len as usize } >= size_of::<rtnexthop>()
        && unsafe { (*rtnh).rtnh_len as i32 } <= len
}
pub unsafe fn RTNH_NEXT(rtnh: *mut rtnexthop) -> *mut rtnexthop {
    unsafe { (rtnh as *mut u8).add(RTNH_ALIGN((*rtnh).rtnh_len as usize)) as *mut rtnexthop }
}
pub const fn RTNH_LENGTH(len: usize) -> usize {
    RTNH_ALIGN(size_of::<rtnexthop>()) + len
}
pub const fn RTNH_SPACE(len: usize) -> usize {
    RTNH_ALIGN(RTNH_LENGTH(len))
}
pub unsafe fn RTNH_DATA(rtnh: *mut rtnexthop) -> *mut rtattr {
    unsafe { (rtnh as *mut u8).add(RTNH_LENGTH(0)) as *mut rtattr }
}

/* RTA_VIA */
#[repr(C)]
pub struct rtvia {
    pub rtvia_family: __kernel_sa_family_t,
    pub rtvia_addr: [__u8; 0],
}

/* RTM_CACHEINFO */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rta_cacheinfo {
    pub rta_clntref: __u32,
    pub rta_lastuse: __u32,
    pub rta_expires: __s32,
    pub rta_error: __u32,
    pub rta_used: __u32,
    pub rta_id: __u32,
    pub rta_ts: __u32,
    pub rta_tsage: __u32,
}

pub const RTNETLINK_HAVE_PEERINFO: u32 = 1;

/* RTM_METRICS --- array of struct rtattr with types of RTAX_* */

pub const RTAX_UNSPEC: u32 = 0;
pub const RTAX_LOCK: u32 = 1;
pub const RTAX_MTU: u32 = 2;
pub const RTAX_WINDOW: u32 = 3;
pub const RTAX_RTT: u32 = 4;
pub const RTAX_RTTVAR: u32 = 5;
pub const RTAX_SSTHRESH: u32 = 6;
pub const RTAX_CWND: u32 = 7;
pub const RTAX_ADVMSS: u32 = 8;
pub const RTAX_REORDERING: u32 = 9;
pub const RTAX_HOPLIMIT: u32 = 10;
pub const RTAX_INITCWND: u32 = 11;
pub const RTAX_FEATURES: u32 = 12;
pub const RTAX_RTO_MIN: u32 = 13;
pub const RTAX_INITRWND: u32 = 14;
pub const RTAX_QUICKACK: u32 = 15;
pub const RTAX_CC_ALGO: u32 = 16;
pub const RTAX_FASTOPEN_NO_COOKIE: u32 = 17;
pub const __RTAX_MAX: u32 = 18;
pub const RTAX_MAX: u32 = __RTAX_MAX - 1;

pub const RTAX_FEATURE_ECN: u32 = 1 << 0;
pub const RTAX_FEATURE_SACK: u32 = 1 << 1; /* unused */
pub const RTAX_FEATURE_TIMESTAMP: u32 = 1 << 2; /* unused */
pub const RTAX_FEATURE_ALLFRAG: u32 = 1 << 3; /* unused */
pub const RTAX_FEATURE_TCP_USEC_TS: u32 = 1 << 4;

pub const RTAX_FEATURE_MASK: u32 = RTAX_FEATURE_ECN
    | RTAX_FEATURE_SACK
    | RTAX_FEATURE_TIMESTAMP
    | RTAX_FEATURE_ALLFRAG
    | RTAX_FEATURE_TCP_USEC_TS;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rta_session_ports {
    pub sport: __u16,
    pub dport: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rta_session_icmpt {
    pub type_: __u8,
    pub code: __u8,
    pub ident: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rta_session_u {
    pub ports: rta_session_ports,
    pub icmpt: rta_session_icmpt,
    pub spi: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rta_session {
    pub proto: __u8,
    pub pad1: __u8,
    pub pad2: __u16,
    pub u: rta_session_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rta_mfc_stats {
    pub mfcs_packets: __u64,
    pub mfcs_bytes: __u64,
    pub mfcs_wrong_if: __u64,
}

/****
 *		General form of address family dependent message.
 ****/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtgenmsg {
    pub rtgen_family: u8,
}

/*****************************************************************
 *		Link layer specific messages.
 ****/

/* struct ifinfomsg
 * passes link level specific information, not dependent
 * on network protocol.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifinfomsg {
    pub ifi_family: u8,
    pub __ifi_pad: u8,
    pub ifi_type: u16, /* ARPHRD_* */
    pub ifi_index: i32, /* Link index	*/
    pub ifi_flags: u32, /* IFF_* flags	*/
    pub ifi_change: u32, /* IFF_* change mask */
}

/********************************************************************
 *		prefix information
 ****/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prefixmsg {
    pub prefix_family: u8,
    pub prefix_pad1: u8,
    pub prefix_pad2: u16,
    pub prefix_ifindex: i32,
    pub prefix_type: u8,
    pub prefix_len: u8,
    pub prefix_flags: u8,
    pub prefix_pad3: u8,
}

pub const PREFIX_UNSPEC: u32 = 0;
pub const PREFIX_ADDRESS: u32 = 1;
pub const PREFIX_CACHEINFO: u32 = 2;
pub const __PREFIX_MAX: u32 = 3;
pub const PREFIX_MAX: u32 = __PREFIX_MAX - 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prefix_cacheinfo {
    pub preferred_time: __u32,
    pub valid_time: __u32,
}

/*****************************************************************
 *		Traffic control messages.
 ****/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcmsg {
    pub tcm_family: u8,
    pub tcm__pad1: u8,
    pub tcm__pad2: u16,
    pub tcm_ifindex: i32,
    pub tcm_handle: __u32,
    pub tcm_parent: __u32,
    /* tcm_block_index is used instead of tcm_parent
     * in case tcm_ifindex == TCM_IFINDEX_MAGIC_BLOCK
     */
    pub tcm_info: __u32,
}

pub unsafe fn tcm_block_index(r: *mut tcmsg) -> *mut __u32 {
    unsafe { &mut (*r).tcm_parent }
}

/* For manipulation of filters in shared block, tcm_ifindex is set to
 * TCM_IFINDEX_MAGIC_BLOCK, and tcm_parent is aliased to tcm_block_index
 * which is the block index.
 */
pub const TCM_IFINDEX_MAGIC_BLOCK: u32 = 0xFFFFFFFF;

pub const TCA_UNSPEC: u32 = 0;
pub const TCA_KIND: u32 = 1;
pub const TCA_OPTIONS: u32 = 2;
pub const TCA_STATS: u32 = 3;
pub const TCA_XSTATS: u32 = 4;
pub const TCA_RATE: u32 = 5;
pub const TCA_FCNT: u32 = 6;
pub const TCA_STATS2: u32 = 7;
pub const TCA_STAB: u32 = 8;
pub const TCA_PAD: u32 = 9;
pub const TCA_DUMP_INVISIBLE: u32 = 10;
pub const TCA_CHAIN: u32 = 11;
pub const TCA_HW_OFFLOAD: u32 = 12;
pub const TCA_INGRESS_BLOCK: u32 = 13;
pub const TCA_EGRESS_BLOCK: u32 = 14;
pub const TCA_DUMP_FLAGS: u32 = 15;
pub const TCA_EXT_WARN_MSG: u32 = 16;
pub const __TCA_MAX: u32 = 17;
pub const TCA_MAX: u32 = __TCA_MAX - 1;

pub const TCA_DUMP_FLAGS_TERSE: u32 = 1 << 0; /* Means that in dump user gets only basic
                                               * data necessary to identify the objects
                                               * (handle, cookie, etc.) and stats.
                                               */

pub unsafe fn TCA_RTA(r: *mut tcmsg) -> *mut rtattr {
    unsafe { (r as *mut u8).add(NLMSG_ALIGN(size_of::<tcmsg>())) as *mut rtattr }
}
pub unsafe fn TCA_PAYLOAD(n: *const c_void) -> i32 {
    unsafe { NLMSG_PAYLOAD(n, size_of::<tcmsg>()) }
}

/********************************************************************
 *		Neighbor Discovery userland options
 ****/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nduseroptmsg {
    pub nduseropt_family: u8,
    pub nduseropt_pad1: u8,
    pub nduseropt_opts_len: u16, /* Total length of options */
    pub nduseropt_ifindex: i32,
    pub nduseropt_icmp_type: __u8,
    pub nduseropt_icmp_code: __u8,
    pub nduseropt_pad2: u16,
    pub nduseropt_pad3: u32,
    /* Followed by one or more ND options */
}

pub const NDUSEROPT_UNSPEC: u32 = 0;
pub const NDUSEROPT_SRCADDR: u32 = 1;
pub const __NDUSEROPT_MAX: u32 = 2;
pub const NDUSEROPT_MAX: u32 = __NDUSEROPT_MAX - 1;

/* RTnetlink multicast groups - backwards compatibility for userspace.
 * Original condition: #ifndef __KERNEL__
 */
pub const RTMGRP_LINK: u32 = 1;
pub const RTMGRP_NOTIFY: u32 = 2;
pub const RTMGRP_NEIGH: u32 = 4;
pub const RTMGRP_TC: u32 = 8;
pub const RTMGRP_IPV4_IFADDR: u32 = 0x10;
pub const RTMGRP_IPV4_MROUTE: u32 = 0x20;
pub const RTMGRP_IPV4_ROUTE: u32 = 0x40;
pub const RTMGRP_IPV4_RULE: u32 = 0x80;
pub const RTMGRP_IPV6_IFADDR: u32 = 0x100;
pub const RTMGRP_IPV6_MROUTE: u32 = 0x200;
pub const RTMGRP_IPV6_ROUTE: u32 = 0x400;
pub const RTMGRP_IPV6_IFINFO: u32 = 0x800;
pub const RTMGRP_DECnet_IFADDR: u32 = 0x1000;
pub const RTMGRP_DECnet_ROUTE: u32 = 0x4000;
pub const RTMGRP_IPV6_PREFIX: u32 = 0x20000;

/* RTnetlink multicast groups */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rtnetlink_groups {
    RTNLGRP_NONE = 0,
    RTNLGRP_LINK = 1,
    RTNLGRP_NOTIFY = 2,
    RTNLGRP_NEIGH = 3,
    RTNLGRP_TC = 4,
    RTNLGRP_IPV4_IFADDR = 5,
    RTNLGRP_IPV4_MROUTE = 6,
    RTNLGRP_IPV4_ROUTE = 7,
    RTNLGRP_IPV4_RULE = 8,
    RTNLGRP_IPV6_IFADDR = 9,
    RTNLGRP_IPV6_MROUTE = 10,
    RTNLGRP_IPV6_ROUTE = 11,
    RTNLGRP_IPV6_IFINFO = 12,
    RTNLGRP_DECnet_IFADDR = 13,
    RTNLGRP_NOP2 = 14,
    RTNLGRP_DECnet_ROUTE = 15,
    RTNLGRP_DECnet_RULE = 16,
    RTNLGRP_NOP4 = 17,
    RTNLGRP_IPV6_PREFIX = 18,
    RTNLGRP_IPV6_RULE = 19,
    RTNLGRP_ND_USEROPT = 20,
    RTNLGRP_PHONET_IFADDR = 21,
    RTNLGRP_PHONET_ROUTE = 22,
    RTNLGRP_DCB = 23,
    RTNLGRP_IPV4_NETCONF = 24,
    RTNLGRP_IPV6_NETCONF = 25,
    RTNLGRP_MDB = 26,
    RTNLGRP_MPLS_ROUTE = 27,
    RTNLGRP_NSID = 28,
    RTNLGRP_MPLS_NETCONF = 29,
    RTNLGRP_IPV4_MROUTE_R = 30,
    RTNLGRP_IPV6_MROUTE_R = 31,
    RTNLGRP_NEXTHOP = 32,
    RTNLGRP_BRVLAN = 33,
    RTNLGRP_MCTP_IFADDR = 34,
    RTNLGRP_TUNNEL = 35,
    RTNLGRP_STATS = 36,
    RTNLGRP_IPV4_MCADDR = 37,
    RTNLGRP_IPV6_MCADDR = 38,
    RTNLGRP_IPV6_ACADDR = 39,
    __RTNLGRP_MAX = 40,
}
pub const RTNLGRP_MAX: u32 = 39;

/* TC action piece */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcamsg {
    pub tca_family: u8,
    pub tca__pad1: u8,
    pub tca__pad2: u16,
}

pub const TCA_ROOT_UNSPEC: u32 = 0;
pub const TCA_ROOT_TAB: u32 = 1;
pub const TCA_ACT_TAB: u32 = TCA_ROOT_TAB;
pub const TCAA_MAX: u32 = TCA_ROOT_TAB;
pub const TCA_ROOT_FLAGS: u32 = 2;
pub const TCA_ROOT_COUNT: u32 = 3;
pub const TCA_ROOT_TIME_DELTA: u32 = 4; /* in msecs */
pub const TCA_ROOT_EXT_WARN_MSG: u32 = 5;
pub const __TCA_ROOT_MAX: u32 = 6;
pub const TCA_ROOT_MAX: u32 = __TCA_ROOT_MAX - 1;

pub unsafe fn TA_RTA(r: *mut tcamsg) -> *mut rtattr {
    unsafe { (r as *mut u8).add(NLMSG_ALIGN(size_of::<tcamsg>())) as *mut rtattr }
}
pub unsafe fn TA_PAYLOAD(n: *const c_void) -> i32 {
    unsafe { NLMSG_PAYLOAD(n, size_of::<tcamsg>()) }
}

/* tcamsg flags stored in attribute TCA_ROOT_FLAGS
 *
 * TCA_ACT_FLAG_LARGE_DUMP_ON user->kernel to request for larger than
 * TCA_ACT_MAX_PRIO actions in a dump. All dump responses will contain the
 * number of actions being dumped stored in for user app's consumption in
 * TCA_ROOT_COUNT
 *
 * TCA_ACT_FLAG_TERSE_DUMP user->kernel to request terse (brief) dump that only
 * includes essential action info (kind, index, etc.)
 *
 */
pub const TCA_FLAG_LARGE_DUMP_ON: u32 = 1 << 0;
pub const TCA_ACT_FLAG_LARGE_DUMP_ON: u32 = TCA_FLAG_LARGE_DUMP_ON;
pub const TCA_ACT_FLAG_TERSE_DUMP: u32 = 1 << 1;

/* New extended info filters for IFLA_EXT_MASK */
pub const RTEXT_FILTER_VF: u32 = 1 << 0;
pub const RTEXT_FILTER_BRVLAN: u32 = 1 << 1;
pub const RTEXT_FILTER_BRVLAN_COMPRESSED: u32 = 1 << 2;
pub const RTEXT_FILTER_SKIP_STATS: u32 = 1 << 3;
pub const RTEXT_FILTER_MRP: u32 = 1 << 4;
pub const RTEXT_FILTER_CFM_CONFIG: u32 = 1 << 5;
pub const RTEXT_FILTER_CFM_STATUS: u32 = 1 << 6;
pub const RTEXT_FILTER_MST: u32 = 1 << 7;
pub const RTEXT_FILTER_NAME_ONLY: u32 = 1 << 8;

/* End of information exported to user level */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
