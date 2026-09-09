/*
 * net/tipc/monitor.h
 *
 * Copyright (c) 2015, Ericsson AB
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

use core::ffi::{c_int, c_void};

/* Dependency supplied by netlink.h. */
#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tipc_nl_msg {
    _private: [u8; 0],
}

/* struct tipc_mon_state: link instance's cache of monitor list and domain state
 * @list_gen: current generation of this node's monitor list
 * @gen: current generation of this node's local domain
 * @peer_gen: most recent domain generation received from peer
 * @acked_gen: most recent generation of self's domain acked by peer
 * @monitoring: this peer endpoint should continuously monitored
 * @probing: peer endpoint should be temporarily probed for potential loss
 * @synched: domain record's generation has been synched with peer after reset
 */
#[repr(C)]
pub struct tipc_mon_state {
    pub list_gen: u16,
    pub peer_gen: u16,
    pub acked_gen: u16,
    /* C bit-fields occupy one byte; each field is represented by its bit value. */
    pub monitoring: u8,
    pub probing: u8,
    pub reset: u8,
    pub synched: u8,
}

extern "C" {
    pub fn tipc_mon_create(net: *mut net, bearer_id: c_int) -> c_int;
    pub fn tipc_mon_delete(net: *mut net, bearer_id: c_int);

    pub fn tipc_mon_peer_up(net: *mut net, addr: u32, bearer_id: c_int);
    pub fn tipc_mon_peer_down(net: *mut net, addr: u32, bearer_id: c_int);
    pub fn tipc_mon_prep(
        net: *mut net,
        data: *mut c_void,
        dlen: *mut c_int,
        state: *mut tipc_mon_state,
        bearer_id: c_int,
    );
    pub fn tipc_mon_rcv(
        net: *mut net,
        data: *mut c_void,
        dlen: u16,
        addr: u32,
        state: *mut tipc_mon_state,
        bearer_id: c_int,
    );
    pub fn tipc_mon_get_state(
        net: *mut net,
        addr: u32,
        state: *mut tipc_mon_state,
        bearer_id: c_int,
    );
    pub fn tipc_mon_remove_peer(net: *mut net, addr: u32, bearer_id: c_int);

    pub fn tipc_nl_monitor_set_threshold(net: *mut net, cluster_size: u32) -> c_int;
    pub fn tipc_nl_monitor_get_threshold(net: *mut net) -> c_int;
    pub fn __tipc_nl_add_monitor(
        net: *mut net,
        msg: *mut tipc_nl_msg,
        bearer_id: u32,
    ) -> c_int;
    pub fn tipc_nl_add_monitor_peer(
        net: *mut net,
        msg: *mut tipc_nl_msg,
        bearer_id: u32,
        prev_node: *mut u32,
    ) -> c_int;
    pub fn tipc_mon_reinit_self(net: *mut net);

    pub static tipc_max_domain_size: c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
