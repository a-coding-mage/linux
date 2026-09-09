// SPDX-License-Identifier: GPL-2.0-only
/*
 * TCP Westwood+: end-to-end bandwidth estimation for TCP
 *
 *      Angelo Dell'Aera: author of the first version of TCP Westwood+ in Linux 2.4
 *
 * Support at http://c3lab.poliba.it/index.php/Westwood
 * Main references in literature:
 *
 * - Mascolo S, Casetti, M. Gerla et al.
 *   "TCP Westwood: bandwidth estimation for TCP" Proc. ACM Mobicom 2001
 *
 * - A. Grieco, s. Mascolo
 *   "Performance evaluation of New Reno, Vegas, Westwood+ TCP" ACM Computer
 *     Comm. Review, 2004
 *
 * - A. Dell'Aera, L. Grieco, S. Mascolo.
 *   "Linux 2.4 Implementation of Westwood+ TCP with Rate-Halving :
 *    A Performance Evaluation Over the Internet" (ICC 2004), Paris, June 2004
 *
 * Westwood+ employs end-to-end bandwidth measurement to set cwnd and
 * ssthresh after packet loss. The probing phase is as the original Reno.
 */

// Kernel dependencies supplied by other translation units.

/* TCP Westwood structure */
#[repr(C)]
pub struct westwood {
    pub bw_ns_est: u32,    /* first bandwidth estimation..not too smoothed 8) */
    pub bw_est: u32,       /* bandwidth estimate */
    pub rtt_win_sx: u32,   /* here starts a new evaluation... */
    pub bk: u32,
    pub snd_una: u32,      /* used for evaluating the number of acked bytes */
    pub cumul_ack: u32,
    pub accounted: u32,
    pub rtt: u32,
    pub rtt_min: u32,      /* minimum observed RTT */
    pub first_ack: u8,     /* flag which infers that this is the first ack */
    pub reset_rtt_min: u8, /* Reset RTT min to next RTT sample*/
}

/* TCP Westwood functions and constants */
pub const TCP_WESTWOOD_RTT_MIN: u32 = HZ / 20; /* 50ms */
pub const TCP_WESTWOOD_INIT_RTT: u32 = 20 * HZ; /* maybe too conservative?! */

/*
 * @tcp_westwood_create
 * This function initializes fields used in TCP Westwood+,
 * it is called after the initial SYN, so the sequence numbers
 * are correct but new passive connections we have no
 * information about RTTmin at this time so we simply set it to
 * TCP_WESTWOOD_INIT_RTT. This value was chosen to be too conservative
 * since in this way we're sure it will be updated in a consistent
 * way as soon as possible. It will reasonably happen within the first
 * RTT period of the connection lifetime.
 */
unsafe fn tcp_westwood_init(sk: *mut sock) {
    let w: *mut westwood = inet_csk_ca(sk);

    (*w).bk = 0;
    (*w).bw_ns_est = 0;
    (*w).bw_est = 0;
    (*w).accounted = 0;
    (*w).cumul_ack = 0;
    (*w).reset_rtt_min = 1;
    (*w).rtt_min = TCP_WESTWOOD_INIT_RTT;
    (*w).rtt = TCP_WESTWOOD_INIT_RTT;
    (*w).rtt_win_sx = tcp_jiffies32;
    (*w).snd_una = (*tcp_sk(sk)).snd_una;
    (*w).first_ack = 1;
}

/*
 * @westwood_do_filter
 * Low-pass filter. Implemented using constant coefficients.
 */
#[inline]
fn westwood_do_filter(a: u32, b: u32) -> u32 {
    ((7u32.wrapping_mul(a)).wrapping_add(b)) >> 3
}

unsafe fn westwood_filter(w: *mut westwood, delta: u32) {
    /* If the filter is empty fill it with the first sample of bandwidth  */
    if (*w).bw_ns_est == 0 && (*w).bw_est == 0 {
        (*w).bw_ns_est = (*w).bk / delta;
        (*w).bw_est = (*w).bw_ns_est;
    } else {
        (*w).bw_ns_est = westwood_do_filter((*w).bw_ns_est, (*w).bk / delta);
        (*w).bw_est = westwood_do_filter((*w).bw_est, (*w).bw_ns_est);
    }
}

/*
 * @westwood_pkts_acked
 * Called after processing group of packets.
 * but all westwood needs is the last sample of srtt.
 */
unsafe fn tcp_westwood_pkts_acked(sk: *mut sock, sample: *const ack_sample) {
    let w: *mut westwood = inet_csk_ca(sk);

    if (*sample).rtt_us > 0 {
        (*w).rtt = usecs_to_jiffies((*sample).rtt_us);
    }
}

/*
 * @westwood_update_window
 * It updates RTT evaluation window if it is the right moment to do
 * it. If so it calls filter for evaluating bandwidth.
 */
unsafe fn westwood_update_window(sk: *mut sock) {
    let w: *mut westwood = inet_csk_ca(sk);
    let delta: i32 = tcp_jiffies32.wrapping_sub((*w).rtt_win_sx) as i32;

    /* Initialize w->snd_una with the first acked sequence number in order
     * to fix mismatch between tp->snd_una and w->snd_una for the first
     * bandwidth sample
     */
    if (*w).first_ack != 0 {
        (*w).snd_una = (*tcp_sk(sk)).snd_una;
        (*w).first_ack = 0;
    }

    /*
     * See if a RTT-window has passed.
     * Be careful since if RTT is less than
     * 50ms we don't filter but we continue 'building the sample'.
     * This minimum limit was chosen since an estimation on small
     * time intervals is better to avoid...
     * Obviously on a LAN we reasonably will always have
     * right_bound = left_bound + WESTWOOD_RTT_MIN
     */
    if (*w).rtt != 0 && delta > core::cmp::max((*w).rtt, TCP_WESTWOOD_RTT_MIN) as i32 {
        westwood_filter(w, delta as u32);

        (*w).bk = 0;
        (*w).rtt_win_sx = tcp_jiffies32;
    }
}

#[inline]
unsafe fn update_rtt_min(w: *mut westwood) {
    if (*w).reset_rtt_min != 0 {
        (*w).rtt_min = (*w).rtt;
        (*w).reset_rtt_min = 0;
    } else {
        (*w).rtt_min = core::cmp::min((*w).rtt, (*w).rtt_min);
    }
}

/*
 * @westwood_fast_bw
 * It is called when we are in fast path. In particular it is called when
 * header prediction is successful. In such case in fact update is
 * straight forward and doesn't need any particular care.
 */
#[inline]
unsafe fn westwood_fast_bw(sk: *mut sock) {
    let tp: *const tcp_sock = tcp_sk(sk);
    let w: *mut westwood = inet_csk_ca(sk);

    westwood_update_window(sk);

    (*w).bk = (*w).bk.wrapping_add((*tp).snd_una.wrapping_sub((*w).snd_una));
    (*w).snd_una = (*tp).snd_una;
    update_rtt_min(w);
}

/*
 * @westwood_acked_count
 * This function evaluates cumul_ack for evaluating bk in case of
 * delayed or partial acks.
 */
#[inline]
unsafe fn westwood_acked_count(sk: *mut sock) -> u32 {
    let tp: *const tcp_sock = tcp_sk(sk);
    let w: *mut westwood = inet_csk_ca(sk);

    (*w).cumul_ack = (*tp).snd_una.wrapping_sub((*w).snd_una);

    /* If cumul_ack is 0 this is a dupack since it's not moving
     * tp->snd_una.
     */
    if (*w).cumul_ack == 0 {
        (*w).accounted = (*w).accounted.wrapping_add((*tp).mss_cache);
        (*w).cumul_ack = (*tp).mss_cache;
    }

    if (*w).cumul_ack > (*tp).mss_cache {
        /* Partial or delayed ack */
        if (*w).accounted >= (*w).cumul_ack {
            (*w).accounted = (*w).accounted.wrapping_sub((*w).cumul_ack);
            (*w).cumul_ack = (*tp).mss_cache;
        } else {
            (*w).cumul_ack = (*w).cumul_ack.wrapping_sub((*w).accounted);
            (*w).accounted = 0;
        }
    }

    (*w).snd_una = (*tp).snd_una;

    (*w).cumul_ack
}

/*
 * TCP Westwood
 * Here limit is evaluated as Bw estimation*RTTmin (for obtaining it
 * in packets we use mss_cache). Rttmin is guaranteed to be >= 2
 * so avoids ever returning 0.
 */
unsafe fn tcp_westwood_bw_rttmin(sk: *const sock) -> u32 {
    let tp: *const tcp_sock = tcp_sk(sk as *mut sock);
    let w: *const westwood = inet_csk_ca(sk as *mut sock);

    core::cmp::max(((*w).bw_est * (*w).rtt_min) / (*tp).mss_cache, 2)
}

unsafe fn tcp_westwood_ack(sk: *mut sock, ack_flags: u32) {
    if ack_flags & CA_ACK_SLOWPATH != 0 {
        let w: *mut westwood = inet_csk_ca(sk);

        westwood_update_window(sk);
        (*w).bk = (*w).bk.wrapping_add(westwood_acked_count(sk));

        update_rtt_min(w);
        return;
    }

    westwood_fast_bw(sk);
}

unsafe fn tcp_westwood_event(sk: *mut sock, event: tcp_ca_event) {
    let tp: *mut tcp_sock = tcp_sk(sk);
    let w: *mut westwood = inet_csk_ca(sk);

    match event {
        CA_EVENT_COMPLETE_CWR => {
            WRITE_ONCE((*tp).snd_ssthresh, tcp_westwood_bw_rttmin(sk));
            tcp_snd_cwnd_set(tp, (*tp).snd_ssthresh);
        }
        CA_EVENT_LOSS => {
            WRITE_ONCE((*tp).snd_ssthresh, tcp_westwood_bw_rttmin(sk));
            /* Update RTT_min when next ack arrives */
            (*w).reset_rtt_min = 1;
        }
        _ => {
            /* don't care */
        }
    }
}

/* Extract info for Tcp socket info provided via netlink. */
unsafe fn tcp_westwood_info(
    sk: *mut sock,
    ext: u32,
    attr: *mut i32,
    info: *mut tcp_cc_info,
) -> usize {
    let ca: *const westwood = inet_csk_ca(sk);

    if ext & (1u32 << (INET_DIAG_VEGASINFO - 1)) != 0 {
        (*info).vegas.tcpv_enabled = 1;
        (*info).vegas.tcpv_rttcnt = 0;
        (*info).vegas.tcpv_rtt = jiffies_to_usecs((*ca).rtt);
        (*info).vegas.tcpv_minrtt = jiffies_to_usecs((*ca).rtt_min);

        *attr = INET_DIAG_VEGASINFO;
        return core::mem::size_of::<tcpvegas_info>();
    }
    0
}

static mut tcp_westwood: tcp_congestion_ops = tcp_congestion_ops {
    init: Some(tcp_westwood_init),
    ssthresh: Some(tcp_reno_ssthresh),
    cong_avoid: Some(tcp_reno_cong_avoid),
    undo_cwnd: Some(tcp_reno_undo_cwnd),
    cwnd_event: Some(tcp_westwood_event),
    in_ack_event: Some(tcp_westwood_ack),
    get_info: Some(tcp_westwood_info),
    pkts_acked: Some(tcp_westwood_pkts_acked),
    owner: THIS_MODULE,
    name: *b"westwood\0",
};

unsafe fn tcp_westwood_register() -> i32 {
    // BUILD_BUG_ON(sizeof(struct westwood) > ICSK_CA_PRIV_SIZE);
    tcp_register_congestion_control(&raw mut tcp_westwood)
}

unsafe fn tcp_westwood_unregister() {
    tcp_unregister_congestion_control(&raw mut tcp_westwood);
}

// module_init(tcp_westwood_register);
// module_exit(tcp_westwood_unregister);

// MODULE_AUTHOR("Stephen Hemminger, Angelo Dell'Aera");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("TCP Westwood+");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
