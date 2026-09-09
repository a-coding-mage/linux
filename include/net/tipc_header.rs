/*
 * include/net/tipc.h: Include file for TIPC message header routines
 *
 * Copyright (c) 2017 Ericsson AB
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

// Dependency supplied by the surrounding translation unit: Linux random API.

pub const KEEPALIVE_MSG_MASK: u32 = 0x0e080000; /* LINK_PROTOCOL + MSG_IS_KEEPALIVE */

#[repr(C)]
pub struct tipc_basic_hdr {
    pub w: [u32; 4],
}

unsafe extern "C" {
    fn get_random_bytes(buf: *mut core::ffi::c_void, len: usize);
}

pub unsafe fn tipc_hdr_rps_key(hdr: *mut tipc_basic_hdr) -> u32 {
    let w0: u32 = u32::from_be((*hdr).w[0]);
    let keepalive_msg: bool = (w0 & KEEPALIVE_MSG_MASK) == KEEPALIVE_MSG_MASK;
    let mut key: u32;

    /* Return source node identity as key */
    if !keepalive_msg {
        return (*hdr).w[3];
    }

    /* Spread PROBE/PROBE_REPLY messages across the cores */
    key = 0;
    get_random_bytes(
        (&mut key as *mut u32).cast::<core::ffi::c_void>(),
        core::mem::size_of::<u32>(),
    );
    key
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
