// SPDX-License-Identifier: GPL-2.0-or-later
/* RxRPC packet reception */

// Dependency declarations from ar-internal.h are intentionally external.

static mut RXRPC_INPUT_PACKET_ON_CONN: Option<unsafe extern "C" fn(*mut rxrpc_connection, *mut sockaddr_rxrpc, *mut sk_buff) -> i32> = None;

pub unsafe extern "C" fn rxrpc_encap_rcv(udp_sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let local = rcu_dereference_sk_user_data(udp_sk);
    if local.is_null() { kfree_skb(skb); return 0; }
    let io_thread = READ_ONCE((*local).io_thread);
    if io_thread.is_null() { kfree_skb(skb); return 0; }
    if (*skb).tstamp == 0 { (*skb).tstamp = ktime_get_real(); }
    (*skb).mark = RXRPC_SKB_MARK_PACKET;
    rxrpc_new_skb(skb, rxrpc_skb_new_encap_rcv);
    let mut rx_queue = &mut (*local).rx_queue as *mut sk_buff_head;
    #[cfg(CONFIG_AF_RXRPC_INJECT_RX_DELAY)]
    if rxrpc_inject_rx_delay != 0 || !skb_queue_empty(&(*local).rx_delay_queue) {
        (*skb).tstamp = ktime_add_ms((*skb).tstamp, rxrpc_inject_rx_delay);
        rx_queue = &mut (*local).rx_delay_queue;
    }
    skb_queue_tail(rx_queue, skb);
    wake_up_process(io_thread);
    0
}

pub unsafe extern "C" fn rxrpc_error_report(sk: *mut sock) {
    rcu_read_lock();
    let local = rcu_dereference_sk_user_data(sk);
    if local.is_null() { rcu_read_unlock(); return; }
    loop {
        let skb = skb_dequeue(&mut (*sk).sk_error_queue);
        if skb.is_null() { break; }
        (*skb).mark = RXRPC_SKB_MARK_ERROR;
        rxrpc_new_skb(skb, rxrpc_skb_new_error_report);
        skb_queue_tail(&mut (*local).rx_queue, skb);
    }
    rxrpc_wake_up_io_thread(local);
    rcu_read_unlock();
}

pub unsafe extern "C" fn rxrpc_direct_abort(skb: *mut sk_buff, why: rxrpc_abort_reason, abort_code: i32, err: i32) -> bool {
    let sp = rxrpc_skb(skb);
    trace_rxrpc_abort(0, why, (*sp).hdr.cid, (*sp).hdr.callNumber, (*sp).hdr.seq, abort_code, err);
    (*skb).mark = RXRPC_SKB_MARK_REJECT_ABORT;
    (*skb).priority = abort_code;
    false
}

pub unsafe extern "C" fn rxrpc_direct_conn_abort(skb: *mut sk_buff, why: rxrpc_abort_reason, abort_code: i32, err: i32) -> bool {
    let sp = rxrpc_skb(skb);
    trace_rxrpc_abort(0, why, (*sp).hdr.cid, 0, (*sp).hdr.seq, abort_code, err);
    (*skb).mark = RXRPC_SKB_MARK_REJECT_CONN_ABORT;
    (*skb).priority = abort_code;
    false
}

unsafe fn rxrpc_bad_message(skb: *mut sk_buff, why: rxrpc_abort_reason) -> bool { rxrpc_direct_abort(skb, why, RX_PROTOCOL_ERROR, -EBADMSG) }

unsafe fn rxrpc_input_version(local: *mut rxrpc_local, skb: *mut sk_buff) -> bool {
    let sp = rxrpc_skb(skb); let mut v: i8 = 0;
    _enter(""); rxrpc_see_skb(skb, rxrpc_skb_see_version);
    if skb_copy_bits(skb, size_of::<rxrpc_wire_header>(), &mut v as *mut _ as *mut _, 1) >= 0 && v == 0 { rxrpc_send_version_request(local, &(*sp).hdr, skb); }
    true
}

unsafe fn rxrpc_extract_header(sp: *mut rxrpc_skb_priv, skb: *mut sk_buff) -> bool {
    let mut whdr: rxrpc_wire_header = zeroed(); let mut ack: rxrpc_ackpacket = zeroed();
    if skb_copy_bits(skb, 0, &mut whdr as *mut _ as *mut _, size_of::<rxrpc_wire_header>()) < 0 { return rxrpc_bad_message(skb, rxrpc_badmsg_short_hdr); }
    memset(sp as *mut _, 0, size_of::<rxrpc_skb_priv>());
    (*sp).hdr.epoch = u32::from_be(whdr.epoch); (*sp).hdr.cid = u32::from_be(whdr.cid); (*sp).hdr.callNumber = u32::from_be(whdr.callNumber);
    (*sp).hdr.seq = u32::from_be(whdr.seq); (*sp).hdr.serial = u32::from_be(whdr.serial); (*sp).hdr.flags = whdr.flags; (*sp).hdr.type_ = whdr.type_;
    (*sp).hdr.userStatus = whdr.userStatus; (*sp).hdr.securityIndex = whdr.securityIndex; (*sp).hdr._rsvd = u16::from_be(whdr._rsvd); (*sp).hdr.serviceId = u16::from_be(whdr.serviceId);
    if (*sp).hdr.type_ == RXRPC_PACKET_TYPE_ACK {
        if skb_copy_bits(skb, size_of::<rxrpc_wire_header>(), &mut ack as *mut _ as *mut _, size_of::<rxrpc_ackpacket>()) < 0 { return rxrpc_bad_message(skb, rxrpc_badmsg_short_ack); }
        (*sp).ack.first_ack = u32::from_be(ack.firstPacket); (*sp).ack.prev_ack = u32::from_be(ack.previousPacket); (*sp).ack.acked_serial = u32::from_be(ack.serial); (*sp).ack.reason = ack.reason; (*sp).ack.nr_acks = ack.nAcks;
    } true
}

unsafe fn rxrpc_extract_abort(skb: *mut sk_buff) -> bool { let mut wtmp: u32 = 0; if skb_copy_bits(skb, size_of::<rxrpc_wire_header>(), &mut wtmp as *mut _ as *mut _, size_of::<u32>()) < 0 { return false; } (*skb).priority = u32::from_be(wtmp) as i32; true }

unsafe fn rxrpc_input_packet(local: *mut rxrpc_local, skb: *mut sk_buff) -> bool {
    skb_pull(skb, size_of::<udphdr>()); let sp = rxrpc_skb(skb); if !rxrpc_extract_header(sp, skb) { return true; }
    trace_rxrpc_rx_packet(sp);
    match (*sp).hdr.type_ {
        RXRPC_PACKET_TYPE_VERSION => { if rxrpc_to_client(sp) { return true; } return rxrpc_input_version(local, skb); }
        RXRPC_PACKET_TYPE_BUSY | RXRPC_PACKET_TYPE_ACK | RXRPC_PACKET_TYPE_ACKALL => { if (*sp).hdr.callNumber == 0 { return rxrpc_bad_message(skb, rxrpc_badmsg_zero_call); } }
        RXRPC_PACKET_TYPE_ABORT => { if !rxrpc_extract_abort(skb) { return true; } }
        RXRPC_PACKET_TYPE_DATA => { if (*sp).hdr.callNumber == 0 { return rxrpc_bad_message(skb, rxrpc_badmsg_zero_call); } if (*sp).hdr.seq == 0 { return rxrpc_bad_message(skb, rxrpc_badmsg_zero_seq); } }
        RXRPC_PACKET_TYPE_CHALLENGE => { if rxrpc_to_server(sp) { return true; } }
        RXRPC_PACKET_TYPE_RESPONSE => { if rxrpc_to_client(sp) { return true; } }
        RXRPC_PACKET_TYPE_PARAMS | RXRPC_PACKET_TYPE_10 | RXRPC_PACKET_TYPE_11 => return true,
        _ => return rxrpc_bad_message(skb, rxrpc_badmsg_unsupported_packet),
    }
    if (*sp).hdr.serviceId == 0 { return rxrpc_bad_message(skb, rxrpc_badmsg_zero_service); }
    let mut peer_srx: sockaddr_rxrpc = zeroed(); if rxrpc_extract_addr_from_skb(&mut peer_srx, skb) < 0 { return true; }
    let conn = if rxrpc_to_client(sp) { rxrpc_find_client_connection_rcu(local, &mut peer_srx, skb) } else { ptr::null_mut() };
    if !conn.is_null() { let ret = rxrpc_input_packet_on_conn(conn, &mut peer_srx, skb); rxrpc_put_connection(conn, rxrpc_conn_put_call_input); return ret; }
    if rxrpc_to_client(sp) { return rxrpc_protocol_error(skb, rxrpc_eproto_no_client_conn); }
    rxrpc_new_incoming_call(local, ptr::null_mut(), ptr::null_mut(), &mut peer_srx, skb)
}

unsafe fn rxrpc_input_packet_on_conn(conn: *mut rxrpc_connection, peer_srx: *mut sockaddr_rxrpc, skb: *mut sk_buff) -> bool {
    let sp = rxrpc_skb(skb); if (*sp).hdr.securityIndex != (*conn).security_ix { return rxrpc_direct_abort(skb, rxrpc_eproto_wrong_security, RXKADINCONSISTENCY, -EBADMSG); }
    if (*sp).hdr.serviceId != (*conn).service_id { return rxrpc_protocol_error(skb, rxrpc_eproto_reupgrade); }
    if (*sp).hdr.serial > (*conn).hi_serial { (*conn).hi_serial = (*sp).hdr.serial; }
    if (*sp).hdr.callNumber == 0 { return rxrpc_input_conn_packet(conn, skb); }
    let channel = ((*sp).hdr.cid & RXRPC_CHANNELMASK) as usize; let chan = &mut (*conn).channels[channel];
    if (*sp).hdr.callNumber < chan.last_call { return true; }
    if (*sp).hdr.callNumber == chan.last_call { if !chan.call.is_null() || (*sp).hdr.type_ == RXRPC_PACKET_TYPE_ABORT { return true; } rxrpc_conn_retransmit_call(conn, skb, channel as u32); return true; }
    let call = rxrpc_try_get_call(chan.call, rxrpc_call_get_input); if call.is_null() { if rxrpc_to_client(sp) { return rxrpc_protocol_error(skb, rxrpc_eproto_no_client_call); } return rxrpc_new_incoming_call((*conn).local, (*conn).peer, conn, peer_srx, skb); }
    rxrpc_queue_rx_call_packet(call, skb); rxrpc_put_call(call, rxrpc_call_put_input); true
}

pub unsafe extern "C" fn rxrpc_io_thread(data: *mut core::ffi::c_void) -> i32 {
    let local = data as *mut rxrpc_local; complete(&mut (*local).io_thread_ready); let mut rx_queue: sk_buff_head = zeroed(); skb_queue_head_init(&mut rx_queue); set_user_nice(current(), MIN_NICE);
    loop { rxrpc_inc_stat((*local).rxnet, stat_io_loop); while let Some(skb) = nonnull_skb(__skb_dequeue(&mut rx_queue)) { if (*skb).mark == RXRPC_SKB_MARK_PACKET { (*skb).priority = 0; if !rxrpc_input_packet(local, skb) { rxrpc_reject_packet(local, skb); } rxrpc_free_skb(skb, rxrpc_skb_put_input); } else { rxrpc_free_skb(skb, rxrpc_skb_put_unknown); } } if kthread_should_stop() { break; } schedule(); }
    __set_current_state(TASK_RUNNING); rxrpc_see_local(local, rxrpc_local_stop); rxrpc_destroy_local(local); WRITE_ONCE((*local).io_thread, ptr::null_mut()); rxrpc_see_local(local, rxrpc_local_stopped); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
