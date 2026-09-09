// SPDX-License-Identifier: GPL-2.0-only
/*
 * TCP Vegas congestion control
 *
 * This is based on the congestion detection/avoidance scheme described in
 * Lawrence S. Brakmo and Larry L. Peterson, "TCP Vegas: End to end
 * congestion avoidance on a global internet."
 */

// Kernel dependencies corresponding to the original C includes are supplied
// by the surrounding translation environment.

static mut alpha: i32 = 2;
static mut beta: i32 = 4;
static mut gamma: i32 = 1;

// module_param(alpha, int, 0644);
// MODULE_PARM_DESC(alpha, "lower bound of packets in network");
// module_param(beta, int, 0644);
// MODULE_PARM_DESC(beta, "upper bound of packets in network");
// module_param(gamma, int, 0644);
// MODULE_PARM_DESC(gamma, "limit on increase (scale by 2)");

unsafe fn vegas_enable(sk: *mut sock) {
    let tp: *const tcp_sock = tcp_sk(sk);
    let vegas: *mut vegas = inet_csk_ca(sk);

    (*vegas).doing_vegas_now = 1;
    (*vegas).beg_snd_nxt = (*tp).snd_nxt;
    (*vegas).cntRTT = 0;
    (*vegas).minRTT = 0x7fffffff;
}

unsafe fn vegas_disable(sk: *mut sock) {
    let vegas: *mut vegas = inet_csk_ca(sk);
    (*vegas).doing_vegas_now = 0;
}

pub unsafe extern "C" fn tcp_vegas_init(sk: *mut sock) {
    let vegas: *mut vegas = inet_csk_ca(sk);
    (*vegas).baseRTT = 0x7fffffff;
    vegas_enable(sk);
}

pub unsafe extern "C" fn tcp_vegas_pkts_acked(
    sk: *mut sock,
    sample: *const ack_sample,
) {
    let vegas: *mut vegas = inet_csk_ca(sk);
    let mut vrtt: u32;

    if (*sample).rtt_us < 0 {
        return;
    }

    vrtt = (*sample).rtt_us as u32 + 1;
    if vrtt < (*vegas).baseRTT {
        (*vegas).baseRTT = vrtt;
    }
    (*vegas).minRTT = min((*vegas).minRTT, vrtt);
    (*vegas).cntRTT += 1;
}

pub unsafe extern "C" fn tcp_vegas_state(sk: *mut sock, ca_state: u8) {
    if ca_state == TCP_CA_Open {
        vegas_enable(sk);
    } else {
        vegas_disable(sk);
    }
}

pub unsafe extern "C" fn tcp_vegas_cwnd_event(sk: *mut sock, event: tcp_ca_event) {
    if event == CA_EVENT_CWND_RESTART {
        tcp_vegas_init(sk);
    }
}

pub unsafe extern "C" fn tcp_vegas_cwnd_event_tx_start(sk: *mut sock) {
    tcp_vegas_init(sk);
}

unsafe fn tcp_vegas_ssthresh(tp: *mut tcp_sock) -> u32 {
    min((*tp).snd_ssthresh, tcp_snd_cwnd(tp))
}

unsafe fn tcp_vegas_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    let tp: *mut tcp_sock = tcp_sk(sk);
    let vegas: *mut vegas = inet_csk_ca(sk);

    if (*vegas).doing_vegas_now == 0 {
        tcp_reno_cong_avoid(sk, ack, acked);
        return;
    }

    if after(ack, (*vegas).beg_snd_nxt) {
        (*vegas).beg_snd_nxt = (*tp).snd_nxt;

        if (*vegas).cntRTT <= 2 {
            tcp_reno_cong_avoid(sk, ack, acked);
        } else {
            let rtt: u32 = (*vegas).minRTT;
            let mut target_cwnd: u64 = tcp_snd_cwnd(tp) as u64 * (*vegas).baseRTT as u64;
            target_cwnd /= rtt as u64;

            let diff: u32 = tcp_snd_cwnd(tp)
                * (rtt - (*vegas).baseRTT)
                / (*vegas).baseRTT;

            if diff > gamma as u32 && tcp_in_slow_start(tp) {
                tcp_snd_cwnd_set(tp, min(tcp_snd_cwnd(tp), target_cwnd as u32 + 1));
                WRITE_ONCE((*tp).snd_ssthresh, tcp_vegas_ssthresh(tp));
            } else if tcp_in_slow_start(tp) {
                tcp_slow_start(tp, acked);
            } else if diff > beta as u32 {
                tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp) - 1);
                WRITE_ONCE((*tp).snd_ssthresh, tcp_vegas_ssthresh(tp));
            } else if diff < alpha as u32 {
                tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp) + 1);
            }

            if tcp_snd_cwnd(tp) < 2 {
                tcp_snd_cwnd_set(tp, 2);
            } else if tcp_snd_cwnd(tp) > (*tp).snd_cwnd_clamp {
                tcp_snd_cwnd_set(tp, (*tp).snd_cwnd_clamp);
            }
            WRITE_ONCE((*tp).snd_ssthresh, tcp_current_ssthresh(sk));
        }

        (*vegas).cntRTT = 0;
        (*vegas).minRTT = 0x7fffffff;
    } else if tcp_in_slow_start(tp) {
        tcp_slow_start(tp, acked);
    }
}

pub unsafe extern "C" fn tcp_vegas_get_info(
    sk: *mut sock,
    ext: u32,
    attr: *mut i32,
    info: *mut tcp_cc_info,
) -> usize {
    let ca: *const vegas = inet_csk_ca(sk);

    if ext & (1 << (INET_DIAG_VEGASINFO - 1)) != 0 {
        (*info).vegas.tcpv_enabled = (*ca).doing_vegas_now;
        (*info).vegas.tcpv_rttcnt = (*ca).cntRTT;
        (*info).vegas.tcpv_rtt = (*ca).baseRTT;
        (*info).vegas.tcpv_minrtt = (*ca).minRTT;
        *attr = INET_DIAG_VEGASINFO;
        return core::mem::size_of::<tcpvegas_info>();
    }
    0
}

static mut tcp_vegas: tcp_congestion_ops = tcp_congestion_ops {
    init: Some(tcp_vegas_init),
    ssthresh: Some(tcp_reno_ssthresh),
    undo_cwnd: Some(tcp_reno_undo_cwnd),
    cong_avoid: Some(tcp_vegas_cong_avoid),
    pkts_acked: Some(tcp_vegas_pkts_acked),
    set_state: Some(tcp_vegas_state),
    cwnd_event: Some(tcp_vegas_cwnd_event),
    cwnd_event_tx_start: Some(tcp_vegas_cwnd_event_tx_start),
    get_info: Some(tcp_vegas_get_info),
    owner: THIS_MODULE,
    name: b"vegas\0".as_ptr() as *const i8,
};

unsafe fn tcp_vegas_register() -> i32 {
    BUILD_BUG_ON(core::mem::size_of::<vegas>() > ICSK_CA_PRIV_SIZE);
    tcp_register_congestion_control(&raw mut tcp_vegas);
    0
}

unsafe fn tcp_vegas_unregister() {
    tcp_unregister_congestion_control(&raw mut tcp_vegas);
}

// module_init(tcp_vegas_register);
// module_exit(tcp_vegas_unregister);
// MODULE_AUTHOR("Stephen Hemminger");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("TCP Vegas");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
