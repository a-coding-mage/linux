// SPDX-License-Identifier: GPL-2.0-only
/*
 * TCP Low Priority (TCP-LP)
 *
 * TCP Low Priority is a distributed algorithm whose goal is to utilize only
 *   the excess network bandwidth as compared to the ``fair share`` of
 *   bandwidth as targeted by TCP.
 *
 * As of 2.6.13, Linux supports pluggable congestion control algorithms.
 * Due to the limitation of the API, we take the following changes from the
 * original TCP-LP implementation:
 *   o We use newReno in most core CA handling. Only add some checking
 *     within cong_avoid.
 *   o Error correcting in remote HZ, therefore remote HZ will be keeped
 *     on checking and updating.
 *   o Handling calculation of One-Way-Delay (OWD) within rtt_sample, since
 *     OWD have a similar meaning as RTT. Also correct the buggy formular.
 *   o Handle reaction for Early Congestion Indication (ECI) within
 *     pkts_acked, as mentioned within pseudo code.
 *   o OWD is handled in relative format, where local time stamp will in
 *     tcp_time_stamp format.
 *
 * Original Author:
 *   Aleksandar Kuzmanovic <akuzma@northwestern.edu>
 * Available from:
 *   https://users.cs.northwestern.edu/~akuzma/doc/TCP-LP-ToN.pdf
 * Original implementation for 2.4.19:
 *   https://users.cs.northwestern.edu/~akuzma/rice/TCP-LP/linux/tcp-lp-linux.htm
 *
 * 2.6.x module Authors:
 *   Wong Hoi Sing, Edison <hswong3i@gmail.com>
 *   Hung Hing Lun, Mike <hlhung3i@gmail.com>
 * SourceForge project page:
 *   http://tcp-lp-mod.sourceforge.net/
 */

// External kernel declarations and types are supplied by the surrounding build.

const LP_RESOL: u32 = TCP_TS_HZ;

enum TcpLpState {
    LP_VALID_RHZ = 1 << 0,
    LP_VALID_OWD = 1 << 1,
    LP_WITHIN_THR = 1 << 3,
    LP_WITHIN_INF = 1 << 4,
}

#[repr(C)]
struct lp {
    flag: u32,
    sowd: u32,
    owd_min: u32,
    owd_max: u32,
    owd_max_rsv: u32,
    remote_hz: u32,
    remote_ref_time: u32,
    local_ref_time: u32,
    last_drop: u32,
    inference: u32,
}

unsafe fn tcp_lp_init(sk: *mut sock) {
    let lp = inet_csk_ca(sk);
    (*lp).flag = 0;
    (*lp).sowd = 0;
    (*lp).owd_min = 0xffff_ffff;
    (*lp).owd_max = 0;
    (*lp).owd_max_rsv = 0;
    (*lp).remote_hz = 0;
    (*lp).remote_ref_time = 0;
    (*lp).local_ref_time = 0;
    (*lp).last_drop = 0;
    (*lp).inference = 0;
}

unsafe fn tcp_lp_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    let lp = inet_csk_ca(sk);
    if ((*lp).flag & (LP_WITHIN_INF as u32)) == 0 {
        tcp_reno_cong_avoid(sk, ack, acked);
    }
}

unsafe fn tcp_lp_remote_hz_estimator(sk: *mut sock) -> u32 {
    let tp = tcp_sk(sk);
    let lp = inet_csk_ca(sk);
    let mut rhz: i64 = ((*lp).remote_hz as i64) << 6;
    let mut m: i64 = 0;

    if (*lp).remote_ref_time == 0 || (*lp).local_ref_time == 0 { goto_out!(out); }
    if (*tp).rx_opt.rcv_tsval == (*lp).remote_ref_time ||
       (*tp).rx_opt.rcv_tsecr == (*lp).local_ref_time { goto_out!(out); }

    m = (TCP_TS_HZ as i64) *
        ((*tp).rx_opt.rcv_tsval as i64 - (*lp).remote_ref_time as i64) /
        ((*tp).rx_opt.rcv_tsecr as i64 - (*lp).local_ref_time as i64);
    if m < 0 { m = -m; }
    if rhz > 0 {
        m -= rhz >> 6;
        rhz += m;
    } else {
        rhz = m << 6;
    }

out:
    if (rhz >> 6) > 0 { (*lp).flag |= LP_VALID_RHZ as u32; }
    else { (*lp).flag &= !(LP_VALID_RHZ as u32); }
    (*lp).remote_ref_time = (*tp).rx_opt.rcv_tsval;
    (*lp).local_ref_time = (*tp).rx_opt.rcv_tsecr;
    (rhz >> 6) as u32
}

unsafe fn tcp_lp_owd_calculator(sk: *mut sock) -> u32 {
    let tp = tcp_sk(sk);
    let lp = inet_csk_ca(sk);
    let mut owd: i64 = 0;
    (*lp).remote_hz = tcp_lp_remote_hz_estimator(sk);
    if ((*lp).flag & (LP_VALID_RHZ as u32)) != 0 {
        owd = (*tp).rx_opt.rcv_tsval as i64 * (LP_RESOL / (*lp).remote_hz) as i64 -
              (*tp).rx_opt.rcv_tsecr as i64 * (LP_RESOL / TCP_TS_HZ) as i64;
        if owd < 0 { owd = -owd; }
    }
    if owd > 0 { (*lp).flag |= LP_VALID_OWD as u32; }
    else { (*lp).flag &= !(LP_VALID_OWD as u32); }
    owd as u32
}

unsafe fn tcp_lp_rtt_sample(sk: *mut sock, _rtt: u32) {
    let lp = inet_csk_ca(sk);
    let mut mowd = tcp_lp_owd_calculator(sk);
    if ((*lp).flag & (LP_VALID_RHZ as u32)) == 0 || ((*lp).flag & (LP_VALID_OWD as u32)) == 0 { return; }
    if mowd < (*lp).owd_min { (*lp).owd_min = mowd; }
    if mowd > (*lp).owd_max {
        if mowd > (*lp).owd_max_rsv {
            if (*lp).owd_max_rsv == 0 { (*lp).owd_max = mowd; }
            else { (*lp).owd_max = (*lp).owd_max_rsv; }
            (*lp).owd_max_rsv = mowd;
        } else { (*lp).owd_max = mowd; }
    }
    if (*lp).sowd != 0 {
        mowd -= (*lp).sowd >> 3;
        (*lp).sowd += mowd;
    } else { (*lp).sowd = mowd << 3; }
}

unsafe fn tcp_lp_pkts_acked(sk: *mut sock, sample: *const ack_sample) {
    let tp = tcp_sk(sk);
    let lp = inet_csk_ca(sk);
    let now = tcp_time_stamp_ts(tp);
    let mut delta: u32;
    if (*sample).rtt_us > 0 { tcp_lp_rtt_sample(sk, (*sample).rtt_us); }
    delta = now - (*tp).rx_opt.rcv_tsecr;
    if (delta as i32) > 0 { (*lp).inference = 3 * delta; }
    if (*lp).last_drop != 0 && now - (*lp).last_drop < (*lp).inference { (*lp).flag |= LP_WITHIN_INF as u32; }
    else { (*lp).flag &= !(LP_WITHIN_INF as u32); }
    if ((*lp).sowd >> 3) < (*lp).owd_min + 15 * ((*lp).owd_max - (*lp).owd_min) / 100 { (*lp).flag |= LP_WITHIN_THR as u32; }
    else { (*lp).flag &= !(LP_WITHIN_THR as u32); }
    pr_debug!("TCP-LP: %05o|%5u|%5u|%15u|%15u|%15u\n", (*lp).flag, tcp_snd_cwnd(tp), (*lp).remote_hz, (*lp).owd_min, (*lp).owd_max, (*lp).sowd >> 3);
    if ((*lp).flag & (LP_WITHIN_THR as u32)) != 0 { return; }
    (*lp).owd_min = (*lp).sowd >> 3;
    (*lp).owd_max = (*lp).sowd >> 2;
    (*lp).owd_max_rsv = (*lp).sowd >> 2;
    if ((*lp).flag & (LP_WITHIN_INF as u32)) != 0 { tcp_snd_cwnd_set(tp, 1_u32); }
    else { tcp_snd_cwnd_set(tp, core::cmp::max(tcp_snd_cwnd(tp) >> 1, 1_u32)); }
    (*lp).last_drop = now;
}

// The following module registration items retain the source interfaces.
static mut tcp_lp: tcp_congestion_ops = tcp_congestion_ops {
    init: Some(tcp_lp_init), ssthresh: Some(tcp_reno_ssthresh), undo_cwnd: Some(tcp_reno_undo_cwnd),
    cong_avoid: Some(tcp_lp_cong_avoid), pkts_acked: Some(tcp_lp_pkts_acked), owner: THIS_MODULE, name: "lp",
};

unsafe fn tcp_lp_register() -> i32 {
    BUILD_BUG_ON!(core::mem::size_of::<lp>() > ICSK_CA_PRIV_SIZE);
    tcp_register_congestion_control(&raw mut tcp_lp)
}

unsafe fn tcp_lp_unregister() { tcp_unregister_congestion_control(&raw mut tcp_lp); }

module_init!(tcp_lp_register);
module_exit!(tcp_lp_unregister);
MODULE_AUTHOR!("Wong Hoi Sing Edison, Hung Hing Lun Mike");
MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("TCP Low Priority");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
