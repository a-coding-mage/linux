// SPDX-License-Identifier: GPL-2.0-or-later
/* Client connection-specific management code. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Kernel headers and symbols referenced by this translation are supplied by
// the surrounding rxrpc Rust environment.

extern "C" {
    static mut rxrpc_reap_client_connections: u32;
    static mut rxrpc_conn_idle_client_expiry: c_ulong;
    static mut rxrpc_conn_idle_client_fast_expiry: c_ulong;
}

type c_ulong = usize;
type c_long = isize;
type c_int = i32;
type gfp_t = u32;
type u32_ = u32;

unsafe fn rxrpc_activate_bundle(bundle: *mut rxrpc_bundle) { atomic_inc(&mut (*bundle).active); }

unsafe fn rxrpc_put_client_connection_id(local: *mut rxrpc_local, conn: *mut rxrpc_connection) {
    idr_remove(&mut (*local).conn_ids, (*conn).proto.cid >> RXRPC_CIDSHIFT);
}

unsafe fn rxrpc_destroy_client_conn_ids(local: *mut rxrpc_local) {
    if !idr_is_empty(&mut (*local).conn_ids) {
        let mut conn: *mut rxrpc_connection = core::ptr::null_mut();
        let mut id = 0;
        idr_for_each_entry(&mut (*local).conn_ids, &mut conn, &mut id);
        BUG();
    }
    idr_destroy(&mut (*local).conn_ids);
}

unsafe fn rxrpc_alloc_bundle(call: *mut rxrpc_call, gfp: gfp_t) -> *mut rxrpc_bundle {
    static mut rxrpc_bundle_id: atomic_t = atomic_t::default();
    let bundle = kzalloc_bundle(gfp);
    if !bundle.is_null() {
        (*bundle).local = (*call).local;
        (*bundle).peer = rxrpc_get_peer((*call).peer, rxrpc_peer_get_bundle);
        (*bundle).key = key_get((*call).key);
        (*bundle).security = (*call).security;
        (*bundle).exclusive = test_bit(RXRPC_CALL_EXCLUSIVE, &(*call).flags);
        (*bundle).upgrade = test_bit(RXRPC_CALL_UPGRADE, &(*call).flags);
        (*bundle).service_id = (*call).dest_srx.srx_service;
        (*bundle).security_level = (*call).security_level;
        (*bundle).debug_id = atomic_inc_return(&mut rxrpc_bundle_id);
        refcount_set(&mut (*bundle).ref_, 1); atomic_set(&mut (*bundle).active, 1);
        INIT_LIST_HEAD(&mut (*bundle).waiting_calls);
        trace_rxrpc_bundle((*bundle).debug_id, 1, rxrpc_bundle_new);
        write_lock(&mut (*(*bundle).local).rxnet.conn_lock);
        list_add_tail(&mut (*bundle).proc_link, &mut (*(*(*bundle).local).rxnet).bundle_proc_list);
        write_unlock(&mut (*(*bundle).local).rxnet.conn_lock);
    }
    bundle
}

#[no_mangle]
pub unsafe extern "C" fn rxrpc_get_bundle(bundle: *mut rxrpc_bundle, why: rxrpc_bundle_trace) -> *mut rxrpc_bundle {
    let mut r = 0; __refcount_inc(&mut (*bundle).ref_, &mut r);
    trace_rxrpc_bundle((*bundle).debug_id, r + 1, why); bundle
}

unsafe fn rxrpc_free_bundle(bundle: *mut rxrpc_bundle) {
    trace_rxrpc_bundle((*bundle).debug_id, refcount_read(&(*bundle).ref_), rxrpc_bundle_free);
    write_lock(&mut (*(*bundle).local).rxnet.conn_lock);
    list_del(&mut (*bundle).proc_link); write_unlock(&mut (*(*bundle).local).rxnet.conn_lock);
    rxrpc_put_peer((*bundle).peer, rxrpc_peer_put_bundle); key_put((*bundle).key); kfree(bundle as *mut _);
}

#[no_mangle]
pub unsafe extern "C" fn rxrpc_put_bundle(bundle: *mut rxrpc_bundle, why: rxrpc_bundle_trace) {
    if !bundle.is_null() { let id=(*bundle).debug_id; let mut r=0;
        if __refcount_dec_and_test(&mut (*bundle).ref_, &mut r) { trace_rxrpc_bundle(id,r-1,why); rxrpc_free_bundle(bundle); }
        else { trace_rxrpc_bundle(id,r-1,why); }
    }
}

#[no_mangle] pub unsafe extern "C" fn rxrpc_purge_client_connections(local:*mut rxrpc_local){rxrpc_destroy_client_conn_ids(local);}

unsafe fn rxrpc_alloc_client_connection(bundle:*mut rxrpc_bundle)->*mut rxrpc_connection {
    let local=(*bundle).local; let rxnet=(*local).rxnet;
    let conn=rxrpc_alloc_connection(rxnet,GFP_ATOMIC|__GFP_NOWARN); if conn.is_null(){return ERR_PTR(-ENOMEM);}
    let id=idr_alloc_cyclic(&mut (*local).conn_ids,conn,1,0x40000000,GFP_ATOMIC|__GFP_NOWARN);
    if id<0 { kfree(conn as *mut _); return ERR_PTR(id); }
    refcount_set(&mut (*conn).ref_,1); (*conn).proto.cid=id<<RXRPC_CIDSHIFT; (*conn).proto.epoch=(*rxnet).epoch;
    (*conn).out_clientflag=RXRPC_CLIENT_INITIATED; (*conn).bundle=rxrpc_get_bundle(bundle,rxrpc_bundle_get_client_conn);
    (*conn).local=rxrpc_get_local(local,rxrpc_local_get_client_conn); (*conn).peer=rxrpc_get_peer((*bundle).peer,rxrpc_peer_get_client_conn);
    (*conn).key=key_get((*bundle).key); (*conn).security=(*bundle).security; (*conn).exclusive=(*bundle).exclusive;
    (*conn).upgrade=(*bundle).upgrade; (*conn).orig_service_id=(*bundle).service_id; (*conn).security_level=(*bundle).security_level;
    (*conn).state=RXRPC_CONN_CLIENT_UNSECURED; (*conn).service_id=(*conn).orig_service_id;
    if (*conn).security==&rxrpc_no_security {(*conn).state=RXRPC_CONN_CLIENT;}
    atomic_inc(&mut (*rxnet).nr_conns); write_lock(&mut (*rxnet).conn_lock); list_add_tail(&mut (*conn).proc_link,&mut (*rxnet).conn_proc_list); write_unlock(&mut (*rxnet).conn_lock);
    rxrpc_see_connection(conn,rxrpc_conn_new_client); atomic_inc(&mut (*rxnet).nr_client_conns); trace_rxrpc_client(conn,-1,rxrpc_client_alloc); conn
}

unsafe fn rxrpc_may_reuse_conn(conn:*mut rxrpc_connection)->bool {
    if conn.is_null(){return false;} let rxnet=(*conn).rxnet;
    if test_bit(RXRPC_CONN_DONT_REUSE,&(*conn).flags){return false;}
    if ((*conn).state!=RXRPC_CONN_CLIENT_UNSECURED&&(*conn).state!=RXRPC_CONN_CLIENT)||(*conn).proto.epoch!=(*rxnet).epoch {set_bit(RXRPC_CONN_DONT_REUSE,&mut (*conn).flags);return false;}
    let cursor=idr_get_cursor(&mut (*(*conn).local).conn_ids); let id=(*conn).proto.cid>>RXRPC_CIDSHIFT;
    let distance=(id-cursor).abs(); let limit=umax(atomic_read(&(*rxnet).nr_conns)*4,1024); if distance>limit {set_bit(RXRPC_CONN_DONT_REUSE,&mut (*conn).flags);return false;} true
}

#[no_mangle]
pub unsafe extern "C" fn rxrpc_look_up_bundle(call:*mut rxrpc_call,gfp:gfp_t)->c_int {
    if test_bit(RXRPC_CALL_EXCLUSIVE,&(*call).flags){(*call).bundle=rxrpc_alloc_bundle(call,gfp);return if (*call).bundle.is_null(){-ENOMEM}else{0};}
    let local=(*call).local; spin_lock(&mut (*local).client_bundles_lock); let mut b=(*local).client_bundles.rb_node;
    while !b.is_null(){let bundle=rb_entry_bundle(b); if (*bundle).peer==(*call).peer&&(*bundle).key==(*call).key&&(*bundle).security_level==(*call).security_level&&(*bundle).upgrade==test_bit(RXRPC_CALL_UPGRADE,&(*call).flags){(*call).bundle=rxrpc_get_bundle(bundle,rxrpc_bundle_get_client_call);rxrpc_activate_bundle(bundle);spin_unlock(&mut (*local).client_bundles_lock);return 0;} b=if (*bundle).peer<(*call).peer{(*b).rb_right}else{(*b).rb_left};}
    spin_unlock(&mut (*local).client_bundles_lock); -ENOMEM
}

#[no_mangle] pub unsafe extern "C" fn rxrpc_connect_client_calls(local:*mut rxrpc_local){
    let mut call=list_first_entry_or_null(&mut (*local).new_client_calls);
    while !call.is_null(){let bundle=(*call).bundle; list_move_tail(&mut (*call).wait_link,&mut (*bundle).waiting_calls); rxrpc_see_call(call,rxrpc_call_see_waiting_call); if rxrpc_bundle_has_space(bundle){rxrpc_activate_channels(bundle);} call=list_first_entry_or_null(&mut (*local).new_client_calls);}
}

unsafe fn rxrpc_add_conn_to_bundle(bundle:*mut rxrpc_bundle,slot:usize)->bool {let old=(*bundle).conns[slot]; if !old.is_null(){(*bundle).conns[slot]=core::ptr::null_mut();(*bundle).conn_ids[slot]=0;rxrpc_put_connection(old,rxrpc_conn_put_noreuse);} let conn=rxrpc_alloc_client_connection(bundle);if IS_ERR(conn){(*bundle).alloc_error=PTR_ERR(conn);return false;}rxrpc_activate_bundle(bundle);(*conn).bundle_shift=slot*RXRPC_MAXCALLS;(*bundle).conns[slot]=conn;(*bundle).conn_ids[slot]=(*conn).debug_id;for i in 0..RXRPC_MAXCALLS{set_bit((*conn).bundle_shift+i,&mut (*bundle).avail_chans);}true}

unsafe fn rxrpc_bundle_has_space(bundle:*mut rxrpc_bundle)->bool {let mut slot:isize=-1;let mut usable=0;(*bundle).alloc_error=0;for i in 0..(*bundle).conns.len(){if rxrpc_may_reuse_conn((*bundle).conns[i]){usable+=1;}else if slot<0{slot=i as isize;}}if usable==0&&(*bundle).upgrade{(*bundle).try_upgrade=true;}if usable==0||(!(*bundle).avail_chans&& !(*bundle).try_upgrade&&usable<(*bundle).conns.len()){return slot>=0&&rxrpc_add_conn_to_bundle(bundle,slot as usize);}true}

unsafe fn rxrpc_activate_one_channel(conn:*mut rxrpc_connection,channel:usize){let chan=&mut (*conn).channels[channel];let bundle=(*conn).bundle;let call=list_first_entry(&mut (*bundle).waiting_calls);list_del_init(&mut (*call).wait_link);clear_bit(RXRPC_CONN_FINAL_ACK_0+channel,&mut (*conn).flags);clear_bit((*conn).bundle_shift+channel,&mut (*bundle).avail_chans);rxrpc_see_call(call,rxrpc_call_see_activate_client);(*call).conn=rxrpc_get_connection(conn,rxrpc_conn_get_activate_call);(*call).cid=(*conn).proto.cid|channel;(*call).call_id=chan.call_counter+1;(*call).dest_srx.srx_service=(*conn).service_id;chan.call_id=(*call).call_id;chan.call=call;rxrpc_see_call(call,rxrpc_call_see_connected);trace_rxrpc_connect_call(call);(*call).tx_last_sent=ktime_get_real();rxrpc_start_call_timer(call);rxrpc_set_call_state(call,RXRPC_CALL_CLIENT_PRE_SEND);wake_up(&mut (*call).waitq);}

unsafe fn rxrpc_activate_channels(bundle:*mut rxrpc_bundle){let mask=if (*bundle).try_upgrade{1}else{usize::MAX};while !list_empty(&(*bundle).waiting_calls){let avail=(*bundle).avail_chans&mask;if avail==0{break;}let channel=avail.trailing_zeros() as usize;let slot=channel/RXRPC_MAXCALLS;let conn=(*bundle).conns[slot];if conn.is_null(){break;}clear_bit(channel,&mut (*bundle).avail_chans);rxrpc_unidle_conn(conn);rxrpc_activate_one_channel(conn,channel&(RXRPC_MAXCALLS-1));}}
unsafe fn rxrpc_unidle_conn(conn:*mut rxrpc_connection){if !list_empty(&(*conn).cache_link){list_del_init(&mut (*conn).cache_link);rxrpc_put_connection(conn,rxrpc_conn_put_unidle);}}

#[no_mangle] pub unsafe extern "C" fn rxrpc_expose_client_call(call:*mut rxrpc_call){let conn=(*call).conn;let channel=(*call).cid&RXRPC_CHANNELMASK;if !test_and_set_bit(RXRPC_CALL_EXPOSED,&mut (*call).flags){(*conn).channels[channel].call_counter+=1;if (*conn).channels[channel].call_counter>=INT_MAX{set_bit(RXRPC_CONN_DONT_REUSE,&mut (*conn).flags);}}}

#[no_mangle] pub unsafe extern "C" fn rxrpc_disconnect_client_call(bundle:*mut rxrpc_bundle,call:*mut rxrpc_call){let conn=(*call).conn;if conn.is_null(){list_del_init(&mut (*call).wait_link);return;}let channel=(*call).cid&RXRPC_CHANNELMASK;let chan=&mut (*conn).channels[channel];if chan.call!=call{return;}let reuse=rxrpc_may_reuse_conn(conn);if test_bit(RXRPC_CALL_EXPOSED,&(*call).flags){__rxrpc_disconnect_call(conn,call);}if reuse&&!list_empty(&(*bundle).waiting_calls){rxrpc_activate_one_channel(conn,channel);return;}chan.call=core::ptr::null_mut();set_bit((*conn).bundle_shift+channel,&mut (*bundle).avail_chans);(*conn).act_chans&=!(1<<channel);if (*conn).act_chans==0{rxrpc_get_connection(conn,rxrpc_conn_get_idle);list_move_tail(&mut (*conn).cache_link,&mut (*(*conn).local).idle_client_conns);}}

#[no_mangle] pub unsafe extern "C" fn rxrpc_deactivate_bundle(bundle:*mut rxrpc_bundle){if bundle.is_null(){return;}let local=(*bundle).local;if atomic_dec_and_lock(&mut (*bundle).active,&mut (*local).client_bundles_lock){if !(*bundle).exclusive{rb_erase(&mut (*bundle).local_node,&mut (*local).client_bundles);rxrpc_put_bundle(bundle,rxrpc_bundle_put_discard);}spin_unlock(&mut (*local).client_bundles_lock);}}
#[no_mangle] pub unsafe extern "C" fn rxrpc_kill_client_conn(conn:*mut rxrpc_connection){let local=(*conn).local;atomic_dec(&mut (*(*local).rxnet).nr_client_conns);rxrpc_put_client_connection_id(local,conn);}
#[no_mangle] pub unsafe extern "C" fn rxrpc_clean_up_local_conns(local:*mut rxrpc_local){(*local).kill_all_client_conns=true;timer_delete_sync(&mut (*local).client_conn_reap_timer);while let Some(conn)=list_first_entry_or_null(&mut (*local).idle_client_conns){list_del_init(&mut (*conn).cache_link);rxrpc_unbundle_conn(conn);rxrpc_put_connection(conn,rxrpc_conn_put_local_dead);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
