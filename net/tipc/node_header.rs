/*
 * net/tipc/node.h: Include file for TIPC node management routines
 *
 * Copyright (c) 2000-2006, 2014-2016, Ericsson AB
 * Copyright (c) 2005, 2010-2014, Wind River Systems
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
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

// Dependencies supplied by the surrounding translation unit.
pub enum net {}
pub enum tipc_node {}
pub enum tipc_crypto {}
pub enum list_head {}
pub enum tipc_bearer {}
pub enum tipc_media_addr {}
pub enum sk_buff_head {}
pub enum sk_buff {}
pub enum netlink_callback {}
pub enum genl_info {}

pub type u8 = ::core::ffi::c_uchar;
pub type u16 = ::core::ffi::c_ushort;
pub type u32 = ::core::ffi::c_uint;
pub type size_t = usize;

/* Optional capabilities supported by this code version */
pub const TIPC_SYN_BIT: u32 = 1;
pub const TIPC_BCAST_SYNCH: u32 = 1 << 1;
pub const TIPC_BCAST_STATE_NACK: u32 = 1 << 2;
pub const TIPC_BLOCK_FLOWCTL: u32 = 1 << 3;
pub const TIPC_BCAST_RCAST: u32 = 1 << 4;
pub const TIPC_NODE_ID128: u32 = 1 << 5;
pub const TIPC_LINK_PROTO_SEQNO: u32 = 1 << 6;
pub const TIPC_MCAST_RBCTL: u32 = 1 << 7;
pub const TIPC_GAP_ACK_BLOCK: u32 = 1 << 8;
pub const TIPC_TUNNEL_ENHANCED: u32 = 1 << 9;
pub const TIPC_NAGLE: u32 = 1 << 10;
pub const TIPC_NAMED_BCAST: u32 = 1 << 11;

pub const TIPC_NODE_CAPABILITIES: u32 = TIPC_SYN_BIT
    | TIPC_BCAST_SYNCH
    | TIPC_BCAST_STATE_NACK
    | TIPC_BCAST_RCAST
    | TIPC_BLOCK_FLOWCTL
    | TIPC_NODE_ID128
    | TIPC_LINK_PROTO_SEQNO
    | TIPC_MCAST_RBCTL
    | TIPC_GAP_ACK_BLOCK
    | TIPC_TUNNEL_ENHANCED
    | TIPC_NAGLE
    | TIPC_NAMED_BCAST;

pub const INVALID_BEARER_ID: i32 = -1;

extern "C" {
    pub fn tipc_node_stop(net: *mut net);
    pub fn tipc_node_get_id(net: *mut net, addr: u32, id: *mut u8) -> bool;
    pub fn tipc_node_get_addr(node: *mut tipc_node) -> u32;
    pub fn tipc_node_get_id_str(node: *mut tipc_node) -> *mut ::core::ffi::c_char;
    pub fn tipc_node_put(node: *mut tipc_node);
    pub fn tipc_node_get(node: *mut tipc_node);
    pub fn tipc_node_create(net: *mut net, addr: u32, peer_id: *mut u8,
                            capabilities: u16, hash_mixes: u32,
                            preliminary: bool) -> *mut tipc_node;
    // CONFIG_TIPC_CRYPTO conditional declarations.
    #[cfg(CONFIG_TIPC_CRYPTO)]
    pub fn tipc_node_crypto_rx(n: *mut tipc_node) -> *mut tipc_crypto;
    #[cfg(CONFIG_TIPC_CRYPTO)]
    pub fn tipc_node_crypto_rx_by_list(pos: *mut list_head) -> *mut tipc_crypto;
    #[cfg(CONFIG_TIPC_CRYPTO)]
    pub fn tipc_node_crypto_rx_by_addr(net: *mut net, addr: u32) -> *mut tipc_crypto;
    pub fn tipc_node_try_addr(net: *mut net, id: *mut u8, addr: u32) -> u32;
    pub fn tipc_node_check_dest(net: *mut net, onode: u32, peer_id128: *mut u8,
                                bearer: *mut tipc_bearer, capabilities: u16,
                                signature: u32, hash_mixes: u32,
                                maddr: *mut tipc_media_addr, respond: *mut bool,
                                dupl_addr: *mut bool);
    pub fn tipc_node_delete_links(net: *mut net, bearer_id: ::core::ffi::c_int);
    pub fn tipc_node_apply_property(net: *mut net, b: *mut tipc_bearer,
                                    prop: ::core::ffi::c_int);
    pub fn tipc_node_get_linkname(net: *mut net, bearer_id: u32, node: u32,
                                  linkname: *mut ::core::ffi::c_char,
                                  len: usize) -> ::core::ffi::c_int;
    pub fn tipc_node_xmit(net: *mut net, list: *mut sk_buff_head, dnode: u32,
                          selector: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn tipc_node_distr_xmit(net: *mut net, list: *mut sk_buff_head) -> ::core::ffi::c_int;
    pub fn tipc_node_xmit_skb(net: *mut net, skb: *mut sk_buff, dest: u32,
                              selector: u32) -> ::core::ffi::c_int;
    pub fn tipc_node_subscribe(net: *mut net, subscr: *mut list_head, addr: u32);
    pub fn tipc_node_unsubscribe(net: *mut net, subscr: *mut list_head, addr: u32);
    pub fn tipc_node_broadcast(net: *mut net, skb: *mut sk_buff, rc_dests: ::core::ffi::c_int);
    pub fn tipc_node_add_conn(net: *mut net, dnode: u32, port: u32, peer_port: u32) -> ::core::ffi::c_int;
    pub fn tipc_node_remove_conn(net: *mut net, dnode: u32, port: u32);
    pub fn tipc_node_get_mtu(net: *mut net, addr: u32, sel: u32, connected: bool) -> ::core::ffi::c_int;
    pub fn tipc_node_is_up(net: *mut net, addr: u32) -> bool;
    pub fn tipc_node_get_capabilities(net: *mut net, addr: u32) -> u16;
    pub fn tipc_nl_node_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> ::core::ffi::c_int;
    pub fn tipc_nl_node_dump_link(skb: *mut sk_buff, cb: *mut netlink_callback) -> ::core::ffi::c_int;
    pub fn tipc_nl_node_reset_link_stats(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn tipc_nl_node_get_link(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn tipc_nl_node_set_link(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn tipc_nl_peer_rm(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn tipc_nl_node_set_monitor(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn tipc_nl_node_get_monitor(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn tipc_nl_node_dump_monitor(skb: *mut sk_buff, cb: *mut netlink_callback) -> ::core::ffi::c_int;
    pub fn tipc_nl_node_dump_monitor_peer(skb: *mut sk_buff, cb: *mut netlink_callback) -> ::core::ffi::c_int;
    #[cfg(CONFIG_TIPC_CRYPTO)]
    pub fn tipc_nl_node_set_key(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    #[cfg(CONFIG_TIPC_CRYPTO)]
    pub fn tipc_nl_node_flush_key(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn tipc_node_pre_cleanup_net(exit_net: *mut net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
