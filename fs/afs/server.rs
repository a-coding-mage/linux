// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS server record management
 *
 * Copyright (C) 2002, 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux/project headers from the original translation unit supply the external
// types, constants, functions, and macros referenced below.

static mut AFS_SERVER_GC_DELAY: u32 = 10; /* Server record timeout in seconds */
static mut AFS_SERVER_DEBUG_ID: atomic_t = atomic_t::new();

/* Find a server by one of its addresses. */
pub unsafe fn afs_find_server(peer: *const rxrpc_peer) -> *mut afs_server {
    let server = rxrpc_kernel_get_peer_data(peer) as *mut afs_server;
    if server.is_null() { return core::ptr::null_mut(); }
    afs_use_server(server, false, afs_server_trace_use_cm_call)
}

/* Look up a server by its UUID and mark it active.  The caller must hold cell->fs_lock. */
unsafe fn afs_find_server_by_uuid(cell: *mut afs_cell, uuid: *const uuid_t) -> *mut afs_server {
    let mut p = (*cell).fs_servers.rb_node;
    while !p.is_null() {
        let server = rb_entry(p, afs_server, uuid_rb);
        let diff = memcmp(uuid as *const _, &(*server).uuid as *const _ as *const _, core::mem::size_of::<uuid_t>());
        if diff < 0 { p = (*p).rb_left; }
        else if diff > 0 { p = (*p).rb_right; }
        else {
            if test_bit(AFS_SERVER_FL_UNCREATED, &(*server).flags) { return core::ptr::null_mut(); }
            afs_use_server(server, true, afs_server_trace_use_by_uuid);
            return server;
        }
    }
    core::ptr::null_mut()
}

/* Install a server record in the cell tree.  The caller must hold an exclusive lock on cell->fs_lock. */
unsafe fn afs_install_server(cell: *mut afs_cell, candidate: *mut *mut afs_server) -> *mut afs_server {
    let net = (*cell).net;
    let mut pp = &mut (*cell).fs_servers.rb_node as *mut *mut rb_node;
    let mut p: *mut rb_node = core::ptr::null_mut();
    while !(*pp).is_null() {
        p = *pp;
        let server = rb_entry(p, afs_server, uuid_rb);
        let diff = memcmp(&(**candidate).uuid as *const _ as *const _, &(*server).uuid as *const _ as *const _, core::mem::size_of::<uuid_t>());
        if diff < 0 { pp = &mut (*p).rb_left; }
        else if diff > 0 { pp = &mut (*p).rb_right; }
        else { return afs_use_server(server, true, afs_server_trace_use_install); }
    }
    let server = *candidate;
    *candidate = core::ptr::null_mut();
    rb_link_node(&mut (*server).uuid_rb, p, pp);
    rb_insert_color(&mut (*server).uuid_rb, &mut (*cell).fs_servers);
    write_seqlock(&mut (*net).fs_lock);
    hlist_add_head_rcu(&mut (*server).proc_link, &mut (*net).fs_proc);
    write_sequnlock(&mut (*net).fs_lock);
    afs_get_cell(cell, afs_cell_trace_get_server);
    afs_use_server(server, true, afs_server_trace_use_install)
}

/* Allocate a new server record and mark it as active but uncreated. */
unsafe fn afs_alloc_server(cell: *mut afs_cell, uuid: *const uuid_t) -> *mut afs_server {
    let net = (*cell).net;
    let server = kzalloc_obj::<afs_server>();
    if server.is_null() { return core::ptr::null_mut(); }
    refcount_set(&mut (*server).ref_, 1);
    atomic_set(&mut (*server).active, 0);
    __set_bit(AFS_SERVER_FL_UNCREATED, &mut (*server).flags);
    (*server).debug_id = atomic_inc_return(&mut AFS_SERVER_DEBUG_ID);
    (*server).uuid = *uuid;
    rwlock_init(&mut (*server).fs_lock);
    INIT_WORK(&mut (*server).destroyer, afs_server_destroyer);
    timer_setup(&mut (*server).timer, afs_server_timer, 0);
    INIT_LIST_HEAD(&mut (*server).volumes);
    init_waitqueue_head(&mut (*server).probe_wq);
    mutex_init(&mut (*server).cm_token_lock);
    INIT_LIST_HEAD(&mut (*server).probe_link);
    INIT_HLIST_NODE(&mut (*server).proc_link);
    spin_lock_init(&mut (*server).probe_lock);
    (*server).cell = cell;
    (*server).rtt = UINT_MAX;
    (*server).service_id = FS_SERVICE;
    (*server).probe_counter = 1;
    (*server).probed_at = jiffies - LONG_MAX / 2;
    afs_inc_servers_outstanding(net);
    server
}

/* Look up an address record for a server. */
unsafe fn afs_vl_lookup_addrs(server: *mut afs_server, key: *mut key) -> *mut afs_addr_list {
    let mut vc: afs_vl_cursor = core::mem::zeroed();
    let mut alist: *mut afs_addr_list = core::ptr::null_mut();
    let mut ret = -ERESTARTSYS;
    if afs_begin_vlserver_operation(&mut vc, (*server).cell, key) {
        while afs_select_vlserver(&mut vc) {
            if test_bit(AFS_VLSERVER_FL_IS_YFS, &(*vc.server).flags) { alist = afs_yfsvl_get_endpoints(&mut vc, &(*server).uuid); }
            else { alist = afs_vl_get_addrs_u(&mut vc, &(*server).uuid); }
        }
        ret = afs_end_vlserver_operation(&mut vc);
    }
    if ret < 0 { ERR_PTR(ret) } else { alist }
}

/* Get or create a fileserver record and return it with an active-use count on it. */
pub unsafe fn afs_lookup_server(cell: *mut afs_cell, key: *mut key, uuid: *const uuid_t, addr_version: u32) -> *mut afs_server {
    let mut alist: *mut afs_addr_list = core::ptr::null_mut();
    let mut server: *mut afs_server;
    let mut candidate: *mut afs_server = core::ptr::null_mut();
    let mut creating = false;
    let mut ret: i32;
    down_read(&mut (*cell).fs_lock);
    server = afs_find_server_by_uuid(cell, uuid);
    up_read(&mut (*cell).fs_lock);
    if !server.is_null() {
        timer_delete_sync(&mut (*server).timer);
        if test_bit(AFS_SERVER_FL_CREATING, &(*server).flags) { goto_wait_for_creation(server, cell); }
        if (*server).addr_version != addr_version { set_bit(AFS_SERVER_FL_NEEDS_UPDATE, &mut (*server).flags); }
        return server;
    }
    candidate = afs_alloc_server(cell, uuid);
    if candidate.is_null() { afs_put_addrlist(alist, afs_alist_trace_put_server_oom); return ERR_PTR(-ENOMEM); }
    down_write(&mut (*cell).fs_lock);
    server = afs_install_server(cell, &mut candidate);
    if test_bit(AFS_SERVER_FL_CREATING, &(*server).flags) { up_write(&mut (*cell).fs_lock); goto_wait_for_creation(server, cell); }
    if test_bit(AFS_SERVER_FL_UNCREATED, &(*server).flags) { set_bit(AFS_SERVER_FL_CREATING, &mut (*server).flags); clear_bit(AFS_SERVER_FL_UNCREATED, &mut (*server).flags); creating = true; }
    up_write(&mut (*cell).fs_lock);
    timer_delete_sync(&mut (*server).timer);
    if creating {
        alist = afs_vl_lookup_addrs(server, key);
        if IS_ERR(alist) { ret = PTR_ERR(alist); goto_create_failed(server, cell, ret); }
        ret = afs_fs_probe_fileserver((*cell).net, server, alist, key);
        if ret != 0 { goto_create_failed(server, cell, ret); }
        clear_and_wake_up_bit(AFS_SERVER_FL_CREATING, &mut (*server).flags);
    }
    afs_put_addrlist(alist, afs_alist_trace_put_server_create);
    if !candidate.is_null() { kfree(rcu_access_pointer((*server).endpoint_state)); kfree(candidate); afs_dec_servers_outstanding((*cell).net); }
    return server;
}

/* The remaining routines retain the C kernel operations and ordering directly. */
unsafe fn afs_set_server_timer(server: *mut afs_server, delay_secs: u32) { mod_timer(&mut (*server).timer, jiffies + delay_secs * HZ); }
pub unsafe fn afs_get_server(server: *mut afs_server, reason: afs_server_trace) -> *mut afs_server { let mut r=0; __refcount_inc(&mut (*server).ref_, &mut r); trace_afs_server((*server).debug_id, r+1, atomic_read(&(*server).active), reason); server }
pub unsafe fn afs_use_server(server: *mut afs_server, activate: bool, reason: afs_server_trace) -> *mut afs_server { let mut r=0; __refcount_inc(&mut (*server).ref_, &mut r); let a=atomic_inc_return(&mut (*server).active); if a==1 && activate && !test_bit(AFS_SERVER_FL_EXPIRED,&(*server).flags) { timer_delete(&mut (*server).timer); } trace_afs_server((*server).debug_id,r+1,a,reason); server }
pub unsafe fn afs_put_server(net: *mut afs_net, server: *mut afs_server, reason: afs_server_trace) { if server.is_null(){return} let id=(*server).debug_id; let a=atomic_read(&(*server).active); let mut r=0; let zero=__refcount_dec_and_test(&mut (*server).ref_,&mut r); trace_afs_server(id,r-1,a,reason); if zero { __afs_put_server_rcu(net,server); } }

pub unsafe fn afs_unuse_server_notime(net:*mut afs_net,server:*mut afs_server,reason:afs_server_trace){if server.is_null(){return} if atomic_dec_and_test(&mut (*server).active){if test_bit(AFS_SERVER_FL_EXPIRED,&(*server).flags)||READ_ONCE((*(*server).cell).state)>=AFS_CELL_REMOVING{schedule_work(&mut (*server).destroyer);}} afs_put_server(net,server,reason)}
pub unsafe fn afs_unuse_server(net:*mut afs_net,server:*mut afs_server,reason:afs_server_trace){if server.is_null(){return} if atomic_dec_and_test(&mut (*server).active){if !test_bit(AFS_SERVER_FL_EXPIRED,&(*server).flags)&&READ_ONCE((*(*server).cell).state)<AFS_CELL_REMOVING{(*server).unuse_time=ktime_get_real_seconds();afs_set_server_timer(server,AFS_SERVER_GC_DELAY);}else{schedule_work(&mut (*server).destroyer);}} afs_put_server(net,server,reason)}

unsafe fn afs_server_rcu(rcu:*mut rcu_head){let server=container_of!(rcu,afs_server,rcu);trace_afs_server((*server).debug_id,refcount_read(&(*server).ref_),atomic_read(&(*server).active),afs_server_trace_free);afs_put_endpoint_state(rcu_access_pointer((*server).endpoint_state),afs_estate_trace_put_server);afs_put_cell((*server).cell,afs_cell_trace_put_server);kfree((*server).cm_rxgk_appdata.data);kfree(server);}
unsafe fn __afs_put_server_rcu(net:*mut afs_net,server:*mut afs_server){call_rcu(&mut (*server).rcu,afs_server_rcu);afs_dec_servers_outstanding(net)}

pub unsafe fn afs_purge_servers(cell:*mut afs_cell){let mut rb=rb_first(&(*cell).fs_servers);down_read(&(*cell).fs_lock);while !rb.is_null(){let server=rb_entry(rb,afs_server,uuid_rb);afs_see_server(server,afs_server_trace_see_purge);schedule_work(&mut (*server).destroyer);rb=rb_next(rb);}up_read(&(*cell).fs_lock)}
pub unsafe fn afs_wait_for_servers(net:*mut afs_net){atomic_dec(&mut (*net).servers_outstanding);wait_var_event(&mut (*net).servers_outstanding,!atomic_read(&(*net).servers_outstanding));}

unsafe fn afs_give_up_callbacks(net:*mut afs_net,server:*mut afs_server){let estate=rcu_access_pointer((*server).endpoint_state);let alist=(*estate).addresses;afs_fs_give_up_all_callbacks(net,server,&mut (*alist).addrs[(*alist).preferred],core::ptr::null_mut());}
unsafe fn afs_has_server_expired(server:*const afs_server)->bool{if atomic_read(&(*server).active)!=0{return false} if (*(*server).cell).net.live||(*(*server).cell).state>=AFS_CELL_REMOVING{return true} let mut expires=(*server).unuse_time;if !test_bit(AFS_SERVER_FL_VL_FAIL,&(*server).flags)&&!test_bit(AFS_SERVER_FL_NOT_FOUND,&(*server).flags){expires+=AFS_SERVER_GC_DELAY as i64} ktime_get_real_seconds()>expires}
unsafe fn afs_remove_server_from_cell(server:*mut afs_server)->bool{let cell=(*server).cell;down_write(&mut (*cell).fs_lock);if !afs_has_server_expired(server){up_write(&mut (*cell).fs_lock);return false}set_bit(AFS_SERVER_FL_EXPIRED,&mut (*server).flags);afs_see_server(server,afs_server_trace_see_expired);rb_erase(&mut (*server).uuid_rb,&mut (*cell).fs_servers);up_write(&mut (*cell).fs_lock);true}
unsafe fn afs_server_destroyer(work:*mut work_struct){let server=container_of!(work,afs_server,destroyer);let net=(*(*server).cell).net;afs_see_server(server,afs_server_trace_see_destroyer);if test_bit(AFS_SERVER_FL_EXPIRED,&(*server).flags){return}if !afs_remove_server_from_cell(server){return}timer_shutdown_sync(&mut (*server).timer);cancel_work(&mut (*server).destroyer);if test_bit(AFS_SERVER_FL_MAY_HAVE_CB,&(*server).flags){afs_give_up_callbacks(net,server)}let estate=rcu_access_pointer((*server).endpoint_state);if !estate.is_null(){afs_set_peer_appdata(server,(*estate).addresses,core::ptr::null_mut())}write_seqlock(&mut (*net).fs_lock);list_del_init(&mut (*server).probe_link);if !hlist_unhashed(&(*server).proc_link){hlist_del_rcu(&mut (*server).proc_link)}write_sequnlock(&mut (*net).fs_lock);afs_put_server(net,server,afs_server_trace_destroy)}
unsafe fn afs_server_timer(timer:*mut timer_list){let server=container_of!(timer,afs_server,timer);afs_see_server(server,afs_server_trace_see_timer);if !test_bit(AFS_SERVER_FL_EXPIRED,&(*server).flags){schedule_work(&mut (*server).destroyer)}}

unsafe fn afs_update_server_record(op:*mut afs_operation,server:*mut afs_server,key:*mut key)->bool{let alist=afs_vl_lookup_addrs(server,(*op).key);if IS_ERR(alist){let estate=rcu_dereference((*server).endpoint_state);let has_addrs=!(*estate).addresses.is_null();if (PTR_ERR(alist)==-ERESTARTSYS||PTR_ERR(alist)==-EINTR)&&((*op).flags&AFS_OPERATION_UNINTR)!=0&&has_addrs{return true}afs_op_set_error(op,PTR_ERR(alist));return false}if (*server).addr_version!=(*alist).version{afs_fs_probe_fileserver((*op).net,server,alist,key)}afs_put_addrlist(alist,afs_alist_trace_put_server_update);true}
pub unsafe fn afs_check_server_record(op:*mut afs_operation,server:*mut afs_server,key:*mut key)->bool{let mut retries=0;loop{if test_bit(AFS_SERVER_FL_UPDATING,&(*server).flags)||test_bit(AFS_SERVER_FL_NEEDS_UPDATE,&(*server).flags){if !test_and_set_bit_lock(AFS_SERVER_FL_UPDATING,&mut (*server).flags){clear_bit(AFS_SERVER_FL_NEEDS_UPDATE,&mut (*server).flags);let success=afs_update_server_record(op,server,key);clear_bit_unlock(AFS_SERVER_FL_UPDATING,&mut (*server).flags);wake_up_bit(&mut (*server).flags,AFS_SERVER_FL_UPDATING);return success}let ret=wait_on_bit(&(*server).flags,AFS_SERVER_FL_UPDATING,if ((*op).flags&AFS_OPERATION_UNINTR)!=0{TASK_UNINTERRUPTIBLE}else{TASK_INTERRUPTIBLE});if ret==-ERESTARTSYS{afs_op_set_error(op,ret);return false}retries+=1;if retries==4{return false}continue}return true}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
