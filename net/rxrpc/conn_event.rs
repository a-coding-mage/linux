// SPDX-License-Identifier: GPL-2.0-or-later
/* connection-level event handling
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Kernel and local dependencies are supplied by the surrounding translation unit. */

/* Set the completion state on an aborted connection. */
unsafe fn rxrpc_set_conn_aborted(
    conn: *mut rxrpc_connection,
    abort_code: i32,
    err: i32,
    compl: rxrpc_call_completion,
) -> bool {
    let mut aborted = false;
    if (*conn).state != RXRPC_CONN_ABORTED {
        spin_lock_irq(&mut (*conn).state_lock);
        if (*conn).state != RXRPC_CONN_ABORTED {
            (*conn).abort_code = abort_code;
            (*conn).error = err;
            (*conn).completion = compl;
            smp_store_release(&mut (*conn).state, RXRPC_CONN_ABORTED);
            set_bit(RXRPC_CONN_DONT_REUSE, &mut (*conn).flags);
            set_bit(RXRPC_CONN_EV_ABORT_CALLS, &mut (*conn).events);
            aborted = true;
        }
        spin_unlock_irq(&mut (*conn).state_lock);
    }
    aborted
}

/* Mark a socket buffer to indicate that the connection it's on should be aborted. */
pub unsafe fn rxrpc_abort_conn(
    conn: *mut rxrpc_connection, skb: *mut sk_buff, abort_code: i32, err: i32,
    why: rxrpc_abort_reason,
) -> i32 {
    let mut cid = (*conn).proto.cid;
    let mut call = 0u32;
    let mut seq = 0u32;
    if !skb.is_null() {
        let sp = rxrpc_skb(skb);
        cid = (*sp).hdr.cid;
        call = (*sp).hdr.callNumber;
        seq = (*sp).hdr.seq;
    }
    if rxrpc_set_conn_aborted(conn, abort_code, err, RXRPC_CALL_LOCALLY_ABORTED) {
        trace_rxrpc_abort(0, why, cid, call, seq, abort_code, err);
        rxrpc_poke_conn(conn, rxrpc_conn_get_poke_abort);
    }
    -EPROTO
}

/* Mark a connection as being remotely aborted. */
unsafe fn rxrpc_input_conn_abort(conn: *mut rxrpc_connection, skb: *mut sk_buff) {
    trace_rxrpc_rx_conn_abort(conn, skb);
    rxrpc_set_conn_aborted(conn, (*rxrpc_skb(skb)).priority, -ECONNABORTED,
                           RXRPC_CALL_REMOTELY_ABORTED);
}

/* Retransmit terminal ACK or ABORT of the previous call. */
pub unsafe fn rxrpc_conn_retransmit_call(conn: *mut rxrpc_connection, skb: *mut sk_buff,
                                         channel: u32) {
    let sp = if !skb.is_null() { rxrpc_skb(skb) } else { core::ptr::null_mut() };
    let chan: *mut rxrpc_channel;
    let mut pkt: rxrpc_retransmit_packet = core::mem::zeroed();
    let mut trailer: rxrpc_acktrailer = core::mem::zeroed();
    let mut padding = 0u32;
    let mut iov: [kvec; 3] = core::mem::zeroed();
    let mut msg: msghdr = core::mem::zeroed();
    let mut len: usize;
    let mut ioc: i32;
    let serial: u32;
    let mut max_mtu: u32;
    let mut if_mtu: u32;
    let call_id: u32;
    _enter!("%d", (*conn).debug_id);

    if !sp.is_null() && (*sp).hdr.type_ == RXRPC_PACKET_TYPE_ACK {
        if skb_copy_bits(skb, core::mem::size_of::<rxrpc_wire_header>(), &mut pkt.ack as *mut _ as *mut _, core::mem::size_of::<rxrpc_ackpacket>()) < 0 { return; }
        if pkt.ack.reason == RXRPC_ACK_PING_RESPONSE { return; }
    }
    chan = &mut (*conn).channels[channel as usize];
    call_id = (*chan).last_call;
    if !skb.is_null() && call_id != (*sp).hdr.callNumber { return; }

    msg.msg_name = &mut (*(*conn).peer).srx.transport as *mut _ as *mut _;
    msg.msg_namelen = (*(*conn).peer).srx.transport_len;
    iov[0].iov_base = &mut pkt as *mut _ as *mut _;
    iov[0].iov_len = core::mem::size_of::<rxrpc_wire_header>();
    iov[1].iov_base = &mut padding as *mut _ as *mut _;
    iov[1].iov_len = 3;
    iov[2].iov_base = &mut trailer as *mut _ as *mut _;
    iov[2].iov_len = core::mem::size_of::<rxrpc_acktrailer>();
    serial = rxrpc_get_next_serial(conn);
    pkt.whdr.epoch = htonl((*conn).proto.epoch);
    pkt.whdr.cid = htonl((*conn).proto.cid | channel);
    pkt.whdr.callNumber = htonl(call_id);
    pkt.whdr.serial = htonl(serial);
    pkt.whdr.seq = 0;
    pkt.whdr.type_ = (*chan).last_type;
    pkt.whdr.flags = (*conn).out_clientflag;
    pkt.whdr.userStatus = 0;
    pkt.whdr.securityIndex = (*conn).security_ix;
    pkt.whdr._rsvd = 0;
    pkt.whdr.serviceId = htons((*conn).service_id);
    len = core::mem::size_of::<rxrpc_wire_header>();
    match (*chan).last_type {
        RXRPC_PACKET_TYPE_ABORT => { pkt.abort_code = htonl((*chan).last_abort); iov[0].iov_len += 4; len += 4; ioc = 1; }
        RXRPC_PACKET_TYPE_ACK => {
            if_mtu = (*(*conn).peer).if_mtu - (*(*conn).peer).hdrsize;
            max_mtu = if (*(*conn).peer).ackr_adv_pmtud { umax((*(*conn).peer).max_data, rxrpc_rx_mtu) } else { if_mtu = umin(1444, if_mtu); if_mtu };
            pkt.ack.bufferSpace = 0; pkt.ack.maxSkew = htons(if !skb.is_null() { (*skb).priority } else { 0 });
            pkt.ack.firstPacket = htonl((*chan).last_seq + 1); pkt.ack.previousPacket = htonl((*chan).last_seq);
            pkt.ack.serial = htonl(if !skb.is_null() { (*sp).hdr.serial } else { 0 });
            pkt.ack.reason = if !skb.is_null() { RXRPC_ACK_DUPLICATE } else { RXRPC_ACK_IDLE }; pkt.ack.nAcks = 0;
            trailer.maxMTU = htonl(max_mtu); trailer.ifMTU = htonl(if_mtu); trailer.rwind = htonl(rxrpc_rx_window_size); trailer.jumbo_max = 0;
            pkt.whdr.flags |= RXRPC_SLOW_START_OK; iov[0].iov_len += core::mem::size_of::<rxrpc_ackpacket>(); len += core::mem::size_of::<rxrpc_ackpacket>() + 3 + core::mem::size_of::<rxrpc_acktrailer>(); ioc = 3;
            trace_rxrpc_tx_ack((*chan).call_debug_id, serial, ntohl(pkt.ack.firstPacket), ntohl(pkt.ack.serial), pkt.ack.reason, 0, rxrpc_rx_window_size, rxrpc_propose_ack_retransmit);
        }
        _ => return,
    }
    let ret = kernel_sendmsg((*(*conn).local).socket, &mut msg, iov.as_mut_ptr(), ioc, len);
    rxrpc_peer_mark_tx((*conn).peer);
    if ret < 0 { trace_rxrpc_tx_fail((*chan).call_debug_id, serial, ret, rxrpc_tx_point_call_final_resend); }
    else { trace_rxrpc_tx_packet((*chan).call_debug_id, &pkt.whdr, rxrpc_tx_point_call_final_resend); }
    _leave!("");
}

unsafe fn rxrpc_abort_calls(conn: *mut rxrpc_connection) {
    _enter!("{%d},%x", (*conn).debug_id, (*conn).abort_code);
    for i in 0..RXRPC_MAXCALLS {
        let call = (*conn).channels[i].call;
        if !call.is_null() { rxrpc_see_call(call, rxrpc_call_see_conn_abort); rxrpc_set_call_completion(call, (*conn).completion, (*conn).abort_code, (*conn).error); rxrpc_poke_call(call, rxrpc_call_poke_conn_abort); }
    }
    _leave!("");
}

unsafe fn rxrpc_call_is_secure(call: *mut rxrpc_call) {
    if !call.is_null() && __test_and_clear_bit(RXRPC_CALL_CONN_CHALLENGING, &mut (*call).flags) { rxrpc_notify_socket(call); }
}

unsafe fn rxrpc_verify_response(conn: *mut rxrpc_connection, skb: *mut sk_buff) -> i32 {
    let len = (*skb).len - core::mem::size_of::<rxrpc_wire_header>();
    let buffer = kmalloc(len, GFP_NOFS);
    if buffer.is_null() { return -ENOMEM; }
    let mut ret = skb_copy_bits(skb, core::mem::size_of::<rxrpc_wire_header>(), buffer, len);
    if ret >= 0 { ret = (*(*conn).security).verify_response(conn, skb, buffer, len); }
    kfree(buffer); ret
}

unsafe fn rxrpc_process_event(conn: *mut rxrpc_connection, skb: *mut sk_buff) -> i32 {
    let sp = rxrpc_skb(skb);
    if (*conn).state == RXRPC_CONN_ABORTED { return -ECONNABORTED; }
    _enter!("{%d},{%u,%%%u},", (*conn).debug_id, (*sp).hdr.type_, (*sp).hdr.serial);
    match (*sp).hdr.type_ {
        RXRPC_PACKET_TYPE_CHALLENGE => { let ret = (*(*conn).security).respond_to_challenge(conn, skb); (*sp).chall.conn = core::ptr::null_mut(); rxrpc_put_connection(conn, rxrpc_conn_put_challenge_input); ret }
        RXRPC_PACKET_TYPE_RESPONSE => {
            spin_lock_irq(&mut (*conn).state_lock); if (*conn).state != RXRPC_CONN_SERVICE_CHALLENGING { spin_unlock_irq(&mut (*conn).state_lock); return 0; } spin_unlock_irq(&mut (*conn).state_lock);
            let mut ret = rxrpc_verify_response(conn, skb); if ret < 0 { return ret; }
            ret = (*(*conn).security).init_connection_security(conn, (*(*conn).key).payload.data[0]); if ret < 0 { return ret; }
            let mut secured = false; spin_lock_irq(&mut (*conn).state_lock); if (*conn).state == RXRPC_CONN_SERVICE_CHALLENGING { (*conn).state = RXRPC_CONN_SERVICE; secured = true; } spin_unlock_irq(&mut (*conn).state_lock);
            if secured { (*sp).poke_conn = rxrpc_get_connection(conn, rxrpc_conn_get_poke_secured); (*skb).mark = RXRPC_SKB_MARK_SERVICE_CONN_SECURED; rxrpc_get_skb(skb, rxrpc_skb_get_conn_secured); skb_queue_head(&mut (*(*conn).local).rx_queue, skb); rxrpc_wake_up_io_thread((*conn).local); } 0
        }
        _ => { WARN_ON_ONCE(1); -EPROTO }
    }
}

unsafe fn rxrpc_secure_connection(conn: *mut rxrpc_connection) { if (*(*conn).security).issue_challenge(conn) < 0 { rxrpc_abort_conn(conn, core::ptr::null_mut(), RX_CALL_DEAD, -ENOMEM, rxrpc_abort_nomem); } }

pub unsafe fn rxrpc_process_delayed_final_acks(conn: *mut rxrpc_connection, force: bool) {
    let mut j = jiffies; let mut next_j; let mut set;
    'again: loop { next_j = j + LONG_MAX; set = false;
        for channel in 0..RXRPC_MAXCALLS { let chan = &mut (*conn).channels[channel]; if !test_bit(RXRPC_CONN_FINAL_ACK_0 + channel, &(*conn).flags) { continue; } let ack_at = (*chan).final_ack_at; if time_before(j, ack_at) && !force { if time_before(ack_at, next_j) { next_j = ack_at; set = true; } continue; } if test_and_clear_bit(RXRPC_CONN_FINAL_ACK_0 + channel, &mut (*conn).flags) { rxrpc_conn_retransmit_call(conn, core::ptr::null_mut(), channel as u32); } }
        j = jiffies; if time_before_eq(next_j, j) { continue 'again; } if set { rxrpc_reduce_conn_timer(conn, next_j); } break;
    }
}

unsafe fn rxrpc_do_process_connection(conn: *mut rxrpc_connection) { if test_and_clear_bit(RXRPC_CONN_EV_CHALLENGE, &mut (*conn).events) { rxrpc_secure_connection(conn); } while let Some(skb) = skb_dequeue(&mut (*conn).rx_queue) { rxrpc_see_skb(skb, rxrpc_skb_see_conn_work); rxrpc_process_event(conn, skb); rxrpc_free_skb(skb, rxrpc_skb_put_conn_work); } }

pub unsafe fn rxrpc_process_connection(work: *mut work_struct) { let conn = container_of(work, rxrpc_connection, processor); rxrpc_see_connection(conn, rxrpc_conn_see_work); if __rxrpc_use_local((*conn).local, rxrpc_local_use_conn_work) { rxrpc_do_process_connection(conn); rxrpc_unuse_local((*conn).local, rxrpc_local_unuse_conn_work); } }

unsafe fn rxrpc_post_packet_to_conn(conn: *mut rxrpc_connection, skb: *mut sk_buff) { _enter!("%p,%p", conn, skb); rxrpc_get_skb(skb, rxrpc_skb_get_conn_work); skb_queue_tail(&mut (*conn).rx_queue, skb); rxrpc_queue_conn(conn, rxrpc_conn_queue_rx_work); }

unsafe fn rxrpc_post_challenge(conn: *mut rxrpc_connection, skb: *mut sk_buff) -> bool { let sp = rxrpc_skb(skb); let mut call = core::ptr::null_mut(); let mut respond = false; let mut queued = false; (*sp).chall.conn = rxrpc_get_connection(conn, rxrpc_conn_get_challenge_input); if !(*(*conn).security).challenge_to_recvmsg { rxrpc_post_packet_to_conn(conn, skb); return true; } rcu_read_lock(); for i in 0..(*conn).channels.len() { if !(*conn).channels[i].call.is_null() { call = (*conn).channels[i].call; let rx = rcu_dereference((*call).socket); if rx.is_null() { call = core::ptr::null_mut(); continue; } respond = true; if test_bit(RXRPC_SOCK_MANAGE_RESPONSE, &(*rx).flags) { break; } call = core::ptr::null_mut(); } } if !respond { rcu_read_unlock(); rxrpc_put_connection(conn, rxrpc_conn_put_challenge_input); (*sp).chall.conn = core::ptr::null_mut(); return false; } if !call.is_null() { queued = rxrpc_notify_socket_oob(call, skb); } rcu_read_unlock(); if !call.is_null() && !queued { rxrpc_put_connection(conn, rxrpc_conn_put_challenge_input); (*sp).chall.conn = core::ptr::null_mut(); return false; } if call.is_null() { rxrpc_post_packet_to_conn(conn, skb); } true }

pub unsafe fn rxrpc_input_conn_packet(conn: *mut rxrpc_connection, skb: *mut sk_buff) -> bool { let sp = rxrpc_skb(skb); match (*sp).hdr.type_ { RXRPC_PACKET_TYPE_BUSY => true, RXRPC_PACKET_TYPE_ABORT => { if rxrpc_is_conn_aborted(conn) { return true; } rxrpc_input_conn_abort(conn, skb); rxrpc_abort_calls(conn); true }, RXRPC_PACKET_TYPE_CHALLENGE => { rxrpc_see_skb(skb, rxrpc_skb_see_oob_challenge); if rxrpc_is_conn_aborted(conn) { if (*conn).completion == RXRPC_CALL_LOCALLY_ABORTED { rxrpc_send_conn_abort(conn); } return true; } if !(*(*conn).security).validate_challenge(conn, skb) { return false; } rxrpc_post_challenge(conn, skb) }, RXRPC_PACKET_TYPE_RESPONSE => { if rxrpc_is_conn_aborted(conn) { if (*conn).completion == RXRPC_CALL_LOCALLY_ABORTED { rxrpc_send_conn_abort(conn); } return true; } rxrpc_post_packet_to_conn(conn, skb); true }, _ => { WARN_ON_ONCE(1); true } } }

pub unsafe fn rxrpc_input_conn_event(conn: *mut rxrpc_connection, skb: *mut sk_buff) { if test_and_clear_bit(RXRPC_CONN_EV_ABORT_CALLS, &mut (*conn).events) { rxrpc_abort_calls(conn); } if !(*conn).tx_response.is_null() { spin_lock_irq(&mut (*(*conn).local).lock); let response = (*conn).tx_response; (*conn).tx_response = core::ptr::null_mut(); spin_unlock_irq(&mut (*(*conn).local).lock); if (*conn).state != RXRPC_CONN_ABORTED { rxrpc_send_response(conn, response); } rxrpc_free_skb(response, rxrpc_skb_put_response); } if !skb.is_null() && (*skb).mark == RXRPC_SKB_MARK_SERVICE_CONN_SECURED && (*conn).state == RXRPC_CONN_SERVICE { for loop_ in 0..RXRPC_MAXCALLS { rxrpc_call_is_secure((*conn).channels[loop_].call); } } if (*conn).flags & RXRPC_CONN_FINAL_ACK_MASK != 0 { rxrpc_process_delayed_final_acks(conn, false); } }

pub unsafe fn rxrpc_post_response(conn: *mut rxrpc_connection, skb: *mut sk_buff) { let sp = rxrpc_skb(skb); let local = (*conn).local; let mut old; _enter!("%x", (*sp).resp.challenge_serial); spin_lock_irq(&mut (*local).lock); old = (*conn).tx_response; if !old.is_null() { let osp = rxrpc_skb(old); if after((*sp).resp.challenge_serial, (*osp).resp.challenge_serial) { (*conn).tx_response = skb; } else { old = skb; } } else { (*conn).tx_response = skb; } spin_unlock_irq(&mut (*local).lock); rxrpc_poke_conn(conn, rxrpc_conn_get_poke_response); rxrpc_free_skb(old, rxrpc_skb_put_old_response); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
