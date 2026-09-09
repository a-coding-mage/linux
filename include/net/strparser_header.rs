/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Stream Parser
 *
 * Copyright (c) 2016 Tom Herbert <tom@herbertland.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
pub unsafe fn STRP_STATS_ADD<T>(stat: *mut T, count: T)
where
    T: core::ops::AddAssign,
{
    (*stat) += count;
}

#[inline]
pub unsafe fn STRP_STATS_INCR<T>(stat: *mut T)
where
    T: core::ops::AddAssign + From<u8>,
{
    (*stat) += T::from(1);
}

#[repr(C)]
pub struct strp_stats {
    pub msgs: u64,
    pub bytes: u64,
    pub mem_fail: u32,
    pub need_more_hdr: u32,
    pub msg_too_big: u32,
    pub msg_timeouts: u32,
    pub bad_hdr_len: u32,
}

#[repr(C)]
pub struct strp_aggr_stats {
    pub msgs: u64,
    pub bytes: u64,
    pub mem_fail: u32,
    pub need_more_hdr: u32,
    pub msg_too_big: u32,
    pub msg_timeouts: u32,
    pub bad_hdr_len: u32,
    pub aborts: u32,
    pub interrupted: u32,
    pub unrecov_intr: u32,
}

#[repr(C)]
pub struct strparser {
    _private: [u8; 0],
}

// Callbacks are called with lock held for the attached socket.
#[repr(C)]
pub struct strp_callbacks {
    pub parse_msg: Option<unsafe extern "C" fn(*mut strparser, *mut sk_buff) -> i32>,
    pub rcv_msg: Option<unsafe extern "C" fn(*mut strparser, *mut sk_buff)>,
    pub read_sock: Option<unsafe extern "C" fn(*mut strparser, *mut read_descriptor_t, sk_read_actor_t) -> i32>,
    pub read_sock_done: Option<unsafe extern "C" fn(*mut strparser, i32) -> i32>,
    pub abort_parser: Option<unsafe extern "C" fn(*mut strparser, i32)>,
    pub lock: Option<unsafe extern "C" fn(*mut strparser)>,
    pub unlock: Option<unsafe extern "C" fn(*mut strparser)>,
}

#[repr(C)]
pub struct strp_msg {
    pub full_len: i32,
    pub offset: i32,
}

#[repr(C)]
pub struct _strp_msg {
    // struct strp_msg must be first for passing to the upper layer.
    pub strp: strp_msg,
    pub accum_len: i32,
}

#[repr(C)]
pub struct tls_msg {
    pub control: u8,
}

#[repr(C)]
pub struct sk_skb_cb {
    pub data: [u8; 20],
    // Align strp on cache line boundary within skb->cb[].
    pub pad: [u8; 4],
    pub strp: _strp_msg,
    // strp users' data follows.
    pub tls: tls_msg,
    // Temporary register used by bpf_convert_data_end_access when dst_reg == src_reg.
    pub temp_reg: u64,
}

#[inline]
pub unsafe fn strp_msg(skb: *mut sk_buff) -> *mut strp_msg {
    ((&mut (*skb).cb as *mut _ as *mut u8).add(core::mem::offset_of!(sk_skb_cb, strp))) as *mut strp_msg
}

// Structure for an attached lower socket.
#[repr(C)]
pub struct strparser_state {
    pub sk: *mut sock,
    pub stopped: u32,
    pub paused: u32,
    pub aborted: u32,
    pub interrupted: u32,
    pub unrecov_intr: u32,
    pub skb_nextp: *mut *mut sk_buff,
    pub skb_head: *mut sk_buff,
    pub need_bytes: u32,
    pub msg_timer_work: delayed_work,
    pub work: work_struct,
    pub stats: strp_stats,
    pub cb: strp_callbacks,
}

#[inline]
pub unsafe fn strp_pause(strp: *mut strparser_state) {
    (*strp).paused = 1;
}

pub unsafe extern "C" fn strp_unpause(strp: *mut strparser_state);

#[inline]
pub unsafe fn save_strp_stats(strp: *mut strparser_state, agg_stats: *mut strp_aggr_stats) {
    (*agg_stats).msgs += (*strp).stats.msgs;
    (*agg_stats).bytes += (*strp).stats.bytes;
    (*agg_stats).mem_fail += (*strp).stats.mem_fail;
    (*agg_stats).need_more_hdr += (*strp).stats.need_more_hdr;
    (*agg_stats).msg_too_big += (*strp).stats.msg_too_big;
    (*agg_stats).msg_timeouts += (*strp).stats.msg_timeouts;
    (*agg_stats).bad_hdr_len += (*strp).stats.bad_hdr_len;
    if (*strp).aborted != 0 { (*agg_stats).aborts += 1; }
    if (*strp).interrupted != 0 { (*agg_stats).interrupted += 1; }
    if (*strp).unrecov_intr != 0 { (*agg_stats).unrecov_intr += 1; }
}

#[inline]
pub unsafe fn aggregate_strp_stats(stats: *mut strp_aggr_stats, agg_stats: *mut strp_aggr_stats) {
    (*agg_stats).msgs += (*stats).msgs;
    (*agg_stats).bytes += (*stats).bytes;
    (*agg_stats).mem_fail += (*stats).mem_fail;
    (*agg_stats).need_more_hdr += (*stats).need_more_hdr;
    (*agg_stats).msg_too_big += (*stats).msg_too_big;
    (*agg_stats).msg_timeouts += (*stats).msg_timeouts;
    (*agg_stats).bad_hdr_len += (*stats).bad_hdr_len;
    (*agg_stats).aborts += (*stats).aborts;
    (*agg_stats).interrupted += (*stats).interrupted;
    (*agg_stats).unrecov_intr += (*stats).unrecov_intr;
}

pub unsafe extern "C" fn strp_done(strp: *mut strparser_state);
pub unsafe extern "C" fn strp_stop(strp: *mut strparser_state);
pub unsafe extern "C" fn strp_check_rcv(strp: *mut strparser_state);
pub unsafe extern "C" fn strp_init(strp: *mut strparser_state, sk: *mut sock, cb: *const strp_callbacks) -> i32;
pub unsafe extern "C" fn strp_data_ready(strp: *mut strparser_state);
pub unsafe extern "C" fn strp_process(strp: *mut strparser_state, orig_skb: *mut sk_buff,
    orig_offset: u32, orig_len: usize, max_msg_size: usize, timeo: i64) -> i32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
