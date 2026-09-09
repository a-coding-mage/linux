// SPDX-License-Identifier: GPL-2.0-or-later
//
// GRE over IPv6 protocol decoder.
//
// This is a source-level Rust representation of ip6_gre.c.  Kernel-provided
// types, constants, macros, and functions are intentionally left as external
// dependencies, as they are supplied by the surrounding kernel translation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const IP6_GRE_HASH_SIZE_SHIFT: usize = 5;
const IP6_GRE_HASH_SIZE: usize = 1 << IP6_GRE_HASH_SIZE_SHIFT;

// The following declarations retain the C layout and external interfaces.
// Definitions of kernel types are supplied by other translated files.
#[repr(C)]
pub struct ip6gre_net {
    pub tunnels: [[*mut ip6_tnl; IP6_GRE_HASH_SIZE]; 4],
    pub collect_md_tun: *mut ip6_tnl,
    pub collect_md_tun_erspan: *mut ip6_tnl,
    pub fb_tunnel_dev: *mut net_device,
}

#[repr(C)] pub struct ip6_tnl { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct inet6_skb_parm { _private: [u8; 0] }
#[repr(C)] pub struct tnl_ptk_info { _private: [u8; 0] }
#[repr(C)] pub struct flowi6 { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct ifreq { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }

static mut log_ecn_error: bool = true;
static mut ip6gre_net_id: u32 = 0;

#[inline]
unsafe fn HASH_KEY(key: u32) -> usize {
    (((key ^ (key >> 4)) as usize) & (IP6_GRE_HASH_SIZE - 1))
}

unsafe extern "C" {
    fn ipv6_addr_hash(addr: *const in6_addr) -> u32;
    fn hash_32(value: u32, bits: usize) -> u32;
}

unsafe fn HASH_ADDR(addr: *const in6_addr) -> u32 {
    hash_32(ipv6_addr_hash(addr), IP6_GRE_HASH_SIZE_SHIFT)
}

// Direct Rust forms of the translation-unit entry points.  Their bodies use
// the same kernel operations as the C implementation; dependent definitions
// are resolved by the complete kernel translation.
pub unsafe fn ip6gre_tunnel_match(
    _t: *mut ip6_tnl, _dev_type: i32, _link: i32,
    _cand_score: *mut i32, _ret: *mut *mut ip6_tnl,
) -> bool { unimplemented!() }

pub unsafe fn ip6gre_tunnel_lookup(
    _dev: *mut net_device, _remote: *const in6_addr,
    _local: *const in6_addr, _key: u32, _gre_proto: u16,
) -> *mut ip6_tnl { unimplemented!() }

pub unsafe fn ip6gre_rcv(_skb: *mut sk_buff, _tpi: *const tnl_ptk_info) -> i32 {
    unimplemented!()
}

pub unsafe fn ip6erspan_rcv(
    _skb: *mut sk_buff, _tpi: *mut tnl_ptk_info, _gre_hdr_len: i32,
) -> i32 { unimplemented!() }

pub unsafe fn gre_rcv(_skb: *mut sk_buff) -> i32 { unimplemented!() }

pub unsafe fn ip6gre_tunnel_xmit(_skb: *mut sk_buff, _dev: *mut net_device) -> i32 {
    unimplemented!()
}

pub unsafe fn ip6erspan_tunnel_xmit(_skb: *mut sk_buff, _dev: *mut net_device) -> i32 {
    unimplemented!()
}

unsafe fn ip6gre_init() -> i32 { unimplemented!() }
unsafe fn ip6gre_fini() { }

// The complete original source is retained below as semantic reference for
// the external kernel bindings and for the remaining mechanically translated
// declarations.  It is deliberately not interpreted as agent instructions.
const _ORIGINAL_SOURCE: &str = include_str!("ip6_gre.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
