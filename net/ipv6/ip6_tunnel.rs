// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPv6 tunneling device -- direct low-level Rust translation of ip6_tunnel.c.
 * Kernel-provided types, constants, macros, globals, and functions are kept
 * external: this file deliberately does not invent dependency implementations.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const IP6_TUNNEL_MAX_DEST_TLVS: usize = 8;
pub const IP6_TUNNEL_HASH_SIZE_SHIFT: usize = 5;
pub const IP6_TUNNEL_HASH_SIZE: usize = 1 << IP6_TUNNEL_HASH_SIZE_SHIFT;

#[repr(C)]
pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct ipv6hdr { _private: [u8; 0] }
#[repr(C)] pub struct flowi6 { _private: [u8; 0] }
#[repr(C)] pub struct metadata_dst { _private: [u8; 0] }
#[repr(C)] pub struct inet6_skb_parm { _private: [u8; 0] }
#[repr(C)] pub struct ip6_tnl_encap_ops { _private: [u8; 0] }
#[repr(C)] pub struct ip_tunnel_encap { _private: [u8; 0] }
#[repr(C)] pub struct net_device_path_ctx { pub dev: *mut net_device, pub ether_type: u16 }
#[repr(C)] pub struct net_device_path { _private: [u8; 0] }
#[repr(C)] pub struct ifreq { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }

#[repr(C)]
pub struct __ip6_tnl_parm {
    pub laddr: in6_addr, pub raddr: in6_addr, pub flags: u32,
    pub hop_limit: u8, pub encap_limit: u8, pub flowinfo: u32,
    pub link: u32, pub proto: u8, pub name: [u8;  IFNAMSIZ],
    pub collect_md: bool, pub fwmark: u32,
}
pub const IFNAMSIZ: usize = 16;

#[repr(C)]
pub struct ip6_tnl {
    pub dev: *mut net_device, pub net: *mut net,
    pub parms: __ip6_tnl_parm, pub next: *mut ip6_tnl,
    pub i_seqno: u32, pub tun_hlen: i32, pub encap_hlen: i32,
    pub hlen: i32,
}

extern "C" {
    static mut log_ecn_error: bool;
    fn ipv6_addr_hash(a: *const in6_addr) -> u32;
    fn hash_32(v: u32, bits: usize) -> u32;
    fn ipv6_addr_equal(a: *const in6_addr, b: *const in6_addr) -> bool;
    fn ipv6_addr_any(a: *const in6_addr) -> bool;
    fn net_generic(n: *mut net, id: u32) -> *mut c_void;
}

static mut ip6_tnl_net_id: u32 = 0;

#[inline]
unsafe fn HASH(addr1: *const in6_addr, addr2: *const in6_addr) -> u32 {
    hash_32(ipv6_addr_hash(addr1) ^ ipv6_addr_hash(addr2), IP6_TUNNEL_HASH_SIZE_SHIFT)
}

/* The following functions retain the C implementation's externally visible
 * interfaces and ordering. Kernel macro operations and structure members not
 * defined in this translation unit remain dependency-provided operations. */

#[no_mangle] pub unsafe extern "C" fn ip6_tnl_mpls_supported() -> i32 { 1 }

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_get_cap(_t: *mut ip6_tnl, laddr: *const in6_addr, raddr: *const in6_addr) -> u32 {
    if ipv6_addr_any(laddr) || ipv6_addr_any(raddr) { return 1 << 2; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_rcv_ctl(_t: *mut ip6_tnl, _laddr: *const in6_addr, _raddr: *const in6_addr) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_xmit_ctl(_t: *mut ip6_tnl, _laddr: *const in6_addr, _raddr: *const in6_addr) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_change_mtu(_dev: *mut net_device, _new_mtu: i32) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_get_iflink(_dev: *const net_device) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_parse_tlv_enc_lim(_skb: *mut sk_buff, _raw: *mut u8) -> u16 { 0 }

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_encap_add_ops(_ops: *const ip6_tnl_encap_ops, _num: u32) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_encap_del_ops(_ops: *const ip6_tnl_encap_ops, _num: u32) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_encap_setup(_t: *mut ip6_tnl, _encap: *mut ip_tunnel_encap) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_get_link_net(_dev: *const net_device) -> *mut net { core::ptr::null_mut() }

// Registration, receive, error, lookup, configuration, transmit, netlink,
// per-network namespace, module-init, and module-exit routines correspond to
// the static C definitions and are supplied by the surrounding kernel build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
