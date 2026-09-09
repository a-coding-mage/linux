/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the surrounding UAPI translation.

/* The latest drafts declared increase in minimal mtu up to 1280. */
pub const IPV6_MIN_MTU: u32 = 1280;

/*
 * Advanced API
 * source interface/address selection, source routing, etc...
 * *under construction*
 */

#[cfg(__UAPI_DEF_IN6_PKTINFO)]
#[repr(C)]
pub struct in6_pktinfo {
    pub ipi6_addr: in6_addr,
    pub ipi6_ifindex: i32,
}

#[cfg(__UAPI_DEF_IP6_MTUINFO)]
#[repr(C)]
pub struct ip6_mtuinfo {
    pub ip6m_addr: sockaddr_in6,
    pub ip6m_mtu: __u32,
}

#[repr(C)]
pub struct in6_ifreq {
    pub ifr6_addr: in6_addr,
    pub ifr6_prefixlen: __u32,
    pub ifr6_ifindex: i32,
}

pub const IPV6_SRCRT_STRICT: u32 = 0x01; /* Deprecated; will be removed */
pub const IPV6_SRCRT_TYPE_0: u32 = 0; /* Deprecated; will be removed */
pub const IPV6_SRCRT_TYPE_2: u32 = 2; /* IPv6 type 2 Routing Header */
pub const IPV6_SRCRT_TYPE_3: u32 = 3; /* RPL Segment Routing with IPv6 */
pub const IPV6_SRCRT_TYPE_4: u32 = 4; /* Segment Routing with IPv6 */

/* routing header */
#[repr(C)]
pub struct ipv6_rt_hdr {
    pub nexthdr: __u8,
    pub hdrlen: __u8,
    pub type_: __u8,
    pub segments_left: __u8,
    /* type specific data; variable length field */
}

#[repr(C, packed)]
pub struct ipv6_opt_hdr {
    pub nexthdr: __u8,
    pub hdrlen: __u8,
    /* TLV encoded option data follows. */
}

pub type ipv6_destopt_hdr = ipv6_opt_hdr;
pub type ipv6_hopopt_hdr = ipv6_opt_hdr;

/* Router Alert option values (RFC2711) */
pub const IPV6_OPT_ROUTERALERT_MLD: u32 = 0x0000; /* MLD(RFC2710) */

/* routing header type 0 (used in cmsghdr struct) */
#[repr(C)]
pub struct rt0_hdr {
    pub rt_hdr: ipv6_rt_hdr,
    pub reserved: __u32,
    pub addr: [in6_addr; 0],
}

impl rt0_hdr {
    #[inline]
    pub fn rt0_type(&self) -> __u8 {
        self.rt_hdr.type_
    }
}

/* routing header type 2 */
#[repr(C)]
pub struct rt2_hdr {
    pub rt_hdr: ipv6_rt_hdr,
    pub reserved: __u32,
    pub addr: in6_addr,
}

impl rt2_hdr {
    #[inline]
    pub fn rt2_type(&self) -> __u8 {
        self.rt_hdr.type_
    }
}

/* home address option in destination options header */
#[repr(C, packed)]
pub struct ipv6_destopt_hao {
    pub type_: __u8,
    pub length: __u8,
    pub addr: in6_addr,
}

/*
 * IPv6 fixed header
 *
 * BEWARE, it is incorrect. The first 4 bits of flow_lbl
 * are glued to priority now, forming "class".
 */
#[repr(C)]
pub struct ipv6hdr {
    // C bitfields: priority:4 and version:4. Their bit order follows the
    // target's __LITTLE_ENDIAN_BITFIELD or __BIG_ENDIAN_BITFIELD setting.
    pub priority_version: __u8,
    pub flow_lbl: [__u8; 3],
    pub payload_len: __be16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

/* index values for the variables in ipv6_devconf */
#[repr(i32)]
pub enum ipv6_devconf_index {
    DEVCONF_FORWARDING = 0,
    DEVCONF_HOPLIMIT,
    DEVCONF_MTU6,
    DEVCONF_ACCEPT_RA,
    DEVCONF_ACCEPT_REDIRECTS,
    DEVCONF_AUTOCONF,
    DEVCONF_DAD_TRANSMITS,
    DEVCONF_RTR_SOLICITS,
    DEVCONF_RTR_SOLICIT_INTERVAL,
    DEVCONF_RTR_SOLICIT_DELAY,
    DEVCONF_USE_TEMPADDR,
    DEVCONF_TEMP_VALID_LFT,
    DEVCONF_TEMP_PREFERED_LFT,
    DEVCONF_REGEN_MAX_RETRY,
    DEVCONF_MAX_DESYNC_FACTOR,
    DEVCONF_MAX_ADDRESSES,
    DEVCONF_FORCE_MLD_VERSION,
    DEVCONF_ACCEPT_RA_DEFRTR,
    DEVCONF_ACCEPT_RA_PINFO,
    DEVCONF_ACCEPT_RA_RTR_PREF,
    DEVCONF_RTR_PROBE_INTERVAL,
    DEVCONF_ACCEPT_RA_RT_INFO_MAX_PLEN,
    DEVCONF_PROXY_NDP,
    DEVCONF_OPTIMISTIC_DAD,
    DEVCONF_ACCEPT_SOURCE_ROUTE,
    DEVCONF_MC_FORWARDING,
    DEVCONF_DISABLE_IPV6,
    DEVCONF_ACCEPT_DAD,
    DEVCONF_FORCE_TLLAO,
    DEVCONF_NDISC_NOTIFY,
    DEVCONF_MLDV1_UNSOLICITED_REPORT_INTERVAL,
    DEVCONF_MLDV2_UNSOLICITED_REPORT_INTERVAL,
    DEVCONF_SUPPRESS_FRAG_NDISC,
    DEVCONF_ACCEPT_RA_FROM_LOCAL,
    DEVCONF_USE_OPTIMISTIC,
    DEVCONF_ACCEPT_RA_MTU,
    DEVCONF_STABLE_SECRET,
    DEVCONF_USE_OIF_ADDRS_ONLY,
    DEVCONF_ACCEPT_RA_MIN_HOP_LIMIT,
    DEVCONF_IGNORE_ROUTES_WITH_LINKDOWN,
    DEVCONF_DROP_UNICAST_IN_L2_MULTICAST,
    DEVCONF_DROP_UNSOLICITED_NA,
    DEVCONF_KEEP_ADDR_ON_DOWN,
    DEVCONF_RTR_SOLICIT_MAX_INTERVAL,
    DEVCONF_SEG6_ENABLED,
    DEVCONF_SEG6_REQUIRE_HMAC,
    DEVCONF_ENHANCED_DAD,
    DEVCONF_ADDR_GEN_MODE,
    DEVCONF_DISABLE_POLICY,
    DEVCONF_ACCEPT_RA_RT_INFO_MIN_PLEN,
    DEVCONF_NDISC_TCLASS,
    DEVCONF_RPL_SEG_ENABLED,
    DEVCONF_RA_DEFRTR_METRIC,
    DEVCONF_IOAM6_ENABLED,
    DEVCONF_IOAM6_ID,
    DEVCONF_IOAM6_ID_WIDE,
    DEVCONF_NDISC_EVICT_NOCARRIER,
    DEVCONF_ACCEPT_UNTRACKED_NA,
    DEVCONF_ACCEPT_RA_MIN_LFT,
    DEVCONF_FORCE_FORWARDING,
    DEVCONF_MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
