/*
 * net/tipc/net.h: Include file for TIPC network routing code
 *
 * Copyright (c) 1995-2006, 2014, Ericsson AB
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

// C dependency: <net/genetlink.h>

// Opaque declarations supplied by the surrounding kernel interfaces.
pub struct nla_policy;
pub struct net;
pub struct work_struct;
pub struct sk_buff;
pub struct netlink_callback;
pub struct genl_info;

extern "C" {
    pub static tipc_nl_net_policy: [nla_policy; 0];

    pub fn tipc_net_init(net: *mut net, node_id: *mut u8, addr: u32) -> i32;
    pub fn tipc_net_finalize_work(work: *mut work_struct);
    pub fn tipc_net_stop(net: *mut net);
    pub fn tipc_nl_net_dump(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> i32;
    pub fn tipc_nl_net_set(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn __tipc_nl_net_set(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn tipc_nl_net_addr_legacy_get(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
