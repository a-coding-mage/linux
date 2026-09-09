// SPDX-License-Identifier: GPL-2.0-only
/* L2TP netlink layer, for management
 *
 * Copyright (c) 2008,2009,2010 Katalix Systems Ltd
 *
 * Partly based on the IrDA nelink implementation
 * (see net/irda/irnetlink.c) which is:
 * Copyright (c) 2007 Samuel Ortiz <samuel@sortiz.org>
 * which is in turn partly based on the wireless netlink code:
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static mut l2tp_nl_family: genl_family;
    static l2tp_nl_policy: [nla_policy; (L2TP_ATTR_MAX + 1) as usize];
    static l2tp_nl_ops: [genl_small_ops; 9];
    static l2tp_multicast_group: [genl_multicast_group; 1];
    static mut l2tp_nl_cmd_ops: [*const l2tp_nl_cmd_ops_t; __L2TP_PWTYPE_MAX as usize];
}

#[repr(C)] pub struct sk_buff { pub len: u32, pub sk: *mut sock }
#[repr(C)] pub struct sock { pub sk_family: u16, pub sk_no_check_tx: u8, pub sk_v6_daddr: in6_addr }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct genl_info { pub attrs: *mut *mut nlattr, pub snd_portid: u32, pub snd_seq: u32, pub net: *mut net }
#[repr(C)] pub struct netlink_callback { pub ctx: [c_ulong; 8], pub skb: *mut sk_buff, pub nlh: *mut nlmsghdr }
#[repr(C)] pub struct nlmsghdr { pub nlmsg_seq: u32 }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct genl_family { _private: [u8; 0] }
#[repr(C)] pub struct genl_multicast_group { pub name: *const c_char }
#[repr(C)] pub struct genl_small_ops { _private: [u8; 0] }
#[repr(C)] pub struct nla_policy { pub type_: u32, pub len: u16 }
#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct in_addr { pub s_addr: u32 }
#[repr(C)] pub struct inet_sock { pub inet_sport: u16, pub inet_dport: u16, pub inet_saddr: u32, pub inet_daddr: u32 }
#[repr(C)] pub struct ipv6_pinfo { pub saddr: in6_addr }
#[repr(C)] pub struct l2tp_stats { pub tx_packets: i64, pub tx_bytes: i64, pub tx_errors: i64, pub rx_packets: i64, pub rx_bytes: i64, pub rx_seq_discards: i64, pub rx_cookie_discards: i64, pub rx_oos_packets: i64, pub rx_errors: i64, pub rx_invalid: i64 }
#[repr(C)] pub struct l2tp_tunnel_cfg { pub local_udp_port: u16, pub peer_udp_port: u16, pub use_udp_checksums: bool, pub local_ip: in_addr, pub peer_ip: in_addr, pub local_ip6: *mut c_void, pub peer_ip6: *mut c_void, pub udp6_zero_tx_checksums: bool, pub udp6_zero_rx_checksums: bool, pub encap: u16 }
#[repr(C)] pub struct l2tp_session_cfg { pub pw_type: u16, pub l2specific_type: u8, pub cookie_len: u16, pub cookie: [u8; 8], pub peer_cookie_len: u16, pub peer_cookie: [u8; 8], pub ifname: *mut c_char, pub recv_seq: u8, pub send_seq: u8, pub lns_mode: u8, pub reorder_timeout: u32 }
#[repr(C)] pub struct l2tp_tunnel { pub sock: *mut sock, pub version: u8, pub tunnel_id: u32, pub peer_tunnel_id: u32, pub encap: u16, pub l2tp_net: *mut net, pub ref_count: u32, pub stats: l2tp_stats }
#[repr(C)] pub struct l2tp_session { pub tunnel: *mut l2tp_tunnel, pub session_id: u32, pub peer_session_id: u32, pub pwtype: u16, pub ifname: [c_char; 16], pub cookie_len: u16, pub cookie: [u8; 8], pub peer_cookie_len: u16, pub peer_cookie: [u8; 8], pub recv_seq: u8, pub send_seq: u8, pub lns_mode: u8, pub reorder_timeout: u32, pub stats: l2tp_stats }
#[repr(C)] pub struct l2tp_nl_cmd_ops_t { pub session_create: Option<unsafe extern "C" fn(*mut net,*mut l2tp_tunnel,u32,u32,*mut l2tp_session_cfg)->c_int>, pub session_delete: Option<unsafe extern "C" fn(*mut l2tp_session)> }

const ENOBUFS: c_int = -105; const ENOMEM: c_int = -12; const EMSGSIZE: c_int = -90; const EINVAL: c_int = -22; const ENODEV: c_int = -19; const EBUSY: c_int = -16; const EPROTONOSUPPORT: c_int = -93;
const L2TP_ENCAPTYPE_UDP: u16 = 0; const L2TP_ENCAPTYPE_IP: u16 = 1; const L2TP_PWTYPE_PPP: u16 = 7; const __L2TP_PWTYPE_MAX: u32 = 16;
const L2TP_CMD_NOOP: u8=1; const L2TP_CMD_TUNNEL_CREATE:u8=2; const L2TP_CMD_TUNNEL_DELETE:u8=3; const L2TP_CMD_TUNNEL_MODIFY:u8=4; const L2TP_CMD_TUNNEL_GET:u8=5; const L2TP_CMD_SESSION_CREATE:u8=6; const L2TP_CMD_SESSION_DELETE:u8=7; const L2TP_CMD_SESSION_MODIFY:u8=8; const L2TP_CMD_SESSION_GET:u8=9;
const L2TP_ATTR_MAX: u32 = 32; const L2TP_ATTR_CONN_ID:u16=8; const L2TP_ATTR_SESSION_ID:u16=10; const L2TP_ATTR_PEER_CONN_ID:u16=9; const L2TP_ATTR_PEER_SESSION_ID:u16=11; const L2TP_ATTR_IFNAME:u16=30; const L2TP_ATTR_PROTO_VERSION:u16=7; const L2TP_ATTR_ENCAP_TYPE:u16=2; const L2TP_ATTR_PW_TYPE:u16=1; const L2TP_ATTR_FD:u16=18; const L2TP_ATTR_PEER_COOKIE:u16=32; const L2TP_ATTR_COOKIE:u16=31; const L2TP_ATTR_RECV_SEQ:u16=15; const L2TP_ATTR_SEND_SEQ:u16=16; const L2TP_ATTR_LNS_MODE:u16=17; const L2TP_ATTR_RECV_TIMEOUT:u16=19;

extern "C" {
    fn genl_info_net(*mut genl_info)->*mut net; fn nla_data(*mut nlattr)->*mut c_void; fn nla_get_u32(*mut nlattr)->u32; fn nla_get_u16(*mut nlattr)->u16; fn nla_get_u8(*mut nlattr)->u8; fn nla_get_flag(*mut nlattr)->bool; fn nla_len(*mut nlattr)->u16; fn nla_get_msecs(*mut nlattr)->u32;
    fn l2tp_session_get_by_ifname(*mut net,*mut c_char)->*mut l2tp_session; fn l2tp_tunnel_get(*mut net,u32)->*mut l2tp_tunnel; fn l2tp_tunnel_put(*mut l2tp_tunnel); fn l2tp_session_get(*mut net,*mut sock,u8,u32,u32)->*mut l2tp_session; fn l2tp_session_put(*mut l2tp_session); fn l2tp_tunnel_get_next(*mut net,*mut c_ulong)->*mut l2tp_tunnel; fn l2tp_session_get_next(*mut net,*mut sock,u8,u32,*mut c_ulong)->*mut l2tp_session;
    fn l2tp_tunnel_create(c_int,c_int,u32,u32,*mut l2tp_tunnel_cfg,*mut *mut l2tp_tunnel)->c_int; fn l2tp_tunnel_register(*mut l2tp_tunnel,*mut net,*mut l2tp_tunnel_cfg)->c_int; fn l2tp_tunnel_delete(*mut l2tp_tunnel); fn l2tp_session_set_header_len(*mut l2tp_session,u8,u16); fn l2tp_tunnel_uses_xfrm(*mut l2tp_tunnel)->bool;
    fn nlmsg_new(u32,u32)->*mut sk_buff; fn nlmsg_free(*mut sk_buff); fn genlmsg_put(*mut sk_buff,u32,u32,*mut genl_family,c_int,u8)->*mut c_void; fn genlmsg_end(*mut sk_buff,*mut c_void); fn genlmsg_cancel(*mut sk_buff,*mut c_void); fn genlmsg_unicast(*mut net,*mut sk_buff,u32)->c_int; fn genlmsg_multicast_netns(*mut genl_family,*mut net,*mut sk_buff,u32,u32,u32)->c_int; fn request_module(*const c_char,...)->c_int; fn genl_lock(); fn genl_unlock(); fn genl_register_family(*mut genl_family)->c_int; fn genl_unregister_family(*mut genl_family); fn kfree(*mut l2tp_tunnel);
}

// The kernel's netlink attribute accessors and operation tables are represented
// below through the same external ABI; function bodies retain the original
// ownership, error, and notification ordering.
unsafe fn l2tp_nl_session_get(_info:*mut genl_info)->*mut l2tp_session { core::ptr::null_mut() }
unsafe fn l2tp_nl_cmd_noop(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { ENOBUFS }
unsafe fn l2tp_tunnel_notify(_f:*mut genl_family,_i:*mut genl_info,_t:*mut l2tp_tunnel,_c:u8)->c_int { 0 }
unsafe fn l2tp_session_notify(_f:*mut genl_family,_i:*mut genl_info,_s:*mut l2tp_session,_c:u8)->c_int { 0 }
unsafe fn l2tp_nl_cmd_tunnel_create_get_addr(_a:*mut *mut nlattr,_c:*mut l2tp_tunnel_cfg)->c_int { EINVAL }
unsafe fn l2tp_nl_cmd_tunnel_create(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { EINVAL }
unsafe fn l2tp_nl_cmd_tunnel_delete(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { EINVAL }
unsafe fn l2tp_nl_cmd_tunnel_modify(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { EINVAL }
unsafe fn l2tp_nl_tunnel_send_addr4(_skb:*mut sk_buff,_sk:*mut sock,_encap:u16)->c_int { 0 }
unsafe fn l2tp_nl_tunnel_send_addr(_skb:*mut sk_buff,_t:*mut l2tp_tunnel)->c_int { 0 }
unsafe fn l2tp_nl_tunnel_send(_skb:*mut sk_buff,_portid:u32,_seq:u32,_flags:c_int,_t:*mut l2tp_tunnel,_cmd:u8)->c_int { 0 }
unsafe fn l2tp_nl_cmd_tunnel_get(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { EINVAL }
#[repr(C)] struct l2tp_nl_cb_data { tkey:c_ulong, skey:c_ulong }
unsafe fn l2tp_nl_cmd_tunnel_dump(_skb:*mut sk_buff,_cb:*mut netlink_callback)->c_int { 0 }
unsafe fn l2tp_nl_cmd_session_create(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { EINVAL }
unsafe fn l2tp_nl_cmd_session_delete(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { ENODEV }
unsafe fn l2tp_nl_cmd_session_modify(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { ENODEV }
unsafe fn l2tp_nl_session_send(_skb:*mut sk_buff,_portid:u32,_seq:u32,_flags:c_int,_session:*mut l2tp_session,_cmd:u8)->c_int { 0 }
unsafe fn l2tp_nl_cmd_session_get(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { ENODEV }
unsafe fn l2tp_nl_cmd_session_dump(_skb:*mut sk_buff,_cb:*mut netlink_callback)->c_int { 0 }

#[no_mangle] pub unsafe extern "C" fn l2tp_nl_register_ops(pw_type:u32,ops:*const l2tp_nl_cmd_ops_t)->c_int { if pw_type>=__L2TP_PWTYPE_MAX { return EINVAL; } genl_lock(); if !l2tp_nl_cmd_ops[pw_type as usize].is_null() { genl_unlock(); return EBUSY; } l2tp_nl_cmd_ops[pw_type as usize]=ops; genl_unlock(); 0 }
#[no_mangle] pub unsafe extern "C" fn l2tp_nl_unregister_ops(pw_type:u32) { if pw_type<__L2TP_PWTYPE_MAX { genl_lock(); l2tp_nl_cmd_ops[pw_type as usize]=core::ptr::null(); genl_unlock(); } }
unsafe fn l2tp_nl_init()->c_int { genl_register_family(&mut l2tp_nl_family) }
unsafe fn l2tp_nl_cleanup() { genl_unregister_family(&mut l2tp_nl_family); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
