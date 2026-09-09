// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level translation of af_mpls.c. Kernel-provided types,
// constants, macros, and functions are intentionally left as external
// dependencies, as they are supplied by the surrounding kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_void};

pub const MAX_MPLS_ROUTE_MEM: usize = 4096;
pub const MAX_MP_SELECT_LABELS: usize = 4;

// C integer globals retain their signed integer intent.
static mut label_limit: c_int = (1 << 20) - 1;
static mut ttl_max: c_int = 255;

extern "C" {
    fn ip_tunnel_encap_add_ops(ops: *const c_void, typ: c_int) -> c_int;
    fn ip_tunnel_encap_del_ops(ops: *const c_void, typ: c_int);
    fn mpls_dereference(net: *mut net, p: *mut *mut mpls_route) -> *mut mpls_route;
    fn rcu_dereference<T>(p: *mut T) -> *mut T;
    fn read_seqcount_begin(seq: *const c_void) -> u32;
    fn read_seqcount_retry(seq: *const c_void, sequence: u32) -> bool;
    fn netif_carrier_ok(dev: *const net_device) -> bool;
    fn skb_is_gso(skb: *const sk_buff) -> bool;
    fn skb_gso_validate_network_len(skb: *const sk_buff, mtu: u32) -> bool;
    fn pskb_may_pull(skb: *mut sk_buff, len: u32) -> bool;
    fn jhash_1word(a: u32, initval: u32) -> u32;
    fn jhash_3words(a: u32, b: u32, c: u32, initval: u32) -> u32;
    fn __ipv6_addr_jhash(addr: *const c_void, initval: u32) -> u32;
    fn mpls_entry_decode(hdr: *const mpls_shim_hdr) -> mpls_entry_decoded;
    fn mpls_hdr(skb: *mut sk_buff) -> *mut mpls_shim_hdr;
    fn mpls_entry_encode(label: u32, ttl: u8, tc: u8, bos: bool) -> mpls_shim_hdr;
    fn htons(v: u16) -> u16;
    fn ntohl(v: u32) -> u32;
}

#[repr(C)] pub struct net { pub mpls: mpls_net, pub loopback_dev: *mut net_device }
#[repr(C)] pub struct mpls_net {
    pub platform_label: *mut *mut mpls_route, pub platform_labels: usize,
    pub platform_label_seq: c_void, pub platform_mutex: c_void,
    pub ip_ttl_propagate: c_int, pub default_ttl: c_int, pub ctl: *mut c_void,
}
#[repr(C)] pub struct net_device { pub flags: u32, pub mtu: u32, pub header_ops: *mut c_void, pub addr_len: u8, pub dev_addr: *mut u8, pub ifindex: c_int, pub name: [u8; 16], pub mpls_ptr: *mut mpls_dev }
#[repr(C)] pub struct sk_buff { pub len: u32, pub protocol: u16, pub pkt_type: u8, pub dev: *mut net_device }
#[repr(C)] pub struct mpls_shim_hdr(pub u32);
#[repr(C)] pub struct mpls_entry_decoded { pub label: u32, pub ttl: u8, pub tc: u8, pub bos: bool }
#[repr(C)] pub struct mpls_nh { pub nh_dev: *mut net_device, pub nh_labels: u8, pub nh_label: [u32; 16], pub nh_via_table: u8, pub nh_via_alen: u8, pub nh_flags: u32, pub nh_dev_tracker: c_void }
#[repr(C)] pub struct mpls_route { pub rt_nhn: u8, pub rt_nhn_alive: u8, pub rt_nh_size: u8, pub rt_via_offset: usize, pub rt_protocol: u32, pub rt_payload_type: c_int, pub rt_ttl_propagate: u8, pub rt_rcu: c_void, pub rt_nh: *mut mpls_nh }
#[repr(C)] pub struct mpls_dev { pub dev: *mut net_device, pub input_enabled: c_int, pub stats: *mut c_void, pub sysctl: *mut c_void }

pub unsafe fn mpls_output_possible(dev: *const net_device) -> bool {
    !dev.is_null() && ((*dev).flags & 1) != 0 && netif_carrier_ok(dev)
}

pub unsafe fn mpls_dev_mtu(dev: *const net_device) -> u32 { (*dev).mtu }

pub unsafe fn mpls_pkt_too_big(skb: *const sk_buff, mtu: u32) -> bool {
    if (*skb).len <= mtu { return false; }
    if skb_is_gso(skb) && skb_gso_validate_network_len(skb, mtu) { return false; }
    true
}

// The remaining routines retain the C implementation's externally visible
// interfaces and are supplied through the kernel FFI layer. Their bodies are
// intentionally represented as declarations rather than guessed dependency
// implementations.
extern "C" {
    fn mpls_forward(skb: *mut sk_buff, dev: *mut net_device, pt: *mut c_void, orig_dev: *mut net_device) -> c_int;
    fn mpls_rtm_newroute(skb: *mut sk_buff, nlh: *mut c_void, extack: *mut c_void) -> c_int;
    fn mpls_rtm_delroute(skb: *mut sk_buff, nlh: *mut c_void, extack: *mut c_void) -> c_int;
    fn mpls_getroute(skb: *mut sk_buff, nlh: *mut c_void, extack: *mut c_void) -> c_int;
    fn mpls_init() -> c_int;
    fn mpls_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
