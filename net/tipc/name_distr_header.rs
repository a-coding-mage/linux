/*
 * net/tipc/name_distr.h: Include file for TIPC name distribution code
 *
 * Copyright (c) 2000-2006, Ericsson AB
 * Copyright (c) 2005, Wind River Systems
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

// Dependency supplied by the corresponding name_table translation.

pub const ITEM_SIZE: usize = core::mem::size_of::<DistrItem>();

/// struct distr_item - publication info distributed to other nodes
///
/// All fields are stored in network byte order.
///
/// The first 3 fields identify the name or name sequence being published.
/// The reference field uniquely identifies the port that published the name
/// sequence. The key field uniquely identifies the publication, in the event
/// a port has multiple publications of the same name sequence.
///
/// There is no field that identifies the publishing node because it is the
/// same for all items contained within a publication message.
#[repr(C)]
pub struct DistrItem {
    pub type_: u32,
    pub lower: u32,
    pub upper: u32,
    pub port: u32,
    pub key: u32,
}

extern "C" {
    pub fn tipc_named_publish(net: *mut Net, publ: *mut Publication) -> *mut SkBuff;
    pub fn tipc_named_withdraw(net: *mut Net, publ: *mut Publication) -> *mut SkBuff;
    pub fn tipc_named_node_up(net: *mut Net, dnode: u32, capabilities: u16);
    pub fn tipc_named_rcv(
        net: *mut Net,
        namedq: *mut SkBuffHead,
        rcv_nxt: *mut u16,
        open: *mut bool,
    );
    pub fn tipc_named_reinit(net: *mut Net);
    pub fn tipc_publ_notify(
        net: *mut Net,
        nsub_list: *mut ListHead,
        addr: u32,
        capabilities: u16,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
