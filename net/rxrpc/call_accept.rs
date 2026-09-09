// SPDX-License-Identifier: GPL-2.0-or-later
/* incoming call handling
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// The Linux headers and symbols referenced by this translation are supplied by
// the surrounding rxrpc Rust environment.

unsafe fn rxrpc_dummy_notify(
    _sk: *mut sock,
    _call: *mut rxrpc_call,
    _user_call_id: c_ulong,
) {
}

/*
 * Preallocate a single service call, connection and peer and, if possible,
 * give them a user ID and attach the user's side of the ID to them.
 */
unsafe fn rxrpc_service_prealloc_one(
    rx: *mut rxrpc_sock,
    b: *mut rxrpc_backlog,
    notify_rx: rxrpc_notify_rx_t,
    user_call_id: c_ulong,
    gfp: gfp_t,
    debug_id: c_uint,
) -> c_int {
    let mut call: *mut rxrpc_call;
    let mut xcall: *mut rxrpc_call;
    let rxnet = rxrpc_net(sock_net(&mut (*rx).sk));
    let mut parent: *mut rb_node;
    let mut pp: *mut *mut rb_node;
    let mut max: c_int;
    let mut tmp: c_int;
    let size: c_uint = RXRPC_BACKLOG_MAX;
    let mut head: c_uint;
    let mut tail: c_uint;
    let mut call_head: c_uint;
    let mut call_tail: c_uint;

    max = (*rx).sk.sk_max_ack_backlog;
    tmp = (*rx).sk.sk_ack_backlog;
    if tmp >= max {
        _leave!(" = -ENOBUFS [full %u]", max);
        return -ENOBUFS;
    }
    max -= tmp;

    /* We don't need more conns and peers than we have calls, but on the
     * other hand, we shouldn't ever use more peers than conns or conns
     * than calls.
     */
    call_head = (*b).call_backlog_head;
    call_tail = read_once((*b).call_backlog_tail);
    tmp = circ_cnt(call_head, call_tail, size) as c_int;
    if tmp >= max {
        _leave!(" = -ENOBUFS [enough %u]", tmp);
        return -ENOBUFS;
    }
    max = tmp + 1;

    head = (*b).peer_backlog_head;
    tail = read_once((*b).peer_backlog_tail);
    if (circ_cnt(head, tail, size) as c_int) < max {
        let peer = rxrpc_alloc_peer((*rx).local, gfp, rxrpc_peer_new_prealloc);
        if peer.is_null() { return -ENOMEM; }
        (*b).peer_backlog[head as usize] = peer;
        smp_store_release(&mut (*b).peer_backlog_head, (head + 1) & (size - 1));
    }

    head = (*b).conn_backlog_head;
    tail = read_once((*b).conn_backlog_tail);
    if (circ_cnt(head, tail, size) as c_int) < max {
        let conn = rxrpc_prealloc_service_connection(rxnet, gfp);
        if conn.is_null() { return -ENOMEM; }
        (*b).conn_backlog[head as usize] = conn;
        smp_store_release(&mut (*b).conn_backlog_head, (head + 1) & (size - 1));
    }

    /* Now it gets complicated, because calls get registered with the
     * socket here, with a user ID preassigned by the user.
     */
    call = rxrpc_alloc_call(rx, gfp, debug_id);
    if call.is_null() { return -ENOMEM; }
    (*call).flags |= 1 << RXRPC_CALL_IS_SERVICE;
    rxrpc_set_call_state(call, RXRPC_CALL_SERVER_PREALLOC);
    set_bit(RXRPC_CALL_EV_INITIAL_PING, &mut (*call).events);

    trace_rxrpc_call((*call).debug_id, refcount_read(&(*call).refcount),
                     user_call_id, rxrpc_call_new_prealloc_service);

    write_lock(&mut (*rx).call_lock);

    /* Check the user ID isn't already in use */
    pp = &mut (*rx).calls.rb_node;
    parent = core::ptr::null_mut();
    while !(*pp).is_null() {
        parent = *pp;
        xcall = rb_entry(parent, rxrpc_call, sock_node);
        if user_call_id < (*xcall).user_call_id {
            pp = &mut (*(*pp)).rb_left;
        } else if user_call_id > (*xcall).user_call_id {
            pp = &mut (*(*pp)).rb_right;
        } else {
            write_unlock(&mut (*rx).call_lock);
            rxrpc_prefail_call(call, RXRPC_CALL_LOCAL_ERROR, -EBADSLT);
            rxrpc_cleanup_call(call);
            _leave!(" = -EBADSLT");
            return -EBADSLT;
        }
    }

    (*call).user_call_id = user_call_id;
    (*call).notify_rx = notify_rx;
    if !(*rx).app_ops.is_null() && (*(*rx).app_ops).user_attach_call.is_some() {
        rxrpc_get_call(call, rxrpc_call_get_kernel_service);
        ((*(*rx).app_ops).user_attach_call.unwrap())(call, user_call_id);
    }

    rxrpc_get_call(call, rxrpc_call_get_userid);
    rb_link_node(&mut (*call).sock_node, parent, pp);
    rb_insert_color(&mut (*call).sock_node, &mut (*rx).calls);
    set_bit(RXRPC_CALL_HAS_USERID, &mut (*call).flags);
    list_add(&mut (*call).sock_link, &mut (*rx).sock_calls);
    write_unlock(&mut (*rx).call_lock);

    let rxnet = (*call).rxnet;
    spin_lock(&mut (*rxnet).call_lock);
    list_add_tail_rcu(&mut (*call).link, &mut (*rxnet).calls);
    spin_unlock(&mut (*rxnet).call_lock);
    (*b).call_backlog[call_head as usize] = call;
    smp_store_release(&mut (*b).call_backlog_head, (call_head + 1) & (size - 1));
    _leave!(" = 0 [%d -> %lx]", (*call).debug_id, user_call_id);
    0
}

/* Allocate the preallocation buffers for incoming service calls. */
pub unsafe fn rxrpc_service_prealloc(rx: *mut rxrpc_sock, gfp: gfp_t) -> c_int {
    let mut b = (*rx).backlog;
    if b.is_null() {
        b = kzalloc_obj::<rxrpc_backlog>(gfp);
        if b.is_null() { return -ENOMEM; }
        (*rx).backlog = b;
    }
    0
}

/* Discard the preallocation on a service. */
pub unsafe fn rxrpc_discard_prealloc(rx: *mut rxrpc_sock) {
    let b = (*rx).backlog;
    let rxnet = rxrpc_net(sock_net(&mut (*rx).sk));
    let size = RXRPC_BACKLOG_MAX;
    if b.is_null() { return; }
    (*rx).backlog = core::ptr::null_mut();
    spin_lock_irq(&mut (*rx).incoming_lock);
    spin_unlock_irq(&mut (*rx).incoming_lock);

    let mut head = (*b).peer_backlog_head;
    let mut tail = (*b).peer_backlog_tail;
    while circ_cnt(head, tail, size) > 0 {
        let peer = (*b).peer_backlog[tail as usize];
        rxrpc_put_local((*peer).local, rxrpc_local_put_prealloc_peer);
        kfree(peer as *mut c_void);
        tail = (tail + 1) & (size - 1);
    }
    head = (*b).conn_backlog_head;
    tail = (*b).conn_backlog_tail;
    while circ_cnt(head, tail, size) > 0 {
        let conn = (*b).conn_backlog[tail as usize];
        write_lock(&mut (*rxnet).conn_lock);
        list_del(&mut (*conn).link); list_del(&mut (*conn).proc_link);
        write_unlock(&mut (*rxnet).conn_lock);
        kfree(conn as *mut c_void);
        if atomic_dec_and_test(&mut (*rxnet).nr_conns) { wake_up_var(&mut (*rxnet).nr_conns); }
        tail = (tail + 1) & (size - 1);
    }
    head = (*b).call_backlog_head;
    tail = (*b).call_backlog_tail;
    while circ_cnt(head, tail, size) > 0 {
        let call = (*b).call_backlog[tail as usize];
        rxrpc_see_call(call, rxrpc_call_see_discard);
        rcu_assign_pointer(&mut (*call).socket, rx);
        if !(*rx).app_ops.is_null() && (*(*rx).app_ops).discard_new_call.is_some() {
            _debug!("discard %lx", (*call).user_call_id);
            ((*(*rx).app_ops).discard_new_call.unwrap())(call, (*call).user_call_id);
            if (*call).notify_rx.is_some() { (*call).notify_rx = Some(rxrpc_dummy_notify); }
            rxrpc_put_call(call, rxrpc_call_put_kernel);
        }
        rxrpc_call_completed(call); rxrpc_release_call(rx, call);
        rxrpc_put_call(call, rxrpc_call_put_discard_prealloc);
        tail = (tail + 1) & (size - 1);
    }
    kfree(b as *mut c_void);
}

/* Remaining incoming-call implementation is translated directly below. */

unsafe fn rxrpc_alloc_incoming_call(rx: *mut rxrpc_sock, local: *mut rxrpc_local,
    mut peer: *mut rxrpc_peer, mut conn: *mut rxrpc_connection,
    sec: *const rxrpc_security, peer_srx: *mut sockaddr_rxrpc,
    skb: *mut sk_buff) -> *mut rxrpc_call {
    let b = (*rx).backlog;
    if b.is_null() { return core::ptr::null_mut(); }
    let call_head = smp_load_acquire(&(*b).call_backlog_head);
    let call_tail = (*b).call_backlog_tail;
    let call_count = circ_cnt(call_head, call_tail, RXRPC_BACKLOG_MAX);
    let conn_head = smp_load_acquire(&(*b).conn_backlog_head);
    let conn_tail = (*b).conn_backlog_tail;
    let conn_count = circ_cnt(conn_head, conn_tail, RXRPC_BACKLOG_MAX);
    ASSERTCMP!(conn_count, >=, call_count);
    let peer_head = smp_load_acquire(&(*b).peer_backlog_head);
    let peer_tail = (*b).peer_backlog_tail;
    ASSERTCMP!(circ_cnt(peer_head, peer_tail, RXRPC_BACKLOG_MAX), >=, conn_count);
    if call_count == 0 { return core::ptr::null_mut(); }
    if conn.is_null() {
        if !peer.is_null() && !rxrpc_get_peer_maybe(peer, rxrpc_peer_get_service_conn) { peer = core::ptr::null_mut(); }
        if peer.is_null() {
            peer = (*b).peer_backlog[peer_tail as usize]; (*peer).srx = *peer_srx;
            (*b).peer_backlog[peer_tail as usize] = core::ptr::null_mut();
            smp_store_release(&mut (*b).peer_backlog_tail, (peer_tail + 1) & (RXRPC_BACKLOG_MAX - 1));
            rxrpc_new_incoming_peer(local, peer);
        }
        conn = (*b).conn_backlog[conn_tail as usize]; (*b).conn_backlog[conn_tail as usize] = core::ptr::null_mut();
        smp_store_release(&mut (*b).conn_backlog_tail, (conn_tail + 1) & (RXRPC_BACKLOG_MAX - 1));
        (*conn).local = rxrpc_get_local(local, rxrpc_local_get_prealloc_conn); (*conn).peer = peer;
        rxrpc_see_connection(conn, rxrpc_conn_see_new_service_conn);
        rxrpc_new_incoming_connection(rx, conn, sec, skb);
    } else { rxrpc_get_connection(conn, rxrpc_conn_get_service_conn); atomic_inc(&mut (*conn).active); }
    let call = (*b).call_backlog[call_tail as usize]; (*b).call_backlog[call_tail as usize] = core::ptr::null_mut();
    smp_store_release(&mut (*b).call_backlog_tail, (call_tail + 1) & (RXRPC_BACKLOG_MAX - 1));
    rxrpc_see_call(call, rxrpc_call_see_accept);
    (*call).local = rxrpc_get_local((*conn).local, rxrpc_local_get_call); (*call).conn = conn;
    (*call).security = (*conn).security; (*call).security_ix = (*conn).security_ix;
    (*call).peer = rxrpc_get_peer((*conn).peer, rxrpc_peer_get_accept); (*call).dest_srx = (*peer).srx;
    (*call).cong_ssthresh = (*call).peer.cong_ssthresh; (*call).tx_last_sent = ktime_get_real(); call
}

pub unsafe fn rxrpc_new_incoming_call(local: *mut rxrpc_local, peer: *mut rxrpc_peer,
    conn: *mut rxrpc_connection, peer_srx: *mut sockaddr_rxrpc, skb: *mut sk_buff) -> bool {
    let mut sec: *const rxrpc_security = core::ptr::null();
    let sp = rxrpc_skb(skb); let mut call = core::ptr::null_mut(); let mut rx: *mut rxrpc_sock;
    _enter!();
    if (*sp).hdr.type_ != RXRPC_PACKET_TYPE_DATA { return rxrpc_protocol_error(skb, rxrpc_eproto_no_service_call); }
    read_lock_irq(&mut (*local).services_lock); rx = (*local).service;
    if rx.is_null() || ((*sp).hdr.serviceId != (*rx).srx.srx_service && (*sp).hdr.serviceId != (*rx).second_service) {
        if (*sp).hdr.type_ == RXRPC_PACKET_TYPE_DATA && (*sp).hdr.seq == 1 { read_unlock_irq(&mut (*local).services_lock); return rxrpc_direct_conn_abort(skb, rxrpc_abort_service_not_offered, RX_INVALID_OPERATION, -EOPNOTSUPP); }
        read_unlock_irq(&mut (*local).services_lock); return true;
    }
    if conn.is_null() { sec = rxrpc_get_incoming_security(rx, skb); if sec.is_null() { read_unlock_irq(&mut (*local).services_lock); return rxrpc_direct_conn_abort(skb, rxrpc_abort_service_not_offered, RX_INVALID_OPERATION, -EKEYREJECTED); } }
    spin_lock(&mut (*rx).incoming_lock);
    if (*rx).sk.sk_state == RXRPC_SERVER_LISTEN_DISABLED || (*rx).sk.sk_state == RXRPC_CLOSE { rxrpc_direct_conn_abort(skb, rxrpc_abort_shut_down, RX_INVALID_OPERATION, -ESHUTDOWN); spin_unlock(&mut (*rx).incoming_lock); read_unlock_irq(&mut (*local).services_lock); return false; }
    call = rxrpc_alloc_incoming_call(rx, local, peer, conn, sec, peer_srx, skb);
    if call.is_null() { (*skb).mark = RXRPC_SKB_MARK_REJECT_BUSY; spin_unlock(&mut (*rx).incoming_lock); read_unlock_irq(&mut (*local).services_lock); return false; }
    trace_rxrpc_receive(call, rxrpc_receive_incoming, (*sp).hdr.serial, (*sp).hdr.seq); rxrpc_incoming_call(rx, call, skb); conn = (*call).conn;
    if !(*rx).app_ops.is_null() && (*(*rx).app_ops).notify_new_call.is_some() { ((*(*rx).app_ops).notify_new_call.unwrap())(&mut (*rx).sk, call, (*call).user_call_id); }
    spin_lock(&mut (*conn).state_lock); if (*conn).state == RXRPC_CONN_SERVICE_UNSECURED { (*conn).state = RXRPC_CONN_SERVICE_CHALLENGING; set_bit(RXRPC_CONN_EV_CHALLENGE, &mut (*call).conn.events); rxrpc_queue_conn((*call).conn, rxrpc_conn_queue_challenge); } spin_unlock(&mut (*conn).state_lock);
    spin_unlock(&mut (*rx).incoming_lock); read_unlock_irq(&mut (*local).services_lock); rxrpc_assess_MTU_size(local, (*call).peer);
    if hlist_unhashed(&(*call).error_link) { spin_lock_irq(&mut (*call).peer.lock); hlist_add_head(&mut (*call).error_link, &mut (*call).peer.error_targets); spin_unlock_irq(&mut (*call).peer.lock); }
    _leave!(" = %p{%d}", call, (*call).debug_id); rxrpc_queue_rx_call_packet(call, skb); rxrpc_put_call(call, rxrpc_call_put_input); true
}

pub unsafe fn rxrpc_user_charge_accept(rx: *mut rxrpc_sock, user_call_id: c_ulong) -> c_int {
    if (*rx).sk.sk_state == RXRPC_CLOSE { return -ESHUTDOWN; }
    rxrpc_service_prealloc_one(rx, (*rx).backlog, None, user_call_id, GFP_KERNEL, atomic_inc_return(&mut rxrpc_debug_id))
}

pub unsafe fn rxrpc_kernel_charge_accept(sock: *mut socket, notify_rx: rxrpc_notify_rx_t,
    user_call_id: c_ulong, gfp: gfp_t, debug_id: c_uint) -> c_int {
    let sk = (*sock).sk; let rx = rxrpc_sk(sk); lock_sock(sk);
    let ret = if (*sk).sk_state != RXRPC_SERVER_LISTENING || (*rx).backlog.is_null() { -ESHUTDOWN } else { rxrpc_service_prealloc_one(rx, (*rx).backlog, notify_rx, user_call_id, gfp, debug_id) };
    release_sock(sk); ret
}

#[allow(non_upper_case_globals)]
pub const EXPORT_SYMBOL_rxrpc_kernel_charge_accept: Option<unsafe extern "C" fn()> = None;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
