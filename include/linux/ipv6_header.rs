/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

#[inline]
pub unsafe fn ipv6_optlen(p: *const ipv6_opt_hdr) -> u32 { ((*p).hdrlen as u32 + 1) << 3 }
#[inline]
pub unsafe fn ipv6_authlen(p: *const ipv6_opt_hdr) -> u32 { ((*p).hdrlen as u32 + 2) << 2 }

/* This structure contains configuration options per IPv6 link. */
#[repr(C)]
pub struct ipv6_devconf {
    pub disable_ipv6: i32, pub hop_limit: i32, pub mtu6: i32, pub forwarding: i32,
    pub force_forwarding: i32, pub disable_policy: i32, pub proxy_ndp: i32,
    pub accept_ra: i32, pub accept_redirects: i32, pub autoconf: i32,
    pub dad_transmits: i32, pub rtr_solicits: i32, pub rtr_solicit_interval: i32,
    pub rtr_solicit_max_interval: i32, pub rtr_solicit_delay: i32, pub force_mld_version: i32,
    pub mldv1_unsolicited_report_interval: i32, pub mldv2_unsolicited_report_interval: i32,
    pub use_tempaddr: i32, pub temp_valid_lft: i32, pub temp_prefered_lft: i32,
    pub regen_min_advance: i32, pub regen_max_retry: i32, pub max_desync_factor: i32,
    pub max_addresses: i32, pub accept_ra_defrtr: i32, pub ra_defrtr_metric: u32,
    pub accept_ra_min_hop_limit: i32, pub accept_ra_min_lft: i32, pub accept_ra_pinfo: i32,
    pub ignore_routes_with_linkdown: i32,
    /* CONFIG_IPV6_ROUTER_PREF / CONFIG_IPV6_ROUTE_INFO fields are conditional. */
    pub accept_source_route: i32, pub accept_ra_from_local: i32,
    /* CONFIG_IPV6_OPTIMISTIC_DAD fields are conditional. */
    pub optimistic_dad: i32, pub use_optimistic: i32,
    /* CONFIG_IPV6_MROUTE: atomic_t mc_forwarding. */
    pub drop_unicast_in_l2_multicast: i32, pub accept_dad: i32, pub force_tllao: i32,
    pub ndisc_notify: i32, pub suppress_frag_ndisc: i32, pub accept_ra_mtu: i32,
    pub drop_unsolicited_na: i32, pub accept_untracked_na: i32,
    pub stable_secret: ipv6_stable_secret, pub use_oif_addrs_only: i32,
    pub keep_addr_on_down: i32, pub seg6_enabled: i32,
    /* CONFIG_IPV6_SEG6_HMAC: seg6_require_hmac. */
    pub seg6_require_hmac: i32, pub enhanced_dad: u32, pub addr_gen_mode: u32,
    pub ndisc_tclass: i32, pub rpl_seg_enabled: i32, pub ioam6_id: u32, pub ioam6_id_wide: u32,
    pub ioam6_enabled: u8, pub ndisc_evict_nocarrier: u8, pub ra_honor_pio_life: u8,
    pub ra_honor_pio_pflag: u8, pub sysctl_header: *mut ctl_table_header,
}
#[repr(C)] pub struct ipv6_stable_secret { pub initialized: bool, pub secret: in6_addr }
#[repr(C)] pub struct ipv6_params { pub disable_ipv6: i32, pub autoconf: i32 }
pub static mut ipv6_defaults: ipv6_params = ipv6_params { disable_ipv6: 0, autoconf: 0 };

#[inline] pub unsafe fn ipv6_hdr(skb: *const sk_buff) -> *mut ipv6hdr { skb_network_header(skb) as *mut ipv6hdr }
#[inline] pub unsafe fn inner_ipv6_hdr(skb: *const sk_buff) -> *mut ipv6hdr { skb_inner_network_header(skb) as *mut ipv6hdr }
#[inline] pub unsafe fn ipipv6_hdr(skb: *const sk_buff) -> *mut ipv6hdr { skb_transport_header(skb) as *mut ipv6hdr }
#[inline] pub unsafe fn ipv6_transport_len(skb: *const sk_buff) -> u32 { u16::from_be((*ipv6_hdr(skb)).payload_len) as u32 + core::mem::size_of::<ipv6hdr>() as u32 - skb_network_header_len(skb) }
#[inline] pub unsafe fn ipv6_payload_len(skb: *const sk_buff, ip6: *const ipv6hdr) -> u32 {
    let len = u16::from_be((*ip6).payload_len) as u32;
    if len != 0 || !skb_is_gso(skb) || !skb_is_gso_tcp(skb) { len } else { (*skb).len - skb_network_offset(skb) - core::mem::size_of::<ipv6hdr>() as u32 }
}
#[inline] pub unsafe fn skb_ipv6_payload_len(skb: *const sk_buff) -> u32 { ipv6_payload_len(skb, ipv6_hdr(skb)) }
pub const IPV6_MAXPLEN: u32 = 65535;
#[inline] pub unsafe fn ipv6_set_payload_len(ip6: *mut ipv6hdr, len: u32) { (*ip6).payload_len = if len <= IPV6_MAXPLEN { (len as u16).to_be() } else { 0 }; }

#[repr(C)] pub struct inet6_skb_parm {
    pub iif: i32, pub ra: __be16, pub dst0: u16, pub srcrt: u16, pub dst1: u16,
    pub lastopt: u16, pub nhoff: u16, pub flags: u16, pub dsthao: u16,
    pub frag_max_size: u16, pub srhoff: u16,
}
pub const IP6SKB_XFRM_TRANSFORMED: u16 = 1; pub const IP6SKB_FORWARDED: u16 = 2;
pub const IP6SKB_REROUTED: u16 = 4; pub const IP6SKB_ROUTERALERT: u16 = 8;
pub const IP6SKB_FRAGMENTED: u16 = 16; pub const IP6SKB_HOPBYHOP: u16 = 32;
pub const IP6SKB_L3SLAVE: u16 = 64; pub const IP6SKB_JUMBOGRAM: u16 = 128;
pub const IP6SKB_SEG6: u16 = 256; pub const IP6SKB_MULTIPATH: u16 = 1024; pub const IP6SKB_MCROUTE: u16 = 2048;

#[inline] pub fn ipv6_l3mdev_skb(flags: u16) -> bool { (flags & IP6SKB_L3SLAVE) != 0 }
#[inline] pub unsafe fn inet6_iif(skb: *const sk_buff) -> i32 { if ipv6_l3mdev_skb((*IP6CB(skb)).flags) { (*skb).skb_iif } else { (*IP6CB(skb)).iif } }
#[inline] pub unsafe fn inet6_is_jumbogram(skb: *const sk_buff) -> bool { ((*IP6CB(skb)).flags & IP6SKB_JUMBOGRAM) != 0 }
#[inline] pub unsafe fn inet6_sdif(skb: *const sk_buff) -> i32 { if !skb.is_null() && ipv6_l3mdev_skb((*IP6CB(skb)).flags) { (*IP6CB(skb)).iif } else { 0 } }

#[repr(C)] pub struct tcp6_request_sock { pub tcp6rsk_tcp: tcp_request_sock }
pub struct ipv6_mc_socklist; pub struct ipv6_ac_socklist; pub struct ipv6_fl_socklist;
#[repr(C)] pub struct ipv6_pinfo {
    pub saddr: in6_addr, pub daddr: in6_addr, pub flow_label: __be32, pub dst_cookie: u32,
    pub opt: *mut ipv6_txoptions, pub hop_limit: i16, pub pmtudisc: u8, pub tclass: u8,
    pub saddr_cache: bool, pub daddr_cache: bool, pub mcast_hops: u8, pub frag_size: u32,
    pub ucast_oif: i32, pub mcast_oif: i32, pub rxopt: ipv6_rxopt,
    pub srcprefs: u8, pub min_hopcount: u8, pub rcv_flowinfo: __be32, pub sticky_pktinfo: in6_pktinfo,
    pub pktoptions: *mut sk_buff, pub rxpmtu: *mut sk_buff,
    pub ipv6_mc_list: *mut ipv6_mc_socklist, pub ipv6_ac_list: *mut ipv6_ac_socklist,
}
#[repr(C)] pub union ipv6_rxopt { pub bits: u16, pub all: u16 }
#[repr(C)] pub struct raw6_sock { pub inet: inet_sock, pub checksum: u32, pub offset: u32, pub filter: icmp6_filter, pub ip6mr_table: u32, pub drop_counters: numa_drop_counters, pub inet6: ipv6_pinfo }
#[repr(C)] pub struct udp6_sock { pub udp: udp_sock, pub inet6: ipv6_pinfo }
#[repr(C)] pub struct tcp6_sock { pub tcp: tcp_sock, pub inet6: ipv6_pinfo }
extern "C" { pub fn inet6_sk_rebuild_header(sk: *mut sock) -> i32; }
#[repr(C)] pub struct tcp6_timewait_sock { pub tcp6tw_tcp: tcp_timewait_sock }

#[inline] pub fn ipv6_mod_enabled(disable_ipv6_mod: i32) -> bool { disable_ipv6_mod == 0 }
#[inline] pub unsafe fn inet6_sk(sk: *const sock) -> *mut ipv6_pinfo { if sk_fullsock(sk) { inet_sk(sk).pinet6 } else { core::ptr::null_mut() } }
#[inline] pub unsafe fn inet6_rcv_saddr(sk: *const sock) -> *const in6_addr { if (*sk).sk_family == AF_INET6 { &(*sk).sk_v6_rcv_saddr } else { core::ptr::null() } }
#[inline] pub unsafe fn inet_v6_ipv6only(sk: *const sock) -> i32 { (*sk).sk_ipv6only }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
