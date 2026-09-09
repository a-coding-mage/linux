/* Rust translation of inode.c. Kernel and local dependency declarations are
 * supplied by the surrounding translation unit. */

use core::ffi::c_void;

unsafe fn dump_vnode(vnode: *mut afs_vnode, parent_vnode: *mut afs_vnode) {
    static mut ONCE_ONLY: c_ulong = 0;
    pr_warn!("kAFS: AFS vnode with undefined type {}\n", (*vnode).status.type_);
    pr_warn!("kAFS: A={} m={:o} s={:x} v={:x}\n", (*vnode).status.abort_code,
             (*vnode).status.mode, (*vnode).status.size, (*vnode).status.data_version);
    pr_warn!("kAFS: vnode {:x}:{:x}:{:x}\n", (*vnode).fid.vid, (*vnode).fid.vnode,
             (*vnode).fid.unique);
    if !parent_vnode.is_null() {
        pr_warn!("kAFS: dir {:x}:{:x}:{:x}\n", (*parent_vnode).fid.vid,
                 (*parent_vnode).fid.vnode, (*parent_vnode).fid.unique);
    }
    if !test_and_set_bit(0, &mut ONCE_ONLY) { dump_stack(); }
}

unsafe fn afs_set_netfs_context(vnode: *mut afs_vnode, is_file: bool) {
    netfs_inode_init(&mut (*vnode).netfs, &afs_req_ops, is_file);
}

unsafe fn afs_inode_init_from_status(op: *mut afs_operation, vp: *mut afs_vnode_param,
                                     vnode: *mut afs_vnode) -> c_int {
    let status = &(*vp).scb.status;
    let inode = AFS_VNODE_TO_I(vnode);
    let t = status.mtime_client;
    _enter!("{:x}:{}.{} {}", (*vp).fid.vid, (*vp).fid.vnode, (*vp).fid.unique,
            if !(*op).type_.is_null() { (*(*op).type_).name } else { "???" });
    _debug!("FS: ft={} lk={} sz={} ver={} mod={}", status.type_, status.nlink,
            status.size, status.data_version, status.mode);
    write_seqlock(&mut (*vnode).cb_lock);
    (*vnode).cb_v_check = (*op).cb_v_break;
    (*vnode).status = *status;
    inode_set_ctime_to_ts(inode, t); inode_set_mtime_to_ts(inode, t); inode_set_atime_to_ts(inode, t);
    (*inode).i_flags |= S_NOATIME;
    (*inode).i_uid = make_kuid(&init_user_ns, status.owner);
    (*inode).i_gid = make_kgid(&init_user_ns, status.group);
    set_nlink(&mut (*vnode).netfs.inode, status.nlink);
    i_size_write(inode, status.size); inode_set_bytes(inode, status.size);
    afs_set_netfs_context(vnode, status.type_ == AFS_FTYPE_FILE);
    match status.type_ {
        AFS_FTYPE_FILE => { (*inode).i_mode = S_IFREG | (status.mode & S_IALLUGO); (*inode).i_op = &afs_file_inode_operations; (*inode).i_fop = &afs_file_operations; (*(*inode).i_mapping).a_ops = &afs_file_aops; mapping_set_large_folios((*inode).i_mapping); },
        AFS_FTYPE_DIR => { (*inode).i_mode = S_IFDIR | (status.mode & S_IALLUGO); (*inode).i_op = &afs_dir_inode_operations; (*inode).i_fop = &afs_dir_file_operations; (*(*inode).i_mapping).a_ops = &afs_dir_aops; __set_bit(NETFS_ICTX_SINGLE_NO_UPLOAD, &mut (*vnode).netfs.flags); __set_bit(AFS_VNODE_DIR_VALID, &mut (*vnode).flags); },
        AFS_FTYPE_SYMLINK => { if status.mode & 0o777 == 0o644 { (*inode).i_flags |= S_AUTOMOUNT; set_bit(AFS_VNODE_MOUNTPOINT, &mut (*vnode).flags); (*inode).i_mode = S_IFDIR | 0o555; (*inode).i_op = &afs_mntpt_inode_operations; (*inode).i_fop = &afs_mntpt_file_operations; } else { (*inode).i_mode = S_IFLNK | status.mode; (*inode).i_op = &afs_symlink_inode_operations; } (*(*inode).i_mapping).a_ops = &afs_symlink_aops; inode_nohighmem(inode); },
        _ => { dump_vnode(vnode, if (*op).file[0].vnode != vnode { (*op).file[0].vnode } else { core::ptr::null_mut() }); write_sequnlock(&mut (*vnode).cb_lock); return afs_protocol_error(core::ptr::null_mut(), afs_eproto_file_type); }
    }
    (*vnode).invalid_before = status.data_version; trace_afs_set_dv(vnode, status.data_version); inode_set_iversion_raw(&mut (*vnode).netfs.inode, status.data_version);
    if !(*vp).scb.have_cb { afs_clear_cb_promise(vnode, afs_cb_promise_set_new_symlink); } else { (*vnode).cb_server = (*op).server; afs_set_cb_promise(vnode, (*vp).scb.callback.expires_at, afs_cb_promise_set_new_inode); }
    write_sequnlock(&mut (*vnode).cb_lock); 0
}

unsafe fn afs_apply_status(op: *mut afs_operation, vp: *mut afs_vnode_param) {
    let status = &(*vp).scb.status; let vnode = (*vp).vnode; let ictx = &mut (*vnode).netfs; let inode = &mut ictx.inode;
    let mut mode; let mut unexpected_jump = false; let mut data_changed = false; let mut change_size = (*vp).set_size;
    _enter!("{:x}:{}.{} {}", (*vp).fid.vid, (*vp).fid.vnode, (*vp).fid.unique, if !(*op).type_.is_null() { (*(*op).type_).name } else { "???" });
    BUG_ON(test_bit(AFS_VNODE_UNSET, &(*vnode).flags));
    if status.type_ != (*vnode).status.type_ { pr_warn!("Vnode {:x}:{:x}:{:x} changed type {} to {}\n", (*vnode).fid.vid, (*vnode).fid.vnode, (*vnode).fid.unique, status.type_, (*vnode).status.type_); afs_protocol_error(core::ptr::null_mut(), afs_eproto_bad_status); return; }
    if status.nlink != (*vnode).status.nlink { set_nlink(inode, status.nlink); }
    if status.owner != (*vnode).status.owner { (*inode).i_uid = make_kuid(&init_user_ns, status.owner); }
    if status.group != (*vnode).status.group { (*inode).i_gid = make_kgid(&init_user_ns, status.group); }
    if status.mode != (*vnode).status.mode { mode = (*inode).i_mode & !S_IALLUGO; mode |= status.mode & S_IALLUGO; WRITE_ONCE!((*inode).i_mode, mode); }
    let t = status.mtime_client; inode_set_mtime_to_ts(inode, t); if (*vp).update_ctime { inode_set_ctime_to_ts(inode, (*op).ctime); }
    if (*vnode).status.data_version != status.data_version { trace_afs_set_dv(vnode, status.data_version); data_changed = true; }
    (*vnode).status = *status;
    if (*vp).dv_before + (*vp).dv_delta != status.data_version { trace_afs_dv_mismatch(vnode, (*vp).dv_before, (*vp).dv_delta, status.data_version); (*vnode).invalid_before = status.data_version; if (*vnode).status.type_ == AFS_FTYPE_DIR { afs_invalidate_dir(vnode, afs_dir_invalid_dv_mismatch); } else { set_bit(AFS_VNODE_ZAP_DATA, &mut (*vnode).flags); } change_size = true; data_changed = true; unexpected_jump = true; } else if (*vnode).status.type_ == AFS_FTYPE_DIR { if test_bit(AFS_VNODE_DIR_VALID, &(*vnode).flags) { data_changed = false; } change_size = true; }
    if data_changed { let size = status.size; inode_set_iversion_raw(inode, status.data_version); spin_lock(&mut (*inode).i_lock); if change_size || size > i_size_read(inode) { let mut zero_point = ictx._zero_point; if unexpected_jump { zero_point = size; } netfs_write_sizes(inode, size, size, zero_point); inode_set_bytes(inode, size); inode_set_ctime_to_ts(inode, t); inode_set_atime_to_ts(inode, t); } else { netfs_write_remote_i_size(inode, size); } spin_unlock(&mut (*inode).i_lock); if (*op).ops == &afs_fetch_data_operation { (*(*op).fetch.subreq).rreq.i_size = size; } }
}

unsafe fn afs_apply_callback(op: *mut afs_operation, vp: *mut afs_vnode_param) { let vnode = (*vp).vnode; if !afs_cb_is_broken((*vp).cb_break_before, vnode) { if (*op).volume.type_ == AFSVL_RWVOL { (*vnode).cb_server = (*op).server; } afs_set_cb_promise(vnode, (*vp).scb.callback.expires_at, afs_cb_promise_set_apply_cb); } }

pub unsafe fn afs_vnode_commit_status(op: *mut afs_operation, vp: *mut afs_vnode_param) {
    let vnode = (*vp).vnode; _enter!(""); write_seqlock(&mut (*vnode).cb_lock);
    if (*vp).scb.have_error { if (*vp).scb.status.abort_code == VNOVNODE { set_bit(AFS_VNODE_DELETED, &mut (*vnode).flags); clear_nlink(&mut (*vnode).netfs.inode); __afs_break_callback(vnode, afs_cb_break_for_deleted); (*op).flags &= !AFS_OPERATION_DIR_CONFLICT; } }
    else if (*vp).scb.have_status { if (*vp).speculative && (test_bit(AFS_VNODE_MODIFYING, &(*vnode).flags) || (*vp).dv_before != (*vnode).status.data_version) { write_sequnlock(&mut (*vnode).cb_lock); return; } afs_apply_status(op, vp); if (*vp).scb.have_cb { afs_apply_callback(op, vp); } }
    else if (*vp).op_unlinked && ((*op).flags & AFS_OPERATION_DIR_CONFLICT) == 0 { drop_nlink(&mut (*vnode).netfs.inode); if (*vnode).netfs.inode.i_nlink == 0 { set_bit(AFS_VNODE_DELETED, &mut (*vnode).flags); __afs_break_callback(vnode, afs_cb_break_for_deleted); } }
    write_sequnlock(&mut (*vnode).cb_lock); if (*vp).scb.have_status { afs_cache_permit(vnode, (*op).key, (*vp).cb_break_before, &(*vp).scb); }
}

unsafe fn afs_fetch_status_success(op: *mut afs_operation) { let vp = &mut (*op).file[(*op).fetch_status.which]; let vnode = vp.vnode; let ret; if inode_state_read_once(&mut (*vnode).netfs.inode) & I_NEW != 0 { ret = afs_inode_init_from_status(op, vp, vnode); afs_op_set_error(op, ret); if ret == 0 { afs_cache_permit(vnode, (*op).key, vp.cb_break_before, &vp.scb); } } else { afs_vnode_commit_status(op, vp); } }

pub static afs_fetch_status_operation: afs_operation_ops = afs_operation_ops { issue_afs_rpc: afs_fs_fetch_status, issue_yfs_rpc: yfs_fs_fetch_status, success: afs_fetch_status_success, aborted: afs_check_for_remote_deletion };

pub unsafe fn afs_fetch_status(vnode: *mut afs_vnode, key: *mut key, is_new: bool, caller_access: *mut afs_access_t) -> c_int { let op = afs_alloc_operation(key, (*vnode).volume); if IS_ERR(op) { return PTR_ERR(op); } afs_op_set_vnode(op, 0, vnode); (*op).nr_files = 1; (*op).ops = &afs_fetch_status_operation; afs_begin_vnode_operation(op); afs_wait_for_operation(op); if !caller_access.is_null() { *caller_access = (*op).file[0].scb.status.caller_access; } afs_put_operation(op) }

pub unsafe fn afs_ilookup5_test_by_fid(inode: *mut inode, opaque: *mut c_void) -> c_int { let vnode = AFS_FS_I(inode); let fid = opaque as *mut afs_fid; ( (*fid).vnode == (*vnode).fid.vnode && (*fid).vnode_hi == (*vnode).fid.vnode_hi && (*fid).unique == (*vnode).fid.unique ) as c_int }
unsafe fn afs_iget5_test(inode: *mut inode, opaque: *mut c_void) -> c_int { afs_ilookup5_test_by_fid(inode, &(*((opaque as *mut afs_vnode_param))).fid as *const _ as *mut c_void) }
unsafe fn afs_iget5_set(inode: *mut inode, opaque: *mut c_void) -> c_int { let vp = opaque as *mut afs_vnode_param; let as_ = AFS_FS_S((*inode).i_sb); let vnode = AFS_FS_I(inode); (*vnode).volume = (*as_).volume; (*vnode).fid = (*vp).fid; (*inode).i_ino = (*vnode).fid.vnode; (*inode).i_generation = (*vnode).fid.unique; 0 }

unsafe fn afs_get_inode_cache(vnode: *mut afs_vnode) {
    /* CONFIG_AFS_FSCACHE controls the following kernel-only cache-key code. */
    if (*vnode).status.type_ != AFS_FTYPE_FILE && (*vnode).status.type_ != AFS_FTYPE_DIR && (*vnode).status.type_ != AFS_FTYPE_SYMLINK { (*vnode).netfs.cache = core::ptr::null_mut(); return; }
    let key = (htonl((*vnode).fid.vnode), htonl((*vnode).fid.unique), [htonl((*vnode).fid.vnode >> 32), htonl((*vnode).fid.vnode_hi)]);
    let mut aux = core::mem::zeroed(); afs_set_cache_aux(vnode, &mut aux); afs_vnode_set_cache(vnode, fscache_acquire_cookie((*vnode).volume.cache, if (*vnode).status.type_ == AFS_FTYPE_FILE { 0 } else { FSCACHE_ADV_SINGLE_CHUNK }, &key as *const _ as *mut c_void, core::mem::size_of_val(&key), &aux, core::mem::size_of_val(&aux), i_size_read(&(*vnode).netfs.inode)));
}

pub unsafe fn afs_iget(op: *mut afs_operation, vp: *mut afs_vnode_param) -> *mut inode { let sb = (*(*op).file[0].vnode).netfs.inode.i_sb; let inode = iget5_locked(sb, (*vp).fid.vnode, afs_iget5_test, afs_iget5_set, vp as *mut c_void); if inode.is_null() { return ERR_PTR(-ENOMEM); } let vnode = AFS_FS_I(inode); if inode_state_read_once(inode) & I_NEW == 0 { return inode; } let ret = afs_inode_init_from_status(op, vp, vnode); if ret < 0 { iget_failed(inode); return ERR_PTR(ret); } afs_get_inode_cache(vnode); clear_bit(AFS_VNODE_UNSET, &mut (*vnode).flags); unlock_new_inode(inode); inode }

unsafe fn afs_iget5_set_root(inode: *mut inode, _: *mut c_void) -> c_int { let as_ = AFS_FS_S((*inode).i_sb); let vnode = AFS_FS_I(inode); (*vnode).volume = (*as_).volume; (*vnode).fid.vid = (*as_).volume.vid; (*vnode).fid.vnode = 1; (*vnode).fid.unique = 1; (*inode).i_ino = 1; (*inode).i_generation = 1; 0 }

pub unsafe fn afs_root_iget(sb: *mut super_block, key: *mut key) -> *mut inode { let as_ = AFS_FS_S(sb); let inode = iget5_locked(sb, 1, None, afs_iget5_set_root, core::ptr::null_mut()); if inode.is_null() { return ERR_PTR(-ENOMEM); } let vnode = AFS_FS_I(inode); (*vnode).cb_v_check = atomic_read(&(*as_).volume.cb_v_break); let op = afs_alloc_operation(key, (*as_).volume); if IS_ERR(op) { iget_failed(inode); return ERR_PTR(PTR_ERR(op)); } afs_op_set_vnode(op, 0, vnode); (*op).nr_files = 1; (*op).ops = &afs_fetch_status_operation; let ret = afs_do_sync_operation(op); if ret < 0 { iget_failed(inode); return ERR_PTR(ret); } afs_get_inode_cache(vnode); clear_bit(AFS_VNODE_UNSET, &mut (*vnode).flags); unlock_new_inode(inode); inode }

pub unsafe fn afs_getattr(_: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, query_flags: c_uint) -> c_int { let inode = d_inode((*path).dentry); let vnode = AFS_FS_I(inode); if !(*vnode).volume.is_null() && query_flags & AT_STATX_DONT_SYNC == 0 && atomic64_read(&(*vnode).cb_expires_at) == AFS_NO_CB_PROMISE { let key = afs_request_key((*vnode).volume.cell); if IS_ERR(key) { return PTR_ERR(key); } let ret = afs_validate(vnode, key); key_put(key); if ret < 0 { return ret; } } let mut seq; loop { seq = read_seqbegin(&(*vnode).cb_lock); generic_fillattr(&nop_mnt_idmap, request_mask, inode, stat); if test_bit(AFS_VNODE_SILLY_DELETED, &(*vnode).flags) && (*stat).nlink > 0 { (*stat).nlink -= 1; } if S_ISDIR((*inode).i_mode) { (*stat).size = netfs_read_remote_i_size(inode); } if !read_seqretry(&(*vnode).cb_lock, seq) { break; } } 0 }

pub unsafe fn afs_drop_inode(inode: *mut inode) -> c_int { if test_bit(AFS_VNODE_PSEUDODIR, &(*AFS_FS_I(inode)).flags) { inode_just_drop(inode) } else { inode_generic_drop(inode) } }

pub unsafe fn afs_evict_inode(inode: *mut inode) { let mut aux = core::mem::zeroed(); let sbi = AFS_FS_S((*inode).i_sb); let vnode = AFS_FS_I(inode); ASSERTCMP!((*inode).i_ino, ==, (*vnode).fid.vnode); if (S_ISDIR((*inode).i_mode) || S_ISLNK((*inode).i_mode)) && inode_state_read_once(inode) & I_DIRTY != 0 && !(*sbi).dyn_root { let mut wbc = writeback_control { sync_mode: WB_SYNC_ALL, for_sync: true, range_end: LLONG_MAX, ..core::mem::zeroed() }; ((*(*inode).i_mapping).a_ops.writepages)( (*inode).i_mapping, &mut wbc); } flush_delayed_work(&mut (*vnode).lock_work); netfs_wait_for_outstanding_io(inode); truncate_inode_pages_final(&mut (*inode).i_data); netfs_free_folioq_buffer((*vnode).directory); if !(*vnode).symlink.is_null() { afs_evict_symlink(vnode); } afs_set_cache_aux(vnode, &mut aux); netfs_clear_inode_writeback(inode, &aux); clear_inode(inode); while !list_empty(&(*vnode).wb_keys) { let wbk = list_entry((*vnode).wb_keys.next, afs_wb_key, vnode_link); list_del(&mut (*wbk).vnode_link); afs_put_wb_key(wbk); } fscache_relinquish_cookie(afs_vnode_cache(vnode), test_bit(AFS_VNODE_DELETED, &(*vnode).flags)); afs_prune_wb_keys(vnode); afs_put_permits(rcu_access_pointer((*vnode).permit_cache)); key_put((*vnode).silly_key); (*vnode).silly_key = core::ptr::null_mut(); key_put((*vnode).lock_key); (*vnode).lock_key = core::ptr::null_mut(); }

unsafe fn afs_setattr_success(op: *mut afs_operation) { let vp = &mut (*op).file[0]; let inode = &mut (*vp.vnode).netfs.inode; let old = i_size_read(inode); (*op).setattr.old_i_size = old; afs_vnode_commit_status(op, vp); if (*op).setattr.attr.ia_valid & ATTR_SIZE != 0 { let size = (*op).setattr.attr.ia_size; if size > old { pagecache_isize_extended(inode, old, size); } } }
unsafe fn afs_setattr_edit_file(op: *mut afs_operation) { let vp = &mut (*op).file[0]; let vnode = vp.vnode; if (*op).setattr.attr.ia_valid & ATTR_SIZE != 0 { let size = (*op).setattr.attr.ia_size; let old = (*op).setattr.old_i_size; if size != old { truncate_pagecache(&mut (*vnode).netfs.inode, size); netfs_resize_file(&mut (*vnode).netfs, size, true); fscache_resize_cookie(afs_vnode_cache(vnode), size); } } }
static afs_setattr_operation: afs_operation_ops = afs_operation_ops { issue_afs_rpc: afs_fs_setattr, issue_yfs_rpc: yfs_fs_setattr, success: afs_setattr_success, edit_dir: afs_setattr_edit_file };

pub unsafe fn afs_setattr(_: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int { let supported = ATTR_SIZE | ATTR_MODE | ATTR_UID | ATTR_GID | ATTR_MTIME | ATTR_MTIME_SET | ATTR_TIMES_SET | ATTR_TOUCH; let vnode = AFS_FS_I(d_inode(dentry)); let inode = &mut (*vnode).netfs.inode; let mut i_size = i_size_read(inode); if (*attr).ia_valid & supported == 0 { return 0; } if (*attr).ia_valid & ATTR_SIZE != 0 { if !S_ISREG((*inode).i_mode) { return -EISDIR; } let ret = inode_newsize_ok(inode, (*attr).ia_size); if ret != 0 { return ret; } if (*attr).ia_size == i_size { (*attr).ia_valid &= !ATTR_SIZE; } } fscache_use_cookie(afs_vnode_cache(vnode), true); down_write(&mut (*vnode).validate_lock); let op = afs_alloc_operation(if (*attr).ia_valid & ATTR_FILE != 0 { afs_file_key((*attr).ia_file) } else { core::ptr::null_mut() }, (*vnode).volume); if IS_ERR(op) { up_write(&mut (*vnode).validate_lock); fscache_unuse_cookie(afs_vnode_cache(vnode), core::ptr::null_mut(), core::ptr::null_mut()); return PTR_ERR(op); } afs_op_set_vnode(op, 0, vnode); (*op).setattr.attr = attr; if (*attr).ia_valid & ATTR_SIZE != 0 { (*op).file[0].dv_delta = 1; (*op).file[0].set_size = true; } (*op).ctime = (*attr).ia_ctime; (*op).file[0].update_ctime = 1; (*op).file[0].modification = true; (*op).ops = &afs_setattr_operation; let ret = afs_do_sync_operation(op); up_write(&mut (*vnode).validate_lock); fscache_unuse_cookie(afs_vnode_cache(vnode), core::ptr::null_mut(), core::ptr::null_mut()); ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
