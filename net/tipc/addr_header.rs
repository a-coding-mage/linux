/*
 * net/tipc/addr.h: Include file for TIPC address utility routines
 *
 * Copyright (c) 2000-2006, 2018, Ericsson AB
 * Copyright (c) 2004-2005, Wind River Systems
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

/* C includes and header guard omitted; their supplied declarations remain external dependencies. */

/* Struct tipc_uaddr: internal version of struct sockaddr_tipc.
 * Must be kept aligned both regarding field positions and size.
 */
#[repr(C)]
pub struct tipc_uaddr {
    pub family: u16,
    pub addrtype: u8,
    pub scope: i8,
    pub union_data: tipc_uaddr_union,
}

#[repr(C)]
pub union tipc_uaddr_union {
    pub sa: tipc_uaddr_sa,
    pub sr: tipc_service_range,
    pub sk: tipc_socket_addr,
}

#[repr(C)]
pub struct tipc_uaddr_sa {
    pub sa: tipc_service_addr,
    pub lookup_node: u32,
}

#[inline]
pub unsafe fn tipc_uaddr(ua: *mut tipc_uaddr, atype: u32, scope: u32,
                         type_: u32, lower: u32, upper: u32) {
    (*ua).family = AF_TIPC as u16;
    (*ua).addrtype = atype as u8;
    (*ua).scope = scope as i8;
    (*ua).union_data.sr.type_ = type_;
    (*ua).union_data.sr.lower = lower;
    (*ua).union_data.sr.upper = upper;
}

#[inline]
pub unsafe fn tipc_uaddr_valid(ua: *mut tipc_uaddr, len: i32) -> bool {
    let atype: u32;
    if len < core::mem::size_of::<sockaddr_tipc>() as i32 {
        return false;
    }
    atype = (*ua).addrtype as u32;
    if (*ua).family as i32 != AF_TIPC {
        return false;
    }
    if atype == TIPC_SERVICE_ADDR || atype == TIPC_SOCKET_ADDR {
        return true;
    }
    if atype == TIPC_SERVICE_RANGE {
        return (*ua).union_data.sr.upper >= (*ua).union_data.sr.lower;
    }
    false
}

#[inline]
pub unsafe fn tipc_own_addr(net: *mut net) -> u32 {
    (*tipc_net(net)).node_addr
}

#[inline]
pub unsafe fn tipc_own_id(net: *mut net) -> *mut u8 {
    let tn: *mut tipc_net = tipc_net(net);
    if strlen((*tn).node_id_string) == 0 {
        return core::ptr::null_mut();
    }
    (*tn).node_id
}

#[inline]
pub unsafe fn tipc_own_id_string(net: *mut net) -> *mut i8 {
    (*tipc_net(net)).node_id_string
}

#[inline]
pub fn tipc_cluster_mask(addr: u32) -> u32 {
    addr & TIPC_ZONE_CLUSTER_MASK
}

#[inline]
pub fn tipc_node2scope(node: u32) -> i32 {
    if node != 0 { TIPC_NODE_SCOPE } else { TIPC_CLUSTER_SCOPE }
}

#[inline]
pub unsafe fn tipc_scope2node(net: *mut net, sc: i32) -> i32 {
    if sc != TIPC_NODE_SCOPE { 0 } else { tipc_own_addr(net) as i32 }
}

#[inline]
pub unsafe fn in_own_node(net: *mut net, addr: u32) -> i32 {
    if addr == tipc_own_addr(net) || addr == 0 { 1 } else { 0 }
}

extern "C" {
    pub fn tipc_in_scope(legacy_format: bool, domain: u32, addr: u32) -> bool;
    pub fn tipc_set_node_id(net: *mut net, id: *mut u8);
    pub fn tipc_set_node_addr(net: *mut net, addr: u32);
    pub fn tipc_nodeid2string(str_: *mut i8, id: *mut u8) -> i32;
    pub fn strlen(s: *const i8) -> usize;
    pub fn tipc_net(net: *mut net) -> *mut tipc_net;
}

/* External declarations supplied by the translated dependency headers. */
extern "C" {
    pub static AF_TIPC: i32;
    pub static TIPC_SERVICE_ADDR: u32;
    pub static TIPC_SOCKET_ADDR: u32;
    pub static TIPC_SERVICE_RANGE: u32;
    pub static TIPC_ZONE_CLUSTER_MASK: u32;
    pub static TIPC_NODE_SCOPE: i32;
    pub static TIPC_CLUSTER_SCOPE: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
