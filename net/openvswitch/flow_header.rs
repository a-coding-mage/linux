/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2007-2017 Nicira, Inc.
 */

/* Dependencies supplied by the surrounding kernel/Open vSwitch translation. */

use core::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sw_flow_mac_proto {
    MAC_PROTO_NONE = 0,
    MAC_PROTO_ETHERNET,
}

pub const SW_FLOW_KEY_INVALID: u8 = 0x80;
pub const MPLS_LABEL_DEPTH: usize = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ofp12_ipv6exthdr_flags {
    OFPIEH12_NONEXT = 1 << 0,
    OFPIEH12_ESP = 1 << 1,
    OFPIEH12_AUTH = 1 << 2,
    OFPIEH12_DEST = 1 << 3,
    OFPIEH12_FRAG = 1 << 4,
    OFPIEH12_ROUTER = 1 << 5,
    OFPIEH12_HOP = 1 << 6,
    OFPIEH12_UNREP = 1 << 7,
    OFPIEH12_UNSEQ = 1 << 8,
}

/* Store options at the end of the array if they are less than the maximum size. */
/* The C sizeof_field/offsetof expressions depend on the translated sw_flow_key layout. */
pub const fn tun_metadata_offset(opt_len: usize, tun_opts_size: usize) -> usize {
    tun_opts_size - opt_len
}

#[repr(C)]
pub struct ovs_tunnel_info {
    pub tun_dst: *mut metadata_dst,
}

#[repr(C)]
pub struct vlan_head {
    pub tpid: __be16,
    pub tci: __be16,
}

#[repr(C)]
pub struct ovs_key_nsh {
    pub base: ovs_nsh_key_base,
    pub context: [__be32; NSH_MD1_CONTEXT_SIZE],
}

#[repr(C, packed)]
pub struct sw_flow_key_phy {
    pub priority: u32,
    pub skb_mark: u32,
    pub in_port: u16,
}

#[repr(C)]
pub struct sw_flow_key_eth {
    pub src: [u8; ETH_ALEN],
    pub dst: [u8; ETH_ALEN],
    pub vlan: vlan_head,
    pub cvlan: vlan_head,
    pub type_: __be16,
}

#[repr(C)]
pub struct sw_flow_key_ip {
    pub proto: u8,
    pub tos: u8,
    pub ttl: u8,
    pub frag: u8,
}

#[repr(C)]
pub struct sw_flow_key_tp {
    pub src: __be16,
    pub dst: __be16,
    pub flags: __be16,
}

#[repr(C)]
pub struct sw_flow_key_ipv4_addr {
    pub src: __be32,
    pub dst: __be32,
}

#[repr(C)]
pub struct sw_flow_key_ipv4_ct_orig {
    pub src: __be32,
    pub dst: __be32,
}

#[repr(C)]
pub struct sw_flow_key_arp {
    pub sha: [u8; ETH_ALEN],
    pub tha: [u8; ETH_ALEN],
}

#[repr(C)]
pub union sw_flow_key_ipv4_extra {
    pub ct_orig: sw_flow_key_ipv4_ct_orig,
    pub arp: sw_flow_key_arp,
}

#[repr(C)]
pub struct sw_flow_key_ipv4 {
    pub addr: sw_flow_key_ipv4_addr,
    pub extra: sw_flow_key_ipv4_extra,
}

#[repr(C)]
pub struct sw_flow_key_ipv6_addr {
    pub src: in6_addr,
    pub dst: in6_addr,
}

#[repr(C)]
pub struct sw_flow_key_ipv6_ct_orig {
    pub src: in6_addr,
    pub dst: in6_addr,
}

#[repr(C)]
pub struct sw_flow_key_nd {
    pub target: in6_addr,
    pub sll: [u8; ETH_ALEN],
    pub tll: [u8; ETH_ALEN],
}

#[repr(C)]
pub union sw_flow_key_ipv6_extra {
    pub ct_orig: sw_flow_key_ipv6_ct_orig,
    pub nd: sw_flow_key_nd,
}

#[repr(C)]
pub struct sw_flow_key_ipv6 {
    pub addr: sw_flow_key_ipv6_addr,
    pub label: __be32,
    pub exthdrs: u16,
    pub extra: sw_flow_key_ipv6_extra,
}

#[repr(C)]
pub struct sw_flow_key_mpls {
    pub num_labels_mask: u32,
    pub lse: [__be32; MPLS_LABEL_DEPTH],
}

#[repr(C)]
pub union sw_flow_key_l3 {
    pub ipv4: sw_flow_key_ipv4,
    pub ipv6: sw_flow_key_ipv6,
    pub mpls: sw_flow_key_mpls,
    pub nsh: ovs_key_nsh,
}

#[repr(C)]
pub struct sw_flow_key_orig_tp {
    pub src: __be16,
    pub dst: __be16,
}

#[repr(C)]
pub struct sw_flow_key_ct {
    pub orig_tp: sw_flow_key_orig_tp,
    pub mark: u32,
    pub labels: ovs_key_ct_labels,
}

#[repr(C, align(8))]
pub struct sw_flow_key {
    pub tun_opts: [u8; IP_TUNNEL_OPTS_MAX],
    pub tun_opts_len: u8,
    pub tun_key: ip_tunnel_key,
    pub phy: sw_flow_key_phy,
    pub mac_proto: u8,
    pub tun_proto: u8,
    pub ovs_flow_hash: u32,
    pub recirc_id: u32,
    pub eth: sw_flow_key_eth,
    pub ct_state: u8,
    pub ct_orig_proto: u8,
    pub ip: sw_flow_key_ip,
    pub ct_zone: u16,
    pub tp: sw_flow_key_tp,
    pub l3: sw_flow_key_l3,
    pub ct: sw_flow_key_ct,
}

#[inline]
pub unsafe fn sw_flow_key_is_nd(key: *const sw_flow_key) -> bool {
    (*key).eth.type_ == htons(ETH_P_IPV6 as _) &&
        (*key).ip.proto == NEXTHDR_ICMP &&
        (*key).tp.dst == 0 &&
        ((*key).tp.src == htons(NDISC_NEIGHBOUR_SOLICITATION as _) ||
         (*key).tp.src == htons(NDISC_NEIGHBOUR_ADVERTISEMENT as _))
}

#[repr(C)]
pub struct sw_flow_key_range { pub start: u16, pub end: u16 }

#[repr(C)]
pub struct sw_flow_mask {
    pub ref_count: i32,
    pub rcu: rcu_head,
    pub range: sw_flow_key_range,
    pub key: sw_flow_key,
}

#[repr(C)]
pub struct sw_flow_match {
    pub key: *mut sw_flow_key,
    pub range: sw_flow_key_range,
    pub mask: *mut sw_flow_mask,
}

pub const MAX_UFID_LENGTH: usize = 16;

#[repr(C)]
pub union sw_flow_id_value {
    pub ufid: [u32; MAX_UFID_LENGTH / 4],
    pub unmasked_key: *mut sw_flow_key,
}

#[repr(C)]
pub struct sw_flow_id { pub ufid_len: u32, pub value: sw_flow_id_value }

#[repr(C)]
pub struct sw_flow_actions {
    pub rcu: rcu_head,
    pub orig_len: usize,
    pub actions_len: u32,
    pub actions: [nlattr; 0],
}

#[repr(C)]
pub struct sw_flow_stats {
    pub packet_count: u64,
    pub byte_count: u64,
    pub used: c_ulong,
    pub lock: spinlock_t,
    pub tcp_flags: __be16,
}

#[repr(C)]
pub struct sw_flow_flow_table { pub node: [hlist_node; 2], pub hash: u32 }

#[repr(C)]
pub struct sw_flow {
    pub rcu: rcu_head,
    pub flow_table: sw_flow_flow_table,
    pub ufid_table: sw_flow_flow_table,
    pub stats_last_writer: i32,
    pub key: sw_flow_key,
    pub id: sw_flow_id,
    pub cpu_used_mask: *mut cpumask,
    pub mask: *mut sw_flow_mask,
    pub sf_acts: *mut sw_flow_actions,
    pub stats: [*mut sw_flow_stats; 0],
}

#[repr(C, packed)]
pub struct arp_eth_header {
    pub ar_hrd: __be16, pub ar_pro: __be16, pub ar_hln: u8, pub ar_pln: u8, pub ar_op: __be16,
    pub ar_sha: [u8; ETH_ALEN], pub ar_sip: [u8; 4],
    pub ar_tha: [u8; ETH_ALEN], pub ar_tip: [u8; 4],
}

#[inline]
pub unsafe fn ovs_key_mac_proto(key: *const sw_flow_key) -> u8 { (*key).mac_proto & !SW_FLOW_KEY_INVALID }

#[inline]
pub fn __ovs_mac_header_len(mac_proto: u8) -> u16 {
    if mac_proto == MAC_PROTO_ETHERNET as u8 { ETH_HLEN as u16 } else { 0 }
}

#[inline]
pub unsafe fn ovs_mac_header_len(key: *const sw_flow_key) -> u16 { __ovs_mac_header_len(ovs_key_mac_proto(key)) }

#[inline]
pub unsafe fn ovs_identifier_is_ufid(sfid: *const sw_flow_id) -> bool { (*sfid).ufid_len != 0 }

#[inline]
pub unsafe fn ovs_identifier_is_key(sfid: *const sw_flow_id) -> bool { !ovs_identifier_is_ufid(sfid) }

extern "C" {
    pub fn ovs_flow_stats_update(flow: *mut sw_flow, tcp_flags: __be16, skb: *const sk_buff);
    pub fn ovs_flow_stats_get(flow: *const sw_flow, stats: *mut ovs_flow_stats, used: *mut c_ulong, tcp_flags: *mut __be16);
    pub fn ovs_flow_stats_clear(flow: *mut sw_flow);
    pub fn ovs_flow_used_time(flow_jiffies: c_ulong) -> u64;
    pub fn ovs_flow_key_update(skb: *mut sk_buff, key: *mut sw_flow_key) -> i32;
    pub fn ovs_flow_key_update_l3l4(skb: *mut sk_buff, key: *mut sw_flow_key) -> i32;
    pub fn ovs_flow_key_extract(tun_info: *const ip_tunnel_info, skb: *mut sk_buff, key: *mut sw_flow_key) -> i32;
    pub fn ovs_flow_key_extract_userspace(net: *mut net, attr: *const nlattr, skb: *mut sk_buff, key: *mut sw_flow_key, log: bool) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
