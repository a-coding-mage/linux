/*
 * net/tipc/eth_media.c: Ethernet bearer support for TIPC
 *
 * Copyright (c) 2001-2007, 2013-2014, Ericsson AB
 * Copyright (c) 2005-2008, 2011-2013, Wind River Systems
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

/* C dependencies from core.h and bearer.h are supplied externally. */

unsafe extern "C" {
    fn sprintf(format: *mut ::std::ffi::c_char, ...) -> ::std::ffi::c_int;
    fn memset(s: *mut ::std::ffi::c_void, c: ::std::ffi::c_int, n: usize) -> *mut ::std::ffi::c_void;
    fn memcpy(dest: *mut ::std::ffi::c_void, src: *const ::std::ffi::c_void, n: usize) -> *mut ::std::ffi::c_void;
    fn ether_addr_copy(dst: *mut u8, src: *const ::std::ffi::c_char);
    fn is_broadcast_ether_addr(addr: *const u8) -> bool;
    fn tipc_l2_send_msg();
    fn tipc_enable_l2_media();
    fn tipc_disable_l2_media();
}

/* Convert Ethernet address (media address format) to string */
unsafe fn tipc_eth_addr2str(
    addr: *mut tipc_media_addr,
    strbuf: *mut ::std::ffi::c_char,
    bufsz: ::std::ffi::c_int,
) -> ::std::ffi::c_int {
    if bufsz < 18 {
        return 1;
    }

    sprintf(strbuf, b"%pM\0".as_ptr() as *mut ::std::ffi::c_char, (*addr).value);
    0
}

/* Convert from media address format to discovery message addr format */
unsafe fn tipc_eth_addr2msg(msg: *mut ::std::ffi::c_char, addr: *mut tipc_media_addr) -> ::std::ffi::c_int {
    memset(msg as *mut ::std::ffi::c_void, 0, TIPC_MEDIA_INFO_SIZE as usize);
    *msg.add(TIPC_MEDIA_TYPE_OFFSET as usize) = TIPC_MEDIA_TYPE_ETH as ::std::ffi::c_char;
    memcpy(
        msg.add(TIPC_MEDIA_ADDR_OFFSET as usize) as *mut ::std::ffi::c_void,
        (*addr).value as *const ::std::ffi::c_void,
        ETH_ALEN as usize,
    );
    0
}

/* Convert raw mac address format to media addr format */
unsafe fn tipc_eth_raw2addr(
    _b: *mut tipc_bearer,
    addr: *mut tipc_media_addr,
    msg: *const ::std::ffi::c_char,
) -> ::std::ffi::c_int {
    memset(addr as *mut ::std::ffi::c_void, 0, ::std::mem::size_of::<tipc_media_addr>());
    ether_addr_copy((*addr).value, msg);
    (*addr).media_id = TIPC_MEDIA_TYPE_ETH;
    (*addr).broadcast = is_broadcast_ether_addr((*addr).value);
    0
}

/* Convert discovery msg addr format to Ethernet media addr format */
unsafe fn tipc_eth_msg2addr(
    b: *mut tipc_bearer,
    addr: *mut tipc_media_addr,
    msg: *mut ::std::ffi::c_char,
) -> ::std::ffi::c_int {
    /* Skip past preamble: */
    msg = msg.add(TIPC_MEDIA_ADDR_OFFSET as usize);
    tipc_eth_raw2addr(b, addr, msg)
}

/* Ethernet media registration info */
static mut eth_media_info: tipc_media = tipc_media {
    send_msg: tipc_l2_send_msg,
    enable_media: tipc_enable_l2_media,
    disable_media: tipc_disable_l2_media,
    addr2str: tipc_eth_addr2str,
    addr2msg: tipc_eth_addr2msg,
    msg2addr: tipc_eth_msg2addr,
    raw2addr: tipc_eth_raw2addr,
    priority: TIPC_DEF_LINK_PRI,
    tolerance: TIPC_DEF_LINK_TOL,
    min_win: TIPC_DEF_LINK_WIN,
    max_win: TIPC_MAX_LINK_WIN,
    type_id: TIPC_MEDIA_TYPE_ETH,
    hwaddr_len: ETH_ALEN,
    name: *b"eth\0",
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
