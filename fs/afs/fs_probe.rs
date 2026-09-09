// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS fileserver probing
 *
 * Copyright (C) 2018, 2020 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

static mut afs_fs_probe_fast_poll_interval: ::core::ffi::c_uint = 30 * HZ;
static mut afs_fs_probe_slow_poll_interval: ::core::ffi::c_uint = 5 * 60 * HZ;

pub unsafe fn afs_get_endpoint_state(
    estate: *mut afs_endpoint_state,
    where_: afs_estate_trace,
) -> *mut afs_endpoint_state {
    if !estate.is_null() {
        let mut r: ::core::ffi::c_int = 0;
        __refcount_inc(&mut (*estate).ref_, &mut r);
        trace_afs_estate((*estate).server_id, (*estate).probe_seq, r, where_);
    }
    estate
}

unsafe fn afs_endpoint_state_rcu(rcu: *mut rcu_head) {
    let estate = container_of!(rcu, afs_endpoint_state, rcu);
    trace_afs_estate((*estate).server_id, (*estate).probe_seq,
                     refcount_read(&(*estate).ref_), afs_estate_trace_free);
    afs_put_addrlist((*estate).addresses, afs_alist_trace_put_estate);
    kfree(estate as *mut ::core::ffi::c_void);
}

pub unsafe fn afs_put_endpoint_state(estate: *mut afs_endpoint_state, where_: afs_estate_trace) {
    if !estate.is_null() {
        let server_id = (*estate).server_id;
        let probe_seq = (*estate).probe_seq;
        let mut r: ::core::ffi::c_int = 0;
        let dead = __refcount_dec_and_test(&mut (*estate).ref_, &mut r);
        trace_afs_estate(server_id, probe_seq, r, where_);
        if dead { call_rcu(&mut (*estate).rcu, afs_endpoint_state_rcu); }
    }
}

unsafe fn afs_schedule_fs_probe(net: *mut afs_net, server: *mut afs_server, fast: bool) {
    if !(*net).live { return; }
    let mut atj = (*server).probed_at;
    atj += if fast { afs_fs_probe_fast_poll_interval } else { afs_fs_probe_slow_poll_interval } as _;
    afs_inc_servers_outstanding(net);
    if timer_reduce(&mut (*net).fs_probe_timer, atj) { afs_dec_servers_outstanding(net); }
}

unsafe fn afs_finished_fs_probe(net: *mut afs_net, server: *mut afs_server, estate: *mut afs_endpoint_state) {
    let responded = test_bit(AFS_ESTATE_RESPONDED, &(*estate).flags);
    write_seqlock(&mut (*net).fs_lock);
    if responded {
        list_add_tail(&mut (*server).probe_link, &mut (*net).fs_probe_slow);
    } else {
        (*server).rtt = UINT_MAX;
        clear_bit(AFS_SERVER_FL_RESPONDING, &mut (*server).flags);
        list_add_tail(&mut (*server).probe_link, &mut (*net).fs_probe_fast);
    }
    write_sequnlock(&mut (*net).fs_lock);
    afs_schedule_fs_probe(net, server, !responded);
}

unsafe fn afs_done_one_fs_probe(net: *mut afs_net, server: *mut afs_server, estate: *mut afs_endpoint_state) {
    _enter!("");
    if atomic_dec_and_test(&mut (*estate).nr_probing) { afs_finished_fs_probe(net, server, estate); }
    wake_up_all(&mut (*server).probe_wq);
}

unsafe fn afs_fs_probe_not_done(net: *mut afs_net, server: *mut afs_server, estate: *mut afs_endpoint_state, index: ::core::ffi::c_int) {
    _enter!("");
    trace_afs_io_error(0, -ENOMEM, afs_io_error_fs_probe_fail);
    spin_lock(&mut (*server).probe_lock);
    set_bit(AFS_ESTATE_LOCAL_FAILURE, &mut (*estate).flags);
    if (*estate).error == 0 { (*estate).error = -ENOMEM; }
    set_bit(index as _, &mut (*estate).failed_set);
    spin_unlock(&mut (*server).probe_lock);
    afs_done_one_fs_probe(net, server, estate);
}

pub unsafe fn afs_fileserver_probe_result(call: *mut afs_call) {
    let estate = (*call).probe;
    let alist = (*estate).addresses;
    let addr = (*alist).addrs.as_mut_ptr().add((*call).probe_index as usize);
    let server = (*call).server;
    let index = (*call).probe_index;
    let mut rtt_us: ::core::ffi::c_uint = UINT_MAX;
    let mut cap0: ::core::ffi::c_uint;
    let ret = (*call).error;
    _enter!("%pU,%u", &(*server).uuid, index);
    WRITE_ONCE!((*addr).last_error, ret);
    spin_lock(&mut (*server).probe_lock);
    match ret {
        0 => { (*estate).error = 0; }
        -ECONNABORTED => { if !test_bit(AFS_ESTATE_RESPONDED, &(*estate).flags) { (*estate).abort_code = (*call).abort_code; (*estate).error = ret; } }
        -ENOMEM | -ENONET => { clear_bit(index, &mut (*estate).responsive_set); set_bit(AFS_ESTATE_LOCAL_FAILURE, &mut (*estate).flags); trace_afs_io_error((*call).debug_id, ret, afs_io_error_fs_probe_fail); spin_unlock(&mut (*server).probe_lock); trace_afs_fs_probe(server, false, estate, index, (*call).error, (*call).abort_code, rtt_us); return afs_done_one_fs_probe((*call).net, server, estate); }
        _ => { clear_bit(index, &mut (*estate).responsive_set); set_bit(index, &mut (*estate).failed_set); if !test_bit(AFS_ESTATE_RESPONDED, &(*estate).flags) && ((*estate).error == 0 || (*estate).error == -ETIMEDOUT || (*estate).error == -ETIME) { (*estate).error = ret; } trace_afs_io_error((*call).debug_id, ret, afs_io_error_fs_probe_fail); spin_unlock(&mut (*server).probe_lock); trace_afs_fs_probe(server, false, estate, index, (*call).error, (*call).abort_code, rtt_us); return afs_done_one_fs_probe((*call).net, server, estate); }
    }
    clear_bit(index, &mut (*estate).failed_set);
    if (*call).service_id == YFS_FS_SERVICE { set_bit(AFS_ESTATE_IS_YFS, &mut (*estate).flags); set_bit(AFS_SERVER_FL_IS_YFS, &mut (*server).flags); (*server).service_id = (*call).service_id; }
    else { set_bit(AFS_ESTATE_NOT_YFS, &mut (*estate).flags); if !test_bit(AFS_ESTATE_IS_YFS, &(*estate).flags) { clear_bit(AFS_SERVER_FL_IS_YFS, &mut (*server).flags); (*server).service_id = (*call).service_id; } cap0 = ntohl((*call).tmp); if cap0 & AFS3_VICED_CAPABILITY_64BITFILES != 0 { set_bit(AFS_SERVER_FL_HAS_FS64, &mut (*server).flags); } else { clear_bit(AFS_SERVER_FL_HAS_FS64, &mut (*server).flags); } }
    rtt_us = rxrpc_kernel_get_srtt((*addr).peer);
    if rtt_us < (*estate).rtt { (*estate).rtt = rtt_us; (*server).rtt = rtt_us; (*alist).preferred = index; }
    smp_wmb(); set_bit(AFS_ESTATE_RESPONDED, &mut (*estate).flags); set_bit(index, &mut (*estate).responsive_set); set_bit(AFS_SERVER_FL_RESPONDING, &mut (*server).flags);
    spin_unlock(&mut (*server).probe_lock);
    trace_afs_fs_probe(server, false, estate, index, (*call).error, (*call).abort_code, rtt_us);
    afs_done_one_fs_probe((*call).net, server, estate);
}

pub unsafe fn afs_fs_probe_fileserver(net: *mut afs_net, server: *mut afs_server, new_alist: *mut afs_addr_list, key: *mut key) -> ::core::ffi::c_int {
    let estate = kzalloc_obj!(afs_endpoint_state); if estate.is_null() { return -ENOMEM; }
    refcount_set(&mut (*estate).ref_, 2); (*estate).server_id = (*server).debug_id; (*estate).rtt = UINT_MAX;
    write_lock(&mut (*server).fs_lock); let old = rcu_dereference_protected((*server).endpoint_state, lockdep_is_held(&(*server).fs_lock));
    if !old.is_null() { (*estate).responsive_set = (*old).responsive_set; if new_alist.is_null() { new_alist = (*old).addresses; } }
    afs_set_peer_appdata(server, core::ptr::null_mut(), new_alist); (*estate).addresses = afs_get_addrlist(new_alist, afs_alist_trace_get_estate); let alist = (*estate).addresses; (*server).probe_counter += 1; (*estate).probe_seq = (*server).probe_counter; atomic_set(&mut (*estate).nr_probing, (*alist).nr_addrs); if !new_alist.is_null() { (*server).addr_version = (*new_alist).version; } rcu_assign_pointer!((*server).endpoint_state, estate); write_unlock(&mut (*server).fs_lock); if !old.is_null() { set_bit(AFS_ESTATE_SUPERSEDED, &mut (*old).flags); }
    trace_afs_estate((*estate).server_id, (*estate).probe_seq, refcount_read(&(*estate).ref_), afs_estate_trace_alloc_probe); afs_get_address_preferences(net, new_alist); (*server).probed_at = jiffies; let mut unprobed = (1 as ::core::ffi::c_ulong).wrapping_shl((*alist).nr_addrs as u32).wrapping_sub(1);
    while unprobed != 0 { let mut index = 0; let mut best_prio = -1; for i in 0..(*alist).nr_addrs { if test_bit(i, &unprobed) && (*alist).addrs[i as usize].prio > best_prio { index=i; best_prio=(*alist).addrs[i as usize].prio; } } __clear_bit(index, &mut unprobed); trace_afs_fs_probe(server,true,estate,index,0,0,0); if !afs_fs_get_capabilities(net,server,estate,index,key) { afs_fs_probe_not_done(net,server,estate,index as _); } }
    afs_put_endpoint_state(old, afs_estate_trace_put_probe); afs_put_endpoint_state(estate, afs_estate_trace_put_probe); 0
}

pub unsafe fn afs_wait_for_fs_probes(op: *mut afs_operation, states: *mut afs_server_state, intr: bool) -> ::core::ffi::c_int { let slist=(*op).server_list; for i in 0..(*slist).nr_servers { let estate=(*states.add(i as usize)).endpoint_state; if test_bit(AFS_ESTATE_SUPERSEDED,&(*estate).flags){return 2;} if (*estate).responsive_set & (*states.add(i as usize)).untried_addrs != 0{return 1;} } 0 }
pub unsafe fn afs_fs_probe_timer(timer: *mut timer_list) { let net=container_of!(timer,afs_net,fs_probe_timer); if !(*net).live || !queue_work(afs_wq,&mut (*net).fs_prober){afs_dec_servers_outstanding(net);} }
pub unsafe fn afs_probe_fileserver(net: *mut afs_net, server: *mut afs_server) { write_seqlock(&mut (*net).fs_lock); if !list_empty(&(*server).probe_link){return afs_dispatch_fs_probe(net,server);} write_sequnlock(&mut (*net).fs_lock); }
pub unsafe fn afs_fs_probe_dispatcher(work: *mut work_struct) { let net=container_of!(work,afs_net,fs_prober); if !(*net).live {afs_dec_servers_outstanding(net);return;} if list_empty(&(*net).fs_probe_fast)&&list_empty(&(*net).fs_probe_slow){afs_dec_servers_outstanding(net);return;} write_seqlock(&mut (*net).fs_lock); if !list_empty(&(*net).fs_probe_fast){let s=list_first_entry!(&(*net).fs_probe_fast,afs_server,probe_link); afs_dispatch_fs_probe(net,s);} else if !list_empty(&(*net).fs_probe_slow){let s=list_first_entry!(&(*net).fs_probe_slow,afs_server,probe_link); afs_dispatch_fs_probe(net,s);} else {write_sequnlock(&mut (*net).fs_lock);afs_dec_servers_outstanding(net);} }
pub unsafe fn afs_wait_for_one_fs_probe(server:*mut afs_server,estate:*mut afs_endpoint_state,exclude:::core::ffi::c_ulong,is_intr:bool)->::core::ffi::c_int{let _=(server,estate,exclude,is_intr); -EDESTADDRREQ}
pub unsafe fn afs_fs_probe_cleanup(net:*mut afs_net){if timer_delete_sync(&mut (*net).fs_probe_timer){afs_dec_servers_outstanding(net);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
