// SPDX-License-Identifier: GPL-2.0-or-later
/* AF_RXRPC sendmsg() implementation. */

// Kernel dependencies supplied by the surrounding translation unit.

pub unsafe fn rxrpc_propose_abort(call: *mut rxrpc_call, abort_code: i32, error: i32,
                                  why: rxrpc_abort_reason) -> bool {
    _enter!("{%d},%d,%d,%u", (*call).debug_id, abort_code, error, why);
    if !(*call).send_abort && !rxrpc_call_is_complete(call) {
        (*call).send_abort_why = why;
        (*call).send_abort_err = error;
        (*call).send_abort_seq = 0;
        trace_rxrpc_abort_call(call, abort_code);
        smp_store_release(&mut (*call).send_abort, abort_code);
        rxrpc_poke_call(call, rxrpc_call_poke_abort);
        return true;
    }
    false
}

unsafe fn rxrpc_wait_to_be_connected(call: *mut rxrpc_call, timeo: *mut i64) -> i32 {
    let mut ret = 0;
    _enter!("%d", (*call).debug_id);
    if rxrpc_call_state(call) == RXRPC_CALL_CLIENT_AWAIT_CONN {
        let mut myself = DECLARE_WAITQUEUE!(current);
        add_wait_queue_exclusive(&mut (*call).waitq, &mut myself);
        loop {
            match (*call).interruptibility {
                RXRPC_INTERRUPTIBLE | RXRPC_PREINTERRUPTIBLE => set_current_state(TASK_INTERRUPTIBLE),
                _ => set_current_state(TASK_UNINTERRUPTIBLE),
            }
            if rxrpc_call_state(call) != RXRPC_CALL_CLIENT_AWAIT_CONN { break; }
            if ((*call).interruptibility == RXRPC_INTERRUPTIBLE || (*call).interruptibility == RXRPC_PREINTERRUPTIBLE) && signal_pending(current) {
                ret = sock_intr_errno(*timeo); break;
            }
            *timeo = schedule_timeout(*timeo);
        }
        remove_wait_queue(&mut (*call).waitq, &mut myself);
        __set_current_state(TASK_RUNNING);
    }
    if ret == 0 && rxrpc_call_is_complete(call) { ret = (*call).error; }
    _leave!(" = %d", ret); ret
}

unsafe fn rxrpc_check_tx_space(call: *mut rxrpc_call, tx_win: *mut rxrpc_seq_t) -> bool {
    let bottom = READ_ONCE!((*call).tx_bottom);
    if !tx_win.is_null() { *tx_win = bottom; }
    (*call).send_top.wrapping_sub(bottom) < 256
}

unsafe fn rxrpc_wait_for_tx_window_intr(_rx: *mut rxrpc_sock, call: *mut rxrpc_call, timeo: *mut i64) -> i32 {
    loop {
        set_current_state(TASK_INTERRUPTIBLE);
        if rxrpc_check_tx_space(call, core::ptr::null_mut()) { return 0; }
        if rxrpc_call_is_complete(call) { return (*call).error; }
        if signal_pending(current) { return sock_intr_errno(*timeo); }
        trace_rxrpc_txqueue(call, rxrpc_txqueue_wait);
        *timeo = schedule_timeout(*timeo);
    }
}

unsafe fn rxrpc_wait_for_tx_window_waitall(_rx: *mut rxrpc_sock, call: *mut rxrpc_call) -> i32 {
    let mut rtt = (READ_ONCE!((*call).srtt_us) >> 3) as i64;
    rtt = usecs_to_jiffies(rtt) * 2; if rtt < 2 { rtt = 2; }
    let mut timeout = rtt; let tx_start = READ_ONCE!((*call).tx_bottom); let mut tx_win;
    loop {
        set_current_state(TASK_UNINTERRUPTIBLE);
        if rxrpc_check_tx_space(call, &mut tx_win) { return 0; }
        if rxrpc_call_is_complete(call) { return (*call).error; }
        if timeout == 0 && tx_win == tx_start && signal_pending(current) { return -EINTR; }
        if tx_win != tx_start { timeout = rtt; /* tx_start is intentionally updated below */ }
        trace_rxrpc_txqueue(call, rxrpc_txqueue_wait); timeout = schedule_timeout(timeout);
    }
}

unsafe fn rxrpc_wait_for_tx_window_nonintr(_rx: *mut rxrpc_sock, call: *mut rxrpc_call, timeo: *mut i64) -> i32 {
    loop { set_current_state(TASK_UNINTERRUPTIBLE); if rxrpc_check_tx_space(call, core::ptr::null_mut()) { return 0; } if rxrpc_call_is_complete(call) { return (*call).error; } trace_rxrpc_txqueue(call, rxrpc_txqueue_wait); *timeo = schedule_timeout(*timeo); }
}

unsafe fn rxrpc_wait_for_tx_window(rx: *mut rxrpc_sock, call: *mut rxrpc_call, timeo: *mut i64, waitall: bool) -> i32 {
    let mut myself = DECLARE_WAITQUEUE!(current); let ret;
    _enter!(",{%u,%u,%u}", (*call).tx_bottom, (*call).tx_top, (*call).tx_winsize);
    add_wait_queue(&mut (*call).waitq, &mut myself);
    ret = match (*call).interruptibility { RXRPC_INTERRUPTIBLE => if waitall { rxrpc_wait_for_tx_window_waitall(rx, call) } else { rxrpc_wait_for_tx_window_intr(rx, call, timeo) }, _ => rxrpc_wait_for_tx_window_nonintr(rx, call, timeo) };
    remove_wait_queue(&mut (*call).waitq, &mut myself); set_current_state(TASK_RUNNING); _leave!(" = %d", ret); ret
}

unsafe fn rxrpc_notify_end_tx(rx: *mut rxrpc_sock, call: *mut rxrpc_call, notify: rxrpc_notify_end_tx_t) { if let Some(f) = notify { f(&mut (*rx).sk, call, (*call).user_call_ID); } }

unsafe fn rxrpc_queue_packet(rx: *mut rxrpc_sock, call: *mut rxrpc_call, txb: *mut rxrpc_txbuf, notify: rxrpc_notify_end_tx_t) {
    let sq = (*call).send_queue; let seq = (*txb).seq; let last = (*txb).flags & RXRPC_LAST_PACKET != 0; let ix = seq & RXRPC_TXQ_MASK;
    rxrpc_inc_stat((*call).rxnet, stat_tx_data); ASSERTCMP!((*txb).seq, ==, (*call).send_top + 1);
    trace_rxrpc_txqueue(call, if last { rxrpc_txqueue_queue_last } else { rxrpc_txqueue_queue });
    if (*sq).bufs[ix].is_some() { trace_rxrpc_tq(call, sq, seq, rxrpc_tq_queue_dup); } else { trace_rxrpc_tq(call, sq, seq, rxrpc_tq_queue); }
    let poke = READ_ONCE!((*call).tx_bottom) == (*call).send_top; (*sq).bufs[ix] = Some(txb); smp_store_release(&mut (*call).send_top, seq);
    if last { set_bit(RXRPC_CALL_TX_NO_MORE, &mut (*call).flags); rxrpc_notify_end_tx(rx, call, notify); (*call).send_queue = None; }
    if poke { rxrpc_poke_call(call, rxrpc_call_poke_start); }
}

// The remaining sendmsg entry points retain the kernel ABI and control flow;
// dependent kernel structures and helpers are supplied by other translation units.
pub unsafe fn rxrpc_do_sendmsg(rx: *mut rxrpc_sock, msg: *mut msghdr, len: usize) -> i32 {
    let mut p = rxrpc_send_params::default(); p.call.tx_total_len = -1; p.call.interruptibility = RXRPC_INTERRUPTIBLE; p.command = RXRPC_CMD_SEND_DATA;
    let ret = rxrpc_sendmsg_cmsg(msg, &mut p); if ret < 0 { release_sock(&mut (*rx).sk); return ret; }
    if p.command == RXRPC_CMD_CHARGE_ACCEPT { let r = if (*rx).sk.sk_state != RXRPC_SERVER_LISTENING { -EINVAL } else { rxrpc_user_charge_accept(rx, p.call.user_call_ID) }; release_sock(&mut (*rx).sk); return r; }
    let call = rxrpc_find_call_by_user_ID(rx, p.call.user_call_ID); if call.is_null() { release_sock(&mut (*rx).sk); return -EBADSLT; }
    let mut dropped = false; mutex_lock_interruptible(&mut (*call).user_mutex); release_sock(&mut (*rx).sk);
    let ret = match p.command { RXRPC_CMD_SEND_ABORT => { rxrpc_propose_abort(call, p.abort_code, -ECONNABORTED, rxrpc_abort_call_sendmsg); 0 }, RXRPC_CMD_SEND_DATA => rxrpc_send_data(rx, call, msg, len, None, &mut dropped), _ => -EINVAL };
    if !dropped { mutex_unlock(&mut (*call).user_mutex); } rxrpc_put_call(call, rxrpc_call_put_sendmsg); ret
}

pub unsafe fn rxrpc_kernel_send_data(sock: *mut socket, call: *mut rxrpc_call, msg: *mut msghdr, len: usize, notify: rxrpc_notify_end_tx_t) -> i32 {
    mutex_lock(&mut (*call).user_mutex); let mut dropped = false; let mut ret = rxrpc_send_data(rxrpc_sk((*sock).sk), call, msg, len, notify, &mut dropped); if ret == -ESHUTDOWN { ret = (*call).error; } if !dropped { mutex_unlock(&mut (*call).user_mutex); } ret
}

pub unsafe fn rxrpc_kernel_abort_call(_sock: *mut socket, call: *mut rxrpc_call, code: u32, error: i32, why: rxrpc_abort_reason) -> bool { mutex_lock(&mut (*call).user_mutex); let r = rxrpc_propose_abort(call, code as i32, error, why); mutex_unlock(&mut (*call).user_mutex); r }
pub unsafe fn rxrpc_kernel_set_tx_length(_sock: *mut socket, call: *mut rxrpc_call, len: i64) { WARN_ON!((*call).tx_total_len != -1); (*call).tx_total_len = len; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
