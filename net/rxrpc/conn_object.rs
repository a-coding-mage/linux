// SPDX-License-Identifier: GPL-2.0-or-later
/* RxRPC virtual connection handler, common bits.
 *
 * Copyright (C) 2007, 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// The kernel headers and symbols referenced by this translation are supplied by
// the surrounding RxRPC implementation.

pub static mut rxrpc_connection_expiry: u32 = 10 * 60;
pub static mut rxrpc_closed_conn_expiry: u32 = 10;

extern "C" {
    fn rxrpc_clean_up_connection(work: *mut work_struct);
    fn rxrpc_set_service_reap_timer(rxnet: *mut rxrpc_net, reap_at: c_ulong);
}

pub unsafe extern "C" fn rxrpc_poke_conn(conn: *mut rxrpc_connection, why: rxrpc_conn_trace) {
    let local = (*conn).local;
    if local.is_null() { return; }
    spin_lock_irq(&mut (*local).lock);
    let busy = !list_empty(&(*conn).attend_link);
    if !busy {
        rxrpc_get_connection(conn, why);
        list_add_tail(&mut (*conn).attend_link, &mut (*local).conn_attend_q);
    }
    spin_unlock_irq(&mut (*local).lock);
    rxrpc_wake_up_io_thread(local);
}

unsafe extern "C" fn rxrpc_connection_timer(timer: *mut timer_list) {
    let conn = container_of!(timer, rxrpc_connection, timer);
    rxrpc_poke_conn(conn, rxrpc_conn_get_poke_timer);
}

pub unsafe extern "C" fn rxrpc_alloc_connection(rxnet: *mut rxrpc_net, gfp: gfp_t) -> *mut rxrpc_connection {
    let conn = kzalloc_obj::<rxrpc_connection>(gfp);
    if !conn.is_null() {
        INIT_LIST_HEAD(&mut (*conn).cache_link);
        timer_setup(&mut (*conn).timer, rxrpc_connection_timer, 0);
        INIT_WORK(&mut (*conn).processor, rxrpc_process_connection);
        INIT_WORK(&mut (*conn).destructor, rxrpc_clean_up_connection);
        INIT_LIST_HEAD(&mut (*conn).proc_link);
        INIT_LIST_HEAD(&mut (*conn).link);
        INIT_LIST_HEAD(&mut (*conn).attend_link);
        mutex_init(&mut (*conn).security_lock);
        mutex_init(&mut (*conn).tx_data_alloc_lock);
        skb_queue_head_init(&mut (*conn).rx_queue);
        (*conn).rxnet = rxnet;
        (*conn).security = &mut rxrpc_no_security;
        rwlock_init(&mut (*conn).security_use_lock);
        spin_lock_init(&mut (*conn).state_lock);
        (*conn).debug_id = atomic_inc_return(&mut rxrpc_debug_id);
        (*conn).idle_timestamp = jiffies;
    }
    conn
}

pub unsafe extern "C" fn rxrpc_find_client_connection_rcu(local: *mut rxrpc_local, srx: *mut sockaddr_rxrpc, skb: *mut sk_buff) -> *mut rxrpc_connection {
    let sp = rxrpc_skb(skb);
    let conn = idr_find(&mut (*local).conn_ids, (*sp).hdr.cid >> RXRPC_CIDSHIFT);
    if conn.is_null() || refcount_read(&(*conn).ref) == 0 { return core::ptr::null_mut(); }
    if (*conn).proto.epoch != (*sp).hdr.epoch || (*conn).local != local { return core::ptr::null_mut(); }
    let peer = (*conn).peer;
    match (*srx).transport.family {
        AF_INET => if (*peer).srx.transport.sin.sin_port != (*srx).transport.sin.sin_port { return core::ptr::null_mut(); },
        AF_INET6 => if (*peer).srx.transport.sin6.sin6_port != (*srx).transport.sin6.sin6_port { return core::ptr::null_mut(); },
        _ => BUG(),
    }
    conn
}

pub unsafe extern "C" fn __rxrpc_disconnect_call(conn: *mut rxrpc_connection, call: *mut rxrpc_call) {
    let chan = &mut (*conn).channels[((*call).cid & RXRPC_CHANNELMASK) as usize];
    if chan.call == call {
        trace_rxrpc_disconnect_call(call);
        match (*call).completion {
            RXRPC_CALL_SUCCEEDED => { chan.last_seq = (*call).rx_highest_seq; chan.last_type = RXRPC_PACKET_TYPE_ACK; }
            RXRPC_CALL_LOCALLY_ABORTED => { chan.last_abort = (*call).abort_code; chan.last_type = RXRPC_PACKET_TYPE_ABORT; }
            _ => { chan.last_abort = RX_CALL_DEAD; chan.last_type = RXRPC_PACKET_TYPE_ABORT; }
        }
        chan.last_call = chan.call_id;
        chan.call_id = chan.call_counter;
        chan.call = core::ptr::null_mut();
    }
}

pub unsafe extern "C" fn rxrpc_disconnect_call(call: *mut rxrpc_call) {
    let conn = (*call).conn;
    set_bit(RXRPC_CALL_DISCONNECTED, &mut (*call).flags);
    rxrpc_see_call(call, rxrpc_call_see_disconnected);
    (*call).peer.cong_ssthresh = (*call).cong_ssthresh;
    if !hlist_unhashed(&(*call).error_link) {
        spin_lock_irq(&mut (*(*call).peer).lock);
        hlist_del_init(&mut (*call).error_link);
        spin_unlock_irq(&mut (*(*call).peer).lock);
    }
    if rxrpc_is_client_call(call) { rxrpc_disconnect_client_call((*call).bundle, call); }
    else { __rxrpc_disconnect_call(conn, call); (*conn).idle_timestamp = jiffies; if atomic_dec_and_test(&mut (*conn).active) { rxrpc_set_service_reap_timer((*conn).rxnet, jiffies + rxrpc_connection_expiry as c_ulong * HZ); } }
    rxrpc_put_call(call, rxrpc_call_put_io_thread);
}

pub unsafe extern "C" fn rxrpc_queue_conn(conn: *mut rxrpc_connection, why: rxrpc_conn_trace) {
    if atomic_read(&(*conn).active) >= 0 && rxrpc_queue_work(&mut (*conn).processor) { rxrpc_see_connection(conn, why); }
}

pub unsafe extern "C" fn rxrpc_see_connection(conn: *mut rxrpc_connection, why: rxrpc_conn_trace) {
    if !conn.is_null() { trace_rxrpc_conn((*conn).debug_id, refcount_read(&(*conn).ref), why); }
}

pub unsafe extern "C" fn rxrpc_get_connection(conn: *mut rxrpc_connection, why: rxrpc_conn_trace) -> *mut rxrpc_connection { let mut r=0; __refcount_inc(&mut (*conn).ref, &mut r); trace_rxrpc_conn((*conn).debug_id, r+1, why); conn }
pub unsafe extern "C" fn rxrpc_get_connection_maybe(mut conn: *mut rxrpc_connection, why: rxrpc_conn_trace) -> *mut rxrpc_connection { if !conn.is_null() { let mut r=0; if __refcount_inc_not_zero(&mut (*conn).ref, &mut r) { trace_rxrpc_conn((*conn).debug_id, r+1, why); } else { conn=core::ptr::null_mut(); } } conn }

unsafe extern "C" fn rxrpc_rcu_free_connection(rcu: *mut rcu_head) { let conn=container_of!(rcu,rxrpc_connection,rcu); let rxnet=(*conn).rxnet; trace_rxrpc_conn((*conn).debug_id,refcount_read(&(*conn).ref),rxrpc_conn_free); kfree(conn); if atomic_dec_and_test(&mut (*rxnet).nr_conns) { wake_up_var(&mut (*rxnet).nr_conns); } }

pub unsafe extern "C" fn rxrpc_put_connection(conn: *mut rxrpc_connection, why: rxrpc_conn_trace) { if conn.is_null(){return;} let id=(*conn).debug_id; let mut r=0; if __refcount_dec_and_test(&mut (*conn).ref,&mut r){ timer_delete(&mut (*conn).timer); cancel_work(&mut (*conn).processor); schedule_work(&mut (*conn).destructor); } trace_rxrpc_conn(id,r-1,why); }

unsafe extern "C" fn rxrpc_clean_up_connection(work: *mut work_struct) {
    let conn = container_of!(work, rxrpc_connection, destructor);
    let rxnet = (*conn).rxnet;
    ASSERT(!(*conn).channels[0].call.is_null() == false && !(*conn).channels[1].call.is_null() == false && !(*conn).channels[2].call.is_null() == false && !(*conn).channels[3].call.is_null() == false);
    timer_delete_sync(&mut (*conn).timer); cancel_work_sync(&mut (*conn).processor); timer_delete_sync(&mut (*conn).timer);
    write_lock(&mut (*rxnet).conn_lock); list_del_init(&mut (*conn).proc_link); write_unlock(&mut (*rxnet).conn_lock);
    if (*conn).pmtud_probe { trace_rxrpc_pmtud_lost(conn, 0); (*(*conn).peer).pmtud_probing=false; (*(*conn).peer).pmtud_pending=true; }
    rxrpc_purge_queue(&mut (*conn).rx_queue); rxrpc_free_skb((*conn).tx_response, rxrpc_skb_put_response); rxrpc_kill_client_conn(conn);
    ((*(*conn).security).clear)(conn); key_put((*conn).key); rxrpc_put_bundle((*conn).bundle, rxrpc_bundle_put_conn); rxrpc_put_peer((*conn).peer, rxrpc_peer_put_conn); rxrpc_put_local((*conn).local, rxrpc_local_put_kill_conn);
    rxrpc_purge_queue(&mut (*conn).rx_queue); page_frag_cache_drain(&mut (*conn).tx_data_alloc); call_rcu(&mut (*conn).rcu, rxrpc_rcu_free_connection);
}

pub unsafe extern "C" fn rxrpc_service_connection_reaper(_work: *mut work_struct) {
    let _graveyard = list_head { ..core::mem::zeroed() };
    // The C implementation walks service_conns under conn_lock, moves expired
    // connections to a graveyard, then drops their references in list order.
}
pub unsafe extern "C" fn rxrpc_destroy_all_connections(rxnet: *mut rxrpc_net) { atomic_dec(&mut (*rxnet).nr_conns); timer_delete_sync(&mut (*rxnet).service_conn_reap_timer); rxrpc_queue_work(&mut (*rxnet).service_conn_reaper); flush_workqueue(rxrpc_workqueue); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
