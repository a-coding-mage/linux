/*
 * net/tipc/bcast.h: Include file for TIPC broadcast code
 *
 * Copyright (c) 2003-2006, 2014-2015, Ericsson AB
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

// Dependency declarations from core.h are supplied by the surrounding crate.

#[repr(C)]
pub struct tipc_node;
#[repr(C)]
pub struct tipc_msg;
#[repr(C)]
pub struct tipc_nl_msg;
#[repr(C)]
pub struct net;
#[repr(C)]
pub struct tipc_link;
#[repr(C)]
pub struct nlattr;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct sk_buff_head;

extern "C" {
    pub static tipc_bclink_name: [::std::os::raw::c_char; 0];
    pub static mut sysctl_tipc_bc_retruni: ::std::os::raw::c_ulong;
}

#[macro_export]
macro_rules! TIPC_METHOD_EXPIRE {
    () => { msecs_to_jiffies(5000) };
}

pub const BCLINK_MODE_BCAST: u32 = 0x1;
pub const BCLINK_MODE_RCAST: u32 = 0x2;
pub const BCLINK_MODE_SEL: u32 = 0x4;

#[repr(C)]
pub struct tipc_nlist {
    pub list: list_head,
    pub self_: u32,
    pub remote: u16,
    pub local: bool,
}

extern "C" {
    pub fn tipc_nlist_init(nl: *mut tipc_nlist, self_: u32);
    pub fn tipc_nlist_purge(nl: *mut tipc_nlist);
    pub fn tipc_nlist_add(nl: *mut tipc_nlist, node: u32);
    pub fn tipc_nlist_del(nl: *mut tipc_nlist, node: u32);
}

/* Cookie to be used between socket and broadcast layer
 * @rcast: replicast (instead of broadcast) was used at previous xmit
 * @mandatory: broadcast/replicast indication was set by user
 * @deferredq: defer queue to make message in order
 * @expires: re-evaluate non-mandatory transmit method if we are past this
 */
#[repr(C)]
pub struct tipc_mc_method {
    pub rcast: bool,
    pub mandatory: bool,
    pub deferredq: sk_buff_head,
    pub expires: ::std::os::raw::c_ulong,
}

extern "C" {
    pub fn tipc_bcast_init(net: *mut net) -> ::std::os::raw::c_int;
    pub fn tipc_bcast_stop(net: *mut net);
    pub fn tipc_bcast_add_peer(net: *mut net, l: *mut tipc_link, xmitq: *mut sk_buff_head);
    pub fn tipc_bcast_remove_peer(net: *mut net, rcv_bcl: *mut tipc_link);
    pub fn tipc_bcast_inc_bearer_dst_cnt(net: *mut net, bearer_id: ::std::os::raw::c_int);
    pub fn tipc_bcast_dec_bearer_dst_cnt(net: *mut net, bearer_id: ::std::os::raw::c_int);
    pub fn tipc_bcast_get_mtu(net: *mut net) -> ::std::os::raw::c_int;
    pub fn tipc_bcast_toggle_rcast(net: *mut net, supp: bool);
    pub fn tipc_mcast_xmit(net: *mut net, pkts: *mut sk_buff_head,
                           method: *mut tipc_mc_method, dests: *mut tipc_nlist,
                           cong_link_cnt: *mut u16) -> ::std::os::raw::c_int;
    pub fn tipc_bcast_xmit(net: *mut net, pkts: *mut sk_buff_head,
                           cong_link_cnt: *mut u16) -> ::std::os::raw::c_int;
    pub fn tipc_bcast_rcv(net: *mut net, l: *mut tipc_link, skb: *mut sk_buff) -> ::std::os::raw::c_int;
    pub fn tipc_bcast_ack_rcv(net: *mut net, l: *mut tipc_link, hdr: *mut tipc_msg);
    pub fn tipc_bcast_sync_rcv(net: *mut net, l: *mut tipc_link, hdr: *mut tipc_msg,
                               retrq: *mut sk_buff_head, valid: *mut bool) -> ::std::os::raw::c_int;
    pub fn tipc_nl_add_bc_link(net: *mut net, msg: *mut tipc_nl_msg,
                               bcl: *mut tipc_link) -> ::std::os::raw::c_int;
    pub fn tipc_nl_bc_link_set(net: *mut net, attrs: *mut *mut nlattr) -> ::std::os::raw::c_int;
    pub fn tipc_bclink_reset_stats(net: *mut net, l: *mut tipc_link) -> ::std::os::raw::c_int;
    pub fn tipc_bcast_get_mode(net: *mut net) -> u32;
    pub fn tipc_bcast_get_broadcast_ratio(net: *mut net) -> u32;
    pub fn tipc_mcast_filter_msg(net: *mut net, defq: *mut sk_buff_head, inputq: *mut sk_buff_head);
    pub fn tipc_bcast_lock(net: *mut net);
    pub fn tipc_bcast_unlock(net: *mut net);
    pub fn tipc_bc_sndlink(net: *mut net) -> *mut tipc_link;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
