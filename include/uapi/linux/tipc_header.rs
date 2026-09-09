/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * include/uapi/linux/tipc.h: Header for TIPC socket interface
 *
 * Copyright (c) 2003-2006, 2015-2016 Ericsson AB
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

#[repr(C)]
pub struct tipc_socket_addr { pub ref_: u32, pub node: u32 }

#[repr(C)]
pub struct tipc_service_addr { pub type_: u32, pub instance: u32 }

#[repr(C)]
pub struct tipc_service_range { pub type_: u32, pub lower: u32, pub upper: u32 }

pub const TIPC_NODE_STATE: u32 = 0;
pub const TIPC_TOP_SRV: u32 = 1;
pub const TIPC_LINK_STATE: u32 = 2;
pub const TIPC_RESERVED_TYPES: u32 = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tipc_scope { TIPC_CLUSTER_SCOPE = 2, TIPC_NODE_SCOPE = 3 }

pub const TIPC_MAX_USER_MSG_SIZE: u32 = 66000;
pub const TIPC_LOW_IMPORTANCE: u32 = 0;
pub const TIPC_MEDIUM_IMPORTANCE: u32 = 1;
pub const TIPC_HIGH_IMPORTANCE: u32 = 2;
pub const TIPC_CRITICAL_IMPORTANCE: u32 = 3;
pub const TIPC_OK: u32 = 0;
pub const TIPC_ERR_NO_NAME: u32 = 1;
pub const TIPC_ERR_NO_PORT: u32 = 2;
pub const TIPC_ERR_NO_NODE: u32 = 3;
pub const TIPC_ERR_OVERLOAD: u32 = 4;
pub const TIPC_CONN_SHUTDOWN: u32 = 5;
pub const TIPC_SUB_PORTS: u32 = 0x01;
pub const TIPC_SUB_SERVICE: u32 = 0x02;
pub const TIPC_SUB_CANCEL: u32 = 0x04;
pub const TIPC_WAIT_FOREVER: u32 = !0;

#[repr(C)]
pub struct tipc_subscr { pub seq: tipc_service_range, pub timeout: u32, pub filter: u32, pub usr_handle: [i8; 8] }
pub const TIPC_PUBLISHED: u32 = 1;
pub const TIPC_WITHDRAWN: u32 = 2;
pub const TIPC_SUBSCR_TIMEOUT: u32 = 3;

#[repr(C)]
pub struct tipc_event { pub event: u32, pub found_lower: u32, pub found_upper: u32, pub port: tipc_socket_addr, pub s: tipc_subscr }

pub const AF_TIPC: u16 = 30;
pub const PF_TIPC: u16 = AF_TIPC;
pub const SOL_TIPC: u32 = 271;
pub const TIPC_ADDR_MCAST: u32 = 1;
pub const TIPC_SERVICE_RANGE: u32 = 1;
pub const TIPC_SERVICE_ADDR: u32 = 2;
pub const TIPC_SOCKET_ADDR: u32 = 3;

#[repr(C)]
pub union sockaddr_tipc_addr {
    pub id: tipc_socket_addr,
    pub nameseq: tipc_service_range,
    pub name: sockaddr_tipc_name,
}
#[repr(C)]
pub struct sockaddr_tipc_name { pub name: tipc_service_addr, pub domain: u32 }
#[repr(C)]
pub struct sockaddr_tipc { pub family: u16, pub addrtype: u8, pub scope: i8, pub addr: sockaddr_tipc_addr }

pub const TIPC_ERRINFO: u32 = 1;
pub const TIPC_RETDATA: u32 = 2;
pub const TIPC_DESTNAME: u32 = 3;
pub const TIPC_IMPORTANCE: u32 = 127;
pub const TIPC_SRC_DROPPABLE: u32 = 128;
pub const TIPC_DEST_DROPPABLE: u32 = 129;
pub const TIPC_CONN_TIMEOUT: u32 = 130;
pub const TIPC_NODE_RECVQ_DEPTH: u32 = 131;
pub const TIPC_SOCK_RECVQ_DEPTH: u32 = 132;
pub const TIPC_MCAST_BROADCAST: u32 = 133;
pub const TIPC_MCAST_REPLICAST: u32 = 134;
pub const TIPC_GROUP_JOIN: u32 = 135;
pub const TIPC_GROUP_LEAVE: u32 = 136;
pub const TIPC_SOCK_RECVQ_USED: u32 = 137;
pub const TIPC_NODELAY: u32 = 138;
pub const TIPC_GROUP_LOOPBACK: u32 = 0x1;
pub const TIPC_GROUP_MEMBER_EVTS: u32 = 0x2;

#[repr(C)]
pub struct tipc_group_req { pub type_: u32, pub instance: u32, pub scope: u32, pub flags: u32 }

pub const TIPC_NODEID_LEN: usize = 16;
pub const TIPC_MAX_MEDIA_NAME: usize = 16;
pub const TIPC_MAX_IF_NAME: usize = 16;
pub const TIPC_MAX_BEARER_NAME: usize = 32;
pub const TIPC_MAX_LINK_NAME: usize = 68;
// SIOCPROTOPRIVATE is supplied by the corresponding socket header dependency.
pub const SIOCGETLINKNAME: u32 = SIOCPROTOPRIVATE;
pub const SIOCGETNODEID: u32 = SIOCPROTOPRIVATE + 1;

#[repr(C)]
pub struct tipc_sioc_ln_req { pub peer: u32, pub bearer_id: u32, pub linkname: [i8; TIPC_MAX_LINK_NAME] }
#[repr(C)]
pub struct tipc_sioc_nodeid_req { pub peer: u32, pub node_id: [i8; TIPC_NODEID_LEN] }

pub const TIPC_AEAD_ALG_NAME: usize = 32;
#[repr(C)]
pub struct tipc_aead_key { pub alg_name: [i8; TIPC_AEAD_ALG_NAME], pub keylen: u32, pub key: [i8; 0] }
pub const TIPC_AEAD_KEYLEN_MIN: usize = 16 + 4;
pub const TIPC_AEAD_KEYLEN_MAX: usize = 32 + 4;
pub const TIPC_AEAD_KEY_SIZE_MAX: usize = core::mem::size_of::<tipc_aead_key>() + TIPC_AEAD_KEYLEN_MAX;

#[inline]
pub unsafe fn tipc_aead_key_size(key: *mut tipc_aead_key) -> usize {
    core::mem::size_of::<tipc_aead_key>() + (*key).keylen as usize
}

pub const TIPC_REKEYING_NOW: u32 = !0;

// The following macros and functions are deprecated.
pub const TIPC_CFG_SRV: u32 = 0;
pub const TIPC_ZONE_SCOPE: u32 = 1;
pub const TIPC_ADDR_NAMESEQ: u32 = 1;
pub const TIPC_ADDR_NAME: u32 = 2;
pub const TIPC_ADDR_ID: u32 = 3;
pub const TIPC_NODE_BITS: u32 = 12;
pub const TIPC_CLUSTER_BITS: u32 = 12;
pub const TIPC_ZONE_BITS: u32 = 8;
pub const TIPC_NODE_OFFSET: u32 = 0;
pub const TIPC_CLUSTER_OFFSET: u32 = TIPC_NODE_BITS;
pub const TIPC_ZONE_OFFSET: u32 = TIPC_CLUSTER_OFFSET + TIPC_CLUSTER_BITS;
pub const TIPC_NODE_SIZE: u32 = (1u32 << TIPC_NODE_BITS) - 1;
pub const TIPC_CLUSTER_SIZE: u32 = (1u32 << TIPC_CLUSTER_BITS) - 1;
pub const TIPC_ZONE_SIZE: u32 = (1u32 << TIPC_ZONE_BITS) - 1;
pub const TIPC_NODE_MASK: u32 = TIPC_NODE_SIZE << TIPC_NODE_OFFSET;
pub const TIPC_CLUSTER_MASK: u32 = TIPC_CLUSTER_SIZE << TIPC_CLUSTER_OFFSET;
pub const TIPC_ZONE_MASK: u32 = TIPC_ZONE_SIZE << TIPC_ZONE_OFFSET;
pub const TIPC_ZONE_CLUSTER_MASK: u32 = TIPC_ZONE_MASK | TIPC_CLUSTER_MASK;

pub type tipc_portid = tipc_socket_addr;
pub type tipc_name = tipc_service_addr;
pub type tipc_name_seq = tipc_service_range;

#[inline]
pub fn tipc_addr(zone: u32, cluster: u32, node: u32) -> u32 {
    (zone << TIPC_ZONE_OFFSET) | (cluster << TIPC_CLUSTER_OFFSET) | node
}
#[inline]
pub fn tipc_zone(addr: u32) -> u32 { addr >> TIPC_ZONE_OFFSET }
#[inline]
pub fn tipc_cluster(addr: u32) -> u32 { (addr & TIPC_CLUSTER_MASK) >> TIPC_CLUSTER_OFFSET }
#[inline]
pub fn tipc_node(addr: u32) -> u32 { addr & TIPC_NODE_MASK }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
