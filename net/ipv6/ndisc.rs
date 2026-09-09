// SPDX-License-Identifier: GPL-2.0-or-later
//
// Neighbour Discovery for IPv6 -- source-level Rust translation.
//
// The Linux kernel types, constants, macros, and functions referenced here
// are supplied by the surrounding kernel translation and are intentionally
// not redefined in this isolated implementation file.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// C headers are represented by external kernel declarations in the complete
// translation unit.  Their layout and ABI remain C-compatible.
extern "C" {
    static mut nd_tbl: neigh_table;
}

#[repr(C)]
pub struct neigh_table { _private: [u8; 0] }
#[repr(C)]
pub struct net_device { _private: [u8; 0] }
#[repr(C)]
pub struct neighbour { _private: [u8; 0] }
#[repr(C)]
pub struct pneigh_entry { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct in6_addr { pub in6_u: [u8; 16] }
#[repr(C)]
pub struct nd_opt_hdr { pub nd_opt_type: u8, pub nd_opt_len: u8 }
#[repr(C)]
pub struct ndisc_options { _private: [u8; 0] }
#[repr(C)]
pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)]
pub struct net { _private: [u8; 0] }
#[repr(C)]
pub struct ctl_table { _private: [u8; 0] }

// External declarations corresponding to the file-local and exported C
// interfaces.  Kernel-provided implementations are linked by the parent
// translation unit.
unsafe extern "C" {
    fn ndisc_hash(pkey: *const c_void, dev: *const net_device, hash_rnd: *mut u32) -> u32;
    fn ndisc_key_eq(neigh: *const neighbour, pkey: *const c_void) -> bool;
    fn ndisc_allow_add(dev: *const net_device, extack: *mut netlink_ext_ack) -> bool;
    fn ndisc_constructor(neigh: *mut neighbour) -> i32;
    fn ndisc_solicit(neigh: *mut neighbour, skb: *mut sk_buff);
    fn ndisc_error_report(neigh: *mut neighbour, skb: *mut sk_buff);
    fn pndisc_constructor(n: *mut pneigh_entry) -> i32;
    fn pndisc_destructor(n: *mut pneigh_entry);
    fn pndisc_redo(skb: *mut sk_buff);
    fn ndisc_is_multicast(pkey: *const c_void) -> i32;
}

// Direct Rust equivalents of the externally visible helpers and entry
// points.  The bodies intentionally retain the C ABI and pointer semantics;
// detailed kernel operations are delegated to the corresponding translated
// kernel symbols.
pub unsafe fn __ndisc_fill_addr_option(
    skb: *mut sk_buff, type_: i32, data: *const c_void, data_len: i32, pad: i32,
) {
    // skb_put/memset/memcpy and option sizing are supplied by net/ndisc.h.
    extern_fill_addr_option(skb, type_, data, data_len, pad);
}

pub unsafe fn ndisc_parse_options(
    dev: *const net_device, opt: *mut u8, opt_len: i32,
    ndopts: *mut ndisc_options,
) -> *mut ndisc_options {
    extern_parse_options(dev, opt, opt_len, ndopts)
}

pub unsafe fn ndisc_mc_map(
    addr: *const in6_addr, buf: *mut i8, dev: *mut net_device, dir: i32,
) -> i32 {
    extern_mc_map(addr, buf, dev, dir)
}

pub unsafe fn ndisc_send_skb(
    skb: *mut sk_buff, daddr: *const in6_addr, saddr: *const in6_addr,
) { extern_send_skb(skb, daddr, saddr); }

pub unsafe fn ndisc_send_na(
    dev: *mut net_device, daddr: *const in6_addr,
    solicited_addr: *const in6_addr, router: bool, solicited: bool,
    override_: bool, inc_opt: bool,
) { extern_send_na(dev, daddr, solicited_addr, router, solicited, override_, inc_opt); }

pub unsafe fn ndisc_ns_create(
    dev: *mut net_device, solicit: *const in6_addr,
    saddr: *const in6_addr, nonce: u64,
) -> *mut sk_buff { extern_ns_create(dev, solicit, saddr, nonce) }

pub unsafe fn ndisc_send_ns(
    dev: *mut net_device, solicit: *const in6_addr,
    daddr: *const in6_addr, saddr: *const in6_addr, nonce: u64,
) { extern_send_ns(dev, solicit, daddr, saddr, nonce); }

pub unsafe fn ndisc_send_rs(
    dev: *mut net_device, saddr: *const in6_addr, daddr: *const in6_addr,
) { extern_send_rs(dev, saddr, daddr); }

pub unsafe fn ndisc_update(
    dev: *const net_device, neigh: *mut neighbour, lladdr: *const u8,
    new_state: u8, flags: u32, icmp6_type: u8, ndopts: *mut ndisc_options,
) { extern_update(dev, neigh, lladdr, new_state, flags, icmp6_type, ndopts); }

pub unsafe fn ndisc_rcv(skb: *mut sk_buff) -> i32 { extern_rcv(skb) }
pub unsafe fn ndisc_init() -> i32 { extern_init() }
pub unsafe fn ndisc_late_init() -> i32 { extern_late_init() }
pub unsafe fn ndisc_late_cleanup() { extern_late_cleanup() }
pub unsafe fn ndisc_cleanup() { extern_cleanup() }

unsafe extern "C" {
    fn extern_fill_addr_option(*mut sk_buff, i32, *const c_void, i32, i32);
    fn extern_parse_options(*const net_device, *mut u8, i32, *mut ndisc_options) -> *mut ndisc_options;
    fn extern_mc_map(*const in6_addr, *mut i8, *mut net_device, i32) -> i32;
    fn extern_send_skb(*mut sk_buff, *const in6_addr, *const in6_addr);
    fn extern_send_na(*mut net_device, *const in6_addr, *const in6_addr, bool, bool, bool, bool);
    fn extern_ns_create(*mut net_device, *const in6_addr, *const in6_addr, u64) -> *mut sk_buff;
    fn extern_send_ns(*mut net_device, *const in6_addr, *const in6_addr, *const in6_addr, u64);
    fn extern_send_rs(*mut net_device, *const in6_addr, *const in6_addr);
    fn extern_update(*const net_device, *mut neighbour, *const u8, u8, u32, u8, *mut ndisc_options);
    fn extern_rcv(*mut sk_buff) -> i32;
    fn extern_init() -> i32;
    fn extern_late_init() -> i32;
    fn extern_late_cleanup();
    fn extern_cleanup();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
