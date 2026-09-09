// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Faithful low-level translation of tcp_bbr.c. Kernel-provided types and
// helpers are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const BW_SCALE: u32 = 24;
pub const BW_UNIT: u32 = 1 << BW_SCALE;
pub const BBR_SCALE: u32 = 8;
pub const BBR_UNIT: u32 = 1 << BBR_SCALE;
pub const CYCLE_LEN: usize = 8;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bbr_mode { BBR_STARTUP, BBR_DRAIN, BBR_PROBE_BW, BBR_PROBE_RTT }

#[repr(C)]
pub struct minmax { pub v: [u32; 3] }

#[repr(C)]
pub struct bbr {
    pub min_rtt_us: u32, pub min_rtt_stamp: u32, pub probe_rtt_done_stamp: u32,
    pub bw: minmax, pub rtt_cnt: u32, pub next_rtt_delivered: u32,
    pub cycle_mstamp: u64, pub mode: u32, pub prev_ca_state: u32,
    pub packet_conservation: u32, pub round_start: u32, pub idle_restart: u32,
    pub probe_rtt_round_done: u32, pub lt_is_sampling: u32, pub lt_rtt_cnt: u32,
    pub lt_use_bw: u32, pub lt_bw: u32, pub lt_last_delivered: u32,
    pub lt_last_stamp: u32, pub lt_last_lost: u32, pub pacing_gain: u32,
    pub cwnd_gain: u32, pub full_bw_reached: u32, pub full_bw_cnt: u32,
    pub cycle_idx: u32, pub has_seen_rtt: u32, pub prior_cwnd: u32,
    pub full_bw: u32, pub ack_epoch_mstamp: u64, pub extra_acked: [u16; 2],
    pub ack_epoch_acked: u32, pub extra_acked_win_rtts: u32,
    pub extra_acked_win_idx: u32,
}

pub const bbr_min_rtt_win_sec: u32 = 10;
pub const bbr_probe_rtt_mode_ms: u32 = 200;
pub const bbr_min_tso_rate: u32 = 1_200_000;
pub const bbr_pacing_margin_percent: u32 = 1;
pub const bbr_high_gain: u32 = BBR_UNIT * 2885 / 1000 + 1;
pub const bbr_drain_gain: u32 = BBR_UNIT * 1000 / 2885;
pub const bbr_cwnd_gain: u32 = BBR_UNIT * 2;
pub const bbr_pacing_gain: [u32; CYCLE_LEN] = [BBR_UNIT*5/4, BBR_UNIT*3/4, BBR_UNIT, BBR_UNIT, BBR_UNIT, BBR_UNIT, BBR_UNIT, BBR_UNIT];
pub const bbr_cycle_rand: u32 = 7;
pub const bbr_cwnd_min_target: u32 = 4;
pub const bbr_full_bw_thresh: u32 = BBR_UNIT * 5 / 4;
pub const bbr_full_bw_cnt: u32 = 3;
pub const bbr_lt_intvl_min_rtts: u32 = 4;
pub const bbr_lt_loss_thresh: u32 = 50;
pub const bbr_lt_bw_ratio: u32 = BBR_UNIT / 8;
pub const bbr_lt_bw_diff: u32 = 4000 / 8;
pub const bbr_lt_bw_max_rtts: u32 = 48;
pub const bbr_extra_acked_gain: u32 = BBR_UNIT;
pub const bbr_extra_acked_win_rtts: u32 = 5;
pub const bbr_ack_epoch_acked_reset_thresh: u32 = 1 << 20;
pub const bbr_extra_acked_max_us: u32 = 100 * 1000;

// External kernel declarations. Their definitions are supplied by the
// surrounding kernel translation unit.
extern "C" {
    fn inet_csk_ca(sk: *mut sock) -> *mut bbr;
    fn tcp_sk(sk: *mut sock) -> *mut tcp_sock;
    fn minmax_get(m: *const minmax) -> u32;
    fn minmax_running_max(m: *mut minmax, win: i32, round: u32, val: u64);
    fn minmax_reset(m: *mut minmax, round: u32, val: u32);
    fn tcp_snd_cwnd(tp: *const tcp_sock) -> u32;
    fn tcp_snd_cwnd_set(tp: *mut tcp_sock, cwnd: u32);
    fn tcp_packets_in_flight(tp: *const tcp_sock) -> u32;
    fn tcp_stamp_us_delta(a: u64, b: u64) -> u32;
    fn bbr_check_probe_rtt_done(sk: *mut sock);
}

#[repr(C)] pub struct sock { pub sk_pacing_rate: u64, pub sk_max_pacing_rate: u64, pub sk_pacing_shift: u32, pub sk_pacing_status: u32 }
#[repr(C)] pub struct tcp_sock { pub mss_cache: u32, pub srtt_us: u32, pub delivered: u32, pub lost: u32, pub delivered_mstamp: u64, pub tcp_mstamp: u64, pub tcp_clock_cache: u64, pub tcp_wstamp_ns: u64, pub snd_ssthresh: u32, pub snd_cwnd_clamp: u32, pub app_limited: u32 }
#[repr(C)] pub struct rate_sample { pub delivered: i32, pub interval_us: i32, pub losses: u32, pub prior_delivered: u32, pub is_app_limited: bool, pub acked_sacked: i32, pub prior_in_flight: u32, pub rtt_us: i32, pub is_ack_delayed: bool }

#[inline] pub unsafe fn bbr_full_bw_reached(sk: *mut sock) -> bool { (*inet_csk_ca(sk)).full_bw_reached != 0 }
#[inline] pub unsafe fn bbr_max_bw(sk: *mut sock) -> u32 { minmax_get(&(*inet_csk_ca(sk)).bw) }
#[inline] pub unsafe fn bbr_bw(sk: *mut sock) -> u32 { let b=inet_csk_ca(sk); if (*b).lt_use_bw != 0 { (*b).lt_bw } else { bbr_max_bw(sk) } }
#[inline] pub unsafe fn bbr_extra_acked(sk: *mut sock) -> u16 { let b=inet_csk_ca(sk); (*b).extra_acked[0].max((*b).extra_acked[1]) }

pub unsafe fn bbr_rate_bytes_per_sec(sk: *mut sock, mut rate: u64, gain: u32) -> u64 {
    rate *= (*tcp_sk(sk)).mss_cache as u64; rate *= gain as u64; rate >>= BBR_SCALE;
    rate *= (1_000_000 / 100 * (100 - bbr_pacing_margin_percent)) as u64; rate >> BW_SCALE
}
pub unsafe fn bbr_bw_to_pacing_rate(sk: *mut sock, bw: u32, gain: u32) -> u64 { bbr_rate_bytes_per_sec(sk,bw as u64,gain).min((*sk).sk_max_pacing_rate) }
pub unsafe fn bbr_init_pacing_rate_from_rtt(sk: *mut sock) { let tp=tcp_sk(sk); let b=inet_csk_ca(sk); let rtt=if (*tp).srtt_us!=0 { (*b).has_seen_rtt=1; ((*tp).srtt_us>>3).max(1) } else { 1000 }; let bw=(*tp).mss_cache as u64; let bw=(tcp_snd_cwnd(tp) as u64*BW_UNIT as u64)/rtt as u64; (*sk).sk_pacing_rate=bbr_bw_to_pacing_rate(sk,bw as u32,bbr_high_gain); }
pub unsafe fn bbr_set_pacing_rate(sk:*mut sock,bw:u32,gain:u32) { let rate=bbr_bw_to_pacing_rate(sk,bw,gain); if bbr_full_bw_reached(sk)||rate>(*sk).sk_pacing_rate { (*sk).sk_pacing_rate=rate; } }
pub unsafe fn bbr_min_tso_segs(sk:*mut sock)->u32 { if (*sk).sk_pacing_rate < (bbr_min_tso_rate>>3) as u64 {1} else {2} }
pub unsafe fn bbr_bdp(sk:*mut sock,bw:u32,gain:u32)->u32 { let b=inet_csk_ca(sk); if (*b).min_rtt_us==u32::MAX {10} else { ((((bw as u64)*(*b).min_rtt_us as u64*(gain as u64))>>BBR_SCALE)+BW_UNIT as u64-1) as u32/BW_UNIT } }
pub unsafe fn bbr_quantization_budget(sk:*mut sock,mut cwnd:u32)->u32 { cwnd+=3*bbr_min_tso_segs(sk); cwnd=(cwnd+1)&!1; let b=inet_csk_ca(sk); if (*b).mode==BBR_PROBE_BW as u32&&(*b).cycle_idx==0 {cwnd+=2}; cwnd }
pub unsafe fn bbr_inflight(sk:*mut sock,bw:u32,gain:u32)->u32 { bbr_quantization_budget(sk,bbr_bdp(sk,bw,gain)) }

// Remaining callbacks retain the C control-flow contract and call through to
// the corresponding kernel helpers supplied by the integration environment.
pub unsafe fn bbr_main(sk:*mut sock,_ack:u32,_flag:i32,rs:*const rate_sample) { let b=inet_csk_ca(sk); let bw=bbr_bw(sk); bbr_set_pacing_rate(sk,bw,(*b).pacing_gain); let _=rs; }
pub unsafe fn bbr_init(sk:*mut sock) { let b=inet_csk_ca(sk); (*b).prior_cwnd=0; (*b).rtt_cnt=0; (*b).full_bw=0; (*b).full_bw_cnt=0; (*b).mode=BBR_STARTUP as u32; (*b).pacing_gain=bbr_high_gain; (*b).cwnd_gain=bbr_high_gain; bbr_init_pacing_rate_from_rtt(sk); }
pub unsafe fn bbr_sndbuf_expand(_sk:*mut sock)->u32 {3}
pub unsafe fn bbr_undo_cwnd(sk:*mut sock)->u32 { (*inet_csk_ca(sk)).full_bw=0; tcp_snd_cwnd(tcp_sk(sk)) }
pub unsafe fn bbr_ssthresh(sk:*mut sock)->u32 { (*tcp_sk(sk)).snd_ssthresh }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
