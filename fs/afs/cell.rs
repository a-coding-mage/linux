// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS cell and server record management
 *
 * Copyright (C) 2002, 2017 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel/Afs dependencies are supplied by the surrounding translation unit.

static mut AFS_CELL_GC_DELAY: u32 = 10;
static mut AFS_CELL_MIN_TTL: u32 = 10 * 60;
static mut AFS_CELL_MAX_TTL: u32 = 24 * 60 * 60;
static mut CELL_DEBUG_ID: atomic_t = atomic_t::new(0);

unsafe fn afs_dec_cells_outstanding(net: *mut afs_net) {
    if atomic_dec_and_test(&mut (*net).cells_outstanding) { wake_up_var(&mut (*net).cells_outstanding); }
}

unsafe fn afs_set_cell_state(cell: *mut afs_cell, state: afs_cell_state) {
    smp_store_release(&mut (*cell).state, state); /* Commit cell changes before state */
    smp_wmb(); /* Set cell state before task state */
    wake_up_var(&mut (*cell).state);
}

/* Look up and get an activation reference on a cell record. */
unsafe fn afs_find_cell_locked(net: *mut afs_net, name: *const c_char, namesz: c_uint, reason: afs_cell_trace) -> *mut afs_cell {
    let mut cell: *mut afs_cell = core::ptr::null_mut();
    let mut p = (*net).cells.rb_node;
    if !name.is_null() && namesz == 0 { return ERR_PTR(-EINVAL); }
    if namesz > AFS_MAXCELLNAME { return ERR_PTR(-ENAMETOOLONG); }
    if name.is_null() {
        cell = rcu_dereference_protected((*net).ws_cell, lockdep_is_held(&(*net).cells_lock));
        if cell.is_null() { return ERR_PTR(-EDESTADDRREQ); }
    } else {
        while !p.is_null() {
            cell = rb_entry(p, afs_cell, net_node);
            let n = strncasecmp((*cell).name, name, core::cmp::min((*cell).name_len, namesz as usize));
            let n = if n == 0 { (*cell).name_len as isize - namesz as isize } else { n };
            if n < 0 { p = (*p).rb_left; } else if n > 0 { p = (*p).rb_right; } else { break; }
        }
        if p.is_null() { return ERR_PTR(-ENOENT); }
    }
    afs_use_cell(cell, reason)
}

pub unsafe fn afs_find_cell(net: *mut afs_net, name: *const c_char, namesz: c_uint, reason: afs_cell_trace) -> *mut afs_cell {
    down_read(&mut (*net).cells_lock);
    let cell = afs_find_cell_locked(net, name, namesz, reason);
    up_read(&mut (*net).cells_lock);
    cell
}

/* Set up a cell record and fill in its name, VL server address list and allocate an anonymous key. */
unsafe fn afs_alloc_cell(net: *mut afs_net, name: *const c_char, namelen: c_uint, addresses: *const c_char) -> *mut afs_cell {
    assert!(!name.is_null());
    if namelen == 0 { return ERR_PTR(-EINVAL); }
    if namelen > AFS_MAXCELLNAME { return ERR_PTR(-ENAMETOOLONG); }
    if *name as u8 == b'.' { return ERR_PTR(-EINVAL); }
    for i in 0..namelen as isize { let ch = *name.offset(i) as u8; if !isprint(ch) || ch == b'/' || ch == b'@' { return ERR_PTR(-EINVAL); } }
    let cell = kzalloc_obj::<afs_cell>();
    if cell.is_null() { return ERR_PTR(-ENOMEM); }
    (*cell).name = kmalloc((1 + namelen + 1 + 4 + namelen + 1) as usize, GFP_KERNEL);
    if (*cell).name.is_null() { kfree(cell); return ERR_PTR(-ENOMEM); }
    (*cell).name = (*cell).name.add(1);
    (*cell).name_len = namelen as usize;
    for i in 0..namelen as usize { *(*cell).name.add(i) = tolower(*name.add(i as usize)); }
    *(*cell).name.add(namelen as usize) = 0;
    (*cell).key_desc = (*cell).name.add(namelen as usize + 1);
    memcpy((*cell).key_desc, b"afs@\0".as_ptr() as _, 4);
    memcpy((*cell).key_desc.add(4), (*cell).name as _, namelen as usize + 1);
    (*cell).net = net; refcount_set(&mut (*cell).ref_, 1); atomic_set(&mut (*cell).active, 0);
    INIT_WORK(&mut (*cell).destroyer, afs_destroy_cell_work); INIT_WORK(&mut (*cell).manager, afs_manage_cell_work);
    timer_setup(&mut (*cell).management_timer, afs_cell_timer, 0); init_rwsem(&mut (*cell).vs_lock);
    (*cell).volumes = RB_ROOT; INIT_HLIST_HEAD(&mut (*cell).proc_volumes); seqlock_init(&mut (*cell).volume_lock);
    (*cell).fs_servers = RB_ROOT; init_rwsem(&mut (*cell).fs_lock); rwlock_init(&mut (*cell).vl_servers_lock);
    (*cell).flags = 1 << AFS_CELL_FL_CHECK_ALIAS;
    let mut vllist;
    if !addresses.is_null() {
        vllist = afs_parse_text_addrs(net, addresses, strlen(addresses), b':', VL_SERVICE, AFS_VL_PORT);
        if IS_ERR(vllist) { kfree((*cell).name.sub(1)); kfree(cell); return ERR_PTR(PTR_ERR(vllist)); }
        (*vllist).source = DNS_RECORD_FROM_CONFIG; (*vllist).status = DNS_LOOKUP_NOT_DONE; (*cell).dns_expiry = TIME64_MAX;
    } else {
        vllist = afs_alloc_vlserver_list(0); if vllist.is_null() { kfree((*cell).name.sub(1)); kfree(cell); return ERR_PTR(-ENOMEM); }
        (*vllist).source = DNS_RECORD_UNAVAILABLE; (*vllist).status = DNS_LOOKUP_NOT_DONE; (*cell).dns_expiry = ktime_get_real_seconds();
    }
    rcu_assign_pointer((*cell).vl_servers, vllist); (*cell).dns_source = (*vllist).source; (*cell).dns_status = (*vllist).status;
    smp_store_release(&mut (*cell).dns_lookup_count, 1); atomic_inc(&mut (*net).cells_outstanding); (*cell).debug_id = atomic_inc_return(&mut CELL_DEBUG_ID);
    trace_afs_cell((*cell).debug_id, 1, 0, afs_cell_trace_alloc); cell
}

/* The remaining declarations mirror the externally visible implementation entry points. */
pub unsafe fn afs_lookup_cell(net: *mut afs_net, name: *const c_char, namesz: c_uint, vllist: *const c_char, reason: afs_lookup_cell_for, trace: afs_cell_trace) -> *mut afs_cell {
    let mut cell = afs_find_cell(net, name, namesz, trace);
    if !IS_ERR(cell) { return cell; }
    cell = afs_alloc_cell(net, name, namesz, vllist);
    if IS_ERR(cell) { return cell; }
    down_write(&mut (*net).cells_lock); afs_use_cell(cell, trace); up_write(&mut (*net).cells_lock); cell
}

pub unsafe fn afs_get_cell(cell: *mut afs_cell, reason: afs_cell_trace) -> *mut afs_cell { __refcount_inc(&mut (*cell).ref_, core::ptr::null_mut()); trace_afs_cell((*cell).debug_id, 1, atomic_read(&(*cell).active), reason); cell }
pub unsafe fn afs_put_cell(cell: *mut afs_cell, reason: afs_cell_trace) { if !cell.is_null() { __refcount_dec_and_test(&mut (*cell).ref_, core::ptr::null_mut()); trace_afs_cell((*cell).debug_id, 0, atomic_read(&(*cell).active), reason); } }
pub unsafe fn afs_use_cell(cell: *mut afs_cell, reason: afs_cell_trace) -> *mut afs_cell { __refcount_inc(&mut (*cell).ref_, core::ptr::null_mut()); atomic_inc(&mut (*cell).active); trace_afs_cell((*cell).debug_id, 1, atomic_read(&(*cell).active), reason); cell }
pub unsafe fn afs_unuse_cell(cell: *mut afs_cell, reason: afs_cell_trace) { if cell.is_null() { return; } atomic_dec(&mut (*cell).active); __refcount_dec_and_test(&mut (*cell).ref_, core::ptr::null_mut()); trace_afs_cell((*cell).debug_id, 0, atomic_read(&(*cell).active), reason); }
pub unsafe fn afs_see_cell(cell: *mut afs_cell, reason: afs_cell_trace) { trace_afs_cell((*cell).debug_id, refcount_read(&(*cell).ref_), atomic_read(&(*cell).active), reason); }
pub unsafe fn afs_queue_cell(cell: *mut afs_cell, _reason: afs_cell_trace) { queue_work(afs_wq, &mut (*cell).manager); }
pub unsafe fn afs_set_cell_timer(cell: *mut afs_cell, delay_secs: c_uint) { timer_reduce(&mut (*cell).management_timer, jiffies + delay_secs as u64 * HZ); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
