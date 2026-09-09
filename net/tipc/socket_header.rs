/* net/tipc/socket.h: Include file for TIPC socket code
 *
 * Copyright (c) 2014-2016, Ericsson AB
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

// Dependencies supplied by the surrounding kernel translation.

/* Compatibility values for deprecated message based flow control */
pub const FLOWCTL_MSG_WIN: i32 = 512;
pub const FLOWCTL_MSG_LIM: usize =
    ((FLOWCTL_MSG_WIN * 2 + 1) as usize) * unsafe { SKB_TRUESIZE(MAX_MSG_SIZE) };

pub const FLOWCTL_BLK_SZ: i32 = 1024;

/* Socket receive buffer sizes */
pub const RCVBUF_MIN: i32 = FLOWCTL_BLK_SZ * 512;
pub const RCVBUF_DEF: i32 = FLOWCTL_BLK_SZ * 1024 * 2;
pub const RCVBUF_MAX: i32 = FLOWCTL_BLK_SZ * 1024 * 16;

#[repr(C)]
pub struct tipc_sock {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn tipc_socket_init() -> i32;
    pub fn tipc_socket_stop();
    pub fn tipc_sk_rcv(net: *mut net, inputq: *mut sk_buff_head);
    pub fn tipc_sk_mcast_rcv(
        net: *mut net,
        arrvq: *mut sk_buff_head,
        inputq: *mut sk_buff_head,
    );
    pub fn tipc_sk_reinit(net: *mut net);
    pub fn tipc_sk_rht_init(net: *mut net) -> i32;
    pub fn tipc_sk_rht_destroy(net: *mut net);
    pub fn tipc_nl_sk_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn tipc_nl_publ_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn tipc_sk_fill_sock_diag(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
        tsk: *mut tipc_sock,
        sk_filter_state: u32,
        tipc_diag_gen_cookie: Option<unsafe extern "C" fn(*mut sock) -> u64>,
    ) -> i32;
    pub fn tipc_nl_sk_walk(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
        skb_handler: Option<unsafe extern "C" fn(
            *mut sk_buff,
            *mut netlink_callback,
            *mut tipc_sock,
        ) -> i32>,
    ) -> i32;
    pub fn tipc_dump_start(cb: *mut netlink_callback) -> i32;
    pub fn __tipc_dump_start(cb: *mut netlink_callback, net: *mut net) -> i32;
    pub fn tipc_dump_done(cb: *mut netlink_callback) -> i32;
    pub fn tipc_sock_get_portid(sk: *mut sock) -> u32;
    pub fn tipc_sk_overlimit1(sk: *mut sock, skb: *mut sk_buff) -> bool;
    pub fn tipc_sk_overlimit2(sk: *mut sock, skb: *mut sk_buff) -> bool;
    pub fn tipc_sk_bind(sock: *mut socket, skaddr: *mut sockaddr, alen: i32) -> i32;
    pub fn tsk_set_importance(sk: *mut sock, imp: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
