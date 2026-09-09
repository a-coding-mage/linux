// SPDX-License-Identifier: GPL-2.0-only
/*
 * TCP HYBLA
 *
 * TCP-HYBLA Congestion control algorithm, based on:
 *   C.Caini, R.Firrincieli, "TCP-Hybla: A TCP Enhancement
 *   for Heterogeneous Networks",
 *   International Journal on satellite Communications,
 *                                      September 2004
 *    Daniele Lacamera
 *    root at danielinux.net
 */

// Dependencies supplied by the Linux kernel module and TCP networking headers.

/* Tcp Hybla structure. */
#[repr(C)]
struct hybla {
    hybla_en: bool,
    snd_cwnd_cents: u32, /* Keeps increment values when it is <1, <<7 */
    rho: u32,            /* Rho parameter, integer part  */
    rho2: u32,           /* Rho * Rho, integer part */
    rho_3ls: u32,        /* Rho parameter, <<3 */
    rho2_7ls: u32,       /* Rho^2, <<7 */
    minrtt_us: u32,      /* Minimum smoothed round trip time value seen */
}

/* Hybla reference round trip time (default= 1/40 sec = 25 ms), in ms */
static mut rtt0: i32 = 25;

/* This is called to refresh values for hybla parameters */
#[inline]
unsafe fn hybla_recalc_param(sk: *mut sock) {
    let ca: *mut hybla = inet_csk_ca(sk);

    (*ca).rho_3ls = core::cmp::max(
        (*tcp_sk(sk)).srtt_us / ((rtt0 as u32).wrapping_mul(USEC_PER_MSEC)),
        8u32,
    );
    (*ca).rho = (*ca).rho_3ls >> 3;
    (*ca).rho2_7ls = ((*ca).rho_3ls).wrapping_mul((*ca).rho_3ls) << 1;
    (*ca).rho2 = (*ca).rho2_7ls >> 7;
}

unsafe fn hybla_init(sk: *mut sock) {
    let tp: *mut tcp_sock = tcp_sk(sk);
    let ca: *mut hybla = inet_csk_ca(sk);

    (*ca).rho = 0;
    (*ca).rho2 = 0;
    (*ca).rho_3ls = 0;
    (*ca).rho2_7ls = 0;
    (*ca).snd_cwnd_cents = 0;
    (*ca).hybla_en = true;
    tcp_snd_cwnd_set(tp, 2);
    (*tp).snd_cwnd_clamp = 65535;

    /* 1st Rho measurement based on initial srtt */
    hybla_recalc_param(sk);

    /* set minimum rtt as this is the 1st ever seen */
    (*ca).minrtt_us = (*tp).srtt_us;
    tcp_snd_cwnd_set(tp, (*ca).rho);
}

unsafe fn hybla_state(sk: *mut sock, ca_state: u8) {
    let ca: *mut hybla = inet_csk_ca(sk);

    (*ca).hybla_en = ca_state == TCP_CA_Open;
}

#[inline]
fn hybla_fraction(odds: u32) -> u32 {
    const FRACTIONS: [u32; 8] = [128, 139, 152, 165, 181, 197, 215, 234];

    if (odds as usize) < FRACTIONS.len() {
        FRACTIONS[odds as usize]
    } else {
        128
    }
}

/* TCP Hybla main routine.
 * This is the algorithm behavior:
 *     o Recalc Hybla parameters if min_rtt has changed
 *     o Give cwnd a new value based on the model proposed
 *     o remember increments <1
 */
unsafe fn hybla_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    let tp: *mut tcp_sock = tcp_sk(sk);
    let ca: *mut hybla = inet_csk_ca(sk);
    let mut increment: u32;
    let mut odd: u32;
    let mut rho_fractions: u32;
    let mut is_slowstart = 0;

    /*  Recalculate rho only if this srtt is the lowest */
    if (*tp).srtt_us < (*ca).minrtt_us {
        hybla_recalc_param(sk);
        (*ca).minrtt_us = (*tp).srtt_us;
    }

    if !tcp_is_cwnd_limited(sk) {
        return;
    }

    if !(*ca).hybla_en {
        tcp_reno_cong_avoid(sk, ack, acked);
        return;
    }

    if (*ca).rho == 0 {
        hybla_recalc_param(sk);
    }

    rho_fractions = (*ca).rho_3ls - ((*ca).rho << 3);

    if tcp_in_slow_start(tp) {
        /*
         * slow start
         *      INC = 2^RHO - 1
         * This is done by splitting the rho parameter
         * into 2 parts: an integer part and a fraction part.
         * Inrement<<7 is estimated by doing:
         *             [2^(int+fract)]<<7
         * that is equal to:
         *             (2^int)  *  [(2^fract) <<7]
         * 2^int is straightly computed as 1<<int,
         * while we will use hybla_slowstart_fraction_increment() to
         * calculate 2^fract in a <<7 value.
         */
        is_slowstart = 1;
        increment = (1u32 << core::cmp::min((*ca).rho, 16u32))
            .wrapping_mul(hybla_fraction(rho_fractions))
            .wrapping_sub(128);
    } else {
        /*
         * congestion avoidance
         * INC = RHO^2 / W
         * as long as increment is estimated as (rho<<7)/window
         * it already is <<7 and we can easily count its fractions.
         */
        increment = (*ca).rho2_7ls / tcp_snd_cwnd(tp);
        if increment < 128 {
            (*tp).snd_cwnd_cnt += 1;
        }
    }

    odd = increment % 128;
    tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp).wrapping_add(increment >> 7));
    (*ca).snd_cwnd_cents = (*ca).snd_cwnd_cents.wrapping_add(odd);

    /* check when fractions goes >=128 and increase cwnd by 1. */
    while (*ca).snd_cwnd_cents >= 128 {
        tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp).wrapping_add(1));
        (*ca).snd_cwnd_cents -= 128;
        (*tp).snd_cwnd_cnt = 0;
    }
    /* check when cwnd has not been incremented for a while */
    if increment == 0 && odd == 0 && (*tp).snd_cwnd_cnt >= tcp_snd_cwnd(tp) {
        tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp).wrapping_add(1));
        (*tp).snd_cwnd_cnt = 0;
    }
    /* clamp down slowstart cwnd to ssthresh value. */
    if is_slowstart != 0 {
        tcp_snd_cwnd_set(tp, core::cmp::min(tcp_snd_cwnd(tp), (*tp).snd_ssthresh));
    }

    tcp_snd_cwnd_set(tp, core::cmp::min(tcp_snd_cwnd(tp), (*tp).snd_cwnd_clamp));
}

static mut tcp_hybla: tcp_congestion_ops = tcp_congestion_ops {
    init: Some(hybla_init),
    ssthresh: Some(tcp_reno_ssthresh),
    undo_cwnd: Some(tcp_reno_undo_cwnd),
    cong_avoid: Some(hybla_cong_avoid),
    set_state: Some(hybla_state),
    owner: THIS_MODULE,
    name: *b"hybla\0",
};

unsafe fn hybla_register() -> i32 {
    BUILD_BUG_ON!(core::mem::size_of::<hybla>() > ICSK_CA_PRIV_SIZE);
    tcp_register_congestion_control(&mut tcp_hybla)
}

unsafe fn hybla_unregister() {
    tcp_unregister_congestion_control(&mut tcp_hybla);
}

// Module initialization and cleanup registration.
module_init!(hybla_register);
module_exit!(hybla_unregister);

// MODULE_AUTHOR("Daniele Lacamera");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("TCP Hybla");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
