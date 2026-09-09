/*
 * net/tipc/bearer.h: Include file for TIPC bearer code
 *
 * Copyright (c) 1996-2006, 2013-2016, Ericsson AB
 * Copyright (c) 2005, 2010-2011, Wind River Systems
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
 */

// Dependencies supplied by the corresponding kernel/TIPC modules are external.

pub const MAX_MEDIA: usize = 3;
pub const TIPC_MEDIA_INFO_SIZE: usize = 32;
pub const TIPC_MEDIA_TYPE_OFFSET: usize = 3;
pub const TIPC_MEDIA_ADDR_OFFSET: usize = 4;
pub const TIPC_MEDIA_TYPE_ETH: u32 = 1;
pub const TIPC_MEDIA_TYPE_IB: u32 = 2;
pub const TIPC_MEDIA_TYPE_UDP: u32 = 3;
pub const TIPC_MIN_BEARER_MTU: u32 = MAX_H_SIZE + INT_H_SIZE;
pub const TIPC_BROADCAST_SUPPORT: u32 = 1;
pub const TIPC_REPLICAST_SUPPORT: u32 = 2;

#[repr(C)]
pub struct tipc_media_addr {
    pub value: [u8; TIPC_MEDIA_INFO_SIZE],
    pub media_id: u8,
    pub broadcast: u8,
}

#[repr(C)]
pub struct tipc_media {
    pub send_msg: Option<unsafe extern "C" fn(*mut net, *mut sk_buff, *mut tipc_bearer, *mut tipc_media_addr) -> i32>,
    pub enable_media: Option<unsafe extern "C" fn(*mut net, *mut tipc_bearer, *mut *mut nlattr) -> i32>,
    pub disable_media: Option<unsafe extern "C" fn(*mut tipc_bearer)>,
    pub addr2str: Option<unsafe extern "C" fn(*mut tipc_media_addr, *mut i8, i32) -> i32>,
    pub addr2msg: Option<unsafe extern "C" fn(*mut i8, *mut tipc_media_addr) -> i32>,
    pub msg2addr: Option<unsafe extern "C" fn(*mut tipc_bearer, *mut tipc_media_addr, *mut i8) -> i32>,
    pub raw2addr: Option<unsafe extern "C" fn(*mut tipc_bearer, *mut tipc_media_addr, *const i8) -> i32>,
    pub priority: u32,
    pub tolerance: u32,
    pub min_win: u32,
    pub max_win: u32,
    pub mtu: u32,
    pub type_id: u32,
    pub hwaddr_len: u32,
    pub name: [i8; TIPC_MAX_MEDIA_NAME],
}

#[repr(C)]
pub struct tipc_bearer {
    pub media_ptr: *mut core::ffi::c_void,
    pub mtu: u32,
    pub addr: tipc_media_addr,
    pub name: [i8; TIPC_MAX_BEARER_NAME],
    pub media: *mut tipc_media,
    pub bcast_addr: tipc_media_addr,
    pub pt: packet_type,
    pub rcu: rcu_head,
    pub priority: u32,
    pub min_win: u32,
    pub max_win: u32,
    pub tolerance: u32,
    pub domain: u32,
    pub identity: u32,
    pub disc: *mut tipc_discoverer,
    pub net_plane: i8,
    pub encap_hlen: u16,
    pub up: usize,
    pub refcnt: refcount_t,
}

#[repr(C)]
pub struct tipc_bearer_names {
    pub media_name: [i8; TIPC_MAX_MEDIA_NAME],
    pub if_name: [i8; TIPC_MAX_IF_NAME],
}

extern "C" {
    pub fn tipc_rcv(net: *mut net, skb: *mut sk_buff, b: *mut tipc_bearer);
    pub static mut eth_media_info: tipc_media;
    // Declared only when CONFIG_TIPC_MEDIA_IB is enabled.
    #[cfg(feature = "CONFIG_TIPC_MEDIA_IB")]
    pub static mut ib_media_info: tipc_media;
    // Declared only when CONFIG_TIPC_MEDIA_UDP is enabled.
    #[cfg(feature = "CONFIG_TIPC_MEDIA_UDP")]
    pub static mut udp_media_info: tipc_media;
    pub fn tipc_nl_bearer_disable(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn __tipc_nl_bearer_disable(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn tipc_nl_bearer_enable(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn __tipc_nl_bearer_enable(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn tipc_nl_bearer_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn tipc_nl_bearer_get(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn tipc_nl_bearer_set(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn __tipc_nl_bearer_set(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn tipc_nl_bearer_add(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn tipc_nl_media_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn tipc_nl_media_get(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn tipc_nl_media_set(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn __tipc_nl_media_set(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn tipc_media_addr_printf(buf: *mut i8, len: i32, a: *mut tipc_media_addr) -> i32;
    pub fn tipc_enable_l2_media(net: *mut net, b: *mut tipc_bearer, attrs: *mut *mut nlattr) -> i32;
    pub fn tipc_bearer_hold(b: *mut tipc_bearer) -> bool;
    pub fn tipc_bearer_put(b: *mut tipc_bearer);
    pub fn tipc_disable_l2_media(b: *mut tipc_bearer);
    pub fn tipc_l2_send_msg(net: *mut net, buf: *mut sk_buff, b: *mut tipc_bearer, dest: *mut tipc_media_addr) -> i32;
    pub fn tipc_bearer_add_dest(net: *mut net, bearer_id: u32, dest: u32);
    pub fn tipc_bearer_remove_dest(net: *mut net, bearer_id: u32, dest: u32);
    pub fn tipc_bearer_find(net: *mut net, name: *const i8) -> *mut tipc_bearer;
    pub fn tipc_bearer_get_name(net: *mut net, name: *mut i8, bearer_id: u32) -> i32;
    pub fn tipc_media_find(name: *const i8) -> *mut tipc_media;
    pub fn tipc_bearer_setup() -> i32;
    pub fn tipc_bearer_cleanup();
    pub fn tipc_bearer_stop(net: *mut net);
    pub fn tipc_bearer_mtu(net: *mut net, bearer_id: u32) -> i32;
    pub fn tipc_bearer_min_mtu(net: *mut net, bearer_id: u32) -> i32;
    pub fn tipc_bearer_bcast_support(net: *mut net, bearer_id: u32) -> bool;
    pub fn tipc_bearer_xmit_skb(net: *mut net, bearer_id: u32, skb: *mut sk_buff, dest: *mut tipc_media_addr);
    pub fn tipc_bearer_xmit(net: *mut net, bearer_id: u32, xmitq: *mut sk_buff_head, dst: *mut tipc_media_addr, dnode: *mut tipc_node);
    pub fn tipc_bearer_bc_xmit(net: *mut net, bearer_id: u32, xmitq: *mut sk_buff_head);
    pub fn tipc_clone_to_loopback(net: *mut net, pkts: *mut sk_buff_head);
    pub fn tipc_attach_loopback(net: *mut net) -> i32;
    pub fn tipc_detach_loopback(net: *mut net);
}

#[inline]
pub unsafe fn tipc_loopback_trace(net: *mut net, pkts: *mut sk_buff_head) {
    if dev_nit_active((*net).loopback_dev) != 0 {
        tipc_clone_to_loopback(net, pkts);
    }
}

#[inline]
pub unsafe fn tipc_mtu_bad(dev: *mut net_device) -> bool {
    if (*dev).mtu >= TIPC_MIN_BEARER_MTU {
        return false;
    }
    netdev_warn(dev, b"MTU too low for tipc bearer\0".as_ptr() as *const i8);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
