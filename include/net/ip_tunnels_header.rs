/* SPDX-License-Identifier: GPL-2.0 */
#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* C header dependencies are supplied by the surrounding kernel translation. */
extern "C" {
    pub static mut iptun_encaps: [*const ip_tunnel_encap_ops; MAX_IPTUN_ENCAP_OPS];
}

pub const IP_TUNNEL_RECURSION_LIMIT: i32 = 5;
pub const IPTUNNEL_ERR_TIMEO: i32 = 30 * HZ;
pub const IP_TUNNEL_INFO_TX: u8 = 0x01;
pub const IP_TUNNEL_INFO_IPV6: u8 = 0x02;
pub const IP_TUNNEL_INFO_BRIDGE: u8 = 0x04;
pub const PACKET_RCVD: i32 = 0;
pub const PACKET_REJECT: i32 = 1;
pub const PACKET_NEXT: i32 = 2;
pub const IP_TNL_HASH_BITS: usize = 7;
pub const IP_TNL_HASH_SIZE: usize = 1 << IP_TNL_HASH_BITS;
pub const MAX_IPTUN_ENCAP_OPS: usize = 8;

#[repr(C)]
pub struct ip_tunnel_key {
    pub tun_id: __be64,
    pub u: ip_tunnel_key_u,
    pub tun_flags: [c_ulong; __IP_TUNNEL_FLAG_NUM as usize],
    pub label: __be32,
    pub nhid: u32,
    pub tos: u8,
    pub ttl: u8,
    pub tp_src: __be16,
    pub tp_dst: __be16,
    pub flow_flags: u8,
}
#[repr(C)] pub union ip_tunnel_key_u { pub ipv4: ip_tunnel_key_ipv4, pub ipv6: ip_tunnel_key_ipv6 }
#[repr(C)] pub struct ip_tunnel_key_ipv4 { pub src: __be32, pub dst: __be32 }
#[repr(C)] pub struct ip_tunnel_key_ipv6 { pub src: in6_addr, pub dst: in6_addr }

#[repr(C)] pub struct ip_tunnel_encap { pub type_: u16, pub flags: u16, pub sport: __be16, pub dport: __be16 }

#[repr(C)] pub struct ip_tunnel_info {
    pub key: ip_tunnel_key,
    pub encap: ip_tunnel_encap,
    /* CONFIG_DST_CACHE conditional field */
    pub dst_cache: dst_cache,
    pub options_len: u8,
    pub mode: u8,
    pub options: [u8; 0],
}

#[repr(C)] pub struct ip_tunnel_6rd_parm { pub prefix: in6_addr, pub relay_prefix: __be32, pub prefixlen: u16, pub relay_prefixlen: u16 }
#[repr(C)] pub struct ip_tunnel_prl_entry { pub next: *mut ip_tunnel_prl_entry, pub addr: __be32, pub flags: u16, pub rcu_head: rcu_head }
#[repr(C)] pub struct ip_tunnel_parm_kern { pub name: [c_char; IFNAMSIZ], pub i_flags: [c_ulong; __IP_TUNNEL_FLAG_NUM as usize], pub o_flags: [c_ulong; __IP_TUNNEL_FLAG_NUM as usize], pub i_key: __be32, pub o_key: __be32, pub link: c_int, pub iph: iphdr }
#[repr(C)] pub struct ip_tunnel {
    pub next: *mut ip_tunnel, pub hash_node: hlist_node, pub dev: *mut net_device, pub dev_tracker: netdevice_tracker, pub net: *mut net,
    pub err_time: c_ulong, pub err_count: c_int, pub i_seqno: u32, pub o_seqno: atomic_t, pub tun_hlen: c_int,
    pub index: u32, pub erspan_ver: u8, pub dir: u8, pub hwid: u16, pub dst_cache: dst_cache, pub parms: ip_tunnel_parm_kern,
    pub mlink: c_int, pub encap_hlen: c_int, pub hlen: c_int, pub encap: ip_tunnel_encap, pub ip6rd: ip_tunnel_6rd_parm,
    pub prl: *mut ip_tunnel_prl_entry, pub prl_count: c_uint, pub ip_tnl_net_id: c_uint, pub gro_cells: gro_cells,
    pub fwmark: u32, pub collect_md: bool, pub ignore_df: bool,
}
#[repr(C)] pub struct tnl_ptk_info { pub flags: [c_ulong; __IP_TUNNEL_FLAG_NUM as usize], pub proto: __be16, pub key: __be32, pub seq: __be32, pub hdr_len: c_int }
#[repr(C)] pub struct ip_tunnel_net { pub fb_tunnel_dev: *mut net_device, pub rtnl_link_ops: *mut rtnl_link_ops, pub tunnels: [hlist_head; IP_TNL_HASH_SIZE], pub collect_md_tun: *mut ip_tunnel, pub type_: c_int }

#[repr(C)] pub struct ip_tunnel_encap_ops {
    pub encap_hlen: Option<unsafe extern "C" fn(*mut ip_tunnel_encap) -> usize>,
    pub build_header: Option<unsafe extern "C" fn(*mut sk_buff, *mut ip_tunnel_encap, *mut u8, *mut flowi4) -> c_int>,
    pub err_handler: Option<unsafe extern "C" fn(*mut sk_buff, u32) -> c_int>,
}

extern "C" {
    pub fn __ip_tunnel_init(dev: *mut net_device) -> c_int;
    pub fn ip_tunnel_uninit(dev: *mut net_device);
    pub fn ip_tunnel_dellink(dev: *mut net_device, head: *mut list_head);
    pub fn ip_tunnel_get_link_net(dev: *const net_device) -> *mut net;
    pub fn ip_tunnel_get_iflink(dev: *const net_device) -> c_int;
    pub fn ip_tunnel_init_net(net: *mut net, id: c_uint, ops: *mut rtnl_link_ops, devname: *mut c_char) -> c_int;
    pub fn ip_tunnel_delete_net(net: *mut net, id: c_uint, ops: *mut rtnl_link_ops, dev_to_kill: *mut list_head);
    pub fn ip_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device, params: *const iphdr, protocol: u8);
    pub fn ip_md_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device, proto: u8, tunnel_hlen: c_int);
    pub fn ip_tunnel_ctl(dev: *mut net_device, p: *mut ip_tunnel_parm_kern, cmd: c_int) -> c_int;
    pub fn ip_tunnel_lookup(itn: *mut ip_tunnel_net, link: c_int, flags: *const c_ulong, remote: __be32, local: __be32, key: __be32) -> *mut ip_tunnel;
    pub fn ip_tunnel_setup(dev: *mut net_device, net_id: c_uint);
    pub fn ip_tunnel_encap_add_ops(op: *const ip_tunnel_encap_ops, num: c_uint) -> c_int;
    pub fn ip_tunnel_encap_del_ops(op: *const ip_tunnel_encap_ops, num: c_uint) -> c_int;
    pub fn ip_tunnel_encap_setup(t: *mut ip_tunnel, e: *mut ip_tunnel_encap) -> c_int;
    pub fn ip_tunnel_core_init();
    pub fn ip_tunnel_need_metadata();
    pub fn ip_tunnel_unneed_metadata();
    pub fn ip_tunnel_changelink(dev: *mut net_device, tb: *mut *mut nlattr, p: *mut ip_tunnel_parm_kern, fwmark: u32) -> c_int;
    pub fn ip_tunnel_newlink(net: *mut net, dev: *mut net_device, tb: *mut *mut nlattr, p: *mut ip_tunnel_parm_kern, fwmark: u32) -> c_int;
    pub fn ip_tunnel_netlink_encap_parms(data: *mut *mut nlattr, encap: *mut ip_tunnel_encap) -> bool;
    pub fn ip_tunnel_netlink_parms(data: *mut *mut nlattr, parms: *mut ip_tunnel_parm_kern);
    pub static ip_tunnel_header_ops: header_ops;
    pub fn ip_tunnel_parse_protocol(skb: *const sk_buff) -> __be16;
    pub fn __iptunnel_pull_header(skb: *mut sk_buff, hdr_len: c_int, inner_proto: __be16, raw_proto: bool, xnet: bool) -> c_int;
    pub fn iptunnel_xmit(sk: *mut sock, rt: *mut rtable, skb: *mut sk_buff, src: __be32, dst: __be32, proto: u8, tos: u8, ttl: u8, df: __be16, xnet: bool, ipcb_flags: u16);
    pub fn iptunnel_metadata_reply(md: *mut metadata_dst, flags: gfp_t) -> *mut metadata_dst;
    pub fn skb_tunnel_check_pmtu(skb: *mut sk_buff, encap_dst: *mut dst_entry, headroom: c_int, reply: bool) -> c_int;
    pub fn iptunnel_handle_offloads(skb: *mut sk_buff, gso_type_mask: c_int) -> c_int;
}

/* C inline operations represented as callable low-level entry points. */
extern "C" {
    pub fn pskb_inet_may_pull_reason(skb: *mut sk_buff) -> skb_drop_reason;
    pub fn pskb_inet_may_pull(skb: *mut sk_buff) -> bool;
    pub fn skb_vlan_inet_prepare(skb: *mut sk_buff, inner_proto_inherit: bool) -> skb_drop_reason;
    pub fn ip_encap_hlen(e: *mut ip_tunnel_encap) -> c_int;
    pub fn ip_tunnel_encap(skb: *mut sk_buff, e: *mut ip_tunnel_encap, protocol: *mut u8, fl4: *mut flowi4) -> c_int;
    pub fn ip_tunnel_get_dsfield(iph: *const iphdr, skb: *const sk_buff) -> u8;
    pub fn ip_tunnel_get_flowlabel(iph: *const iphdr, skb: *const sk_buff) -> __be32;
    pub fn ip_tunnel_get_ttl(iph: *const iphdr, skb: *const sk_buff) -> u8;
    pub fn ip_tunnel_ecn_encap(tos: u8, iph: *const iphdr, skb: *const sk_buff) -> u8;
    pub fn iptunnel_pull_header(skb: *mut sk_buff, hdr_len: c_int, inner_proto: __be16, xnet: bool) -> c_int;
    pub fn ip_tunnel_limit_headroom(headroom: c_uint) -> c_uint;
    pub fn ip_tunnel_adj_headroom(dev: *mut net_device, headroom: c_uint);
    pub fn iptunnel_pull_offloads(skb: *mut sk_buff) -> c_int;
    pub fn iptunnel_xmit_stats(dev: *mut net_device, pkt_len: c_int);
    pub fn ip_tunnel_info_opts_get(to: *mut c_void, info: *const ip_tunnel_info);
    pub fn ip_tunnel_info_opts_set(info: *mut ip_tunnel_info, from: *const c_void, len: c_int, flags: *const c_ulong);
    pub fn lwt_tun_info(lwtstate: *mut lwtunnel_state) -> *mut ip_tunnel_info;
    pub fn ip_tunnel_collect_metadata() -> c_int;
}

/* The remaining inline functions and declarations retain C kernel semantics. */
extern "C" {
    pub fn ip_tunnel_info_af(info: *const ip_tunnel_info) -> c_ushort;
    pub fn ip_tunnel_dst_cache_usable(skb: *const sk_buff, info: *const ip_tunnel_info) -> bool;
    pub fn ip_tunnel_key_init(key: *mut ip_tunnel_key, saddr: __be32, daddr: __be32, tos: u8, ttl: u8, label: __be32, tp_src: __be16, tp_dst: __be16, tun_id: __be64, flags: *const c_ulong);
}

/* CONFIG_INET and CONFIG_IPV6 branches, bitmap helpers, packet helpers, and
 * statistics helpers are declarations supplied by the kernel environment. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
