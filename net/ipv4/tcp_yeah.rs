// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *   YeAH TCP
 *
 * For further details look at:
 *   https://web.archive.org/web/20080316215752/http://wil.cs.caltech.edu/pfldnet2007/paper/YeAH_TCP.pdf
 *
 */

// Dependencies supplied by the surrounding kernel translation.

const TCP_YEAH_ALPHA: u32 = 80;
const TCP_YEAH_GAMMA: u32 = 1;
const TCP_YEAH_DELTA: u32 = 3;
const TCP_YEAH_EPSILON: u32 = 1;
const TCP_YEAH_PHY: u32 = 8;
const TCP_YEAH_RHO: u32 = 16;
const TCP_YEAH_ZETA: u32 = 50;
const TCP_SCALABLE_AI_CNT: u32 = 100;

#[repr(C)]
struct Yeah {
    vegas: Vegas,
    lastQ: u32,
    doing_reno_now: u32,
    reno_count: u32,
    fast_count: u32,
}

unsafe fn tcp_yeah_init(sk: *mut sock) {
    let tp = tcp_sk(sk);
    let yeah = inet_csk_ca(sk) as *mut Yeah;

    tcp_vegas_init(sk);
    (*yeah).doing_reno_now = 0;
    (*yeah).lastQ = 0;
    (*yeah).reno_count = 2;
    (*tp).snd_cwnd_clamp = min_t((*tp).snd_cwnd_clamp, 0xffff_ffffu32 / 128);
}

unsafe fn tcp_yeah_cong_avoid(sk: *mut sock, mut ack: u32, mut acked: u32) {
    let tp = tcp_sk(sk);
    let yeah = inet_csk_ca(sk) as *mut Yeah;

    if !tcp_is_cwnd_limited(sk) { return; }

    if tcp_in_slow_start(tp) {
        acked = tcp_slow_start(tp, acked);
    }

    if acked != 0 {
        if (*yeah).doing_reno_now == 0 {
            tcp_cong_avoid_ai(tp, min(tcp_snd_cwnd(tp), TCP_SCALABLE_AI_CNT), acked);
        } else {
            tcp_cong_avoid_ai(tp, tcp_snd_cwnd(tp), acked);
        }
    }

    {
        if after(ack, (*yeah).vegas.beg_snd_nxt) {
            if (*yeah).vegas.cntRTT > 2 {
                let rtt = (*yeah).vegas.minRTT;
                let mut bw = tcp_snd_cwnd(tp) as u64;
                bw = bw.wrapping_mul((rtt - (*yeah).vegas.baseRTT) as u64);
                bw = do_div(bw, rtt as u64);
                let queue = bw as u32;

                if queue > TCP_YEAH_ALPHA || rtt - (*yeah).vegas.baseRTT > (*yeah).vegas.baseRTT / TCP_YEAH_PHY {
                    if queue > TCP_YEAH_ALPHA && tcp_snd_cwnd(tp) > (*yeah).reno_count {
                        let reduction = min(queue / TCP_YEAH_GAMMA, tcp_snd_cwnd(tp) >> TCP_YEAH_EPSILON);
                        tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp) - reduction);
                        tcp_snd_cwnd_set(tp, max(tcp_snd_cwnd(tp), (*yeah).reno_count));
                        WRITE_ONCE((*tp).snd_ssthresh, tcp_snd_cwnd(tp));
                    }
                    if (*yeah).reno_count <= 2 { (*yeah).reno_count = max(tcp_snd_cwnd(tp) >> 1, 2); }
                    else { (*yeah).reno_count += 1; }
                    (*yeah).doing_reno_now = min((*yeah).doing_reno_now + 1, 0x00ff_ffff);
                } else {
                    (*yeah).fast_count += 1;
                    if (*yeah).fast_count > TCP_YEAH_ZETA { (*yeah).reno_count = 2; (*yeah).fast_count = 0; }
                    (*yeah).doing_reno_now = 0;
                }
                (*yeah).lastQ = queue;
            }
            (*yeah).vegas.beg_snd_una = (*yeah).vegas.beg_snd_nxt;
            (*yeah).vegas.beg_snd_nxt = (*tp).snd_nxt;
            (*yeah).vegas.beg_snd_cwnd = tcp_snd_cwnd(tp);
            (*yeah).vegas.cntRTT = 0;
            (*yeah).vegas.minRTT = 0x7fff_ffff;
        }
    }
}

unsafe fn tcp_yeah_ssthresh(sk: *mut sock) -> u32 {
    let tp = tcp_sk(sk) as *const tcp_sock;
    let yeah = inet_csk_ca(sk) as *mut Yeah;
    let mut reduction;
    if (*yeah).doing_reno_now < TCP_YEAH_RHO {
        reduction = (*yeah).lastQ;
        reduction = min(reduction, max(tcp_snd_cwnd(tp) >> 1, 2));
        reduction = max(reduction, tcp_snd_cwnd(tp) >> TCP_YEAH_DELTA);
    } else { reduction = max(tcp_snd_cwnd(tp) >> 1, 2); }
    (*yeah).fast_count = 0;
    (*yeah).reno_count = max((*yeah).reno_count >> 1, 2);
    max(tcp_snd_cwnd(tp).wrapping_sub(reduction), 2)
}

// Registration and module metadata are provided by the kernel integration.
static mut tcp_yeah: tcp_congestion_ops = tcp_congestion_ops {
    init: Some(tcp_yeah_init), ssthresh: Some(tcp_yeah_ssthresh),
    undo_cwnd: Some(tcp_reno_undo_cwnd), cong_avoid: Some(tcp_yeah_cong_avoid),
    set_state: Some(tcp_vegas_state), cwnd_event: Some(tcp_vegas_cwnd_event),
    cwnd_event_tx_start: Some(tcp_vegas_cwnd_event_tx_start),
    get_info: Some(tcp_vegas_get_info), pkts_acked: Some(tcp_vegas_pkts_acked),
    owner: THIS_MODULE, name: "yeah",
};

unsafe fn tcp_yeah_register() -> i32 {
    BUILD_BUG_ON(core::mem::size_of::<Yeah>() > ICSK_CA_PRIV_SIZE);
    tcp_register_congestion_control(&mut tcp_yeah);
    0
}

unsafe fn tcp_yeah_unregister() { tcp_unregister_congestion_control(&mut tcp_yeah); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
