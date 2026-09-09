// SPDX-License-Identifier: GPL-2.0-or-later
/* Management of Tx window, Tx resend, ACKs and out-of-sequence reception */

pub unsafe fn rxrpc_propose_ping(call: *mut rxrpc_call, serial: u32,
                                 why: rxrpc_propose_ack_trace) {
    let delay = ms_to_ktime(READ_ONCE(rxrpc_idle_ack_delay));
    let now = ktime_get_real();
    let ping_at = ktime_add(now, delay);
    trace_rxrpc_propose_ack(call, why, RXRPC_ACK_PING, serial);
    if ktime_before(ping_at, (*call).ping_at) {
        (*call).ping_at = ping_at;
        trace_rxrpc_timer_set(call, delay, rxrpc_timer_trace_ping);
    }
}

pub unsafe fn rxrpc_propose_delay_ACK(call: *mut rxrpc_call, serial: rxrpc_serial_t,
                                      why: rxrpc_propose_ack_trace) {
    let now = ktime_get_real();
    let mut delay;
    trace_rxrpc_propose_ack(call, why, RXRPC_ACK_DELAY, serial);
    if (*call).srtt_us != 0 { delay = ((*call).srtt_us >> 3) * NSEC_PER_USEC; }
    else { delay = ms_to_ktime(READ_ONCE(rxrpc_soft_ack_delay)); }
    ktime_add_ms(delay, (*call).tx_backoff);
    (*call).delay_ack_at = ktime_add(now, delay);
    trace_rxrpc_timer_set(call, delay, rxrpc_timer_trace_delayed_ack);
}

unsafe fn rxrpc_retransmit_data(call: *mut rxrpc_call, req: *mut rxrpc_send_data_req) -> bool {
    let tq = (*req).tq;
    let ix = (*req).seq & RXRPC_TXQ_MASK;
    let txb = (*tq).bufs[ix];
    _enter!("%x,%x,%x,%x", (*tq).qbase, (*req).seq, ix, (*txb).debug_id);
    (*req).retrans = true;
    trace_rxrpc_retransmit(call, req, txb);
    (*txb).flags |= RXRPC_TXBUF_RESENT;
    rxrpc_send_data_packet(call, req);
    rxrpc_inc_stat((*call).rxnet, stat_tx_data_retrans);
    (*req).tq = core::ptr::null_mut(); (*req).n = 0; (*req).did_send = true;
    (*req).now = ktime_get_real();
    true
}

unsafe fn rxrpc_resend(call: *mut rxrpc_call) {
    let mut req = rxrpc_send_data_req { now: ktime_get_real(), trace: rxrpc_txdata_retransmit, ..core::mem::zeroed() };
    _enter!("{%d,%d}", (*call).tx_bottom, (*call).tx_top);
    trace_rxrpc_resend(call, (*call).acks_highest_serial);
    let mut tq = (*call).tx_queue;
    while !tq.is_null() {
        let mut lost = (*tq).segment_lost;
        if after((*tq).qbase, (*call).tx_transmitted) { break; }
        _debug!("retr %16lx %u c=%08x [%x]", (*tq).segment_acked, (*tq).nr_reported_acks, (*call).debug_id, (*tq).qbase);
        _debug!("lost %16lx", lost);
        trace_rxrpc_resend_lost(call, tq, lost);
        while lost != 0 {
            let ix = __ffs(lost);
            let txb = (*tq).bufs[ix];
            __clear_bit(ix, &mut lost);
            rxrpc_see_txbuf(txb, rxrpc_txbuf_see_lost);
            (*(&mut req)).tq = tq; req.seq = (*tq).qbase + ix; req.n = 1;
            rxrpc_retransmit_data(call, &mut req);
        }
        tq = (*tq).next;
    }
    rxrpc_get_rto_backoff(call, req.did_send);
    _leave!("");
}

pub unsafe fn rxrpc_resend_tlp(call: *mut rxrpc_call) {
    let mut req = rxrpc_send_data_req { now: ktime_get_real(), seq: (*call).tx_transmitted, n: 1, tlp_probe: true, trace: rxrpc_txdata_tlp_retransmit, ..core::mem::zeroed() };
    req.tq = READ_ONCE((*call).tx_qtail);
    if !req.tq.is_null() && before((*call).tx_transmitted, (*req.tq).qbase + RXRPC_NR_TXQUEUE) { rxrpc_retransmit_data(call, &mut req); return; }
    req.tq = (*call).tx_queue;
    while !req.tq.is_null() {
        if after_eq((*call).tx_transmitted, (*req.tq).qbase) && before((*call).tx_transmitted, (*req.tq).qbase + RXRPC_NR_TXQUEUE) { rxrpc_retransmit_data(call, &mut req); return; }
        req.tq = (*req.tq).next;
    }
}

unsafe fn rxrpc_begin_service_reply(call: *mut rxrpc_call) {
    rxrpc_set_call_state(call, RXRPC_CALL_SERVER_SEND_REPLY);
    if (*call).ackr_reason == RXRPC_ACK_DELAY { (*call).ackr_reason = 0; }
    (*call).delay_ack_at = KTIME_MAX;
    trace_rxrpc_timer_can(call, rxrpc_timer_trace_delayed_ack);
}

unsafe fn rxrpc_close_tx_phase(call: *mut rxrpc_call) {
    _debug!("________awaiting reply/ACK__________");
    match __rxrpc_call_state(call) {
        RXRPC_CALL_CLIENT_SEND_REQUEST => rxrpc_set_call_state(call, RXRPC_CALL_CLIENT_AWAIT_ACK),
        RXRPC_CALL_SERVER_SEND_REPLY => rxrpc_set_call_state(call, RXRPC_CALL_SERVER_AWAIT_ACK),
        _ => (),
    }
}

unsafe fn rxrpc_transmit_fresh_data(call: *mut rxrpc_call, limit: u32, trace: rxrpc_txdata_trace) {
    let mut space = rxrpc_tx_window_space(call);
    if !test_bit(RXRPC_CALL_EXPOSED, &(*call).flags) {
        if (*call).send_top == (*call).tx_top { return; }
        rxrpc_expose_client_call(call);
    }
    while space > 0 {
        let mut req: rxrpc_send_data_req = core::mem::zeroed();
        req.now = ktime_get_real(); req.seq = (*call).tx_transmitted + 1; req.trace = trace;
        let mut tq = (*call).tx_qtail; let mut seq = (*call).tx_top;
        let max = min(space, max((*(*call).peer).pmtud_jumbo, 1));
        let send_top = smp_load_acquire(&(*call).send_top);
        if (*call).tx_top == send_top { break; }
        trace_rxrpc_transmit(call, send_top, space);
        let mut txb;
        loop {
            seq += 1; let ix = seq & RXRPC_TXQ_MASK;
            if ix == 0 { tq = (*tq).next; trace_rxrpc_tq(call, tq, seq, rxrpc_tq_decant_advance); }
            if req.tq.is_null() { req.tq = tq; }
            txb = (*tq).bufs[ix]; req.n += 1;
            if !(*txb).jumboable || req.n >= max || !before(seq, send_top) { break; }
        }
        if __rxrpc_call_state(call) == RXRPC_CALL_CLIENT_PRE_SEND { rxrpc_set_call_state(call, RXRPC_CALL_CLIENT_SEND_REQUEST); }
        if (*txb).flags & RXRPC_LAST_PACKET != 0 { rxrpc_close_tx_phase(call); tq = core::ptr::null_mut(); }
        (*call).tx_qtail = tq; (*call).tx_top = seq; space -= req.n; rxrpc_send_data_packet(call, &mut req);
    }
}

pub unsafe fn rxrpc_transmit_some_data(call: *mut rxrpc_call, limit: u32, trace: rxrpc_txdata_trace) {
    match __rxrpc_call_state(call) {
        RXRPC_CALL_SERVER_ACK_REQUEST => { if (*call).tx_bottom == READ_ONCE((*call).send_top) { return; } rxrpc_begin_service_reply(call); }
        RXRPC_CALL_SERVER_SEND_REPLY | RXRPC_CALL_CLIENT_PRE_SEND | RXRPC_CALL_CLIENT_SEND_REQUEST => {
            if rxrpc_tx_window_space(call) == 0 { return; }
            if (*call).tx_bottom == READ_ONCE((*call).send_top) { rxrpc_inc_stat((*call).rxnet, stat_tx_data_underflow); return; }
            rxrpc_transmit_fresh_data(call, limit, trace);
        }
        _ => return,
    }
}

unsafe fn rxrpc_send_initial_ping(call: *mut rxrpc_call) {
    if (*call).rtt_count < 3 || ktime_before(ktime_add_ms((*call).rtt_last_req, 1000), ktime_get_real()) { rxrpc_send_ACK(call, RXRPC_ACK_PING, 0, rxrpc_propose_ack_ping_for_params); }
}

pub unsafe fn rxrpc_input_call_event(call: *mut rxrpc_call) -> bool {
    let mut did_receive = false; let mut saw_ack = false;
    rxrpc_see_call(call, rxrpc_call_see_input);
    _enter!("{%d,%s,%lx}", (*call).debug_id, rxrpc_call_states[__rxrpc_call_state(call)], (*call).events);
    let abort_code = smp_load_acquire(&(*call).send_abort);
    if abort_code != 0 { rxrpc_abort_call(call, 0, (*call).send_abort, (*call).send_abort_err, (*call).send_abort_why); return rxrpc_input_call_event_out(call, did_receive); }
    loop {
        let skb = __skb_dequeue(&mut (*call).rx_queue);
        if !skb.is_null() {
            let sp = rxrpc_skb(skb);
            if __rxrpc_call_is_complete(call) || (*skb).mark == RXRPC_SKB_MARK_ERROR { rxrpc_free_skb(skb, rxrpc_skb_put_call_rx); return rxrpc_input_call_event_out(call, did_receive); }
            saw_ack |= (*sp).hdr.r#type == RXRPC_PACKET_TYPE_ACK;
            rxrpc_input_call_packet(call, skb); rxrpc_free_skb(skb, rxrpc_skb_put_call_rx); did_receive = true;
        }
        let t = ktime_sub((*call).rack_timo_at, ktime_get_real());
        if t <= 0 { trace_rxrpc_timer_exp(call, t, rxrpc_timer_trace_rack_off + (*call).rack_timer_mode); (*call).rack_timo_at = KTIME_MAX; rxrpc_rack_timer_expired(call, t); }
        if skb_queue_empty(&(*call).rx_queue) { break; }
    }
    let now = ktime_get_real();
    macro_rules! expired { ($timer:expr, $trace:expr) => { let t = ktime_sub($timer, now); if t <= 0 { trace_rxrpc_timer_exp(call, t, $trace); return rxrpc_input_call_event_expired(call, did_receive); } }; }
    expired!((*call).expect_rx_by, rxrpc_timer_trace_expect_rx);
    let t = ktime_sub((*call).expect_req_by, now); if t <= 0 { (*call).expect_req_by = KTIME_MAX; if __rxrpc_call_state(call) == RXRPC_CALL_SERVER_RECV_REQUEST { trace_rxrpc_timer_exp(call, t, rxrpc_timer_trace_idle); return rxrpc_input_call_event_expired(call, did_receive); } }
    expired!(READ_ONCE((*call).expect_term_by), rxrpc_timer_trace_hard);
    let t = ktime_sub((*call).delay_ack_at, now); if t <= 0 { trace_rxrpc_timer_exp(call, t, rxrpc_timer_trace_delayed_ack); (*call).delay_ack_at = KTIME_MAX; rxrpc_send_ACK(call, RXRPC_ACK_DELAY, 0, rxrpc_propose_ack_delayed_ack); }
    let t = ktime_sub((*call).ping_at, now); if t <= 0 { trace_rxrpc_timer_exp(call, t, rxrpc_timer_trace_ping); (*call).ping_at = KTIME_MAX; rxrpc_send_ACK(call, RXRPC_ACK_PING, 0, rxrpc_propose_ack_ping_for_keepalive); }
    let now = ktime_get_real(); let t = ktime_sub((*call).keepalive_at, now); if t <= 0 { trace_rxrpc_timer_exp(call, t, rxrpc_timer_trace_keepalive); (*call).keepalive_at = KTIME_MAX; rxrpc_send_ACK(call, RXRPC_ACK_PING, 0, rxrpc_propose_ack_ping_for_keepalive); }
    if test_and_clear_bit(RXRPC_CALL_EV_INITIAL_PING, &mut (*call).events) { rxrpc_send_initial_ping(call); }
    rxrpc_transmit_some_data(call, u32::MAX, rxrpc_txdata_new_data);
    if saw_ack { rxrpc_congestion_degrade(call); }
    if did_receive && (__rxrpc_call_state(call) == RXRPC_CALL_CLIENT_SEND_REQUEST || __rxrpc_call_state(call) == RXRPC_CALL_SERVER_SEND_REPLY) { trace_rxrpc_rack(call, ktime_sub((*call).rack_timo_at, ktime_get_real())); }
    if test_and_clear_bit(RXRPC_CALL_EV_ACK_LOST, &mut (*call).events) { rxrpc_send_ACK(call, RXRPC_ACK_PING, 0, rxrpc_propose_ack_ping_for_lost_ack); }
    if (*call).tx_nr_lost > 0 && __rxrpc_call_state(call) != RXRPC_CALL_CLIENT_RECV_REPLY && !test_bit(RXRPC_CALL_TX_ALL_ACKED, &(*call).flags) { rxrpc_resend(call); }
    if test_and_clear_bit(RXRPC_CALL_RX_IS_IDLE, &mut (*call).flags) { rxrpc_send_ACK(call, RXRPC_ACK_IDLE, 0, rxrpc_propose_ack_rx_idle); }
    if (*call).ackr_nr_unacked > 2 { if (*call).rtt_count < 3 { rxrpc_send_ACK(call, RXRPC_ACK_PING, 0, rxrpc_propose_ack_ping_for_rtt); } else if ktime_before(ktime_add_ms((*call).rtt_last_req, 1000), ktime_get_real()) { rxrpc_send_ACK(call, RXRPC_ACK_PING, 0, rxrpc_propose_ack_ping_for_old_rtt); } else { rxrpc_send_ACK(call, RXRPC_ACK_IDLE, 0, rxrpc_propose_ack_input_data); } }
    if !__rxrpc_call_is_complete(call) {
        let mut next = READ_ONCE((*call).expect_term_by);
        if ktime_before((*call).expect_req_by, next) { next = (*call).expect_req_by; }
        if ktime_before((*call).expect_rx_by, next) { next = (*call).expect_rx_by; }
        if ktime_before((*call).delay_ack_at, next) { next = (*call).delay_ack_at; }
        if ktime_before((*call).rack_timo_at, next) { next = (*call).rack_timo_at; }
        if ktime_before((*call).keepalive_at, next) { next = (*call).keepalive_at; }
        if ktime_before((*call).ping_at, next) { next = (*call).ping_at; }
        let now = ktime_get_real(); let delay = ktime_sub(next, now);
        if delay <= 0 { rxrpc_poke_call(call, rxrpc_call_poke_timer_now); }
        else {
            let nowj = jiffies; let delayj = umax(nsecs_to_jiffies(delay), 1); let nextj = nowj + delayj;
            if time_before(nextj, (*call).timer.expires) || !timer_pending(&(*call).timer) { trace_rxrpc_timer_restart(call, delay, delayj); timer_reduce(&mut (*call).timer, nextj); }
        }
    }
    rxrpc_input_call_event_out(call, did_receive)
}

unsafe fn rxrpc_input_call_event_expired(call: *mut rxrpc_call, did_receive: bool) -> bool {
    if test_bit(RXRPC_CALL_RX_HEARD, &(*call).flags) && (*call).conn.hi_serial as i32 - (*call).rx_serial as i32 > 0 { trace_rxrpc_call_reset(call); rxrpc_abort_call(call, 0, RX_CALL_DEAD, -ECONNRESET, rxrpc_abort_call_reset); } else { rxrpc_abort_call(call, 0, RX_CALL_TIMEOUT, -ETIME, rxrpc_abort_call_timeout); }
    rxrpc_input_call_event_out(call, did_receive)
}

unsafe fn rxrpc_input_call_event_out(call: *mut rxrpc_call, did_receive: bool) -> bool {
    if __rxrpc_call_is_complete(call) { timer_delete_sync(&mut (*call).timer); if !test_bit(RXRPC_CALL_DISCONNECTED, &(*call).flags) { rxrpc_disconnect_call(call); } if !(*call).security.is_null() { ((*(*call).security).free_call_crypto)(call); } }
    else if did_receive && (*(*call).peer).ackr_adv_pmtud && (*(*call).peer).pmtud_pending { rxrpc_send_probe_for_pmtud(call); }
    _leave!(""); true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
