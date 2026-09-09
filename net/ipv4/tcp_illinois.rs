// SPDX-License-Identifier: GPL-2.0-only
/*
 * TCP Illinois congestion control.
 * Home page:
 *	http://www.ews.uiuc.edu/~shaoliu/tcpillinois/index.html
 *
 * The algorithm is described in:
 * "TCP-Illinois: A Loss and Delay-Based Congestion Control Algorithm
 *  for High-Speed Networks"
 * http://tamerbasar.csl.illinois.edu/LiuBasarSrikantPerfEvalArtJun2008.pdf
 *
 * Implemented from description in paper and ns-2 simulation.
 * Copyright (C) 2007 Stephen Hemminger <shemminger@linux-foundation.org>
 */

// Kernel dependencies supplied by other translation units.

const ALPHA_SHIFT: u32 = 7;
const ALPHA_SCALE: u32 = 1u32 << ALPHA_SHIFT;
const ALPHA_MIN: u32 = (3 * ALPHA_SCALE) / 10;
const ALPHA_MAX: u32 = 10 * ALPHA_SCALE;
const ALPHA_BASE: u32 = ALPHA_SCALE;
const RTT_MAX: u32 = u32::MAX / ALPHA_MAX;

const BETA_SHIFT: u32 = 6;
const BETA_SCALE: u32 = 1u32 << BETA_SHIFT;
const BETA_MIN: u32 = BETA_SCALE / 8;
const BETA_MAX: u32 = BETA_SCALE / 2;
const BETA_BASE: u32 = BETA_MAX;

static mut win_thresh: i32 = 15;
static mut theta: i32 = 5;

#[repr(C)]
struct illinois {
    sum_rtt: u64,
    cnt_rtt: u16,
    base_rtt: u32,
    max_rtt: u32,
    end_seq: u32,
    alpha: u32,
    beta: u32,
    acked: u16,
    rtt_above: u8,
    rtt_low: u8,
}

extern "C" {
    fn tcp_sk(sk: *mut sock) -> *mut tcp_sock;
    fn inet_csk_ca(sk: *mut sock) -> *mut illinois;
    fn tcp_snd_cwnd(tp: *mut tcp_sock) -> u32;
    fn tcp_snd_cwnd_set(tp: *mut tcp_sock, val: u32);
    fn tcp_is_cwnd_limited(sk: *mut sock) -> bool;
    fn tcp_in_slow_start(tp: *mut tcp_sock) -> bool;
    fn tcp_slow_start(tp: *mut tcp_sock, acked: u32);
    fn tcp_reno_undo_cwnd(sk: *mut sock) -> u32;
    fn after(seq1: u32, seq2: u32) -> bool;
}

#[repr(C)]
struct sock;
#[repr(C)]
struct tcp_sock {
    snd_nxt: u32,
    snd_cwnd_cnt: u32,
    snd_cwnd_clamp: u32,
}

#[repr(C)]
struct ack_sample {
    rtt_us: i32,
    pkts_acked: u16,
}

unsafe fn rtt_reset(sk: *mut sock) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    (*ca).end_seq = (*tp).snd_nxt;
    (*ca).cnt_rtt = 0;
    (*ca).sum_rtt = 0;
}

unsafe fn tcp_illinois_init(sk: *mut sock) {
    let ca = inet_csk_ca(sk);
    (*ca).alpha = ALPHA_MAX;
    (*ca).beta = BETA_BASE;
    (*ca).base_rtt = 0x7fffffff;
    (*ca).max_rtt = 0;
    (*ca).acked = 0;
    (*ca).rtt_low = 0;
    (*ca).rtt_above = 0;
    rtt_reset(sk);
}

unsafe fn tcp_illinois_acked(sk: *mut sock, sample: *const ack_sample) {
    let ca = inet_csk_ca(sk);
    let mut rtt_us = (*sample).rtt_us;
    (*ca).acked = (*sample).pkts_acked;
    if rtt_us < 0 { return; }
    if rtt_us as u32 > RTT_MAX { rtt_us = RTT_MAX as i32; }
    if (*ca).base_rtt > rtt_us as u32 { (*ca).base_rtt = rtt_us as u32; }
    if (*ca).max_rtt < rtt_us as u32 { (*ca).max_rtt = rtt_us as u32; }
    (*ca).cnt_rtt = (*ca).cnt_rtt.wrapping_add(1);
    (*ca).sum_rtt = (*ca).sum_rtt.wrapping_add(rtt_us as u32 as u64);
}

unsafe fn max_delay(ca: *const illinois) -> u32 { (*ca).max_rtt - (*ca).base_rtt }

unsafe fn avg_delay(ca: *const illinois) -> u32 {
    ((*ca).sum_rtt / (*ca).cnt_rtt as u64) as u32 - (*ca).base_rtt
}

unsafe fn alpha(ca: *mut illinois, mut da: u32, mut dm: u32) -> u32 {
    let d1 = dm / 100;
    if da <= d1 {
        if (*ca).rtt_above == 0 { return ALPHA_MAX; }
        (*ca).rtt_low = (*ca).rtt_low.wrapping_add(1);
        if (*ca).rtt_low < theta as u8 { return (*ca).alpha; }
        (*ca).rtt_low = 0;
        (*ca).rtt_above = 0;
        return ALPHA_MAX;
    }
    (*ca).rtt_above = 1;
    dm -= d1;
    da -= d1;
    (dm * ALPHA_MAX) / (dm + (da * (ALPHA_MAX - ALPHA_MIN)) / ALPHA_MIN)
}

fn beta(da: u32, dm: u32) -> u32 {
    let d2 = dm / 10;
    if da <= d2 { return BETA_MIN; }
    let d3 = (8 * dm) / 10;
    if da >= d3 || d3 <= d2 { return BETA_MAX; }
    (BETA_MIN * d3 - BETA_MAX * d2 + (BETA_MAX - BETA_MIN) * da) / (d3 - d2)
}

unsafe fn update_params(sk: *mut sock) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    if tcp_snd_cwnd(tp) < win_thresh as u32 {
        (*ca).alpha = ALPHA_BASE;
        (*ca).beta = BETA_BASE;
    } else if (*ca).cnt_rtt > 0 {
        let dm = max_delay(ca);
        let da = avg_delay(ca);
        (*ca).alpha = alpha(ca, da, dm);
        (*ca).beta = beta(da, dm);
    }
    rtt_reset(sk);
}

unsafe fn tcp_illinois_state(sk: *mut sock, new_state: u8) {
    let ca = inet_csk_ca(sk);
    if new_state == TCP_CA_Loss {
        (*ca).alpha = ALPHA_BASE;
        (*ca).beta = BETA_BASE;
        (*ca).rtt_low = 0;
        (*ca).rtt_above = 0;
        rtt_reset(sk);
    }
}

unsafe fn tcp_illinois_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    if after(ack, (*ca).end_seq) { update_params(sk); }
    if !tcp_is_cwnd_limited(sk) { return; }
    if tcp_in_slow_start(tp) {
        tcp_slow_start(tp, acked);
    } else {
        let mut delta: u32;
        (*tp).snd_cwnd_cnt += (*ca).acked as u32;
        (*ca).acked = 1;
        delta = ((*tp).snd_cwnd_cnt * (*ca).alpha) >> ALPHA_SHIFT;
        if delta >= tcp_snd_cwnd(tp) {
            let cwnd = tcp_snd_cwnd(tp);
            tcp_snd_cwnd_set(tp, core::cmp::min(cwnd + delta / cwnd, (*tp).snd_cwnd_clamp));
            (*tp).snd_cwnd_cnt = 0;
        }
    }
}

unsafe fn tcp_illinois_ssthresh(sk: *mut sock) -> u32 {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    let decr = (tcp_snd_cwnd(tp) * (*ca).beta) >> BETA_SHIFT;
    core::cmp::max(tcp_snd_cwnd(tp) - decr, 2)
}

const TCP_CA_Loss: u8 = 0;

#[repr(C)]
struct tcpvegas_info {
    tcpv_enabled: u32,
    tcpv_rttcnt: u32,
    tcpv_rtt: u32,
    tcpv_minrtt: u32,
}

#[repr(C)]
union tcp_cc_info {
    vegas: tcpvegas_info,
}

unsafe fn tcp_illinois_info(
    sk: *mut sock,
    ext: u32,
    attr: *mut i32,
    info: *mut tcp_cc_info,
) -> usize {
    let ca = inet_csk_ca(sk);
    if ext & (1u32 << (INET_DIAG_VEGASINFO - 1)) != 0 {
        (*info).vegas.tcpv_enabled = 1;
        (*info).vegas.tcpv_rttcnt = (*ca).cnt_rtt as u32;
        (*info).vegas.tcpv_minrtt = (*ca).base_rtt;
        (*info).vegas.tcpv_rtt = 0;
        if (*info).vegas.tcpv_rttcnt > 0 {
            (*info).vegas.tcpv_rtt =
                ((*ca).sum_rtt / (*info).vegas.tcpv_rttcnt as u64) as u32;
        }
        *attr = INET_DIAG_VEGASINFO as i32;
        return core::mem::size_of::<tcpvegas_info>();
    }
    0
}

// The kernel congestion-control registration object and module lifecycle are
// external integration points corresponding to tcp_congestion_ops/module_init.
extern "C" {
    static INET_DIAG_VEGASINFO: u32;
    fn tcp_register_congestion_control(ops: *mut tcp_congestion_ops) -> i32;
    fn tcp_unregister_congestion_control(ops: *mut tcp_congestion_ops);
}

#[repr(C)]
struct tcp_congestion_ops {
    init: Option<unsafe extern "C" fn(*mut sock)>,
    ssthresh: Option<unsafe extern "C" fn(*mut sock) -> u32>,
    undo_cwnd: Option<unsafe extern "C" fn(*mut sock) -> u32>,
    cong_avoid: Option<unsafe extern "C" fn(*mut sock, u32, u32)>,
    set_state: Option<unsafe extern "C" fn(*mut sock, u8)>,
    get_info: Option<unsafe extern "C" fn(*mut sock, u32, *mut i32, *mut tcp_cc_info) -> usize>,
    pkts_acked: Option<unsafe extern "C" fn(*mut sock, *const ack_sample)>,
}

static mut tcp_illinois: tcp_congestion_ops = tcp_congestion_ops {
    init: Some(tcp_illinois_init),
    ssthresh: Some(tcp_illinois_ssthresh),
    undo_cwnd: None,
    cong_avoid: Some(tcp_illinois_cong_avoid),
    set_state: Some(tcp_illinois_state),
    get_info: Some(tcp_illinois_info),
    pkts_acked: Some(tcp_illinois_acked),
};

unsafe fn tcp_illinois_register() -> i32 {
    tcp_register_congestion_control(&raw mut tcp_illinois)
}

unsafe fn tcp_illinois_unregister() {
    tcp_unregister_congestion_control(&raw mut tcp_illinois);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
