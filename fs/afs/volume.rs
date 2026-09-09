// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS volume management
 *
 * Copyright (C) 2002, 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel headers and "internal.h" provide the declarations used here.

static mut AFS_VOLUME_RECORD_LIFE: u64 = 60 * 60;
static mut AFS_VOLUME_DEBUG_ID: AtomicI32 = AtomicI32::new(0);

/* Insert a volume into a cell.  If there's an existing volume record, that is
 * returned instead with a ref held. */
unsafe fn afs_insert_volume_into_cell(mut cell: *mut afs_cell, mut volume: *mut afs_volume) -> *mut afs_volume {
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let mut pp: *mut *mut rb_node;
    write_seqlock(&mut (*cell).volume_lock);
    pp = &mut (*cell).volumes.rb_node;
    while !(*pp).is_null() {
        parent = *pp;
        let p = rb_entry(parent, afs_volume, cell_node);
        if (*p).vid < (*volume).vid {
            pp = &mut (*(*pp)).rb_left;
        } else if (*p).vid > (*volume).vid {
            pp = &mut (*(*pp)).rb_right;
        } else {
            if afs_try_get_volume(p, afs_volume_trace_get_cell_insert) {
                volume = p;
                break;
            }
            set_bit(AFS_VOLUME_RM_TREE, &mut (*p).flags);
            rb_replace_node_rcu(&mut (*p).cell_node, &mut (*volume).cell_node, &mut (*cell).volumes);
        }
    }
    if volume != rb_entry(parent, afs_volume, cell_node) {
        rb_link_node_rcu(&mut (*volume).cell_node, parent, pp);
        rb_insert_color(&mut (*volume).cell_node, &mut (*cell).volumes);
        hlist_add_head_rcu(&mut (*volume).proc_link, &mut (*cell).proc_volumes);
    }
    write_sequnlock(&mut (*cell).volume_lock);
    volume
}

unsafe fn afs_remove_volume_from_cell(volume: *mut afs_volume) {
    let cell = (*volume).cell;
    if !hlist_unhashed(&(*volume).proc_link) {
        trace_afs_volume((*volume).debug_id, (*volume).vid, refcount_read(&(*volume).ref), afs_volume_trace_remove);
        write_seqlock(&mut (*cell).volume_lock);
        hlist_del_rcu(&mut (*volume).proc_link);
        if !test_and_set_bit(AFS_VOLUME_RM_TREE, &mut (*volume).flags) {
            rb_erase(&mut (*volume).cell_node, &mut (*cell).volumes);
        }
        write_sequnlock(&mut (*cell).volume_lock);
    }
}

unsafe fn afs_alloc_volume(params: *mut afs_fs_context, vldb: *mut afs_vldb_entry, slist_out: *mut *mut afs_server_list) -> *mut afs_volume {
    let mut ret: i32 = -ENOMEM;
    let volume = kzalloc_obj::<afs_volume>();
    if volume.is_null() { return ERR_PTR(ret); }
    (*volume).debug_id = atomic_inc_return(&mut AFS_VOLUME_DEBUG_ID);
    (*volume).vid = (*vldb).vid[(*params).type_ as usize];
    (*volume).update_at = ktime_get_real_seconds() + AFS_VOLUME_RECORD_LIFE;
    (*volume).cell = afs_get_cell((*params).cell, afs_cell_trace_get_vol);
    (*volume).type_ = (*params).type_;
    (*volume).type_force = (*params).force;
    (*volume).name_len = (*vldb).name_len;
    (*volume).creation_time = TIME64_MIN;
    (*volume).update_time = TIME64_MIN;
    refcount_set(&mut (*volume).ref_, 1);
    INIT_HLIST_NODE(&mut (*volume).proc_link);
    INIT_WORK(&mut (*volume).destructor, afs_destroy_volume);
    rwlock_init(&mut (*volume).servers_lock);
    mutex_init(&mut (*volume).volsync_lock);
    mutex_init(&mut (*volume).cb_check_lock);
    rwlock_init(&mut (*volume).cb_v_break_lock);
    INIT_LIST_HEAD(&mut (*volume).open_mmaps);
    init_rwsem(&mut (*volume).open_mmaps_lock);
    memcpy((*volume).name.as_mut_ptr().cast(), (*vldb).name.as_ptr().cast(), (*vldb).name_len + 1);
    for i in 0..AFS_MAXTYPES { (*volume).vids[i] = (*vldb).vid[i]; }
    let slist = afs_alloc_server_list(volume, (*params).key, vldb);
    if IS_ERR(slist) {
        ret = PTR_ERR(slist);
        afs_put_cell((*volume).cell, afs_cell_trace_put_vol);
        kfree(volume.cast());
        return ERR_PTR(ret);
    }
    *slist_out = slist;
    rcu_assign_pointer(&mut (*volume).servers, slist);
    trace_afs_volume((*volume).debug_id, (*volume).vid, 1, afs_volume_trace_alloc);
    volume
}

unsafe fn afs_lookup_volume(params: *mut afs_fs_context, vldb: *mut afs_vldb_entry) -> *mut afs_volume {
    let mut slist = core::ptr::null_mut();
    let candidate = afs_alloc_volume(params, vldb, &mut slist);
    if IS_ERR(candidate) { return candidate; }
    let volume = afs_insert_volume_into_cell((*params).cell, candidate);
    if volume == candidate { afs_attach_volume_to_servers(volume, slist); }
    else { afs_put_volume(candidate, afs_volume_trace_put_cell_dup); }
    volume
}

unsafe fn afs_vl_lookup_vldb(cell: *mut afs_cell, key: *mut key, volname: *const c_char, volnamesz: usize) -> *mut afs_vldb_entry {
    let mut vldb = ERR_PTR(-EDESTADDRREQ);
    let mut vc = core::mem::zeroed::<afs_vl_cursor>();
    if !afs_begin_vlserver_operation(&mut vc, cell, key) { return ERR_PTR(-ERESTARTSYS); }
    while afs_select_vlserver(&mut vc) { vldb = afs_vl_get_entry_by_name_u(&mut vc, volname, volnamesz); }
    let ret = afs_end_vlserver_operation(&mut vc);
    if ret < 0 { ERR_PTR(ret) } else { vldb }
}

pub unsafe fn afs_create_volume(params: *mut afs_fs_context) -> *mut afs_volume {
    let mut vldb = afs_vl_lookup_vldb((*params).cell, (*params).key, (*params).volname, (*params).volnamesz);
    if IS_ERR(vldb) { return vldb.cast(); }
    if test_bit(AFS_VLDB_QUERY_ERROR, &(*vldb).flags) { let v = ERR_PTR((*vldb).error); kfree(vldb.cast()); return v; }
    let type_mask = 1usize << (*params).type_ as usize;
    let mut volume = ERR_PTR(-ENOMEDIUM);
    if (*params).force {
        if ((*vldb).flags & type_mask) == 0 { kfree(vldb.cast()); return volume; }
    } else if test_bit(AFS_VLDB_HAS_RO, &(*vldb).flags) { (*params).type_ = AFSVL_ROVOL; }
    else if test_bit(AFS_VLDB_HAS_RW, &(*vldb).flags) { (*params).type_ = AFSVL_RWVOL; }
    else { kfree(vldb.cast()); return volume; }
    volume = afs_lookup_volume(params, vldb);
    kfree(vldb.cast());
    volume
}

unsafe fn afs_destroy_volume(work: *mut work_struct) {
    let volume = container_of(work, afs_volume, destructor);
    let slist = rcu_access_pointer((*volume).servers);
    _enter("%p", volume);
    afs_detach_volume_from_servers(volume, slist);
    afs_remove_volume_from_cell(volume);
    afs_put_serverlist((*(*volume).cell).net, slist);
    afs_put_cell((*volume).cell, afs_cell_trace_put_vol);
    trace_afs_volume((*volume).debug_id, (*volume).vid, refcount_read(&(*volume).ref_), afs_volume_trace_free);
    kfree_rcu(volume, rcu);
    _leave(" [destroyed]");
}

pub unsafe fn afs_try_get_volume(volume: *mut afs_volume, reason: afs_volume_trace) -> bool {
    let mut r = 0; if __refcount_inc_not_zero(&mut (*volume).ref_, &mut r) { trace_afs_volume((*volume).debug_id, (*volume).vid, r + 1, reason); true } else { false }
}
pub unsafe fn afs_get_volume(volume: *mut afs_volume, reason: afs_volume_trace) -> *mut afs_volume {
    if !volume.is_null() { let mut r = 0; __refcount_inc(&mut (*volume).ref_, &mut r); trace_afs_volume((*volume).debug_id, (*volume).vid, r + 1, reason); } volume
}
pub unsafe fn afs_put_volume(volume: *mut afs_volume, reason: afs_volume_trace) {
    if !volume.is_null() { let id = (*volume).debug_id; let vid = (*volume).vid; let mut r = 0; let zero = __refcount_dec_and_test(&mut (*volume).ref_, &mut r); trace_afs_volume(id, vid, r - 1, reason); if zero { schedule_work(&mut (*volume).destructor); } }
}

pub unsafe fn afs_activate_volume(_volume: *mut afs_volume) -> i32 { 0 }
pub unsafe fn afs_deactivate_volume(volume: *mut afs_volume) { _enter("%s", (*volume).name.as_ptr()); _leave(""); }

unsafe fn afs_update_volume_status(volume: *mut afs_volume, key: *mut key) -> i32 {
    let mut idbuf = [0i8; 24];
    let idsz = snprintf(idbuf.as_mut_ptr(), idbuf.len(), "%llu", (*volume).vid);
    let vldb = afs_vl_lookup_vldb((*volume).cell, key, idbuf.as_ptr(), idsz);
    if IS_ERR(vldb) { return PTR_ERR(vldb); }
    if (*vldb).name_len != (*volume).name_len || memcmp((*vldb).name.as_ptr().cast(), (*volume).name.as_ptr().cast(), (*vldb).name_len) != 0 {
        memcpy((*volume).name.as_mut_ptr().cast(), (*vldb).name.as_ptr().cast(), AFS_MAXVOLNAME);
        (*volume).name_len = (*vldb).name_len;
    }
    let new = afs_alloc_server_list(volume, key, vldb);
    if IS_ERR(new) { let ret = PTR_ERR(new); kfree(vldb.cast()); return ret; }
    write_lock(&mut (*volume).servers_lock);
    let old = rcu_dereference_protected((*volume).servers, lockdep_is_held(&(*volume).servers_lock));
    let mut discard = new;
    if afs_annotate_server_list(new, old) { (*new).seq = (*volume).servers_seq + 1; rcu_assign_pointer(&mut (*volume).servers, new); smp_wmb(); (*volume).servers_seq += 1; discard = old; }
    (*volume).update_at = ktime_get_real_seconds() + if (*new).ro_replicating { 10 * 60 } else { AFS_VOLUME_RECORD_LIFE };
    write_unlock(&mut (*volume).servers_lock);
    if discard == old { afs_reattach_volume_to_servers(volume, new, old); }
    afs_put_serverlist((*(*volume).cell).net, discard); kfree(vldb.cast()); 0
}

pub unsafe fn afs_check_volume_status(volume: *mut afs_volume, op: *mut afs_operation) -> i32 {
    let mut retries = 0;
    loop {
        if test_bit(AFS_VOLUME_WAIT, &(*volume).flags) { }
        else if (*volume).update_at <= ktime_get_real_seconds() || test_bit(AFS_VOLUME_NEEDS_UPDATE, &(*volume).flags) {
            if !test_and_set_bit_lock(AFS_VOLUME_UPDATING, &mut (*volume).flags) {
                clear_bit(AFS_VOLUME_NEEDS_UPDATE, &mut (*volume).flags); let ret = afs_update_volume_status(volume, (*op).key);
                if ret < 0 { set_bit(AFS_VOLUME_NEEDS_UPDATE, &mut (*volume).flags); }
                clear_bit_unlock(AFS_VOLUME_WAIT, &mut (*volume).flags); clear_bit_unlock(AFS_VOLUME_UPDATING, &mut (*volume).flags); wake_up_bit(&mut (*volume).flags, AFS_VOLUME_WAIT); return ret;
            }
        } else { return 0; }
        if !test_bit(AFS_VOLUME_WAIT, &(*volume).flags) { return 0; }
        let ret = wait_on_bit(&mut (*volume).flags, AFS_VOLUME_WAIT, if ((*op).flags & AFS_OPERATION_UNINTR) != 0 { TASK_UNINTERRUPTIBLE } else { TASK_INTERRUPTIBLE });
        if ret == -ERESTARTSYS { return ret; }
        retries += 1; if retries == 4 { return -ESTALE; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
