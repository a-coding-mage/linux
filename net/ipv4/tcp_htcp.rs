// SPDX-License-Identifier: GPL-2.0-only
/*
 * H-TCP congestion control. The algorithm is detailed in:
 * R.N.Shorten, D.J.Leith:
 *   "H-TCP: TCP for high-speed and long-distance networks"
 *   Proc. PFLDnet, Argonne, 2004.
 * https://www.hamilton.ie/net/htcp3.pdf
 */

const ALPHA_BASE: u32 = 1 << 7; /* 1.0 with shift << 7 */
const BETA_MIN: u8 = 1 << 6; /* 0.5 with shift << 7 */
const BETA_MAX: u8 = 102; /* 0.8 with shift << 7 */

static mut USE_RTT_SCALING: i32 = 1;
static mut USE_BANDWIDTH_SWITCH: i32 = 1;

#[repr(C)]
struct htcp {
    alpha: u32, /* Fixed point arith, << 7 */
    beta: u8, /* Fixed point arith, << 7 */
    modeswitch: u8, /* Delay modeswitch
                       until we had at least one congestion event */
    pkts_acked: u16,
    packetcount: u32,
    minRTT: u32,
    maxRTT: u32,
    last_cong: u32, /* Time since last congestion event end */
    undo_last_cong: u32,

    undo_maxRTT: u32,
    undo_old_maxB: u32,

    /* Bandwidth estimation */
    minB: u32,
    maxB: u32,
    old_maxB: u32,
    Bi: u32,
    lasttime: u32,
}

unsafe fn htcp_cong_time(ca: *const htcp) -> u32 {
    jiffies.wrapping_sub((*ca).last_cong)
}

unsafe fn htcp_ccount(ca: *const htcp) -> u32 {
    htcp_cong_time(ca) / (*ca).minRTT
}

unsafe fn htcp_reset(ca: *mut htcp) {
    (*ca).undo_last_cong = (*ca).last_cong;
    (*ca).undo_maxRTT = (*ca).maxRTT;
    (*ca).undo_old_maxB = (*ca).old_maxB;
    (*ca).last_cong = jiffies;
}

unsafe fn htcp_cwnd_undo(sk: *mut sock) -> u32 {
    let ca = inet_csk_ca(sk);
    if (*ca).undo_last_cong != 0 {
        (*ca).last_cong = (*ca).undo_last_cong;
        (*ca).maxRTT = (*ca).undo_maxRTT;
        (*ca).old_maxB = (*ca).undo_old_maxB;
        (*ca).undo_last_cong = 0;
    }
    tcp_reno_undo_cwnd(sk)
}

unsafe fn measure_rtt(sk: *mut sock, srtt: u32) {
    let icsk = inet_csk(sk);
    let ca = inet_csk_ca(sk);
    /* keep track of minimum RTT seen so far, minRTT is zero at first */
    if (*ca).minRTT > srtt || (*ca).minRTT == 0 { (*ca).minRTT = srtt; }
    /* max RTT */
    if (*icsk).icsk_ca_state == TCP_CA_Open {
        if (*ca).maxRTT < (*ca).minRTT { (*ca).maxRTT = (*ca).minRTT; }
        if (*ca).maxRTT < srtt && srtt <= (*ca).maxRTT + msecs_to_jiffies(20) {
            (*ca).maxRTT = srtt;
        }
    }
}

unsafe fn measure_achieved_throughput(sk: *mut sock, sample: *const ack_sample) {
    let icsk = inet_csk(sk);
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    let now = tcp_jiffies32;
    if (*icsk).icsk_ca_state == TCP_CA_Open { (*ca).pkts_acked = (*sample).pkts_acked; }
    if (*sample).rtt_us > 0 { measure_rtt(sk, usecs_to_jiffies((*sample).rtt_us)); }
    if USE_BANDWIDTH_SWITCH == 0 { return; }
    if !((1 << (*icsk).icsk_ca_state) & (TCPF_CA_Open | TCPF_CA_Disorder)) != 0 {
        (*ca).packetcount = 0;
        (*ca).lasttime = now;
        return;
    }
    (*ca).packetcount += (*sample).pkts_acked;
    if (*ca).packetcount >= tcp_snd_cwnd(tp) - if ((*ca).alpha >> 7) != 0 { (*ca).alpha >> 7 } else { 1 }
        && now.wrapping_sub((*ca).lasttime) >= (*ca).minRTT && (*ca).minRTT > 0 {
        let cur_Bi: u32 = (*ca).packetcount * HZ / now.wrapping_sub((*ca).lasttime);
        if htcp_ccount(ca) <= 3 {
            (*ca).minB = cur_Bi; (*ca).maxB = cur_Bi; (*ca).Bi = cur_Bi;
        } else {
            (*ca).Bi = (3 * (*ca).Bi + cur_Bi) / 4;
            if (*ca).Bi > (*ca).maxB { (*ca).maxB = (*ca).Bi; }
            if (*ca).minB > (*ca).maxB { (*ca).minB = (*ca).maxB; }
        }
        (*ca).packetcount = 0;
        (*ca).lasttime = now;
    }
}

unsafe fn htcp_beta_update(ca: *mut htcp, minRTT: u32, maxRTT: u32) {
    if USE_BANDWIDTH_SWITCH != 0 {
        let maxB = (*ca).maxB;
        let old_maxB = (*ca).old_maxB;
        (*ca).old_maxB = (*ca).maxB;
        if !between(5 * maxB, 4 * old_maxB, 6 * old_maxB) {
            (*ca).beta = BETA_MIN; (*ca).modeswitch = 0; return;
        }
    }
    if (*ca).modeswitch != 0 && minRTT > msecs_to_jiffies(10) && maxRTT != 0 {
        (*ca).beta = (minRTT << 7) as u8 / maxRTT as u8;
        if (*ca).beta < BETA_MIN { (*ca).beta = BETA_MIN; }
        else if (*ca).beta > BETA_MAX { (*ca).beta = BETA_MAX; }
    } else { (*ca).beta = BETA_MIN; (*ca).modeswitch = 1; }
}

unsafe fn htcp_alpha_update(ca: *mut htcp) {
    let minRTT = (*ca).minRTT;
    let mut factor: u32 = 1;
    let mut diff = htcp_cong_time(ca);
    if diff > HZ {
        diff -= HZ;
        factor = 1 + (10 * diff + ((diff / 2) * (diff / 2) / HZ)) / HZ;
    }
    if USE_RTT_SCALING != 0 && minRTT != 0 {
        let mut scale = (HZ << 3) / (10 * minRTT);
        scale = clamp(scale, 1u32 << 2, 10u32 << 3);
        factor = (factor << 3) / scale;
        if factor == 0 { factor = 1; }
    }
    (*ca).alpha = 2 * factor * ((1 << 7) - (*ca).beta as u32);
    if (*ca).alpha == 0 { (*ca).alpha = ALPHA_BASE; }
}

/*
 * After we have the rtt data to calculate beta, we'd still prefer to wait one
 * rtt before we adjust our beta to ensure we are working from a consistent
 * data.
 *
 * This function should be called when we hit a congestion event since only at
 * that point do we really have a real sense of maxRTT (the queues en route
 * were getting just too full now).
 */
unsafe fn htcp_param_update(sk: *mut sock) {
    let ca = inet_csk_ca(sk);
    let minRTT = (*ca).minRTT;
    let maxRTT = (*ca).maxRTT;
    htcp_beta_update(ca, minRTT, maxRTT);
    htcp_alpha_update(ca);
    /* add slowly fading memory for maxRTT to accommodate routing changes */
    if minRTT > 0 && maxRTT > minRTT { (*ca).maxRTT = minRTT + ((maxRTT - minRTT) * 95) / 100; }
}

unsafe fn htcp_recalc_ssthresh(sk: *mut sock) -> u32 {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    htcp_param_update(sk);
    max((tcp_snd_cwnd(tp) * (*ca).beta as u32) >> 7, 2u32)
}

unsafe fn htcp_cong_avoid(sk: *mut sock, _ack: u32, acked: u32) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    if !tcp_is_cwnd_limited(sk) { return; }
    if tcp_in_slow_start(tp) { tcp_slow_start(tp, acked); }
    else {
        /* In dangerous area, increase slowly.
         * In theory this is tp->snd_cwnd += alpha / tp->snd_cwnd
         */
        if (tp.snd_cwnd_cnt * (*ca).alpha) >> 7 >= tcp_snd_cwnd(tp) {
            if tcp_snd_cwnd(tp) < tp.snd_cwnd_clamp { tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp) + 1); }
            tp.snd_cwnd_cnt = 0;
            htcp_alpha_update(ca);
        } else { tp.snd_cwnd_cnt += (*ca).pkts_acked as u32; }
        (*ca).pkts_acked = 1;
    }
}

unsafe fn htcp_init(sk: *mut sock) {
    let ca = inet_csk_ca(sk);
    core::ptr::write_bytes(ca, 0, 1);
    (*ca).alpha = ALPHA_BASE;
    (*ca).beta = BETA_MIN;
    (*ca).pkts_acked = 1;
    (*ca).last_cong = jiffies;
}

unsafe fn htcp_state(sk: *mut sock, new_state: u8) {
    match new_state {
        TCP_CA_Open => {
            let ca = inet_csk_ca(sk);
            if (*ca).undo_last_cong != 0 { (*ca).last_cong = jiffies; (*ca).undo_last_cong = 0; }
        }
        TCP_CA_CWR | TCP_CA_Recovery | TCP_CA_Loss => htcp_reset(inet_csk_ca(sk)),
        _ => {}
    }
}

static mut htcp: tcp_congestion_ops = tcp_congestion_ops {
    init: Some(htcp_init),
    ssthresh: Some(htcp_recalc_ssthresh),
    cong_avoid: Some(htcp_cong_avoid),
    set_state: Some(htcp_state),
    undo_cwnd: Some(htcp_cwnd_undo),
    pkts_acked: Some(measure_achieved_throughput),
    owner: THIS_MODULE,
    name: *b"htcp\0",
};

unsafe fn htcp_register() -> i32 {
    BUILD_BUG_ON(core::mem::size_of::<htcp>() > ICSK_CA_PRIV_SIZE);
    BUILD_BUG_ON(BETA_MIN >= BETA_MAX);
    tcp_register_congestion_control(&raw mut htcp)
}

unsafe fn htcp_unregister() {
    tcp_unregister_congestion_control(&raw mut htcp);
}

module_init!(htcp_register);
module_exit!(htcp_unregister);

MODULE_AUTHOR!("Baruch Even");
MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("H-TCP");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
