/*
 * net/tipc/ib_media.c: Infiniband bearer support for TIPC
 *
 * Copyright (c) 2013 Patrick McHardy <kaber@trash.net>
 *
 * Based on eth_media.c, which carries the following copyright notice:
 *
 * Copyright (c) 2001-2007, Ericsson AB
 * Copyright (c) 2005-2008, 2011, Wind River Systems
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

// Dependencies are supplied by the corresponding TIPC/kernel translation units.

const TIPC_MAX_IB_LINK_WIN: i32 = 500;

/* convert InfiniBand address (media address format) media address to string */
unsafe extern "C" fn tipc_ib_addr2str(
    a: *mut tipc_media_addr,
    str_buf: *mut core::ffi::c_char,
    str_size: i32,
) -> i32 {
    if str_size < 60 { /* 60 = 19 * strlen("xx:") + strlen("xx\0") */
        return 1;
    }

    sprintf(str_buf, c"%20phC".as_ptr(), (*a).value.as_ptr());
    0
}

/* Convert from media address format to discovery message addr format */
unsafe extern "C" fn tipc_ib_addr2msg(
    msg: *mut core::ffi::c_char,
    addr: *mut tipc_media_addr,
) -> i32 {
    memset(msg.cast(), 0, TIPC_MEDIA_INFO_SIZE as usize);
    memcpy(
        msg.cast(),
        (*addr).value.as_ptr().cast(),
        INFINIBAND_ALEN as usize,
    );
    0
}

/* Convert raw InfiniBand address format to media addr format */
unsafe extern "C" fn tipc_ib_raw2addr(
    b: *mut tipc_bearer,
    addr: *mut tipc_media_addr,
    msg: *const core::ffi::c_char,
) -> i32 {
    memset(addr.cast(), 0, core::mem::size_of::<tipc_media_addr>());
    memcpy((*addr).value.as_mut_ptr().cast(), msg.cast(), INFINIBAND_ALEN as usize);
    (*addr).media_id = TIPC_MEDIA_TYPE_IB;
    (*addr).broadcast = (memcmp(
        msg.cast(),
        (*b).bcast_addr.value.as_ptr().cast(),
        INFINIBAND_ALEN as usize,
    ) == 0) as _;
    0
}

/* Convert discovery msg addr format to InfiniBand media addr format */
unsafe extern "C" fn tipc_ib_msg2addr(
    b: *mut tipc_bearer,
    addr: *mut tipc_media_addr,
    msg: *mut core::ffi::c_char,
) -> i32 {
    tipc_ib_raw2addr(b, addr, msg)
}

/* InfiniBand media registration info */
pub static mut ib_media_info: tipc_media = tipc_media {
    send_msg: tipc_l2_send_msg,
    enable_media: tipc_enable_l2_media,
    disable_media: tipc_disable_l2_media,
    addr2str: tipc_ib_addr2str,
    addr2msg: tipc_ib_addr2msg,
    msg2addr: tipc_ib_msg2addr,
    raw2addr: tipc_ib_raw2addr,
    priority: TIPC_DEF_LINK_PRI,
    tolerance: TIPC_DEF_LINK_TOL,
    min_win: TIPC_DEF_LINK_WIN,
    max_win: TIPC_MAX_IB_LINK_WIN,
    type_id: TIPC_MEDIA_TYPE_IB,
    hwaddr_len: INFINIBAND_ALEN,
    name: *b"ib\0",
};

extern "C" {
    fn sprintf(
        s: *mut core::ffi::c_char,
        format: *const core::ffi::c_char,
        ...,
    ) -> i32;
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
    fn memcmp(
        s1: *const core::ffi::c_void,
        s2: *const core::ffi::c_void,
        n: usize,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
