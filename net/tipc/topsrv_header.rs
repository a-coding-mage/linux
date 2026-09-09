/*
 * net/tipc/server.h: Include file for TIPC server code
 *
 * Copyright (c) 2012-2013, Wind River Systems
 * Copyright (c) 2017, Ericsson AB
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

// Dependency supplied by the corresponding TIPC core declarations.

pub const TIPC_SERVER_NAME_LEN: usize = 32;
pub const TIPC_SUB_CLUSTER_SCOPE: u32 = 0x20;
pub const TIPC_SUB_NODE_SCOPE: u32 = 0x40;
pub const TIPC_SUB_NO_STATUS: u32 = 0x80;

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tipc_event {
    _private: [u8; 0],
}

extern "C" {
    pub fn tipc_topsrv_queue_evt(
        net: *mut net,
        conid: ::core::ffi::c_int,
        event: u32,
        evt: *mut tipc_event,
    );

    pub fn tipc_topsrv_kern_subscr(
        net: *mut net,
        port: u32,
        type_: u32,
        lower: u32,
        upper: u32,
        filter: u32,
        conid: *mut ::core::ffi::c_int,
    ) -> bool;

    pub fn tipc_topsrv_kern_unsubscr(net: *mut net, conid: ::core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
