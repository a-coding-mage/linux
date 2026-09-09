// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS filesystem file handling
 *
 * Copyright (C) 2002, 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies are supplied by the surrounding kernel/AFS translation.

unsafe extern "C" {
    fn afs_request_key(cell: *mut afs_cell) -> *mut key;
    fn afs_validate(vnode: *mut afs_vnode, key: *mut key) -> i32;
    fn afs_issue_read(subreq: *mut netfs_io_subrequest);
    fn afs_init_request(rreq: *mut netfs_io_request, file: *mut file) -> i32;
}

static mut AFS_FILE_OPERATIONS: file_operations = file_operations {
    open: Some(afs_open), release: Some(afs_release), llseek: Some(generic_file_llseek),
    read_iter: Some(afs_file_read_iter), write_iter: Some(netfs_file_write_iter),
    mmap_prepare: Some(afs_file_mmap_prepare), splice_read: Some(afs_file_splice_read),
    splice_write: Some(iter_file_splice_write), fsync: Some(afs_fsync),
    lock: Some(afs_lock), flock: Some(afs_flock),
};

static mut AFS_FILE_INODE_OPERATIONS: inode_operations = inode_operations {
    getattr: Some(afs_getattr), setattr: Some(afs_setattr), permission: Some(afs_permission),
};

static mut AFS_FILE_AOPS: address_space_operations = address_space_operations {
    direct_IO: Some(noop_direct_IO), read_folio: Some(netfs_read_folio),
    readahead: Some(netfs_readahead), dirty_folio: Some(netfs_dirty_folio),
    release_folio: Some(netfs_release_folio), invalidate_folio: Some(netfs_invalidate_folio),
    migrate_folio: Some(filemap_migrate_folio), writepages: Some(afs_writepages),
};

static mut AFS_VM_OPS: vm_operations_struct = vm_operations_struct {
    mapped: Some(afs_mapped), open: Some(afs_vm_open), close: Some(afs_vm_close),
    fault: Some(filemap_fault), map_pages: Some(afs_vm_map_pages), page_mkwrite: Some(afs_page_mkwrite),
};

#[no_mangle]
pub unsafe extern "C" fn afs_put_wb_key(wbk: *mut afs_wb_key) {
    if !wbk.is_null() && refcount_dec_and_test(&mut (*wbk).usage) {
        key_put((*wbk).key); kfree(wbk as *mut core::ffi::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn afs_cache_wb_key(vnode: *mut afs_vnode, af: *mut afs_file) -> i32 {
    let wbk = kzalloc_obj::<afs_wb_key>();
    if wbk.is_null() { return -ENOMEM; }
    refcount_set(&mut (*wbk).usage, 2); (*wbk).key = (*af).key;
    spin_lock(&mut (*vnode).wb_lock);
    let mut p = (*vnode).wb_keys.first_entry::<afs_wb_key>();
    while !p.is_null() {
        if (*p).key == (*wbk).key { refcount_inc(&mut (*p).usage); spin_unlock(&mut (*vnode).wb_lock); (*af).wb = p; kfree(wbk as *mut _); return 0; }
        p = (*p).vnode_link.next_entry::<afs_wb_key>();
    }
    key_get((*wbk).key); list_add_tail(&mut (*wbk).vnode_link, &mut (*vnode).wb_keys);
    spin_unlock(&mut (*vnode).wb_lock); (*af).wb = wbk; 0
}

#[no_mangle]
pub unsafe extern "C" fn afs_open(inode: *mut inode, file: *mut file) -> i32 {
    let vnode = AFS_FS_I(inode); let key = afs_request_key((*vnode).volume.cell);
    if IS_ERR(key) { return PTR_ERR(key); }
    let af = kzalloc_obj::<afs_file>();
    if af.is_null() { key_put(key); return -ENOMEM; }
    (*af).key = key; let mut ret = afs_validate(vnode, key);
    if ret < 0 { kfree(af as *mut _); key_put(key); return ret; }
    if (*file).f_mode & FMODE_WRITE != 0 { ret = afs_cache_wb_key(vnode, af); if ret < 0 { kfree(af as *mut _); key_put(key); return ret; } }
    if (*file).f_flags & O_TRUNC != 0 { set_bit(AFS_VNODE_NEW_CONTENT, &mut (*vnode).flags); }
    fscache_use_cookie(afs_vnode_cache(vnode), (*file).f_mode & FMODE_WRITE != 0);
    (*file).private_data = af as *mut _; 0
}

#[no_mangle]
pub unsafe extern "C" fn afs_release(inode: *mut inode, file: *mut file) -> i32 {
    let vnode = AFS_FS_I(inode); let af = (*file).private_data as *mut afs_file; let mut ret = 0;
    if (*file).f_mode & FMODE_WRITE != 0 { ret = vfs_fsync(file, 0); }
    (*file).private_data = core::ptr::null_mut(); if !(*af).wb.is_null() { afs_put_wb_key((*af).wb); }
    if (*file).f_mode & FMODE_WRITE != 0 { let size = i_size_read(&mut (*vnode).netfs.inode); let mut aux = afs_vnode_cache_aux::default(); afs_set_cache_aux(vnode, &mut aux); fscache_unuse_cookie(afs_vnode_cache(vnode), &mut aux, &size); } else { fscache_unuse_cookie(afs_vnode_cache(vnode), core::ptr::null_mut(), core::ptr::null_mut()); }
    key_put((*af).key); kfree(af as *mut _); afs_prune_wb_keys(vnode); ret
}

unsafe fn afs_file_mmap_prepare(desc: *mut vm_area_desc) -> i32 { let ret = generic_file_mmap_prepare(desc); if ret != 0 { return ret; } (*desc).vm_ops = &raw mut AFS_VM_OPS; 0 }
unsafe fn afs_mapped(_: usize, _: usize, _: pgoff_t, file: *const file, _: *mut *mut core::ffi::c_void) -> i32 { afs_add_open_mmap(AFS_FS_I(file_inode(file as *mut _))); 0 }
unsafe fn afs_vm_open(vma: *mut vm_area_struct) { afs_add_open_mmap(AFS_FS_I(file_inode((*vma).vm_file))); }
unsafe fn afs_vm_close(vma: *mut vm_area_struct) { afs_drop_open_mmap(AFS_FS_I(file_inode((*vma).vm_file))); }
unsafe fn afs_vm_map_pages(vmf: *mut vm_fault, start: pgoff_t, end: pgoff_t) -> vm_fault_t { let v = AFS_FS_I(file_inode((*(*vmf).vma).vm_file)); if afs_check_validity(v) { filemap_map_pages(vmf, start, end) } else { 0 } }

unsafe fn afs_file_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize { let inode = file_inode((*iocb).ki_filp); let v = AFS_FS_I(inode); let af = (*iocb).ki_filp.private_data as *mut afs_file; if (*iocb).ki_flags & IOCB_DIRECT != 0 { return netfs_unbuffered_read_iter(iocb, iter); } let mut ret = netfs_start_io_read(inode); if ret < 0 { return ret as isize; } ret = afs_validate(v, (*af).key); if ret == 0 { ret = filemap_read(iocb, iter, 0) as i32; } netfs_end_io_read(inode); ret as isize }
unsafe fn afs_file_splice_read(in: *mut file, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: usize, flags: u32) -> isize { let inode = file_inode(in); let v = AFS_FS_I(inode); let af = (*in).private_data as *mut afs_file; let mut ret = netfs_start_io_read(inode); if ret < 0 { return ret as isize; } ret = afs_validate(v, (*af).key); if ret == 0 { ret = filemap_splice_read(in, ppos, pipe, len, flags) as i32; } netfs_end_io_read(inode); ret as isize }

unsafe fn afs_fetch_data_notify(op: *mut afs_operation) { (*(*op).fetch.subreq).error = afs_op_error(op); netfs_read_subreq_terminated((*op).fetch.subreq); }
unsafe fn afs_fetch_data_success(op: *mut afs_operation) { let vnode = (*op).file[0].vnode; afs_vnode_commit_status(op, &mut (*op).file[0]); afs_stat_v(vnode, n_fetches); atomic_long_add((*(*op).fetch.subreq).transferred, &mut (*(*op).net).n_fetch_bytes); afs_fetch_data_notify(op); }
unsafe fn afs_fetch_data_aborted(op: *mut afs_operation) { afs_check_for_remote_deletion(op); afs_fetch_data_notify(op); }
unsafe fn afs_issue_read_call(op: *mut afs_operation) { (*op).call_responded = false; (*op).call_error = 0; (*op).call_abort_code = 0; if test_bit(AFS_SERVER_FL_IS_YFS, &(*(*op).server).flags) { yfs_fs_fetch_data(op); } else { afs_fs_fetch_data(op); } }
unsafe fn afs_end_read(op: *mut afs_operation) { if (*op).call_responded && !(*op).server.is_null() { set_bit(AFS_SERVER_FL_RESPONDING, &mut (*(*op).server).flags); } if afs_op_error(op) == 0 { afs_fetch_data_success(op); } else if (*op).cumul_error.aborted { afs_fetch_data_aborted(op); } else { afs_fetch_data_notify(op); } afs_end_vnode_operation(op); afs_put_operation(op); }
unsafe fn afs_read_receive(call: *mut afs_call) { let op = (*call).op; let mut state = READ_ONCE((*call).state); if state == AFS_CALL_COMPLETE { return; } while state < AFS_CALL_COMPLETE && READ_ONCE((*call).need_attention) { WRITE_ONCE((*call).need_attention, false); afs_deliver_to_call(call); state = READ_ONCE((*call).state); } if state < AFS_CALL_COMPLETE { netfs_read_subreq_progress((*op).fetch.subreq); if rxrpc_kernel_check_life((*call).net.socket, (*call).rxcall) { return; } afs_set_call_complete(call, (*call).error, (*call).abort_code); } (*op).call_abort_code = (*call).abort_code; (*op).call_error = (*call).error; (*op).call_responded = (*call).responded; (*op).call = core::ptr::null_mut(); (*call).op = core::ptr::null_mut(); afs_put_call(call); if afs_select_fileserver(op) { afs_issue_read_call(op); } else { afs_end_read(op); } }
unsafe extern "C" fn afs_fetch_data_async_rx(work: *mut work_struct) { let call = container_of(work, async_work); afs_read_receive(call); afs_put_call(call); }
unsafe extern "C" fn afs_fetch_data_immediate_cancel(call: *mut afs_call) { if (*call).async_ { afs_get_call(call, afs_call_trace_wake); if !queue_work(afs_async_calls, &mut (*call).async_work) { afs_deferred_put_call(call); } flush_work(&mut (*call).async_work); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
