// SPDX-License-Identifier: GPL-2.0-or-later
/* RxRPC individual remote procedure call handling */

// Kernel headers and symbols are supplied by the surrounding Rust translation.

pub static RXRPC_CALL_STATES: [&str; NR__RXRPC_CALL_STATES] = [
    "Uninit  ", "ClWtConn", "ClPreSnd", "ClSndReq", "ClAwtAck", "ClAwtRpl",
    "ClRcvRpl", "SvPrealc", "SvRcvReq", "SvAckReq", "SvSndRpl", "SvAwtACK",
    "Complete",
];
pub static RXRPC_CALL_COMPLETIONS: [&str; NR__RXRPC_CALL_COMPLETIONS] =
    ["Complete", "RmtAbort", "LocAbort", "LocError", "NetError"];

pub static mut rxrpc_call_jar: *mut kmem_cache = core::ptr::null_mut();
static mut rxrpc_call_limiter: semaphore = DEFINE_SEMAPHORE!(1000);
static mut rxrpc_kernel_call_limiter: semaphore = DEFINE_SEMAPHORE!(1000);

pub unsafe fn rxrpc_poke_call(call: *mut rxrpc_call, what: rxrpc_call_poke_trace) {
    let local = (*call).local;
    let mut busy: bool;
    if !test_bit(RXRPC_CALL_DISCONNECTED, &(*call).flags) {
        spin_lock_irq(&mut (*local).lock);
        busy = !list_empty(&(*call).attend_link);
        trace_rxrpc_poke_call(call, busy, what);
        if !busy && rxrpc_try_get_call(call, rxrpc_call_get_poke).is_null() { busy = true; }
        if !busy { list_add_tail(&mut (*call).attend_link, &mut (*local).call_attend_q); }
        spin_unlock_irq(&mut (*local).lock);
        if !busy { rxrpc_wake_up_io_thread(local); }
    }
}

unsafe fn rxrpc_call_timer_expired(t: *mut timer_list) {
    let call = timer_container_of!(call, t, timer);
    _enter!("%d", (*call).debug_id);
    if !__rxrpc_call_is_complete(call) {
        trace_rxrpc_timer_expired(call);
        rxrpc_poke_call(call, rxrpc_call_poke_timer);
    }
}

static mut rxrpc_call_user_mutex_lock_class_key: lock_class_key = lock_class_key {};
unsafe extern "C" { fn rxrpc_destroy_call(work: *mut work_struct); }

pub unsafe fn rxrpc_find_call_by_user_ID(rx: *mut rxrpc_sock, user_call_ID: c_ulong) -> *mut rxrpc_call {
    _enter!("%p,%lx", rx, user_call_ID);
    read_lock(&mut (*rx).call_lock);
    let mut p = (*rx).calls.rb_node;
    while !p.is_null() {
        let call = rb_entry!(p, rxrpc_call, sock_node);
        if user_call_ID < (*call).user_call_ID { p = (*p).rb_left; }
        else if user_call_ID > (*call).user_call_ID { p = (*p).rb_right; }
        else { rxrpc_get_call(call, rxrpc_call_get_sendmsg); read_unlock(&mut (*rx).call_lock); return call; }
    }
    read_unlock(&mut (*rx).call_lock); core::ptr::null_mut()
}

pub unsafe fn rxrpc_alloc_call(rx: *mut rxrpc_sock, gfp: gfp_t, debug_id: c_uint) -> *mut rxrpc_call {
    let rxnet = rxrpc_net(sock_net(&(*rx).sk));
    let call = kmem_cache_zalloc(rxrpc_call_jar, gfp) as *mut rxrpc_call;
    if call.is_null() { return core::ptr::null_mut(); }
    mutex_init(&mut (*call).user_mutex);
    if (*rx).sk.sk_kern_sock { lockdep_set_class(&mut (*call).user_mutex, &mut rxrpc_call_user_mutex_lock_class_key); }
    timer_setup(&mut (*call).timer, rxrpc_call_timer_expired, 0);
    INIT_WORK!(&mut (*call).destroyer, rxrpc_destroy_call);
    INIT_LIST_HEAD!(&mut (*call).link); INIT_LIST_HEAD!(&mut (*call).wait_link);
    INIT_LIST_HEAD!(&mut (*call).accept_link); INIT_LIST_HEAD!(&mut (*call).recvmsg_link);
    INIT_LIST_HEAD!(&mut (*call).sock_link); INIT_LIST_HEAD!(&mut (*call).attend_link);
    skb_queue_head_init(&mut (*call).recvmsg_queue); skb_queue_head_init(&mut (*call).rx_queue);
    skb_queue_head_init(&mut (*call).rx_oos_queue); init_waitqueue_head(&mut (*call).waitq);
    spin_lock_init(&mut (*call).notify_lock); refcount_set(&mut (*call).ref_, 1);
    (*call).debug_id = debug_id; (*call).rx_pkt_offset = USHRT_MAX; (*call).tx_total_len = -1;
    (*call).tx_jumbo_max = 1; (*call).next_rx_timo = 20 * HZ; (*call).next_req_timo = HZ;
    (*call).ackr_window = 1; (*call).ackr_wtop = 1; (*call).delay_ack_at = KTIME_MAX;
    (*call).rack_timo_at = KTIME_MAX; (*call).ping_at = KTIME_MAX; (*call).keepalive_at = KTIME_MAX;
    (*call).expect_rx_by = KTIME_MAX; (*call).expect_req_by = KTIME_MAX; (*call).expect_term_by = KTIME_MAX;
    core::ptr::write_bytes(&mut (*call).sock_node as *mut _, 0xed, 1);
    (*call).rx_winsize = rxrpc_rx_window_size; (*call).tx_winsize = 16;
    (*call).cong_cwnd = RXRPC_MIN_CWND; (*call).cong_ssthresh = RXRPC_TX_MAX_WINDOW;
    rxrpc_call_init_rtt(call); (*call).rxnet = rxnet; (*call).rtt_avail = RXRPC_CALL_RTT_AVAIL_MASK;
    atomic_inc(&mut (*rxnet).nr_calls); call
}

pub unsafe fn rxrpc_start_call_timer(call: *mut rxrpc_call) {
    if (*call).hard_timo != 0 { let delay = ms_to_ktime((*call).hard_timo * 1000); (*call).expect_term_by = ktime_add(ktime_get_real(), delay); trace_rxrpc_timer_set(call, delay, rxrpc_timer_trace_hard); }
    (*call).timer.expires = jiffies;
}

pub unsafe fn rxrpc_see_call(call: *mut rxrpc_call, why: rxrpc_call_trace) { if !call.is_null() { trace_rxrpc_call((*call).debug_id, refcount_read(&(*call).ref_), 0, why); } }
pub unsafe fn rxrpc_try_get_call(call: *mut rxrpc_call, why: rxrpc_call_trace) -> *mut rxrpc_call { if call.is_null() || !__refcount_inc_not_zero(&mut (*call).ref_, &mut 0) { return core::ptr::null_mut(); } trace_rxrpc_call((*call).debug_id, refcount_read(&(*call).ref_), 0, why); call }
pub unsafe fn rxrpc_get_call(call: *mut rxrpc_call, why: rxrpc_call_trace) { __refcount_inc(&mut (*call).ref_, &mut 0); trace_rxrpc_call((*call).debug_id, refcount_read(&(*call).ref_), 0, why); }

unsafe fn rxrpc_cleanup_tx_buffers(call: *mut rxrpc_call) {
    let mut tq = (*call).tx_queue;
    while !tq.is_null() { let next = (*tq).next; for i in 0..RXRPC_NR_TXQUEUE { if !(*tq).bufs[i].is_null() { rxrpc_put_txbuf((*tq).bufs[i], rxrpc_txbuf_put_cleaned); } } trace_rxrpc_tq(call, tq, 0, rxrpc_tq_cleaned); kfree(tq); tq = next; }
}
unsafe fn rxrpc_cleanup_rx_buffers(call: *mut rxrpc_call) { rxrpc_purge_queue(&mut (*call).recvmsg_queue); rxrpc_purge_queue(&mut (*call).rx_queue); rxrpc_purge_queue(&mut (*call).rx_oos_queue); kfree((*call).rx_dec_buffer); }

pub unsafe fn rxrpc_put_call(call: *mut rxrpc_call, why: rxrpc_call_trace) {
    let rxnet = (*call).rxnet; let debug_id = (*call).debug_id; let mut r = 0;
    if __refcount_dec_and_test(&mut (*call).ref_, &mut r) { trace_rxrpc_call(debug_id, r - 1, 0, why); ASSERTCMP!(__rxrpc_call_state(call), ==, RXRPC_CALL_COMPLETE); spin_lock(&mut (*rxnet).call_lock); list_del_rcu(&mut (*call).link); spin_unlock(&mut (*rxnet).call_lock); rxrpc_cleanup_call(call); } else { trace_rxrpc_call(debug_id, r - 1, 0, why); }
}

unsafe fn rxrpc_rcu_free_call(rcu: *mut rcu_head) { let call = container_of!(rcu, rxrpc_call, rcu); let rxnet = READ_ONCE((*call).rxnet); kmem_cache_free(rxrpc_call_jar, call); if atomic_dec_and_test(&mut (*rxnet).nr_calls) { wake_up_var(&mut (*rxnet).nr_calls); } }
unsafe fn rxrpc_destroy_call_impl(work: *mut work_struct) { let call = container_of!(work, rxrpc_call, destroyer); timer_delete_sync(&mut (*call).timer); rxrpc_cleanup_tx_buffers(call); rxrpc_cleanup_rx_buffers(call); rxrpc_put_txbuf((*call).tx_pending, rxrpc_txbuf_put_cleaned); rxrpc_put_connection((*call).conn, rxrpc_conn_put_call); rxrpc_deactivate_bundle((*call).bundle); rxrpc_put_bundle((*call).bundle, rxrpc_bundle_put_call); rxrpc_put_peer((*call).peer, rxrpc_peer_put_call); rxrpc_put_local((*call).local, rxrpc_local_put_call); key_put((*call).key); call_rcu(&mut (*call).rcu, rxrpc_rcu_free_call); }

pub unsafe fn rxrpc_cleanup_call(call: *mut rxrpc_call) { core::ptr::write_bytes(&mut (*call).sock_node as *mut _, 0xcd, 1); ASSERTCMP!(__rxrpc_call_state(call), ==, RXRPC_CALL_COMPLETE); ASSERT!(test_bit(RXRPC_CALL_RELEASED, &(*call).flags)); timer_delete(&mut (*call).timer); if rcu_read_lock_held() { schedule_work(&mut (*call).destroyer); } else { rxrpc_destroy_call_impl(&mut (*call).destroyer); } }

pub unsafe fn rxrpc_destroy_all_calls(rxnet: *mut rxrpc_net) { if !list_empty(&(*rxnet).calls) { let mut shown = 0; spin_lock(&mut (*rxnet).call_lock); list_for_each_entry!(call, &(*rxnet).calls, link, { rxrpc_see_call(call, rxrpc_call_see_still_live); pr_err!("Call %p still in use (%d,%s,%lx,%lx)!\n", call, refcount_read(&(*call).ref_), RXRPC_CALL_STATES[__rxrpc_call_state(call)], (*call).flags, (*call).events); shown += 1; if shown >= 10 { break; } }); spin_unlock(&mut (*rxnet).call_lock); } atomic_dec(&mut (*rxnet).nr_calls); wait_var_event!(&mut (*rxnet).nr_calls, atomic_read(&(*rxnet).nr_calls) == 0); }

pub unsafe fn rxrpc_kernel_query_call_security(call: *mut rxrpc_call, service_id: *mut u16, enctype: *mut u32) -> u8 { *service_id = (*call).dest_srx.srx_service; *enctype = (*call).security_enctype; (*call).security_ix }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
