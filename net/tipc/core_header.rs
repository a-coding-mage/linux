/*
 * net/tipc/core.h: Include file for TIPC global declarations
 *
 * Copyright (c) 2005-2006, 2013-2018 Ericsson AB
 * Copyright (c) 2005-2007, 2010-2013, Wind River Systems
 * Copyright (c) 2020, Red Hat Inc
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the names of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed under the terms of the
 * GNU General Public License ("GPL") version 2 as published by the Free
 * Software Foundation.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.
 */

// Linux kernel includes and build-time configuration are supplied by dependencies.

pub struct tipc_node;
pub struct tipc_bearer;
pub struct tipc_bc_base;
pub struct tipc_link;
pub struct tipc_topsrv;
pub struct tipc_monitor;
#[cfg(CONFIG_TIPC_CRYPTO)]
pub struct tipc_crypto;

pub const TIPC_MOD_VER: &str = "2.0.0";
pub const NODE_HTABLE_SIZE: usize = 512;
pub const MAX_BEARERS: usize = 3;
pub const TIPC_DEF_MON_THRESHOLD: u32 = 32;
pub const NODE_ID_LEN: usize = 16;
pub const NODE_ID_STR_LEN: usize = NODE_ID_LEN * 2 + 1;

extern "C" {
    pub static mut tipc_net_id: ::core::ffi::c_uint;
    pub static mut sysctl_tipc_rmem: [::core::ffi::c_int; 3];
    pub static mut sysctl_tipc_named_timeout: ::core::ffi::c_int;
}

#[repr(C)]
pub struct tipc_net {
    pub node_id: [u8; NODE_ID_LEN],
    pub node_addr: u32,
    pub trial_addr: u32,
    pub addr_trial_end: ::core::ffi::c_ulong,
    pub node_id_string: [::core::ffi::c_char; NODE_ID_STR_LEN],
    pub net_id: ::core::ffi::c_int,
    pub random: ::core::ffi::c_int,
    pub legacy_addr_format: bool,
    pub node_list_lock: spinlock_t,
    pub node_htable: [hlist_head; NODE_HTABLE_SIZE],
    pub node_list: list_head,
    pub num_nodes: u32,
    pub num_links: u32,
    pub monitors: [*mut tipc_monitor; MAX_BEARERS],
    pub mon_threshold: ::core::ffi::c_int,
    pub bearer_list: [*mut tipc_bearer; MAX_BEARERS + 1],
    pub bclock: spinlock_t,
    pub bcbase: *mut tipc_bc_base,
    pub bcl: *mut tipc_link,
    pub sk_rht: rhashtable,
    pub nametbl_lock: spinlock_t,
    pub nametbl: *mut name_table,
    pub topsrv: *mut tipc_topsrv,
    pub subscription_count: atomic_t,
    pub capabilities: u16,
    pub loopback_pt: packet_type,
    #[cfg(CONFIG_TIPC_CRYPTO)]
    pub crypto_tx: *mut tipc_crypto,
    pub work: work_struct,
    pub wq_count: atomic_t,
}

#[inline]
pub unsafe fn tipc_net(net: *mut net) -> *mut tipc_net {
    net_generic(net, tipc_net_id)
}

#[inline]
pub unsafe fn tipc_netid(net: *mut net) -> ::core::ffi::c_int {
    (*tipc_net(net)).net_id
}

#[inline]
pub unsafe fn tipc_nodes(net: *mut net) -> *mut list_head {
    &mut (*tipc_net(net)).node_list
}

#[inline]
pub unsafe fn tipc_name_table(net: *mut net) -> *mut name_table {
    (*tipc_net(net)).nametbl
}

#[inline]
pub unsafe fn tipc_topsrv(net: *mut net) -> *mut tipc_topsrv {
    (*tipc_net(net)).topsrv
}

#[inline]
pub const fn tipc_hashfn(addr: u32) -> u32 {
    addr & (NODE_HTABLE_SIZE as u32 - 1)
}

#[inline]
pub const fn mod_(x: u16) -> u16 { x & 0xffffu16 }

#[inline]
pub const fn less_eq(left: u16, right: u16) -> bool {
    mod_(right.wrapping_sub(left)) < 32768u16
}

#[inline]
pub const fn more(left: u16, right: u16) -> bool { !less_eq(left, right) }

#[inline]
pub const fn less(left: u16, right: u16) -> bool {
    less_eq(left, right) && mod_(right) != mod_(left)
}

#[inline]
pub const fn tipc_in_range(val: u16, min: u16, max: u16) -> bool {
    !less(val, min) && !more(val, max)
}

#[inline]
pub unsafe fn tipc_net_hash_mixes(net: *mut net, tn_rand: ::core::ffi::c_int) -> u32 {
    net_hash_mix(&init_net) ^ net_hash_mix(net) ^ tn_rand as u32
}

#[inline]
pub unsafe fn hash128to32(bytes: *mut ::core::ffi::c_char) -> u32 {
    let tmp = bytes as *mut __be32;
    let res = ntohl(*tmp ^ *tmp.add(1) ^ *tmp.add(2) ^ *tmp.add(3));
    if res != 0 { return res; }
    ntohl(*tmp | *tmp.add(1) | *tmp.add(2) | *tmp.add(3))
}

#[cfg(CONFIG_SYSCTL)]
extern "C" {
    pub fn tipc_register_sysctl() -> ::core::ffi::c_int;
    pub fn tipc_unregister_sysctl();
}

#[cfg(not(CONFIG_SYSCTL))]
#[inline]
pub const fn tipc_register_sysctl() -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_SYSCTL))]
#[inline]
pub const fn tipc_unregister_sysctl() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
