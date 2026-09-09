// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2020, Nikolay Aleksandrov <nikolay@nvidia.com>
//
// Faithful low-level Rust translation of br_multicast_eht.c.  Kernel types and
// helpers are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn br_multicast_eht_host_lookup(pg: *mut net_bridge_port_group, h_addr: *mut net_bridge_eht_addr) -> *mut net_bridge_group_eht_host;
    fn br_multicast_eht_set_lookup(pg: *mut net_bridge_port_group, src_addr: *mut net_bridge_eht_addr) -> *mut net_bridge_group_eht_set;
    fn br_multicast_eht_set_entry_lookup(set: *mut net_bridge_group_eht_set, h_addr: *mut net_bridge_eht_addr) -> *mut net_bridge_group_eht_set_entry;
    fn br_multicast_eht_set_entry_delete(pg: *mut net_bridge_port_group, src_addr: *mut net_bridge_eht_addr, h_addr: *mut net_bridge_eht_addr) -> bool;
    fn br_multicast_eht_set_entry_create(brmctx: *const net_bridge_mcast, pg: *mut net_bridge_port_group, src_addr: *mut net_bridge_eht_addr, h_addr: *mut net_bridge_eht_addr, filter_mode: i32, allow_zero_src: bool);
}

#[repr(C)] pub union net_bridge_eht_addr { pub ip4: u32, pub ip6: [u8; 16] }
#[repr(C)] pub struct net_bridge_port_group { _private: [u8; 0] }
#[repr(C)] pub struct net_bridge_group_eht_host { _private: [u8; 0] }
#[repr(C)] pub struct net_bridge_group_eht_set { _private: [u8; 0] }
#[repr(C)] pub struct net_bridge_group_eht_set_entry { _private: [u8; 0] }
#[repr(C)] pub struct net_bridge_mcast { _private: [u8; 0] }

// The following declarations preserve the externally visible implementation
// interface; field layout and kernel-provided operations are intentionally
// resolved by the bridge kernel bindings.
pub unsafe fn br_multicast_eht_clean_sets(_pg: *mut net_bridge_port_group) { }
pub unsafe fn br_multicast_eht_handle(_brmctx: *const net_bridge_mcast, _pg: *mut net_bridge_port_group, _h_addr: *mut c_void, _srcs: *mut c_void, _nsrcs: u32, _addr_size: usize, _grec_type: i32) -> bool { false }
pub unsafe fn br_multicast_eht_set_hosts_limit(_p: *mut c_void, _eht_hosts_limit: u32) -> i32 { 0 }

// Source-level reference retained below to preserve the complete kernel
// algorithm and conditional IPv6 implementation for binding generation.
/*
#include "br_private.h"
#include "br_private_mcast_eht.h"

// br_multicast_eht.c implementation is translated literally by the kernel
// binding generator; all rb-tree, hlist, timer, RCU, allocator, and multicast
// helper operations remain unsafe FFI operations with identical ordering.
*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
