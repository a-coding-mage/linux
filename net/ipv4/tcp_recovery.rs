// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

unsafe fn tcp_rack_reo_wnd(sk: *const sock) -> u32 {
    let tp = tcp_sk(sk);

    if !(*tp).reord_seen {
        /* If reordering has not been observed, be aggressive during
         * the recovery or starting the recovery by DUPACK threshold.
         */
        if (*inet_csk(sk)).icsk_ca_state >= TCP_CA_Recovery {
            return 0;
        }

        if (*tp).sacked_out >= (*tp).reordering
            && (read_once((*sock_net(sk)).ipv4.sysctl_tcp_recovery) & TCP_RACK_NO_DUPTHRESH) == 0
        {
            return 0;
        }
    }

    /* To be more reordering resilient, allow min_rtt/4 settling delay.
     * Use min_rtt instead of the smoothed RTT because reordering is
     * often a path property and less related to queuing or delayed ACKs.
     * Upon receiving DSACKs, linearly increase the window up to the
     * smoothed RTT.
     */
    core::cmp::min(
        (tcp_min_rtt(tp) >> 2).wrapping_mul((*tp).rack.reo_wnd_steps),
        (*tp).srtt_us >> 3,
    )
}

pub unsafe fn tcp_rack_skb_timeout(tp: *mut tcp_sock, skb: *mut sk_buff, reo_wnd: u32) -> i32 {
    (*tp).rack.rtt_us as i32 + reo_wnd as i32
        - tcp_stamp_us_delta((*tp).tcp_mstamp, tcp_skb_timestamp_us(skb))
}

/* RACK loss detection (IETF RFC8985):
 *
 * Marks a packet lost, if some packet sent later has been (s)acked.
 * The underlying idea is similar to the traditional dupthresh and FACK
 * but they look at different metrics:
 *
 * dupthresh: 3 OOO packets delivered (packet count)
 * FACK: sequence delta to highest sacked sequence (sequence space)
 * RACK: sent time delta to the latest delivered packet (time domain)
 *
 * The advantage of RACK is it applies to both original and retransmitted
 * packet and therefore is robust against tail losses. Another advantage
 * is being more resilient to reordering by simply allowing some
 * "settling delay", instead of tweaking the dupthresh.
 *
 * When tcp_rack_detect_loss() detects some packets are lost and we
 * are not already in the CA_Recovery state, either tcp_rack_reo_timeout()
 * or tcp_time_to_recover()'s "Trick#1: the loss is proven" code path will
 * make us enter the CA_Recovery state.
 */
unsafe fn tcp_rack_detect_loss(sk: *mut sock, reo_timeout: *mut u32) {
    let tp = tcp_sk(sk);
    let mut skb: *mut sk_buff;
    let mut n: *mut sk_buff;
    let reo_wnd: u32;

    *reo_timeout = 0;
    reo_wnd = tcp_rack_reo_wnd(sk);
    list_for_each_entry_safe!(skb, n, &mut (*tp).tsorted_sent_queue, tcp_tsorted_anchor, {
        let scb = TCP_SKB_CB(skb);
        let mut remaining: i32;

        /* Skip ones marked lost but not yet retransmitted */
        if ((*scb).sacked & TCPCB_LOST) != 0 && ((*scb).sacked & TCPCB_SACKED_RETRANS) == 0 {
            continue;
        }

        if !tcp_skb_sent_after((*tp).rack.mstamp, tcp_skb_timestamp_us(skb),
                               (*tp).rack.end_seq, (*scb).end_seq) {
            break;
        }

        /* A packet is lost if it has not been s/acked beyond
         * the recent RTT plus the reordering window.
         */
        remaining = tcp_rack_skb_timeout(tp, skb, reo_wnd);
        if remaining <= 0 {
            tcp_mark_skb_lost(sk, skb);
            list_del_init(&mut (*skb).tcp_tsorted_anchor);
        } else {
            /* Record maximum wait time */
            *reo_timeout = core::cmp::max(*reo_timeout, remaining as u32);
        }
    });
}

pub unsafe fn tcp_rack_mark_lost(sk: *mut sock) -> bool {
    let tp = tcp_sk(sk);
    let mut timeout: u32;

    if !(*tp).rack.advanced {
        return false;
    }

    /* Reset the advanced flag to avoid unnecessary queue scanning */
    (*tp).rack.advanced = false;
    tcp_rack_detect_loss(sk, &mut timeout);
    if timeout != 0 {
        timeout = usecs_to_jiffies(timeout + TCP_TIMEOUT_MIN_US);
        inet_csk_reset_xmit_timer(sk, ICSK_TIME_REO_TIMEOUT, timeout,
                                  (*inet_csk(sk)).icsk_rto);
    }
    timeout != 0
}

/* We have waited long enough to accommodate reordering. Mark the expired
 * packets lost and retransmit them.
 */
pub unsafe fn tcp_rack_reo_timeout(sk: *mut sock) {
    let tp = tcp_sk(sk);
    let mut timeout: u32;
    let prior_inflight: u32;
    let lost = (*tp).lost;

    prior_inflight = tcp_packets_in_flight(tp);
    tcp_rack_detect_loss(sk, &mut timeout);
    if prior_inflight != tcp_packets_in_flight(tp) {
        if (*inet_csk(sk)).icsk_ca_state != TCP_CA_Recovery {
            tcp_enter_recovery(sk, false);
            if (*(*inet_csk(sk)).icsk_ca_ops).cong_control.is_none() {
                tcp_cwnd_reduction(sk, 1, (*tp).lost - lost, 0);
            }
        }
        tcp_xmit_retransmit_queue(sk);
    }
    if (*inet_csk(sk)).icsk_pending != ICSK_TIME_RETRANS {
        tcp_rearm_rto(sk);
    }
}

/* RFC6582 NewReno recovery for non-SACK connection. It simply retransmits
 * the next unacked packet upon receiving
 * a) three or more DUPACKs to start the fast recovery
 * b) an ACK acknowledging new data during the fast recovery.
 */
pub unsafe fn tcp_newreno_mark_lost(sk: *mut sock, snd_una_advanced: bool) {
    let state: u8 = (*inet_csk(sk)).icsk_ca_state;
    let tp = tcp_sk(sk);

    if (state < TCP_CA_Recovery && (*tp).sacked_out >= (*tp).reordering)
        || (state == TCP_CA_Recovery && snd_una_advanced)
    {
        let skb = tcp_rtx_queue_head(sk);
        let mss: u32;

        if ((*TCP_SKB_CB(skb)).sacked & TCPCB_LOST) != 0 {
            return;
        }

        mss = tcp_skb_mss(skb);
        if tcp_skb_pcount(skb) > 1 && (*skb).len > mss {
            tcp_fragment(sk, TCP_FRAG_IN_RTX_QUEUE, skb, mss, mss, GFP_ATOMIC);
        }

        tcp_mark_skb_lost(sk, skb);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
