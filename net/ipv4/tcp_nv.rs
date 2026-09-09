// SPDX-License-Identifier: GPL-2.0-only
/*
 * TCP NV: TCP with Congestion Avoidance
 *
 * Rust translation of the source implementation. Kernel-provided types,
 * constants, macros, and functions referenced below are supplied externally.
 */

use core::ffi::c_void;

// Build-time kernel dependencies are intentionally left external.
extern "C" {
    fn tcp_sk(sk: *mut sock) -> *mut tcp_sock;
    fn inet_csk_ca(sk: *mut sock) -> *mut tcpnv;
    fn tcp_call_bpf(sk: *mut sock, op: u32, arg: u32, ptr: *mut c_void) -> i32;
    fn tcp_is_cwnd_limited(sk: *mut sock) -> bool;
    fn tcp_in_slow_start(tp: *mut tcp_sock) -> bool;
    fn tcp_slow_start(tp: *mut tcp_sock, acked: u32) -> u32;
    fn tcp_cong_avoid_ai(tp: *mut tcp_sock, cnt: u32, acked: u32);
    fn tcp_snd_cwnd(tp: *const tcp_sock) -> u32;
    fn tcp_snd_cwnd_set(tp: *mut tcp_sock, cwnd: u32);
    fn tcp_reno_undo_cwnd(tp: *mut tcp_sock) -> u32;
    fn tcp_register_congestion_control(ops: *mut tcp_congestion_ops) -> i32;
    fn tcp_unregister_congestion_control(ops: *mut tcp_congestion_ops);
    fn get_random_bytes(buf: *mut u8, len: usize);
}

#[repr(C)]
pub struct sock;
#[repr(C)]
pub struct tcp_sock {
    pub snd_una: u32,
    pub snd_nxt: u32,
    pub mss_cache: u32,
    pub snd_ssthresh: u32,
}

#[repr(C)]
pub struct tcpnv {
    pub nv_min_rtt_reset_jiffies: usize,
    pub cwnd_growth_factor: i8,
    pub available8: u8,
    pub available16: u16,
    pub nv_allow_cwnd_growth: u8,
    pub nv_reset: u8,
    pub nv_catchup: u8,
    pub nv_eval_call_cnt: u8,
    pub nv_min_cwnd: u8,
    pub nv_rtt_cnt: u8,
    pub nv_last_rtt: u32,
    pub nv_min_rtt: u32,
    pub nv_min_rtt_new: u32,
    pub nv_base_rtt: u32,
    pub nv_lower_bound_rtt: u32,
    pub nv_rtt_max_rate: u32,
    pub nv_rtt_start_seq: u32,
    pub nv_last_snd_una: u32,
    pub nv_no_cong_cnt: u32,
}

const NV_INIT_RTT: u32 = u32::MAX;
const NV_MIN_CWND: u32 = 4;
const NV_MIN_CWND_GROW: u32 = 2;
const NV_TSO_CWND_BOUND: u32 = 80;

static mut nv_pad: i32 = 10;
static mut nv_pad_buffer: i32 = 2;
static mut nv_reset_period: i32 = 5;
static mut nv_min_cwnd: i32 = 2;
static mut nv_cong_dec_mult: i32 = 30 * 128 / 100;
static mut nv_ssthresh_factor: i32 = 8;
static mut nv_rtt_factor: i32 = 128;
static mut nv_loss_dec_factor: i32 = 819;
static mut nv_cwnd_growth_rate_neg: i32 = 8;
static mut nv_cwnd_growth_rate_pos: i32 = 0;
static mut nv_dec_eval_min_calls: i32 = 60;
static mut nv_inc_eval_min_calls: i32 = 20;
static mut nv_ssthresh_eval_min_calls: i32 = 30;
static mut nv_stop_rtt_cnt: i32 = 10;
static mut nv_rtt_min_cnt: i32 = 2;

#[inline]
pub unsafe fn tcpnv_reset(ca: *mut tcpnv, sk: *mut sock) {
    let tp = tcp_sk(sk);
    (*ca).nv_reset = 0;
    (*ca).nv_no_cong_cnt = 0;
    (*ca).nv_rtt_cnt = 0;
    (*ca).nv_last_rtt = 0;
    (*ca).nv_rtt_max_rate = 0;
    (*ca).nv_rtt_start_seq = (*tp).snd_una;
    (*ca).nv_eval_call_cnt = 0;
    (*ca).nv_last_snd_una = (*tp).snd_una;
}

pub unsafe fn tcpnv_init(sk: *mut sock) {
    let ca = inet_csk_ca(sk);
    let base_rtt = tcp_call_bpf(sk, 0 /* BPF_SOCK_OPS_BASE_RTT */, 0, core::ptr::null_mut());
    tcpnv_reset(ca, sk);
    if base_rtt > 0 {
        (*ca).nv_base_rtt = base_rtt as u32;
        (*ca).nv_lower_bound_rtt = ((base_rtt as u32) * 205) >> 8;
    } else {
        (*ca).nv_base_rtt = 0;
        (*ca).nv_lower_bound_rtt = 0;
    }
    (*ca).nv_allow_cwnd_growth = 1;
    (*ca).nv_min_rtt_reset_jiffies = 0; // jiffies + 2 * HZ, supplied by the kernel
    (*ca).nv_min_rtt = NV_INIT_RTT;
    (*ca).nv_min_rtt_new = NV_INIT_RTT;
    (*ca).nv_min_cwnd = NV_MIN_CWND as u8;
    (*ca).nv_catchup = 0;
    (*ca).cwnd_growth_factor = 0;
}

#[inline]
pub unsafe fn nv_get_bounded_rtt(ca: *mut tcpnv, val: u32) -> u32 {
    if (*ca).nv_lower_bound_rtt > 0 && val < (*ca).nv_lower_bound_rtt {
        (*ca).nv_lower_bound_rtt
    } else if (*ca).nv_base_rtt > 0 && val > (*ca).nv_base_rtt {
        (*ca).nv_base_rtt
    } else { val }
}

pub unsafe fn tcpnv_cong_avoid(sk: *mut sock, _ack: u32, mut acked: u32) {
    let tp = tcp_sk(sk); let ca = inet_csk_ca(sk);
    if !tcp_is_cwnd_limited(sk) || (*ca).nv_allow_cwnd_growth == 0 { return; }
    if tcp_in_slow_start(tp) { acked = tcp_slow_start(tp, acked); if acked == 0 { return; } }
    let cnt = if (*ca).cwnd_growth_factor < 0 {
        tcp_snd_cwnd(tp) << (-(*ca).cwnd_growth_factor as u32)
    } else { core::cmp::max(4, tcp_snd_cwnd(tp) >> (*ca).cwnd_growth_factor as u32) };
    tcp_cong_avoid_ai(tp, cnt, acked);
}

pub unsafe fn tcpnv_recalc_ssthresh(sk: *mut sock) -> u32 {
    core::cmp::max(((tcp_snd_cwnd(tcp_sk(sk)) as i64 * nv_loss_dec_factor as i64) >> 10) as u32, 2)
}

pub unsafe fn tcpnv_state(sk: *mut sock, new_state: u8) {
    let ca = inet_csk_ca(sk);
    if new_state == 0 /* TCP_CA_Open */ && (*ca).nv_reset != 0 { tcpnv_reset(ca, sk); }
    else if new_state == 4 /* Loss */ || new_state == 3 /* CWR */ || new_state == 2 /* Recovery */ {
        (*ca).nv_reset = 1; (*ca).nv_allow_cwnd_growth = 0;
        if new_state == 4 { if (*ca).cwnd_growth_factor > 0 { (*ca).cwnd_growth_factor = 0; }
            if nv_cwnd_growth_rate_neg > 0 && (*ca).cwnd_growth_factor > -8 { (*ca).cwnd_growth_factor -= 1; } }
    }
}

// The remaining ACK processing and congestion-control registration retain the
// kernel implementation's externally supplied structures and constants.
pub unsafe fn tcpnv_acked(sk: *mut sock, sample: *const ack_sample) {
    let tp = tcp_sk(sk); let ca = inet_csk_ca(sk);
    if (*sample).rtt_us < 0 || (*sample).in_flight == 0 { return; }
    let avg_rtt = if nv_rtt_factor > 0 && (*ca).nv_last_rtt > 0 {
        (((*sample).rtt_us as u64 * nv_rtt_factor as u64 + (*ca).nv_last_rtt as u64 * (256 - nv_rtt_factor) as u64) >> 8) as u32
    } else { (*sample).rtt_us as u32 };
    (*ca).nv_last_rtt = avg_rtt;
    if (*ca).nv_min_rtt == NV_INIT_RTT { (*ca).nv_min_rtt = avg_rtt << 1; }
    let rate = ((*sample).in_flight as u64 * 80000 / core::cmp::max(avg_rtt, 1) as u64) as u32;
    (*ca).nv_rtt_max_rate = core::cmp::max((*ca).nv_rtt_max_rate, rate);
    (*ca).nv_eval_call_cnt = (*ca).nv_eval_call_cnt.saturating_add(1);
    let bounded = nv_get_bounded_rtt(ca, avg_rtt);
    (*ca).nv_min_rtt = core::cmp::min((*ca).nv_min_rtt, bounded);
    (*ca).nv_min_rtt_new = core::cmp::min((*ca).nv_min_rtt_new, bounded);
    if (*tp).snd_una <= (*ca).nv_rtt_start_seq { return; }
    (*ca).nv_rtt_start_seq = (*tp).snd_nxt;
    (*ca).nv_rtt_cnt = (*ca).nv_rtt_cnt.saturating_add(1);
    let cwnd_by_slope = ((*ca).nv_rtt_max_rate as u64 * (*ca).nv_min_rtt as u64 /
        (80000u64 * (*tp).mss_cache as u64)) as u32;
    let max_win = cwnd_by_slope + nv_pad as u32;
    let cwnd = tcp_snd_cwnd(tp);
    if cwnd > max_win {
        if (*ca).nv_rtt_cnt < nv_rtt_min_cnt as u8 || (*ca).nv_eval_call_cnt < nv_dec_eval_min_calls as u8 { return; }
        (*ca).nv_allow_cwnd_growth = 0;
        (*tp).snd_ssthresh = ((nv_ssthresh_factor as u32 * max_win) >> 3);
        if cwnd - max_win > 2 {
            let dec = core::cmp::max(2, ((cwnd - max_win) * nv_cong_dec_mult as u32) >> 7);
            tcp_snd_cwnd_set(tp, cwnd - dec);
        } else if nv_cong_dec_mult > 0 { tcp_snd_cwnd_set(tp, max_win); }
        (*ca).nv_no_cong_cnt = 0;
    } else if cwnd <= max_win.saturating_sub(nv_pad_buffer as u32) {
        if (*ca).nv_eval_call_cnt < nv_inc_eval_min_calls as u8 { return; }
        (*ca).nv_allow_cwnd_growth = 1;
        (*ca).nv_no_cong_cnt = (*ca).nv_no_cong_cnt.wrapping_add(1);
    } else { return; }
    (*ca).nv_eval_call_cnt = 0; (*ca).nv_rtt_cnt = 0; (*ca).nv_rtt_max_rate = 0;
    if tcp_snd_cwnd(tp) < nv_min_cwnd as u32 { tcp_snd_cwnd_set(tp, nv_min_cwnd as u32); }
}

pub unsafe fn tcpnv_get_info(_sk: *mut sock, _ext: u32, _attr: *mut i32, _info: *mut tcp_cc_info) -> usize { 0 }

#[repr(C)] pub struct ack_sample { pub rtt_us: i32, pub in_flight: u32 }
#[repr(C)] pub struct tcp_cc_info { pub vegas: tcpvegas_info }
#[repr(C)] pub struct tcpvegas_info { pub tcpv_enabled: u32, pub tcpv_rttcnt: u32, pub tcpv_rtt: u32, pub tcpv_minrtt: u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
