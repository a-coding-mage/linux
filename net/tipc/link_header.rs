/*
 * net/tipc/link.h: Include file for TIPC link code
 *
 * Copyright (c) 1995-2006, 2013-2014, Ericsson AB
 * Copyright (c) 2004-2005, 2010-2011, Wind River Systems
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

use core::ffi::{c_char, c_int, c_ulong};

/* Types supplied by the included TIPC and networking headers. */
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct tipc_link { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff_head { _private: [u8; 0] }
#[repr(C)] pub struct tipc_msg { _private: [u8; 0] }
#[repr(C)] pub struct tipc_nl_msg { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct tipc_gap_ack_blks { _private: [u8; 0] }

/* TIPC-specific error codes: link congestion <=> resource unavailable. */
pub const ELINKCONG: c_int = EAGAIN;

/* Link FSM events. */
pub const LINK_ESTABLISH_EVT: c_int = 0x0ec1ab1e;
pub const LINK_PEER_RESET_EVT: c_int = 0x009eed0e;
pub const LINK_FAILURE_EVT: c_int = 0x00fa110e;
pub const LINK_RESET_EVT: c_int = 0x10ca1d0e;
pub const LINK_FAILOVER_BEGIN_EVT: c_int = 0xfa110bee;
pub const LINK_FAILOVER_END_EVT: c_int = 0xfa110ede;
pub const LINK_SYNCH_BEGIN_EVT: c_int = 0x0c1ccbee;
pub const LINK_SYNCH_END_EVT: c_int = 0x0c1ccede;

/* Events returned from link at packet reception or at timeout. */
pub const TIPC_LINK_UP_EVT: c_int = 1;
pub const TIPC_LINK_DOWN_EVT: c_int = 1 << 1;
pub const TIPC_LINK_SND_STATE: c_int = 1 << 2;

/* Starting value for maximum packet size negotiation on unicast links
 * (unless bearer MTU is less).
 */
pub const MAX_PKT_DEFAULT: c_int = 1500;

extern "C" {
    pub fn tipc_link_create(net: *mut net, if_name: *mut c_char, bearer_id: c_int,
        tolerance: c_int, net_plane: c_char, mtu: u32, priority: c_int,
        min_win: u32, max_win: u32, session: u32, ownnode: u32, peer: u32,
        peer_id: *mut u8, peer_caps: u16, bc_sndlink: *mut tipc_link,
        bc_rcvlink: *mut tipc_link, inputq: *mut sk_buff_head,
        namedq: *mut sk_buff_head, link: *mut *mut tipc_link) -> bool;
    pub fn tipc_link_bc_create(net: *mut net, ownnode: u32, peer: u32,
        peer_id: *mut u8, mtu: c_int, min_win: u32, max_win: u32,
        peer_caps: u16, inputq: *mut sk_buff_head, namedq: *mut sk_buff_head,
        bc_sndlink: *mut tipc_link, link: *mut *mut tipc_link) -> bool;
    pub fn tipc_link_tnl_prepare(l: *mut tipc_link, tnl: *mut tipc_link,
        mtyp: c_int, xmitq: *mut sk_buff_head);
    pub fn tipc_link_create_dummy_tnl_msg(tnl: *mut tipc_link, xmitq: *mut sk_buff_head);
    pub fn tipc_link_failover_prepare(l: *mut tipc_link, tnl: *mut tipc_link,
        xmitq: *mut sk_buff_head);
    pub fn tipc_link_build_reset_msg(l: *mut tipc_link, xmitq: *mut sk_buff_head);
    pub fn tipc_link_fsm_evt(l: *mut tipc_link, evt: c_int) -> c_int;
    pub fn tipc_link_is_up(l: *mut tipc_link) -> bool;
    pub fn tipc_link_peer_is_down(l: *mut tipc_link) -> bool;
    pub fn tipc_link_is_reset(l: *mut tipc_link) -> bool;
    pub fn tipc_link_is_establishing(l: *mut tipc_link) -> bool;
    pub fn tipc_link_is_synching(l: *mut tipc_link) -> bool;
    pub fn tipc_link_is_failingover(l: *mut tipc_link) -> bool;
    pub fn tipc_link_is_blocked(l: *mut tipc_link) -> bool;
    pub fn tipc_link_set_active(l: *mut tipc_link, active: bool);
    pub fn tipc_link_reset(l: *mut tipc_link);
    pub fn tipc_link_reset_stats(l: *mut tipc_link);
    pub fn tipc_link_xmit(link: *mut tipc_link, list: *mut sk_buff_head,
        xmitq: *mut sk_buff_head) -> c_int;
    pub fn tipc_link_inputq(l: *mut tipc_link) -> *mut sk_buff_head;
    pub fn tipc_link_rcv_nxt(l: *mut tipc_link) -> u16;
    pub fn tipc_link_acked(l: *mut tipc_link) -> u16;
    pub fn tipc_link_id(l: *mut tipc_link) -> u32;
    pub fn tipc_link_name(l: *mut tipc_link) -> *mut c_char;
    pub fn tipc_link_state(l: *mut tipc_link) -> u32;
    pub fn tipc_link_plane(l: *mut tipc_link) -> c_char;
    pub fn tipc_link_prio(l: *mut tipc_link) -> c_int;
    pub fn tipc_link_min_win(l: *mut tipc_link) -> c_int;
    pub fn tipc_link_max_win(l: *mut tipc_link) -> c_int;
    pub fn tipc_link_update_caps(l: *mut tipc_link, capabilities: u16);
    pub fn tipc_link_validate_msg(l: *mut tipc_link, hdr: *mut tipc_msg) -> bool;
    pub fn tipc_link_tolerance(l: *mut tipc_link) -> c_ulong;
    pub fn tipc_link_set_tolerance(l: *mut tipc_link, tol: u32, xmitq: *mut sk_buff_head);
    pub fn tipc_link_set_prio(l: *mut tipc_link, prio: u32, xmitq: *mut sk_buff_head);
    pub fn tipc_link_set_abort_limit(l: *mut tipc_link, limit: u32);
    pub fn tipc_link_set_queue_limits(l: *mut tipc_link, min_win: u32, max_win: u32);
    pub fn __tipc_nl_add_link(net: *mut net, msg: *mut tipc_nl_msg,
        link: *mut tipc_link, nlflags: c_int) -> c_int;
    pub fn tipc_nl_parse_link_prop(prop: *mut nlattr, props: *mut *mut nlattr) -> c_int;
    pub fn tipc_link_timeout(l: *mut tipc_link, xmitq: *mut sk_buff_head) -> c_int;
    pub fn tipc_link_rcv(l: *mut tipc_link, skb: *mut sk_buff,
        xmitq: *mut sk_buff_head) -> c_int;
    pub fn tipc_link_build_state_msg(l: *mut tipc_link, xmitq: *mut sk_buff_head) -> c_int;
    pub fn tipc_link_add_bc_peer(snd_l: *mut tipc_link, uc_l: *mut tipc_link,
        xmitq: *mut sk_buff_head);
    pub fn tipc_link_remove_bc_peer(snd_l: *mut tipc_link, rcv_l: *mut tipc_link,
        xmitq: *mut sk_buff_head);
    pub fn tipc_link_bc_peers(l: *mut tipc_link) -> c_int;
    pub fn tipc_link_set_mtu(l: *mut tipc_link, mtu: c_int);
    pub fn tipc_link_mtu(l: *mut tipc_link) -> c_int;
    pub fn tipc_link_mss(l: *mut tipc_link) -> c_int;
    pub fn tipc_get_gap_ack_blks(ga: *mut *mut tipc_gap_ack_blks, l: *mut tipc_link,
        hdr: *mut tipc_msg, uc: bool) -> u16;
    pub fn tipc_link_bc_ack_rcv(l: *mut tipc_link, acked: u16, gap: u16,
        ga: *mut tipc_gap_ack_blks, xmitq: *mut sk_buff_head,
        retrq: *mut sk_buff_head) -> c_int;
    pub fn tipc_link_bc_init_rcv(l: *mut tipc_link, hdr: *mut tipc_msg);
    pub fn tipc_link_bc_sync_rcv(l: *mut tipc_link, hdr: *mut tipc_msg,
        xmitq: *mut sk_buff_head) -> c_int;
    pub fn tipc_link_bc_nack_rcv(l: *mut tipc_link, skb: *mut sk_buff,
        xmitq: *mut sk_buff_head) -> c_int;
    pub fn tipc_link_too_silent(l: *mut tipc_link) -> bool;
    pub fn tipc_link_net(l: *mut tipc_link) -> *mut net;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
