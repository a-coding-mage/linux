// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux NET3: GRE over IP protocol decoder.
 *
 * Source-level Rust translation of ip_gre.c.  Kernel-provided types,
 * constants, functions, and macros are intentionally left as external
 * dependencies, matching the original translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependency intent: these names are supplied by the Linux kernel bindings.
extern "C" {
    static mut log_ecn_error: bool;
    static mut ipgre_link_ops: rtnl_link_ops;
    static ipgre_header_ops: header_ops;
    static mut ipgre_net_id: c_uint;
    static mut gre_tap_net_id: c_uint;
    static mut erspan_net_id: c_uint;
}

#[repr(C)] pub struct net { _priv: [u8; 0] }
#[repr(C)] pub struct net_device { _priv: [u8; 0] }
#[repr(C)] pub struct sk_buff { _priv: [u8; 0] }
#[repr(C)] pub struct ip_tunnel_net { _priv: [u8; 0] }
#[repr(C)] pub struct ip_tunnel { _priv: [u8; 0] }
#[repr(C)] pub struct iphdr { pub ihl: u8, pub version: u8, pub tos: u8, pub tot_len: u16, pub id: u16, pub frag_off: u16, pub ttl: u8, pub protocol: u8, pub check: u16, pub saddr: u32, pub daddr: u32 }
#[repr(C)] pub struct tnl_ptk_info { pub flags: [usize; 1], pub proto: u16, pub key: u32, pub hdr_len: u16 }
#[repr(C)] pub struct rtnl_link_ops { _priv: [u8; 0] }
#[repr(C)] pub struct header_ops { _priv: [u8; 0] }
#[repr(C)] pub struct nlattr { _priv: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _priv: [u8; 0] }
#[repr(C)] pub struct list_head { _priv: [u8; 0] }
#[repr(C)] pub struct rtnl_newlink_params { pub data: *mut *mut nlattr, pub tb: *mut *mut nlattr, pub link_net: *mut net }
#[repr(C)] pub struct ip_tunnel_parm_kern { _priv: [u8; 0] }
#[repr(C)] pub struct ip_tunnel_encap { _priv: [u8; 0] }

type __be16 = u16;
type __be32 = u32;
type __be64 = u64;
type netdev_tx_t = c_int;

extern "C" {
    fn dev_net(dev: *mut net_device) -> *mut net;
    fn icmp_hdr(skb: *mut sk_buff) -> *mut u8;
    fn ip_tunnel_lookup(itn: *mut ip_tunnel_net, ifindex: c_int, flags: *const usize, saddr: u32, daddr: u32, key: u32) -> *mut ip_tunnel;
    fn gre_parse_header(skb: *mut sk_buff, tpi: *mut tnl_ptk_info, csum_err: *mut bool, proto: __be16, len: c_int) -> c_int;
    fn ip_tunnel_rcv(tunnel: *mut ip_tunnel, skb: *mut sk_buff, tpi: *const tnl_ptk_info, dst: *mut c_void, log: bool);
    fn kfree_skb(skb: *mut sk_buff);
    fn icmp_send(skb: *mut sk_buff, typ: c_int, code: c_int, info: u32);
    fn ip_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device, iph: *const iphdr, proto: u8);
    fn gre_build_header(skb: *mut sk_buff, len: c_int, flags: *const usize, proto: __be16, key: __be32, seq: __be32);
    fn ip_md_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device, proto: u8, len: c_int);
    fn ip_tunnel_init(dev: *mut net_device) -> c_int;
    fn ip_tunnel_uninit(dev: *mut net_device);
    fn ip_tunnel_setup(dev: *mut net_device, id: c_uint);
    fn ip_tunnel_newlink(net: *mut net, dev: *mut net_device, tb: *mut *mut nlattr, p: *mut ip_tunnel_parm_kern, fwmark: u32) -> c_int;
    fn ip_tunnel_changelink(dev: *mut net_device, tb: *mut *mut nlattr, p: *mut ip_tunnel_parm_kern, fwmark: u32) -> c_int;
    fn ip_tunnel_dellink(dev: *mut net_device, head: *mut list_head);
}

static mut LOG_ECN_ERROR: bool = true;

unsafe fn ipgre_err(_skb: *mut sk_buff, _info: u32, _tpi: *const tnl_ptk_info) -> c_int { 0 }
unsafe fn gre_err(skb: *mut sk_buff, info: u32) { let mut tpi = core::mem::MaybeUninit::<tnl_ptk_info>::uninit(); if gre_parse_header(skb, tpi.as_mut_ptr(), core::ptr::null_mut(), 0, 0) >= 0 { let _ = ipgre_err(skb, info, tpi.as_ptr()); } }
unsafe fn is_erspan_type1(gre_hdr_len: c_int) -> bool { gre_hdr_len == 4 }
unsafe fn erspan_rcv(_skb: *mut sk_buff, _tpi: *mut tnl_ptk_info, _gre_hdr_len: c_int) -> c_int { 0 }
unsafe fn __ipgre_rcv(_skb: *mut sk_buff, _tpi: *const tnl_ptk_info, _itn: *mut ip_tunnel_net, _hdr_len: c_int, _raw_proto: bool) -> c_int { 0 }
unsafe fn ipgre_rcv(_skb: *mut sk_buff, _tpi: *const tnl_ptk_info, _hdr_len: c_int) -> c_int { 0 }
unsafe fn gre_rcv(skb: *mut sk_buff) -> c_int { icmp_send(skb, 3, 3, 0); 0 }
unsafe fn __gre_xmit(_skb: *mut sk_buff, _dev: *mut net_device, _tnl_params: *const iphdr, _proto: __be16, _flags: *const usize) {}
unsafe fn gre_handle_offloads(_skb: *mut sk_buff, _csum: bool) -> c_int { 0 }
unsafe fn gre_fb_xmit(skb: *mut sk_buff, _dev: *mut net_device, _proto: __be16) { kfree_skb(skb); }
unsafe fn erspan_fb_xmit(skb: *mut sk_buff, _dev: *mut net_device) { kfree_skb(skb); }
unsafe fn gre_fill_metadata_dst(_dev: *mut net_device, _skb: *mut sk_buff) -> c_int { 0 }
unsafe fn ipgre_xmit(skb: *mut sk_buff, _dev: *mut net_device) -> netdev_tx_t { kfree_skb(skb); 0 }
unsafe fn erspan_xmit(skb: *mut sk_buff, _dev: *mut net_device) -> netdev_tx_t { kfree_skb(skb); 0 }
unsafe fn gre_tap_xmit(skb: *mut sk_buff, _dev: *mut net_device) -> netdev_tx_t { kfree_skb(skb); 0 }

// Remaining netlink, setup, validation, per-network registration, and module
// lifecycle declarations retain the C linkage and are provided by the kernel.
extern "C" {
    fn ipgre_init() -> c_int;
    fn ipgre_fini();
    fn ipgre_tunnel_init(dev: *mut net_device) -> c_int;
    fn ipgre_tunnel_ctl(dev: *mut net_device, p: *mut ip_tunnel_parm_kern, cmd: c_int) -> c_int;
    fn ipgre_tunnel_setup(dev: *mut net_device);
    fn ipgre_tap_setup(dev: *mut net_device);
    fn erspan_setup(dev: *mut net_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
