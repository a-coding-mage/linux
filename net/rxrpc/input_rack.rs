// SPDX-License-Identifier: GPL-2.0-or-later
/* RACK-TLP [RFC8958] Implementation
 *
 * Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependency declarations supplied by ar-internal.h are intentionally external.

unsafe fn rxrpc_rack_sent_after(t1: ktime_t, seq1: rxrpc_seq_t,
                                t2: ktime_t, seq2: rxrpc_seq_t) -> bool {
    if ktime_after(t1, t2) { return true; }
    t1 == t2 && after(seq1, seq2)
}

unsafe fn rxrpc_rack_mark_lost(call: *mut rxrpc_call, tq: *mut rxrpc_txqueue, ix: c_uint) {
    if __test_and_set_bit(ix, &mut (*tq).segment_lost) {
        if __test_and_clear_bit(ix, &mut (*tq).segment_retransmitted) { (*call).tx_nr_resent -= 1; }
    } else { (*call).tx_nr_lost += 1; }
    (*tq).segment_xmit_ts[ix as usize] = UINT_MAX;
}

unsafe fn rxrpc_get_xmit_ts(tq: *const rxrpc_txqueue, ix: c_uint) -> ktime_t {
    if (*tq).segment_xmit_ts[ix as usize] == UINT_MAX { return KTIME_MAX; }
    ktime_add_us((*tq).xmit_ts_base, (*tq).segment_xmit_ts[ix as usize])
}

unsafe fn rxrpc_tq_nacks(tq: *const rxrpc_txqueue) -> c_ulong {
    let mut nacks = !(*tq).segment_acked;
    if (*tq).nr_reported_acks < RXRPC_NR_TXQUEUE { nacks &= (1UL << (*tq).nr_reported_acks) - 1; }
    nacks
}

unsafe fn rxrpc_rack_update(call: *mut rxrpc_call, _summary: *mut rxrpc_ack_summary,
                            tq: *mut rxrpc_txqueue, ix: c_uint) {
    let seq = (*tq).qbase + ix;
    let xmit_ts = rxrpc_get_xmit_ts(tq, ix);
    let rtt = ktime_sub((*call).acks_latest_ts, xmit_ts);
    if __test_and_clear_bit(ix, &mut (*tq).segment_lost) { (*call).tx_nr_lost -= 1; }
    if test_bit(ix, &(*tq).segment_retransmitted) {
        if before((*call).acks_highest_serial, (*tq).segment_serial[ix as usize]) { return; }
        if rtt < minmax_get(&(*call).min_rtt) { return; }
    }
    if ktime_after(xmit_ts, (*call).rack_rtt_ts) { (*call).rack_rtt = rtt; (*call).rack_rtt_ts = xmit_ts; }
    if rxrpc_rack_sent_after(xmit_ts, seq, (*call).rack_xmit_ts, (*call).rack_end_seq) {
        (*call).rack_rtt = rtt; (*call).rack_xmit_ts = xmit_ts; (*call).rack_end_seq = seq;
    }
}

unsafe fn rxrpc_rack_detect_reordering(call: *mut rxrpc_call, tq: *mut rxrpc_txqueue, ix: c_uint) {
    let seq = (*tq).qbase + ix;
    if after(seq, (*call).rack_fack) { (*call).rack_fack = seq; }
    else if before(seq, (*call).rack_fack) && test_bit(ix, &(*tq).segment_retransmitted) { (*call).rack_reordering_seen = true; }
}

pub unsafe fn rxrpc_input_rack_one(call: *mut rxrpc_call, summary: *mut rxrpc_ack_summary, tq: *mut rxrpc_txqueue, ix: c_uint) {
    rxrpc_rack_update(call, summary, tq, ix);
    rxrpc_rack_detect_reordering(call, tq, ix);
}

pub unsafe fn rxrpc_input_rack(call: *mut rxrpc_call, summary: *mut rxrpc_ack_summary, tq: *mut rxrpc_txqueue, mut new_acks: c_ulong) {
    while new_acks != 0 { let ix = __ffs(new_acks); __clear_bit(ix, &mut new_acks); rxrpc_input_rack_one(call, summary, tq, ix); }
    trace_rxrpc_rack_update(call, summary);
}

unsafe fn rxrpc_rack_update_reo_wnd(call: *mut rxrpc_call, summary: *mut rxrpc_ack_summary) -> ktime_t {
    let snd_una = (*call).acks_lowest_nak;
    let snd_nxt = (*call).tx_transmitted + 1;
    let have_dsack_option = (*summary).ack_reason == RXRPC_ACK_DUPLICATE;
    let dup_thresh = 3;
    if !(*call).rack_dsack_round_none && after_eq(snd_una, (*call).rack_dsack_round) { (*call).rack_dsack_round_none = true; }
    if (*call).rack_dsack_round_none && have_dsack_option { (*call).rack_dsack_round_none = false; (*call).rack_dsack_round = snd_nxt; (*call).rack_reo_wnd_mult += 1; (*call).rack_reo_wnd_persist = 16; }
    else if (*summary).exiting_fast_or_rto_recovery { (*call).rack_reo_wnd_persist -= 1; if (*call).rack_reo_wnd_persist <= 0 { (*call).rack_reo_wnd_mult = 1; } }
    if !(*call).rack_reordering_seen { if (*summary).in_fast_or_rto_recovery { return 0; } if (*call).acks_nr_sacks >= dup_thresh { return 0; } }
    us_to_ktime(umin((*call).rack_reo_wnd_mult * minmax_get(&(*call).min_rtt) / 4, (*call).srtt_us >> 3))
}

unsafe fn rxrpc_rack_detect_loss(call: *mut rxrpc_call, summary: *mut rxrpc_ack_summary) -> ktime_t {
    let mut timeout = 0; let lost_after; let now = ktime_get_real();
    (*call).rack_reo_wnd = rxrpc_rack_update_reo_wnd(call, summary); lost_after = ktime_add((*call).rack_rtt, (*call).rack_reo_wnd); trace_rxrpc_rack_scan_loss(call);
    let mut tq = (*call).tx_queue;
    while !tq.is_null() { let mut nacks = rxrpc_tq_nacks(tq); if after((*tq).qbase, (*call).tx_transmitted) { break; } trace_rxrpc_rack_scan_loss_tq(call, tq, nacks); nacks &= !(*tq).segment_lost | (*tq).segment_retransmitted;
        while nacks != 0 { let ix = __ffs(nacks); let seq = (*tq).qbase + ix; let xmit_ts = rxrpc_get_xmit_ts(tq, ix); __clear_bit(ix, &mut nacks); if rxrpc_rack_sent_after((*call).rack_xmit_ts, (*call).rack_end_seq, xmit_ts, seq) { let remaining = ktime_sub(ktime_add(xmit_ts, lost_after), now); if remaining <= 0 { rxrpc_rack_mark_lost(call, tq, ix); trace_rxrpc_rack_detect_loss(call, summary, seq); } else { timeout = max(remaining, timeout); } } }
        tq = (*tq).next;
    } timeout
}

pub unsafe fn rxrpc_rack_detect_loss_and_arm_timer(call: *mut rxrpc_call, summary: *mut rxrpc_ack_summary) { let timeout = rxrpc_rack_detect_loss(call, summary); if timeout != 0 { (*call).rack_timer_mode = RXRPC_CALL_RACKTIMER_RACK_REORDER; (*call).rack_timo_at = ktime_add(ktime_get_real(), timeout); trace_rxrpc_rack_timer(call, timeout, false); trace_rxrpc_timer_set(call, timeout, rxrpc_timer_trace_rack_reo); } }

unsafe fn rxrpc_rack_mark_losses_on_rto(call: *mut rxrpc_call) {
    let snd_una = (*call).acks_lowest_nak; let lost_after = ktime_add((*call).rack_rtt, (*call).rack_reo_wnd); let deadline = ktime_sub(ktime_get_real(), lost_after); let mut tq = (*call).tx_queue;
    while !tq.is_null() { let mut unacked = !(*tq).segment_acked; trace_rxrpc_rack_mark_loss_tq(call, tq); while unacked != 0 { let ix = __ffs(unacked); let seq = (*tq).qbase + ix; let xmit_ts = rxrpc_get_xmit_ts(tq, ix); if after(seq, (*call).tx_transmitted) { return; } __clear_bit(ix, &mut unacked); if seq == snd_una || ktime_before(xmit_ts, deadline) { rxrpc_rack_mark_lost(call, tq, ix); } } tq = (*tq).next; }
}

pub unsafe fn rxrpc_tlp_calc_pto(call: *mut rxrpc_call, now: ktime_t) -> ktime_t {
    let flight_size = rxrpc_tx_in_flight(call); let rto_at = ktime_add((*call).tx_last_sent, rxrpc_get_rto_backoff(call, false)); let mut pto;
    if (*call).rtt_count > 0 { pto = ns_to_ktime((*call).srtt_us * NSEC_PER_USEC / 4); if flight_size != 0 { pto = ktime_add(pto, (*call).tlp_max_ack_delay); } } else { pto = NSEC_PER_SEC; }
    if ktime_after(ktime_add(now, pto), rto_at) { pto = ktime_sub(rto_at, now); } pto
}

pub unsafe fn rxrpc_tlp_send_probe(call: *mut rxrpc_call) {
    let mut in_flight = rxrpc_tx_in_flight(call); if after_eq((*call).acks_hard_ack, (*call).tx_transmitted) { return; }
    if (*call).tlp_serial == 0 && (*call).tlp_rtt_taken != (*call).rtt_taken { (*call).tlp_is_retrans = false; if after((*call).send_top, (*call).tx_transmitted) && rxrpc_tx_window_space(call) > 0 { (*call).tx_last_serial = 0; rxrpc_transmit_some_data(call, 1, rxrpc_txdata_tlp_new_data); (*call).tlp_serial = (*call).tx_last_serial; (*call).tlp_seq = (*call).tx_transmitted; trace_rxrpc_tlp_probe(call, rxrpc_tlp_probe_trace_transmit_new); in_flight = rxrpc_tx_in_flight(call); } else { (*call).tx_last_serial = 0; rxrpc_resend_tlp(call); (*call).tlp_is_retrans = true; trace_rxrpc_tlp_probe(call, rxrpc_tlp_probe_trace_retransmit); } } else { trace_rxrpc_tlp_probe(call, rxrpc_tlp_probe_trace_busy); }
    if in_flight != 0 { let rto = rxrpc_get_rto_backoff(call, false); (*call).rack_timer_mode = RXRPC_CALL_RACKTIMER_RTO; (*call).rack_timo_at = ktime_add(ktime_get_real(), rto); trace_rxrpc_rack_timer(call, rto, false); trace_rxrpc_timer_set(call, rto, rxrpc_timer_trace_rack_rto); }
}

pub unsafe fn rxrpc_tlp_process_ack(call: *mut rxrpc_call, summary: *mut rxrpc_ack_summary) {
    if (*call).tlp_serial == 0 || after((*call).tlp_seq, (*call).acks_hard_ack) { return; }
    if !(*call).tlp_is_retrans { trace_rxrpc_tlp_ack(call, summary, rxrpc_tlp_ack_trace_new_data); (*call).tlp_serial = 0; }
    else if (*summary).ack_reason == RXRPC_ACK_DUPLICATE && (*summary).acked_serial == (*call).tlp_serial { trace_rxrpc_tlp_ack(call, summary, rxrpc_tlp_ack_trace_dup_acked); (*call).tlp_serial = 0; }
    else if after((*call).acks_hard_ack, (*call).tlp_seq) { trace_rxrpc_tlp_ack(call, summary, rxrpc_tlp_ack_trace_hard_beyond); (*call).tlp_serial = 0; }
    else if (*summary).tlp_probe_acked { trace_rxrpc_tlp_ack(call, summary, rxrpc_tlp_ack_trace_acked); (*call).tlp_serial = 0; }
    else { trace_rxrpc_tlp_ack(call, summary, rxrpc_tlp_ack_trace_incomplete); }
}

pub unsafe fn rxrpc_rack_timer_expired(call: *mut rxrpc_call, overran_by: ktime_t) {
    let mut summary: rxrpc_ack_summary = core::mem::zeroed(); let mode = (*call).rack_timer_mode; trace_rxrpc_rack_timer(call, overran_by, true); (*call).rack_timer_mode = RXRPC_CALL_RACKTIMER_OFF;
    match mode { RXRPC_CALL_RACKTIMER_RACK_REORDER => rxrpc_rack_detect_loss_and_arm_timer(call, &mut summary), RXRPC_CALL_RACKTIMER_TLP_PTO => rxrpc_tlp_send_probe(call), RXRPC_CALL_RACKTIMER_RTO => rxrpc_rack_mark_losses_on_rto(call), _ => pr_warn!("Unexpected rack timer %u", mode), }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
