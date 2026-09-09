/*
 * net/tipc/addr.c: TIPC address utility routines
 *
 * Copyright (c) 2000-2006, 2018, Ericsson AB
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

// Dependencies supplied by addr.h and core.h are intentionally external.

pub unsafe fn tipc_in_scope(legacy_format: bool, domain: u32, addr: u32) -> bool {
    if domain == 0 || domain == addr {
        return true;
    }
    if !legacy_format {
        return false;
    }
    if domain == tipc_cluster_mask(addr) { /* domain <Z.C.0> */
        return true;
    }
    if domain == (addr & TIPC_ZONE_CLUSTER_MASK) { /* domain <Z.C.0> */
        return true;
    }
    if domain == (addr & TIPC_ZONE_MASK) { /* domain <Z.0.0> */
        return true;
    }
    false
}

pub unsafe fn tipc_set_node_id(net: *mut net, id: *mut u8) {
    let tn: *mut tipc_net = tipc_net(net);

    memcpy((*tn).node_id.as_mut_ptr() as *mut core::ffi::c_void,
           id as *const core::ffi::c_void, NODE_ID_LEN);
    tipc_nodeid2string((*tn).node_id_string.as_mut_ptr() as *mut i8, id);
    (*tn).trial_addr = hash128to32(id);
    pr_info!("Node identity %s, cluster identity %u\n",
             tipc_own_id_string(net), (*tn).net_id);
}

pub unsafe fn tipc_set_node_addr(net: *mut net, addr: u32) {
    let tn: *mut tipc_net = tipc_net(net);
    let mut node_id: [u8; NODE_ID_LEN] = [0; NODE_ID_LEN];

    (*tn).node_addr = addr;
    if !tipc_own_id(net) {
        sprintf(node_id.as_mut_ptr() as *mut i8, "%x\0".as_ptr() as *const i8, addr);
        tipc_set_node_id(net, node_id.as_mut_ptr());
    }
    (*tn).trial_addr = addr;
    (*tn).addr_trial_end = jiffies;
    pr_info!("Node number set to %u\n", addr);
}

pub unsafe fn tipc_nodeid2string(str_: *mut i8, id: *mut u8) -> i32 {
    let mut i: i32;
    let mut c: u8;

    /* Already a string ? */
    i = 0;
    while i < NODE_ID_LEN as i32 {
        c = *id.offset(i as isize);
        if (c >= b'0' && c <= b'9') || (c >= b'A' && c <= b'Z') ||
           (c >= b'a' && c <= b'z') || c == b'.' || c == b':' ||
           c == b'_' || c == b'-' || c == b'@' {
            i += 1;
            continue;
        }
        if c != 0 {
            break;
        }
        i += 1;
    }
    if i == NODE_ID_LEN as i32 {
        memcpy(str_ as *mut core::ffi::c_void, id as *const core::ffi::c_void, NODE_ID_LEN);
        *str_.offset(NODE_ID_LEN as isize) = 0;
        return i;
    }

    /* Translate to hex string */
    i = 0;
    while i < NODE_ID_LEN as i32 {
        sprintf(str_.offset((2 * i) as isize), "%02x\0".as_ptr() as *const i8,
                *id.offset(i as isize));
        i += 1;
    }

    /* Strip off trailing zeroes */
    i = NODE_ID_STR_LEN as i32 - 2;
    while *str_.offset(i as isize) == b'0' as i8 {
        *str_.offset(i as isize) = 0;
        i -= 1;
    }

    i + 1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
