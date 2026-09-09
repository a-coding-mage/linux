/*
 * net/tipc/group.h: Include file for TIPC group unicast/multicast functions
 *
 * Copyright (c) 2017, Ericsson AB
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
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

// Dependency intent: declarations from "core.h" are supplied by other files.

#[repr(C)]
pub struct tipc_group {
    _private: [u8; 0],
}
#[repr(C)]
pub struct tipc_member {
    _private: [u8; 0],
}
#[repr(C)]
pub struct tipc_msg {
    _private: [u8; 0],
}
#[repr(C)]
pub struct net {
    _private: [u8; 0],
}
#[repr(C)]
pub struct tipc_group_req {
    _private: [u8; 0],
}
#[repr(C)]
pub struct tipc_nlist {
    _private: [u8; 0],
}
#[repr(C)]
pub struct tipc_service_range {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sk_buff_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

extern "C" {
    pub fn tipc_group_create(
        net: *mut net,
        portid: u32,
        mreq: *mut tipc_group_req,
        group_is_open: *mut bool,
    ) -> *mut tipc_group;
    pub fn tipc_group_join(net: *mut net, grp: *mut tipc_group, sk_rcv_buf: *mut i32);
    pub fn tipc_group_delete(net: *mut net, grp: *mut tipc_group);
    pub fn tipc_group_add_member(grp: *mut tipc_group, node: u32, port: u32, instance: u32);
    pub fn tipc_group_dests(grp: *mut tipc_group) -> *mut tipc_nlist;
    pub fn tipc_group_self(
        grp: *mut tipc_group,
        seq: *mut tipc_service_range,
        scope: *mut i32,
    );
    pub fn tipc_group_exclude(grp: *mut tipc_group) -> u32;
    pub fn tipc_group_filter_msg(
        grp: *mut tipc_group,
        inputq: *mut sk_buff_head,
        xmitq: *mut sk_buff_head,
    );
    pub fn tipc_group_member_evt(
        grp: *mut tipc_group,
        wakeup: *mut bool,
        sk_rcvbuf: *mut i32,
        hdr: *mut tipc_msg,
        inputq: *mut sk_buff_head,
        xmitq: *mut sk_buff_head,
    );
    pub fn tipc_group_proto_rcv(
        grp: *mut tipc_group,
        wakeup: *mut bool,
        hdr: *mut tipc_msg,
        inputq: *mut sk_buff_head,
        xmitq: *mut sk_buff_head,
    );
    pub fn tipc_group_update_bc_members(grp: *mut tipc_group, len: i32, ack: bool);
    pub fn tipc_group_cong(
        grp: *mut tipc_group,
        dnode: u32,
        dport: u32,
        len: i32,
        m: *mut *mut tipc_member,
    ) -> bool;
    pub fn tipc_group_bc_cong(grp: *mut tipc_group, len: i32) -> bool;
    pub fn tipc_group_update_rcv_win(
        grp: *mut tipc_group,
        blks: i32,
        node: u32,
        port: u32,
        xmitq: *mut sk_buff_head,
    );
    pub fn tipc_group_bc_snd_nxt(grp: *mut tipc_group) -> u16;
    pub fn tipc_group_update_member(m: *mut tipc_member, len: i32);
    pub fn tipc_group_fill_sock_diag(grp: *mut tipc_group, skb: *mut sk_buff) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
