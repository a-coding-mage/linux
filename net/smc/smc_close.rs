// SPDX-License-Identifier: GPL-2.0
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 * Socket Closing - normal and abnormal
 * Copyright IBM Corp. 2016
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

pub unsafe fn smc_clcsock_release(smc: *mut smc_sock) {
    let mut tcp: *mut socket;
    if (*smc).listen_smc && current_work() != &mut (*smc).smc_listen_work {
        cancel_work_sync(&mut (*smc).smc_listen_work);
    }
    mutex_lock(&mut (*smc).clcsock_release_lock);
    if !(*smc).clcsock.is_null() {
        tcp = (*smc).clcsock;
        (*smc).clcsock = core::ptr::null_mut();
        sock_release(tcp);
    }
    mutex_unlock(&mut (*smc).clcsock_release_lock);
}

unsafe fn smc_close_cleanup_listen(parent: *mut sock) {
    let mut sk: *mut sock;
    while {
        sk = smc_accept_dequeue(parent, core::ptr::null_mut());
        !sk.is_null()
    } {
        smc_close_non_accepted(sk);
    }
}

unsafe fn smc_close_stream_wait(smc: *mut smc_sock, mut timeout: c_long) {
    let mut wait = DEFINE_WAIT_FUNC!(woken_wake_function);
    let sk = &mut (*smc).sk;
    if timeout == 0 || !smc_tx_prepared_sends(&mut (*smc).conn) { return; }
    smc_tx_pending(&mut (*smc).conn);
    (*smc).wait_close_tx_prepared = 1;
    add_wait_queue(sk_sleep(sk), &mut wait);
    while !signal_pending(current) && timeout != 0 {
        let rc = sk_wait_event(sk, &mut timeout,
            !smc_tx_prepared_sends(&mut (*smc).conn) ||
            READ_ONCE!((*sk).sk_err) == ECONNABORTED ||
            READ_ONCE!((*sk).sk_err) == ECONNRESET || (*smc).conn.killed,
            &mut wait);
        if rc != 0 { break; }
    }
    remove_wait_queue(sk_sleep(sk), &mut wait);
    (*smc).wait_close_tx_prepared = 0;
}

pub unsafe fn smc_close_wake_tx_prepared(smc: *mut smc_sock) {
    if (*smc).wait_close_tx_prepared { (*smc).sk.sk_state_change(&mut (*smc).sk); }
}

unsafe fn smc_close_wr(conn: *mut smc_connection) -> c_int {
    (*conn).local_tx_ctrl.conn_state_flags.peer_done_writing = true;
    smc_cdc_get_slot_and_msg_send(conn)
}

unsafe fn smc_close_final(conn: *mut smc_connection) -> c_int {
    if atomic_read(&(*conn).bytes_to_rcv) != 0 {
        (*conn).local_tx_ctrl.conn_state_flags.peer_conn_abort = true;
    } else {
        (*conn).local_tx_ctrl.conn_state_flags.peer_conn_closed = true;
    }
    if (*conn).killed { return -EPIPE; }
    smc_cdc_get_slot_and_msg_send(conn)
}

pub unsafe fn smc_close_abort(conn: *mut smc_connection) -> c_int {
    (*conn).local_tx_ctrl.conn_state_flags.peer_conn_abort = true;
    smc_cdc_get_slot_and_msg_send(conn)
}

unsafe fn smc_close_cancel_work(smc: *mut smc_sock) {
    let sk = &mut (*smc).sk;
    release_sock(sk);
    if cancel_work_sync(&mut (*smc).conn.close_work) != 0 { sock_put(sk); }
    cancel_delayed_work_sync(&mut (*smc).conn.tx_work);
    lock_sock(sk);
}

pub unsafe fn smc_close_active_abort(smc: *mut smc_sock) {
    let sk = &mut (*smc).sk;
    let mut release_clcsock = false;
    if (*sk).sk_state != SMC_INIT && !(*smc).clcsock.is_null() && !(*(*smc).clcsock).sk.is_null() {
        (*sk).sk_err = ECONNABORTED;
        tcp_abort((*(*smc).clcsock).sk, ECONNABORTED);
    }
    match (*sk).sk_state {
        SMC_ACTIVE | SMC_APPCLOSEWAIT1 | SMC_APPCLOSEWAIT2 => {
            (*sk).sk_state = SMC_PEERABORTWAIT; smc_close_cancel_work(smc);
            if (*sk).sk_state == SMC_PEERABORTWAIT { (*sk).sk_state = SMC_CLOSED; sock_put(sk); }
        }
        SMC_PEERCLOSEWAIT1 | SMC_PEERCLOSEWAIT2 | SMC_PEERFINCLOSEWAIT => {
            (*sk).sk_state = SMC_PEERABORTWAIT; smc_close_cancel_work(smc);
            if (*sk).sk_state == SMC_PEERABORTWAIT { (*sk).sk_state = SMC_CLOSED; smc_conn_free(&mut (*smc).conn); release_clcsock = true; sock_put(sk); }
        }
        SMC_PROCESSABORT | SMC_APPFINCLOSEWAIT => {
            (*sk).sk_state = SMC_PEERABORTWAIT; smc_close_cancel_work(smc);
            if (*sk).sk_state == SMC_PEERABORTWAIT { (*sk).sk_state = SMC_CLOSED; smc_conn_free(&mut (*smc).conn); release_clcsock = true; }
        }
        SMC_INIT | SMC_PEERABORTWAIT | SMC_CLOSED => {}
        _ => {}
    }
    smc_sock_set_flag(sk, SOCK_DEAD); (*sk).sk_state_change(sk);
    if release_clcsock { release_sock(sk); smc_clcsock_release(smc); lock_sock(sk); }
}

#[inline]
unsafe fn smc_close_sent_any_close(conn: *mut smc_connection) -> bool {
    (*conn).local_tx_ctrl.conn_state_flags.peer_conn_abort || (*conn).local_tx_ctrl.conn_state_flags.peer_conn_closed
}

pub unsafe fn smc_close_active(smc: *mut smc_sock) -> c_int {
    let txflags = &mut (*smc).conn.local_tx_ctrl.conn_state_flags;
    let conn = &mut (*smc).conn; let sk = &mut (*smc).sk;
    let old_state = (*sk).sk_state;
    let timeout = if current.flags & PF_EXITING != 0 { 0 } else if sock_flag(sk, SOCK_LINGER) { (*sk).sk_lingertime } else { SMC_MAX_STREAM_WAIT_TIMEOUT };
    let mut rc = 0; let mut rc1;
    'again: loop {
        match (*sk).sk_state {
            SMC_INIT => (*sk).sk_state = SMC_CLOSED,
            SMC_LISTEN => { (*sk).sk_state = SMC_CLOSED; (*sk).sk_state_change(sk); if !(*smc).clcsock.is_null() && !(*(*smc).clcsock).sk.is_null() { rc = kernel_sock_shutdown((*smc).clcsock, SHUT_RDWR); } smc_close_cleanup_listen(sk); release_sock(sk); flush_work(&mut (*smc).tcp_listen_work); lock_sock(sk); }
            SMC_ACTIVE => { smc_close_stream_wait(smc, timeout); release_sock(sk); cancel_delayed_work_sync(&mut conn.tx_work); lock_sock(sk); if (*sk).sk_state == SMC_ACTIVE { rc = smc_close_final(conn); (*sk).sk_state = SMC_PEERCLOSEWAIT1; if !(*smc).clcsock.is_null() && !(*(*smc).clcsock).sk.is_null() { rc1 = kernel_sock_shutdown((*smc).clcsock, SHUT_RDWR); if rc == 0 { rc = rc1; } } } else { continue 'again; } }
            SMC_APPFINCLOSEWAIT => { if txflags.peer_done_writing && !smc_close_sent_any_close(conn) { rc = smc_close_final(conn); } (*sk).sk_state = SMC_CLOSED; }
            SMC_APPCLOSEWAIT1 | SMC_APPCLOSEWAIT2 => { if !smc_cdc_rxed_any_close(conn) { smc_close_stream_wait(smc, timeout); } release_sock(sk); cancel_delayed_work_sync(&mut conn.tx_work); lock_sock(sk); if (*sk).sk_state != SMC_APPCLOSEWAIT1 && (*sk).sk_state != SMC_APPCLOSEWAIT2 { continue 'again; } rc = smc_close_final(conn); if smc_cdc_rxed_any_close(conn) { (*sk).sk_state = SMC_CLOSED; sock_put(sk); } else { (*sk).sk_state = SMC_PEERFINCLOSEWAIT; } }
            SMC_PEERCLOSEWAIT1 | SMC_PEERCLOSEWAIT2 => { if txflags.peer_done_writing && !smc_close_sent_any_close(conn) { rc = smc_close_final(conn); } }
            SMC_PEERFINCLOSEWAIT => {}
            SMC_PROCESSABORT => { rc = smc_close_abort(conn); (*sk).sk_state = SMC_CLOSED; }
            SMC_PEERABORTWAIT => (*sk).sk_state = SMC_CLOSED,
            SMC_CLOSED => {}
            _ => {}
        }
        break;
    }
    if old_state != (*sk).sk_state { (*sk).sk_state_change(sk); } rc
}

// Passive close state handling and workqueue callback.
unsafe fn smc_close_passive_abort_received(smc: *mut smc_sock) { let txflags = &(*smc).conn.local_tx_ctrl.conn_state_flags; let sk = &mut (*smc).sk; match (*sk).sk_state { SMC_INIT | SMC_ACTIVE | SMC_APPCLOSEWAIT1 => { (*sk).sk_state = SMC_PROCESSABORT; sock_put(sk); }, SMC_APPFINCLOSEWAIT => (*sk).sk_state = SMC_PROCESSABORT, SMC_PEERCLOSEWAIT1 | SMC_PEERCLOSEWAIT2 => { if txflags.peer_done_writing && !smc_close_sent_any_close(&mut (*smc).conn) { (*sk).sk_state = SMC_PROCESSABORT; } else { (*sk).sk_state = SMC_CLOSED; } sock_put(sk); }, SMC_APPCLOSEWAIT2 | SMC_PEERFINCLOSEWAIT => { (*sk).sk_state = SMC_CLOSED; sock_put(sk); }, SMC_PEERABORTWAIT => (*sk).sk_state = SMC_CLOSED, SMC_PROCESSABORT => {}, _ => {} } }

unsafe fn smc_close_passive_work(work: *mut work_struct) {
    let conn = container_of!(work, smc_connection, close_work); let smc = container_of!(conn, smc_sock, conn); let sk = &mut (*smc).sk; let old_state = (*sk).sk_state; let rxflags = &(*conn).local_rx_ctrl.conn_state_flags; let mut release_clcsock = false;
    lock_sock(sk);
    if rxflags.peer_conn_abort { smc_close_passive_abort_received(smc); release_sock(sk); cancel_delayed_work_sync(&mut (*conn).tx_work); lock_sock(sk); }
    else { match (*sk).sk_state { SMC_INIT | SMC_ACTIVE => (*sk).sk_state = SMC_APPCLOSEWAIT1, SMC_PEERCLOSEWAIT1 => { if rxflags.peer_done_writing { (*sk).sk_state = SMC_PEERCLOSEWAIT2; } if smc_cdc_rxed_any_close(conn) { (*sk).sk_state = if sock_flag(sk, SOCK_DEAD) && smc_close_sent_any_close(conn) { SMC_CLOSED } else { SMC_APPFINCLOSEWAIT }; sock_put(sk); } }, SMC_PEERCLOSEWAIT2 => { if smc_cdc_rxed_any_close(conn) { (*sk).sk_state = if sock_flag(sk, SOCK_DEAD) && smc_close_sent_any_close(conn) { SMC_CLOSED } else { SMC_APPFINCLOSEWAIT }; sock_put(sk); } }, SMC_PEERFINCLOSEWAIT => if smc_cdc_rxed_any_close(conn) { (*sk).sk_state = SMC_CLOSED; sock_put(sk); }, SMC_APPCLOSEWAIT1 | SMC_APPCLOSEWAIT2 | SMC_APPFINCLOSEWAIT | SMC_PEERABORTWAIT | SMC_PROCESSABORT | SMC_CLOSED => {}, _ => {} } }
    sk.sk_data_ready(sk); sk.sk_write_space(sk);
    if old_state != sk.sk_state { sk.sk_state_change(sk); if sk.sk_state == SMC_CLOSED && (sock_flag(sk, SOCK_DEAD) || sk.sk_socket.is_null()) { smc_conn_free(conn); if !(*smc).clcsock.is_null() { release_clcsock = true; } } }
    release_sock(sk); if release_clcsock { smc_clcsock_release(smc); } sock_put(sk);
}

pub unsafe fn smc_close_shutdown_write(smc: *mut smc_sock) -> c_int {
    let conn = &mut (*smc).conn; let sk = &mut (*smc).sk; let old_state = (*sk).sk_state; let timeout = if current.flags & PF_EXITING != 0 { 0 } else if sock_flag(sk, SOCK_LINGER) { sk.sk_lingertime } else { SMC_MAX_STREAM_WAIT_TIMEOUT }; let mut rc = 0;
    'again: loop { match sk.sk_state { SMC_ACTIVE => { smc_close_stream_wait(smc, timeout); release_sock(sk); cancel_delayed_work_sync(&mut conn.tx_work); lock_sock(sk); if sk.sk_state != SMC_ACTIVE { continue 'again; } rc = smc_close_wr(conn); sk.sk_state = SMC_PEERCLOSEWAIT1; }, SMC_APPCLOSEWAIT1 => { if !smc_cdc_rxed_any_close(conn) { smc_close_stream_wait(smc, timeout); } release_sock(sk); cancel_delayed_work_sync(&mut conn.tx_work); lock_sock(sk); if sk.sk_state != SMC_APPCLOSEWAIT1 { continue 'again; } rc = smc_close_wr(conn); sk.sk_state = SMC_APPCLOSEWAIT2; }, SMC_APPCLOSEWAIT2 | SMC_PEERFINCLOSEWAIT | SMC_PEERCLOSEWAIT1 | SMC_PEERCLOSEWAIT2 | SMC_APPFINCLOSEWAIT | SMC_PROCESSABORT | SMC_PEERABORTWAIT => {}, _ => {} } break; }
    if old_state != sk.sk_state { sk.sk_state_change(sk); } rc
}

pub unsafe fn smc_close_init(smc: *mut smc_sock) { INIT_WORK!(&mut (*smc).conn.close_work, smc_close_passive_work); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
