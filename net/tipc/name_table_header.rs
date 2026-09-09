/*
 * net/tipc/name_table.h: Include file for TIPC name table code
 *
 * Copyright (c) 2000-2006, 2014-2018, Ericsson AB
 * Copyright (c) 2004-2005, 2010-2011, Wind River Systems
 * Copyright (c) 2020-2021, Red Hat Inc
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

pub struct tipc_subscription;
pub struct tipc_plist;
pub struct tipc_nlist;
pub struct tipc_group;
pub struct tipc_uaddr;

pub const TIPC_ZM_SRV: i32 = 3; /* zone master service name type */
pub const TIPC_PUBL_SCOPE_NUM: i32 = TIPC_NODE_SCOPE + 1;
pub const TIPC_NAMETBL_SIZE: usize = 1024; /* must be a power of 2 */
pub const TIPC_ANY_SCOPE: i32 = 10; /* Both node and cluster scope will match */

/* struct publication - info about a published service address or range */
#[repr(C)]
pub struct publication {
    pub sr: tipc_service_range,
    pub sk: tipc_socket_addr,
    pub scope: u16,
    pub key: u32,
    pub id: u32,
    pub binding_node: list_head,
    pub binding_sock: list_head,
    pub local_publ: list_head,
    pub all_publ: list_head,
    pub list: list_head,
    pub rcu: rcu_head,
}

/* struct name_table - table containing all existing port name publications */
#[repr(C)]
pub struct name_table {
    pub rcu: rcu_head,
    pub services: [hlist_head; TIPC_NAMETBL_SIZE],
    pub node_scope: list_head,
    pub cluster_scope: list_head,
    pub cluster_scope_lock: rwlock_t,
    pub local_publ_count: u32,
    pub rc_dests: u32,
    pub snd_nxt: u32,
}

pub unsafe extern "C" fn tipc_nl_name_table_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
pub unsafe extern "C" fn tipc_nametbl_lookup_anycast(net: *mut net, ua: *mut tipc_uaddr, sk: *mut tipc_socket_addr) -> bool;
pub unsafe extern "C" fn tipc_nametbl_lookup_mcast_sockets(net: *mut net, ua: *mut tipc_uaddr, dports: *mut list_head);
pub unsafe extern "C" fn tipc_nametbl_lookup_mcast_nodes(net: *mut net, ua: *mut tipc_uaddr, nodes: *mut tipc_nlist);
pub unsafe extern "C" fn tipc_nametbl_lookup_group(net: *mut net, ua: *mut tipc_uaddr, dsts: *mut list_head, dstcnt: *mut i32, exclude: u32, mcast: bool) -> bool;
pub unsafe extern "C" fn tipc_nametbl_build_group(net: *mut net, grp: *mut tipc_group, ua: *mut tipc_uaddr);
pub unsafe extern "C" fn tipc_nametbl_publish(net: *mut net, ua: *mut tipc_uaddr, sk: *mut tipc_socket_addr, key: u32) -> *mut publication;
pub unsafe extern "C" fn tipc_nametbl_withdraw(net: *mut net, ua: *mut tipc_uaddr, sk: *mut tipc_socket_addr, key: u32);
pub unsafe extern "C" fn tipc_nametbl_insert_publ(net: *mut net, ua: *mut tipc_uaddr, sk: *mut tipc_socket_addr, key: u32) -> *mut publication;
pub unsafe extern "C" fn tipc_nametbl_remove_publ(net: *mut net, ua: *mut tipc_uaddr, sk: *mut tipc_socket_addr, key: u32) -> *mut publication;
pub unsafe extern "C" fn tipc_nametbl_subscribe(s: *mut tipc_subscription) -> bool;
pub unsafe extern "C" fn tipc_nametbl_unsubscribe(s: *mut tipc_subscription);
pub unsafe extern "C" fn tipc_nametbl_init(net: *mut net) -> i32;
pub unsafe extern "C" fn tipc_nametbl_stop(net: *mut net);

#[repr(C)]
pub struct tipc_dest {
    pub list: list_head,
    pub port: u32,
    pub node: u32,
}

pub unsafe extern "C" fn tipc_dest_find(l: *mut list_head, node: u32, port: u32) -> *mut tipc_dest;
pub unsafe extern "C" fn tipc_dest_push(l: *mut list_head, node: u32, port: u32) -> bool;
pub unsafe extern "C" fn tipc_dest_pop(l: *mut list_head, node: *mut u32, port: *mut u32) -> bool;
pub unsafe extern "C" fn tipc_dest_del(l: *mut list_head, node: u32, port: u32) -> bool;
pub unsafe extern "C" fn tipc_dest_list_purge(l: *mut list_head);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
