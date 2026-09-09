/* SPDX-License-Identifier: GPL-2.0 */

// External kernel types, functions, and constants referenced by this header:
// `sock`, `tcp_sock`, `tcp_ca_event`, `tcp_sk`, `inet_csk`, `__tcp_send_ack`,
// `TCP_ECN_DEMAND_CWR`, `CA_EVENT_ECN_IS_CE`, `ICSK_ACK_TIMER`, and
// `ICSK_ACK_NOW`.

pub unsafe fn dctcp_ece_ack_cwr(sk: *mut sock, ce_state: u32) {
    let tp = tcp_sk(sk);

    if ce_state == 1 {
        (*tp).ecn_flags |= TCP_ECN_DEMAND_CWR;
    } else {
        (*tp).ecn_flags &= !TCP_ECN_DEMAND_CWR;
    }
}

/* Minimal DCTP CE state machine:
 *
 * S:\t0 <- last pkt was non-CE
 *\t1 <- last pkt was CE
 */
pub unsafe fn dctcp_ece_ack_update(
    sk: *mut sock,
    evt: tcp_ca_event,
    prior_rcv_nxt: *mut u32,
    ce_state: *mut u32,
) {
    let new_ce_state: u32 = if evt == CA_EVENT_ECN_IS_CE { 1 } else { 0 };

    if *ce_state != new_ce_state {
        /* CE state has changed, force an immediate ACK to
         * reflect the new CE state. If an ACK was delayed,
         * send that first to reflect the prior CE state.
         */
        if ((*inet_csk(sk)).icsk_ack.pending & ICSK_ACK_TIMER) != 0 {
            dctcp_ece_ack_cwr(sk, *ce_state);
            __tcp_send_ack(sk, *prior_rcv_nxt, 0);
        }
        (*inet_csk(sk)).icsk_ack.pending |= ICSK_ACK_NOW;
    }
    *prior_rcv_nxt = (*tcp_sk(sk)).rcv_nxt;
    *ce_state = new_ce_state;
    dctcp_ece_ack_cwr(sk, new_ce_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
