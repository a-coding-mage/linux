/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Kernel Connection Multiplexor
 *
 * Copyright (c) 2016 Tom Herbert <tom@herbertland.com>
 */

// Translated from net/kcm.h. Types supplied by the included kernel headers are
// intentionally referenced here rather than redefined.

unsafe extern "C" {
    pub static mut kcm_net_id: core::ffi::c_uint;
}

#[inline]
pub unsafe fn KCM_STATS_ADD<T>(stat: *mut T, count: T)
where
    T: core::ops::AddAssign,
{
    (*stat) += count;
}

#[inline]
pub unsafe fn KCM_STATS_INCR<T>(stat: *mut T)
where
    T: core::ops::AddAssign + From<u8>,
{
    (*stat) += T::from(1);
}

#[repr(C)]
pub struct kcm_psock_stats {
    pub tx_msgs: u64,
    pub tx_bytes: u64,
    pub reserved: u64,
    pub unreserved: u64,
    pub tx_aborts: u32,
}

#[repr(C)]
pub struct kcm_mux_stats {
    pub rx_msgs: u64,
    pub rx_bytes: u64,
    pub tx_msgs: u64,
    pub tx_bytes: u64,
    pub rx_ready_drops: u32,
    pub tx_retries: u32,
    pub psock_attach: u32,
    pub psock_unattach_rsvd: u32,
    pub psock_unattach: u32,
}

#[repr(C)]
pub struct kcm_stats {
    pub rx_msgs: u64,
    pub rx_bytes: u64,
    pub tx_msgs: u64,
    pub tx_bytes: u64,
}

#[repr(C)]
pub struct kcm_tx_msg {
    pub sent: u32,
    pub frag_offset: u32,
    pub msg_flags: u32,
    pub started_tx: bool,
    pub frag_skb: *mut sk_buff,
    pub last_skb: *mut sk_buff,
}

#[repr(C)]
pub struct kcm_sock {
    pub sk: sock,
    pub mux: *mut kcm_mux,
    pub kcm_sock_list: list_head,
    pub index: core::ffi::c_int,
    pub done: u32, // C bit-field: done : 1
    pub done_work: work_struct,
    pub stats: kcm_stats,
    pub tx_psock: *mut kcm_psock,
    pub tx_work: work_struct,
    pub wait_psock_list: list_head,
    pub seq_skb: *mut sk_buff,
    pub tx_mutex: mutex,
    pub tx_wait: bool,
    pub tx_wait_more: bool,
    pub rx_psock: *mut kcm_psock,
    pub wait_rx_list: list_head,
    pub rx_wait: bool,
    pub rx_disabled: u32, // C bit-field: rx_disabled : 1
}

pub struct bpf_prog;

#[repr(C)]
pub struct kcm_psock {
    pub sk: *mut sock,
    pub strp: strparser,
    pub mux: *mut kcm_mux,
    pub index: core::ffi::c_int,
    pub tx_stopped: u32, // C bit-field: tx_stopped : 1
    pub done: u32, // C bit-field: done : 1
    pub unattaching: u32, // C bit-field: unattaching : 1
    pub save_state_change: Option<unsafe extern "C" fn(*mut sock)>,
    pub save_data_ready: Option<unsafe extern "C" fn(*mut sock)>,
    pub save_write_space: Option<unsafe extern "C" fn(*mut sock)>,
    pub psock_list: list_head,
    pub stats: kcm_psock_stats,
    pub psock_ready_list: list_head,
    pub bpf_prog: *mut bpf_prog,
    pub rx_kcm: *mut kcm_sock,
    pub saved_rx_bytes: u64,
    pub saved_rx_msgs: u64,
    pub ready_rx_msg: *mut sk_buff,
    pub tx_kcm: *mut kcm_sock,
    pub psock_avail_list: list_head,
    pub saved_tx_bytes: u64,
    pub saved_tx_msgs: u64,
}

#[repr(C)]
pub struct kcm_net {
    pub mutex: mutex,
    pub aggregate_psock_stats: kcm_psock_stats,
    pub aggregate_mux_stats: kcm_mux_stats,
    pub aggregate_strp_stats: strp_aggr_stats,
    pub mux_list: list_head,
    pub count: core::ffi::c_int,
}

#[repr(C)]
pub struct kcm_mux {
    pub kcm_mux_list: list_head,
    pub rcu: rcu_head,
    pub knet: *mut kcm_net,
    pub kcm_socks: list_head,
    pub kcm_socks_cnt: core::ffi::c_int,
    pub psocks: list_head,
    pub psocks_cnt: core::ffi::c_int,
    pub stats: kcm_mux_stats,
    pub aggregate_psock_stats: kcm_psock_stats,
    pub aggregate_strp_stats: strp_aggr_stats,
    pub rx_lock: spinlock_t,
    pub kcm_rx_waiters: list_head,
    pub psocks_ready: list_head,
    pub rx_hold_queue: sk_buff_head,
    pub lock: spinlock_t,
    pub psocks_avail: list_head,
    pub kcm_tx_waiters: list_head,
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe extern "C" {
    pub fn kcm_proc_init() -> core::ffi::c_int;
    pub fn kcm_proc_exit();
}

#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub fn kcm_proc_init() -> core::ffi::c_int { 0 }

#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub fn kcm_proc_exit() {}

#[inline]
pub unsafe fn aggregate_psock_stats(
    stats: *mut kcm_psock_stats,
    agg_stats: *mut kcm_psock_stats,
) {
    (*agg_stats).tx_msgs += (*stats).tx_msgs;
    (*agg_stats).tx_bytes += (*stats).tx_bytes;
    (*agg_stats).reserved += (*stats).reserved;
    (*agg_stats).unreserved += (*stats).unreserved;
    (*agg_stats).tx_aborts += (*stats).tx_aborts;
}

#[inline]
pub unsafe fn aggregate_mux_stats(
    stats: *mut kcm_mux_stats,
    agg_stats: *mut kcm_mux_stats,
) {
    (*agg_stats).rx_msgs += (*stats).rx_msgs;
    (*agg_stats).rx_bytes += (*stats).rx_bytes;
    (*agg_stats).tx_msgs += (*stats).tx_msgs;
    (*agg_stats).tx_bytes += (*stats).tx_bytes;
    (*agg_stats).rx_ready_drops += (*stats).rx_ready_drops;
    (*agg_stats).psock_attach += (*stats).psock_attach;
    (*agg_stats).psock_unattach_rsvd += (*stats).psock_unattach_rsvd;
    (*agg_stats).psock_unattach += (*stats).psock_unattach;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
