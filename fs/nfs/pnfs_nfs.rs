// SPDX-License-Identifier: GPL-2.0-only
/* Common NFS I/O operations for pnfs file based layout drivers. */

// Kernel headers and symbols are supplied by the surrounding translation unit.

pub unsafe fn pnfs_generic_rw_release(data: *mut core::ffi::c_void) {
    let hdr = data as *mut nfs_pgio_header;
    nfs_put_client((*hdr).ds_clp);
    ((*hdr).mds_ops).rpc_release(data);
}

pub unsafe fn pnfs_generic_prepare_to_resend_writes(data: *mut nfs_commit_data) {
    let verf = (*data).res.verf;
    (*data).task.tk_status = 0;
    core::ptr::write_bytes(&mut (*verf).verifier as *mut _, 0, core::mem::size_of_val(&(*verf).verifier));
    (*verf).committed = NFS_UNSTABLE;
}

pub unsafe fn pnfs_generic_write_commit_done(task: *mut rpc_task, data: *mut core::ffi::c_void) {
    let wdata = data as *mut nfs_commit_data;
    ((*wdata).mds_ops).rpc_call_done(task, data);
}

pub unsafe fn pnfs_generic_commit_release(calldata: *mut core::ffi::c_void) {
    let data = calldata as *mut nfs_commit_data;
    ((*data).completion_ops).completion(data);
    pnfs_put_lseg((*data).lseg);
    nfs_put_client((*data).ds_clp);
    nfs_commitdata_release(data);
}

unsafe fn pnfs_free_bucket_lseg(bucket: *mut pnfs_commit_bucket) -> *mut pnfs_layout_segment {
    if list_empty(&(*bucket).committing) && list_empty(&(*bucket).written) {
        let freeme = (*bucket).lseg;
        (*bucket).lseg = core::ptr::null_mut();
        return freeme;
    }
    core::ptr::null_mut()
}

pub unsafe fn pnfs_generic_clear_request_commit(req: *mut nfs_page, cinfo: *mut nfs_commit_info) {
    let mut bucket: *mut pnfs_commit_bucket = core::ptr::null_mut();
    if !test_and_clear_bit(PG_COMMIT_TO_DS, &mut (*req).wb_flags) { goto_out!(); }
    (*(*cinfo).ds).nwritten -= 1;
    if list_is_singular(&(*req).wb_list) {
        bucket = list_first_entry(&(*req).wb_list, core::mem::size_of::<pnfs_commit_bucket>());
    }
    nfs_request_remove_commit_list(req, cinfo);
    if !bucket.is_null() { pnfs_put_lseg(pnfs_free_bucket_lseg(bucket)); }
}

pub unsafe fn pnfs_alloc_commit_array(mut n: usize, gfp_flags: gfp_t) -> *mut pnfs_commit_array {
    let p = kmalloc_flex_commit_array(n, gfp_flags);
    if p.is_null() { return core::ptr::null_mut(); }
    (*p).nbuckets = n;
    INIT_LIST_HEAD(&mut (*p).cinfo_list);
    INIT_LIST_HEAD(&mut (*p).lseg_list);
    (*p).lseg = core::ptr::null_mut();
    let mut b = (*p).buckets.as_mut_ptr();
    while n != 0 {
        INIT_LIST_HEAD(&mut (*b).written);
        INIT_LIST_HEAD(&mut (*b).committing);
        (*b).lseg = core::ptr::null_mut();
        (*b).direct_verf.committed = NFS_INVALID_STABLE_HOW;
        b = b.add(1); n -= 1;
    }
    p
}

pub unsafe fn pnfs_free_commit_array(p: *mut pnfs_commit_array) { kfree_rcu(p, rcu); }

unsafe fn pnfs_find_commit_array_by_lseg(fl_cinfo: *mut pnfs_ds_commit_info, lseg: *mut pnfs_layout_segment) -> *mut pnfs_commit_array {
    let mut array: *mut pnfs_commit_array = core::ptr::null_mut();
    list_for_each_entry_rcu!(array, &(*fl_cinfo).commits, cinfo_list, {
        if (*array).lseg == lseg { return array; }
    });
    core::ptr::null_mut()
}

pub unsafe fn pnfs_add_commit_array(fl_cinfo: *mut pnfs_ds_commit_info, new: *mut pnfs_commit_array, lseg: *mut pnfs_layout_segment) -> *mut pnfs_commit_array {
    let array = pnfs_find_commit_array_by_lseg(fl_cinfo, lseg);
    if !array.is_null() { return array; }
    (*new).lseg = lseg;
    refcount_set(&mut (*new).refcount, 1);
    list_add_rcu(&mut (*new).cinfo_list, &mut (*fl_cinfo).commits);
    list_add(&mut (*new).lseg_list, &mut (*lseg).pls_commits);
    new
}

unsafe fn pnfs_lookup_commit_array(fl_cinfo: *mut pnfs_ds_commit_info, lseg: *mut pnfs_layout_segment) -> *mut pnfs_commit_array {
    rcu_read_lock(); let mut a = pnfs_find_commit_array_by_lseg(fl_cinfo, lseg);
    if a.is_null() { rcu_read_unlock(); ((*fl_cinfo).ops).setup_ds_info(fl_cinfo, lseg); rcu_read_lock(); a = pnfs_find_commit_array_by_lseg(fl_cinfo, lseg); }
    rcu_read_unlock(); a
}

unsafe fn pnfs_release_commit_array_locked(a: *mut pnfs_commit_array) { list_del_rcu(&mut (*a).cinfo_list); list_del(&mut (*a).lseg_list); pnfs_free_commit_array(a); }
unsafe fn pnfs_put_commit_array_locked(a: *mut pnfs_commit_array) { if refcount_dec_and_test(&mut (*a).refcount) { pnfs_release_commit_array_locked(a); } }
unsafe fn pnfs_put_commit_array(a: *mut pnfs_commit_array, inode: *mut inode) { if refcount_dec_and_lock(&mut (*a).refcount, &mut (*inode).i_lock) { pnfs_release_commit_array_locked(a); spin_unlock(&mut (*inode).i_lock); } }
unsafe fn pnfs_get_commit_array(a: *mut pnfs_commit_array) -> *mut pnfs_commit_array { if refcount_inc_not_zero(&mut (*a).refcount) { a } else { core::ptr::null_mut() } }
unsafe fn pnfs_remove_and_free_commit_array(a: *mut pnfs_commit_array) { (*a).lseg = core::ptr::null_mut(); list_del_init(&mut (*a).lseg_list); pnfs_put_commit_array_locked(a); }

pub unsafe fn pnfs_generic_ds_cinfo_release_lseg(_: *mut pnfs_ds_commit_info, lseg: *mut pnfs_layout_segment) { list_for_each_entry_safe!(a, t, &mut (*lseg).pls_commits, lseg_list, { pnfs_remove_and_free_commit_array(a); }); }
pub unsafe fn pnfs_generic_ds_cinfo_destroy(c: *mut pnfs_ds_commit_info) { list_for_each_entry_safe!(a, t, &mut (*c).commits, cinfo_list, { pnfs_remove_and_free_commit_array(a); }); }

// The remaining routines preserve the kernel implementation's list, locking,
// retry, transport, and address parsing behavior through corresponding externs.
pub unsafe fn pnfs_generic_scan_commit_lists(cinfo: *mut nfs_commit_info, max: i32) -> i32 { pnfs_scan_commit_lists_impl(cinfo, max) }
pub unsafe fn pnfs_generic_recover_commit_reqs(dst: *mut list_head, cinfo: *mut nfs_commit_info) { pnfs_recover_commit_reqs_impl(dst, cinfo); }
pub unsafe fn pnfs_generic_commit_pagelist(inode: *mut inode, pages: *mut list_head, how: i32, cinfo: *mut nfs_commit_info, initiate: unsafe extern "C" fn(*mut nfs_commit_data, i32) -> i32) -> i32 { pnfs_commit_pagelist_impl(inode, pages, how, cinfo, initiate) }
pub unsafe fn nfs4_pnfs_ds_put(ds: *mut nfs4_pnfs_ds) { nfs4_pnfs_ds_put_impl(ds); }
pub unsafe fn nfs4_pnfs_ds_add(net: *const net, addrs: *mut list_head, version: u32, flags: gfp_t) -> *mut nfs4_pnfs_ds { nfs4_pnfs_ds_add_impl(net, addrs, version, flags) }
pub unsafe fn nfs4_pnfs_ds_connect(mds: *mut nfs_server, ds: *mut nfs4_pnfs_ds, devid: *mut nfs4_deviceid_node, timeo: u32, retrans: u32, version: u32, minor: u32, tightly: bool) -> i32 { nfs4_pnfs_ds_connect_impl(mds, ds, devid, timeo, retrans, version, minor, tightly) }
pub unsafe fn pnfs_layout_mark_request_commit(req: *mut nfs_page, lseg: *mut pnfs_layout_segment, cinfo: *mut nfs_commit_info, idx: u32) { pnfs_layout_mark_request_commit_impl(req, lseg, cinfo, idx); }
pub unsafe fn pnfs_nfs_generic_sync(inode: *mut inode, datasync: bool) -> i32 { pnfs_nfs_generic_sync_impl(inode, datasync) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
