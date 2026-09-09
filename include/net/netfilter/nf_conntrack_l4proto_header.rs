/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header for use in defining a given L4 protocol for connection tracking.
 *
 * 16 Dec 2003: Yasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
 *	- generalized L3 protocol dependent part.
 *
 * Derived from include/linux/netfiter_ipv4/ip_conntrack_protcol.h
 */

/* C dependencies: linux/netlink.h, net/netlink.h,
 * net/netfilter/nf_conntrack.h, and net/netns/generic.h. */

extern "C" {

pub struct seq_file;

#[repr(C)]
pub struct ctnl_timeout {
	pub nlattr_to_obj: Option<unsafe extern "C" fn(tb: *mut *mut nlattr, net: *mut net, data: *mut libc::c_void) -> i32>,
	pub obj_to_nlattr: Option<unsafe extern "C" fn(skb: *mut sk_buff, data: *const libc::c_void) -> i32>,
	pub obj_size: u16,
	pub nlattr_max: u16,
	pub nla_policy: *const nla_policy,
}

#[repr(C)]
pub struct nf_conntrack_l4proto {
	/* L4 Protocol number. */
	pub l4proto: u8,
	/* Resolve clashes on insertion races. */
	pub allow_clash: bool,
	/* protoinfo nlattr size, closes a hole */
	pub nlattr_size: u16,
	/* called by gc worker if table is full */
	pub can_early_drop: Option<unsafe extern "C" fn(ct: *const nf_conn) -> bool>,
	/* convert protoinfo to nfnetlink attributes */
	pub to_nlattr: Option<unsafe extern "C" fn(skb: *mut sk_buff, nla: *mut nlattr, ct: *mut nf_conn, destroy: bool) -> i32>,
	/* convert nfnetlink attributes to protoinfo */
	pub from_nlattr: Option<unsafe extern "C" fn(tb: *mut *mut nlattr, ct: *mut nf_conn) -> i32>,
	pub tuple_to_nlattr: Option<unsafe extern "C" fn(skb: *mut sk_buff, t: *const nf_conntrack_tuple) -> i32>,
	/* Calculate tuple nlattr size */
	pub nlattr_tuple_size: Option<unsafe extern "C" fn() -> libc::c_uint>,
	pub nlattr_to_tuple: Option<unsafe extern "C" fn(tb: *mut *mut nlattr, t: *mut nf_conntrack_tuple, flags: u32) -> i32>,
	pub nla_policy: *const nla_policy,

	pub ctnl_timeout: ctnl_timeout,
	/* CONFIG_NF_CONNTRACK_PROCFS */
	pub print_conntrack: Option<unsafe extern "C" fn(s: *mut seq_file, ct: *mut nf_conn)>,
}

pub fn icmp_pkt_to_tuple(skb: *const sk_buff, dataoff: libc::c_uint, net: *mut net, tuple: *mut nf_conntrack_tuple) -> bool;
pub fn icmpv6_pkt_to_tuple(skb: *const sk_buff, dataoff: libc::c_uint, net: *mut net, tuple: *mut nf_conntrack_tuple) -> bool;
pub fn nf_conntrack_invert_icmp_tuple(tuple: *mut nf_conntrack_tuple, orig: *const nf_conntrack_tuple) -> bool;
pub fn nf_conntrack_invert_icmpv6_tuple(tuple: *mut nf_conntrack_tuple, orig: *const nf_conntrack_tuple) -> bool;
pub fn nf_conntrack_inet_error(tmpl: *mut nf_conn, skb: *mut sk_buff, dataoff: libc::c_uint, state: *const nf_hook_state, l4proto: u8, outer_daddr: *mut nf_inet_addr) -> i32;
pub fn nf_conntrack_icmpv4_error(tmpl: *mut nf_conn, skb: *mut sk_buff, dataoff: libc::c_uint, state: *const nf_hook_state) -> i32;
pub fn nf_conntrack_icmpv6_error(tmpl: *mut nf_conn, skb: *mut sk_buff, dataoff: libc::c_uint, state: *const nf_hook_state) -> i32;
pub fn nf_conntrack_icmp_packet(ct: *mut nf_conn, skb: *mut sk_buff, ctinfo: ip_conntrack_info, state: *const nf_hook_state) -> i32;
pub fn nf_conntrack_icmpv6_packet(ct: *mut nf_conn, skb: *mut sk_buff, ctinfo: ip_conntrack_info, state: *const nf_hook_state) -> i32;
pub fn nf_conntrack_udp_packet(ct: *mut nf_conn, skb: *mut sk_buff, dataoff: libc::c_uint, ctinfo: ip_conntrack_info, state: *const nf_hook_state) -> i32;
pub fn nf_conntrack_tcp_packet(ct: *mut nf_conn, skb: *mut sk_buff, dataoff: libc::c_uint, ctinfo: ip_conntrack_info, state: *const nf_hook_state) -> i32;
pub fn nf_conntrack_sctp_packet(ct: *mut nf_conn, skb: *mut sk_buff, dataoff: libc::c_uint, ctinfo: ip_conntrack_info, state: *const nf_hook_state) -> i32;
pub fn nf_conntrack_gre_packet(ct: *mut nf_conn, skb: *mut sk_buff, dataoff: libc::c_uint, ctinfo: ip_conntrack_info, state: *const nf_hook_state) -> i32;

pub fn nf_conntrack_generic_init_net(net: *mut net);
pub fn nf_conntrack_tcp_init_net(net: *mut net);
pub fn nf_conntrack_udp_init_net(net: *mut net);
pub fn nf_conntrack_gre_init_net(net: *mut net);
pub fn nf_conntrack_sctp_init_net(net: *mut net);
pub fn nf_conntrack_icmp_init_net(net: *mut net);
pub fn nf_conntrack_icmpv6_init_net(net: *mut net);

pub static nf_conntrack_l4proto_generic: nf_conntrack_l4proto;
pub fn nf_ct_l4proto_find(l4proto: u8) -> *const nf_conntrack_l4proto;

pub fn nf_ct_port_tuple_to_nlattr(skb: *mut sk_buff, tuple: *const nf_conntrack_tuple) -> i32;
pub fn nf_ct_port_nlattr_to_tuple(tb: *mut *mut nlattr, t: *mut nf_conntrack_tuple, flags: u32) -> i32;
pub fn nf_ct_port_nlattr_tuple_size() -> libc::c_uint;
pub static nf_ct_port_nla_policy: [nla_policy; 0];

/* CONFIG_SYSCTL */
pub fn nf_ct_l4proto_log_invalid(skb: *const sk_buff, ct: *const nf_conn, state: *const nf_hook_state, fmt: *const libc::c_char, ...) ;
pub fn nf_l4proto_log_invalid(skb: *const sk_buff, state: *const nf_hook_state, protonum: u8, fmt: *const libc::c_char, ...);

/* CONFIG_NF_CONNTRACK: these inline functions require the corresponding
 * nf_conn and per-network namespace layouts from nf_conntrack.h. */
pub unsafe fn nf_generic_pernet(net: *mut net) -> *mut nf_generic_net { &mut (*net).ct.nf_ct_proto.generic }
pub unsafe fn nf_tcp_pernet(net: *mut net) -> *mut nf_tcp_net { &mut (*net).ct.nf_ct_proto.tcp }
pub unsafe fn nf_udp_pernet(net: *mut net) -> *mut nf_udp_net { &mut (*net).ct.nf_ct_proto.udp }
pub unsafe fn nf_icmp_pernet(net: *mut net) -> *mut nf_icmp_net { &mut (*net).ct.nf_ct_proto.icmp }
pub unsafe fn nf_icmpv6_pernet(net: *mut net) -> *mut nf_icmp_net { &mut (*net).ct.nf_ct_proto.icmpv6 }

/* Caller must check nf_ct_protonum(ct) is IPPROTO_TCP before calling. */
pub unsafe fn nf_ct_set_tcp_be_liberal(ct: *mut nf_conn) {
	(*ct).proto.tcp.seen[0].flags |= IP_CT_TCP_FLAG_BE_LIBERAL;
	(*ct).proto.tcp.seen[1].flags |= IP_CT_TCP_FLAG_BE_LIBERAL;
}

/* Caller must check nf_ct_protonum(ct) is IPPROTO_TCP before calling. */
pub unsafe fn nf_conntrack_tcp_established(ct: *const nf_conn) -> bool {
	(*ct).proto.tcp.state == TCP_CONNTRACK_ESTABLISHED && test_bit(IPS_ASSURED_BIT, &(*ct).status)
}

pub unsafe fn nf_sctp_pernet(net: *mut net) -> *mut nf_sctp_net { &mut (*net).ct.nf_ct_proto.sctp }
pub unsafe fn nf_gre_pernet(net: *mut net) -> *mut nf_gre_net { &mut (*net).ct.nf_ct_proto.gre }

}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
