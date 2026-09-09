// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS cell alias detection
 *
 * Copyright (C) 2020 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel headers and "internal.h" are supplied by the surrounding crate.

/* Sample a volume. */
unsafe fn afs_sample_volume(
    cell: *mut afs_cell,
    key: *mut key,
    name: *const core::ffi::c_char,
    namelen: u32,
) -> *mut afs_volume {
    let fc = afs_fs_context {
        r#type: 0, // Explicitly leave it to the VLDB
        volnamesz: namelen,
        volname: name,
        net: (*cell).net,
        cell,
        key, // This might need to be something
    };
    let volume = afs_create_volume(&fc);
    _leave!(" = %p", volume);
    volume
}

/* Compare the address lists of a pair of fileservers. */
unsafe fn afs_compare_fs_alists(
    server_a: *const afs_server,
    server_b: *const afs_server,
) -> i32 {
    let la = (*rcu_dereference((*server_a).endpoint_state)).addresses;
    let lb = (*rcu_dereference((*server_b).endpoint_state)).addresses;
    let (mut a, mut b, mut addr_matches) = (0i32, 0i32, 0i32);

    while a < (*la).nr_addrs && b < (*lb).nr_addrs {
        let pa = (*la).addrs[a as usize].peer as usize;
        let pb = (*lb).addrs[b as usize].peer as usize;
        let diff = pa.wrapping_sub(pb) as isize;
        if diff < 0 { a += 1; }
        else if diff > 0 { b += 1; }
        else { addr_matches += 1; a += 1; b += 1; }
    }
    addr_matches
}

/* Compare the fileserver lists of two volumes.  The server lists are sorted in
 * order of ascending UUID. */
unsafe fn afs_compare_volume_slists(
    vol_a: *const afs_volume,
    vol_b: *const afs_volume,
) -> i32 {
    let la = rcu_dereference((*vol_a).servers);
    let lb = rcu_dereference((*vol_b).servers);
    for i in 0..AFS_MAXTYPES as usize {
        if (*vol_a).vids[i] != (*vol_b).vids[i] { return 0; }
    }
    let (mut a, mut b, mut uuid_matches, mut addr_matches) = (0i32, 0i32, 0i32, 0i32);
    while a < (*la).nr_servers && b < (*lb).nr_servers {
        let server_a = (*la).servers[a as usize].server;
        let server_b = (*lb).servers[b as usize].server;
        let diff = memcmp(
            &(*server_a).uuid as *const _ as *const core::ffi::c_void,
            &(*server_b).uuid as *const _ as *const core::ffi::c_void,
            core::mem::size_of::<uuid_t>(),
        );
        if diff < 0 { a += 1; }
        else if diff > 0 { b += 1; }
        else {
            uuid_matches += 1;
            addr_matches += afs_compare_fs_alists(server_a, server_b);
            a += 1; b += 1;
        }
    }
    _leave!(" = %d [um %d]", addr_matches, uuid_matches);
    addr_matches
}

/* Compare root.cell volumes. */
unsafe fn afs_compare_cell_roots(cell: *mut afs_cell) -> i32 {
    _enter!("");
    rcu_read_lock();
    let mut p: *mut afs_cell = core::ptr::null_mut();
    hlist_for_each_entry_rcu!(p, (*(*cell).net).proc_cells, proc_link, {
        if p == cell || !(*p).alias_of.is_null() || (*p).root_volume.is_null() { continue; }
        if afs_compare_volume_slists((*cell).root_volume, (*p).root_volume) != 0 {
            rcu_read_unlock();
            (*cell).alias_of = afs_use_cell(p, afs_cell_trace_use_alias);
            return 1;
        }
    });
    rcu_read_unlock();
    _leave!(" = 0");
    0
}

/* Query the new cell for a volume from a cell we're already using. */
unsafe fn afs_query_for_alias_one(cell: *mut afs_cell, key: *mut key, p: *mut afs_cell) -> i32 {
    let mut pvol: *mut afs_volume = core::ptr::null_mut();
    read_seqlock_excl(&mut (*p).volume_lock);
    if !rb_empty_root(&(*p).volumes) {
        pvol = afs_get_volume(rb_entry((*p).volumes.rb_node), afs_volume_trace_get_query_alias);
    }
    read_sequnlock_excl(&mut (*p).volume_lock);
    if pvol.is_null() { return 0; }
    _enter!("%s:%s", (*cell).name, (*pvol).name);
    let volume = afs_sample_volume(cell, key, (*pvol).name, (*pvol).name_len);
    if is_err(volume) {
        afs_put_volume(pvol, afs_volume_trace_put_query_alias);
        if ptr_err(volume) != -ENOMEDIUM { return ptr_err(volume); }
        return 0;
    }
    let mut ret = 0;
    if (*pvol).vid == (*volume).vid {
        rcu_read_lock();
        if afs_compare_volume_slists(volume, pvol) != 0 { ret = 1; }
        rcu_read_unlock();
    }
    afs_put_volume(volume, afs_volume_trace_put_query_alias);
    afs_put_volume(pvol, afs_volume_trace_put_query_alias);
    ret
}

/* Query the new cell for volumes we know exist in cells we're already using. */
unsafe fn afs_query_for_alias(cell: *mut afs_cell, key: *mut key) -> i32 {
    _enter!("%s", (*cell).name);
    if mutex_lock_interruptible(&mut (*(*cell).net).proc_cells_lock) < 0 { return -ERESTARTSYS; }
    let mut p: *mut afs_cell = core::ptr::null_mut();
    hlist_for_each_entry!(p, (*(*cell).net).proc_cells, proc_link, {
        if p == cell || !(*p).alias_of.is_null() || rb_empty_root(&(*p).volumes) || !(*p).root_volume.is_null() { continue; }
        afs_use_cell(p, afs_cell_trace_use_check_alias);
        mutex_unlock(&mut (*(*cell).net).proc_cells_lock);
        if afs_query_for_alias_one(cell, key, p) != 0 { (*cell).alias_of = p; return 1; }
        if mutex_lock_interruptible(&mut (*(*cell).net).proc_cells_lock) < 0 {
            afs_unuse_cell(p, afs_cell_trace_unuse_check_alias); return -ERESTARTSYS;
        }
        afs_unuse_cell(p, afs_cell_trace_unuse_check_alias);
    });
    mutex_unlock(&mut (*(*cell).net).proc_cells_lock);
    _leave!(" = 0");
    0
}

/* Look up a VLDB record for a volume. */
unsafe fn afs_vl_get_cell_name(cell: *mut afs_cell, key: *mut key) -> *mut core::ffi::c_char {
    let mut vc: afs_vl_cursor = core::mem::zeroed();
    let mut cell_name = err_ptr(-EDESTADDRREQ);
    let (mut skipped, mut not_skipped) = (false, false);
    if !afs_begin_vlserver_operation(&mut vc, cell, key) { return err_ptr(-ERESTARTSYS); }
    while afs_select_vlserver(&mut vc) {
        if !test_bit(AFS_VLSERVER_FL_IS_YFS, &(*vc.server).flags) { vc.call_error = -EOPNOTSUPP; skipped = true; continue; }
        not_skipped = true; cell_name = afs_yfsvl_get_cell_name(&mut vc);
    }
    let mut ret = afs_end_vlserver_operation(&mut vc);
    if skipped && !not_skipped { ret = -EOPNOTSUPP; }
    if ret < 0 { err_ptr(ret) } else { cell_name }
}

unsafe fn yfs_check_canonical_cell_name(cell: *mut afs_cell, key: *mut key) -> i32 {
    let cell_name = afs_vl_get_cell_name(cell, key);
    if is_err(cell_name) { return ptr_err(cell_name); }
    if strcmp(cell_name, (*cell).name) == 0 { kfree(cell_name); return 0; }
    let name_len = strlen(cell_name);
    let master = if name_len == 0 || name_len > AFS_MAXCELLNAME { err_ptr(-EOPNOTSUPP) } else {
        afs_lookup_cell((*cell).net, cell_name, name_len, core::ptr::null_mut(), AFS_LOOKUP_CELL_ALIAS_CHECK, afs_cell_trace_use_lookup_canonical)
    };
    kfree(cell_name);
    if is_err(master) { return ptr_err(master); }
    (*cell).alias_of = master; 1
}

unsafe fn afs_do_cell_detect_alias(cell: *mut afs_cell, key: *mut key) -> i32 {
    _enter!("%s", (*cell).name);
    let ret = yfs_check_canonical_cell_name(cell, key);
    if ret != -EOPNOTSUPP { return ret; }
    let root_volume = afs_sample_volume(cell, key, b"root.cell\0".as_ptr() as *const core::ffi::c_char, 9);
    if !is_err(root_volume) { (*cell).root_volume = root_volume; return afs_compare_cell_roots(cell); }
    if ptr_err(root_volume) != -ENOMEDIUM { return ptr_err(root_volume); }
    afs_query_for_alias(cell, key)
}

pub unsafe fn afs_cell_detect_alias(cell: *mut afs_cell, key: *mut key) -> i32 {
    let net = (*cell).net;
    if mutex_lock_interruptible(&mut (*net).cells_alias_lock) < 0 { return -ERESTARTSYS; }
    let ret;
    if test_bit(AFS_CELL_FL_CHECK_ALIAS, &(*cell).flags) {
        ret = afs_do_cell_detect_alias(cell, key);
        if ret >= 0 { clear_bit_unlock(AFS_CELL_FL_CHECK_ALIAS, &mut (*cell).flags); }
    } else { ret = if !(*cell).alias_of.is_null() { 1 } else { 0 }; }
    mutex_unlock(&mut (*net).cells_alias_lock);
    if ret == 1 { pr_notice!("kAFS: Cell %s is an alias of %s\n", (*cell).name, (*(*cell).alias_of).name); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
