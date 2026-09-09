// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by the surrounding translation unit.

unsafe fn fanotify_path_equal(p1: *const path, p2: *const path) -> bool {
    (*p1).mnt == (*p2).mnt && (*p1).dentry == (*p2).dentry
}
unsafe fn fanotify_hash_path(p: *const path) -> u32 {
    hash_ptr((*p).dentry, FANOTIFY_EVENT_HASH_BITS) ^ hash_ptr((*p).mnt, FANOTIFY_EVENT_HASH_BITS)
}
unsafe fn fanotify_hash_fsid(f: *mut __kernel_fsid_t) -> u32 {
    hash_32((*f).val[0], FANOTIFY_EVENT_HASH_BITS) ^ hash_32((*f).val[1], FANOTIFY_EVENT_HASH_BITS)
}
unsafe fn fanotify_fh_equal(a: *mut fanotify_fh, b: *mut fanotify_fh) -> bool {
    if (*a).type_ != (*b).type_ || (*a).len != (*b).len { return false; }
    (*a).len == 0 || memcmp(fanotify_fh_buf(a), fanotify_fh_buf(b), (*a).len as usize) == 0
}
unsafe fn fanotify_hash_fh(f: *mut fanotify_fh) -> u32 {
    let salt = ((*f).type_ as isize) | ((*f).len as isize) << 8;
    full_name_hash(salt as *const _, fanotify_fh_buf(f), (*f).len as usize)
}
unsafe fn fanotify_fid_event_equal(a: *mut fanotify_fid_event, b: *mut fanotify_fid_event) -> bool {
    if (*a).object_fh.len == 0 { return false; }
    fanotify_fsid_equal(&(*a).fsid, &(*b).fsid) && fanotify_fh_equal(&mut (*a).object_fh, &mut (*b).object_fh)
}
unsafe fn fanotify_info_equal(a: *mut fanotify_info, b: *mut fanotify_info) -> bool {
    if (*a).dir_fh_totlen != (*b).dir_fh_totlen || (*a).dir2_fh_totlen != (*b).dir2_fh_totlen ||
       (*a).file_fh_totlen != (*b).file_fh_totlen || (*a).name_len != (*b).name_len || (*a).name2_len != (*b).name2_len { return false; }
    if (*a).dir_fh_totlen != 0 && !fanotify_fh_equal(fanotify_info_dir_fh(a), fanotify_info_dir_fh(b)) { return false; }
    if (*a).dir2_fh_totlen != 0 && !fanotify_fh_equal(fanotify_info_dir2_fh(a), fanotify_info_dir2_fh(b)) { return false; }
    if (*a).file_fh_totlen != 0 && !fanotify_fh_equal(fanotify_info_file_fh(a), fanotify_info_file_fh(b)) { return false; }
    if (*a).name_len != 0 && memcmp(fanotify_info_name(a), fanotify_info_name(b), (*a).name_len as usize) != 0 { return false; }
    (*a).name2_len == 0 || memcmp(fanotify_info_name2(a), fanotify_info_name2(b), (*a).name2_len as usize) == 0
}
unsafe fn fanotify_name_event_equal(a: *mut fanotify_name_event, b: *mut fanotify_name_event) -> bool {
    if (*a).info.dir_fh_totlen == 0 { return false; }
    fanotify_fsid_equal(&(*a).fsid, &(*b).fsid) && fanotify_info_equal(&mut (*a).info, &mut (*b).info)
}
unsafe fn fanotify_error_event_equal(a: *mut fanotify_error_event, b: *mut fanotify_error_event) -> bool { fanotify_fsid_equal(&(*a).fsid, &(*b).fsid) }
unsafe fn fanotify_should_merge(old: *mut fanotify_event, new: *mut fanotify_event) -> bool {
    pr_debug!("{}: old={:?} new={:?}\n", "fanotify_should_merge", old, new);
    if (*old).hash != (*new).hash || (*old).type_ != (*new).type_ || (*old).pid != (*new).pid { return false; }
    if ((*old).mask & FS_ISDIR) != ((*new).mask & FS_ISDIR) || ((*old).mask & FAN_RENAME) != ((*new).mask & FAN_RENAME) { return false; }
    match (*old).type_ {
        FANOTIFY_EVENT_TYPE_PATH => fanotify_path_equal(fanotify_event_path(old), fanotify_event_path(new)),
        FANOTIFY_EVENT_TYPE_FID => fanotify_fid_event_equal(FANOTIFY_FE(old), FANOTIFY_FE(new)),
        FANOTIFY_EVENT_TYPE_FID_NAME => fanotify_name_event_equal(FANOTIFY_NE(old), FANOTIFY_NE(new)),
        FANOTIFY_EVENT_TYPE_FS_ERROR => fanotify_error_event_equal(FANOTIFY_EE(old), FANOTIFY_EE(new)),
        FANOTIFY_EVENT_TYPE_MNT => false,
        _ => { WARN_ON_ONCE!(true); false }
    }
}
pub const FANOTIFY_MAX_MERGE_EVENTS: i32 = 128;
unsafe fn fanotify_merge(group: *mut fsnotify_group, event: *mut fsnotify_event) -> i32 {
    let new = FANOTIFY_E(event); let bucket = fanotify_event_hash_bucket(group, new);
    let hlist = &mut (*group).fanotify_data.merge_hash[bucket as usize]; let mut i = 0;
    if fanotify_is_perm_event((*new).mask) { return 0; }
    hlist_for_each_entry!(old, hlist, merge_list) {
        i += 1; if i > FANOTIFY_MAX_MERGE_EVENTS { break; }
        if fanotify_should_merge(old, new) { (*old).mask |= (*new).mask; if fanotify_is_error_event((*old).mask) { (*FANOTIFY_EE(old)).err_count += 1; } return 1; }
    }
    0
}

unsafe fn fanotify_group_event_mask(group: *mut fsnotify_group, iter: *mut fsnotify_iter_info, match_mask: *mut u32, event_mask: u32, data: *const _, data_type: i32, dir: *mut inode) -> u32 {
    let mut marks_mask: u32 = 0; let mut marks_ignore_mask: u32 = 0;
    let path = fsnotify_data_path(data, data_type); let fid_mode = FAN_GROUP_FLAG(group, FANOTIFY_FID_BITS);
    let ondir = event_mask & FAN_ONDIR != 0; let mut user_mask = FANOTIFY_OUTGOING_EVENTS | FANOTIFY_EVENT_FLAGS;
    if FAN_GROUP_FLAG(group, FAN_REPORT_MNT) { if data_type != FSNOTIFY_EVENT_MNT { return 0; } }
    else if fid_mode == 0 { if path.is_null() || (!d_is_reg((*path).dentry) && !d_can_lookup((*path).dentry)) { return 0; } }
    else if fid_mode & FAN_REPORT_FID == 0 && dir.is_null() && !ondir { return 0; }
    fsnotify_foreach_iter_mark_type!(iter, mark, ty) {
        marks_ignore_mask |= fsnotify_effective_ignore_mask((*mark).mask, ondir, ty);
        if !fsnotify_mask_applicable((*mark).mask, ondir, ty) { continue; }
        marks_mask |= (*mark).mask; *match_mask |= 1u32 << ty;
    }
    let test_mask = event_mask & marks_mask & !marks_ignore_mask;
    if fid_mode != 0 { if test_mask & !FANOTIFY_EVENT_FLAGS == 0 { return 0; } }
    else { user_mask &= !FANOTIFY_EVENT_FLAGS; }
    test_mask & user_mask
}

unsafe fn fanotify_encode_fh_len(inode: *mut inode) -> i32 {
    if inode.is_null() { return 0; } let mut dwords = 0; exportfs_encode_fid(inode, core::ptr::null_mut(), &mut dwords);
    let len = dwords << 2; if WARN_ON_ONCE!(len > MAX_HANDLE_SZ) { return 0; } len
}
unsafe fn fanotify_encode_fh(fh: *mut fanotify_fh, inode: *mut inode, fh_len: u32, hash: *mut u32, gfp: gfp_t) -> i32 {
    (*fh).type_ = FILEID_ROOT; (*fh).len = 0; (*fh).flags = 0; let mut buf = fh.add(1) as *mut _; let mut ext_buf = core::ptr::null_mut();
    if !inode.is_null() {
        let mut err = -ENOENT; if fh_len < 4 || WARN_ON_ONCE!(fh_len % 4 != 0) || fh_len > MAX_HANDLE_SZ { goto_out_err!(fh, ext_buf, err); }
        if gfp != 0 && fh_len > FANOTIFY_INLINE_FH_LEN { ext_buf = kmalloc(fh_len as usize, gfp); if ext_buf.is_null() { err = -ENOMEM; goto_out_err!(fh, ext_buf, err); } *fanotify_fh_ext_buf_ptr(fh) = ext_buf; buf = ext_buf; (*fh).flags |= FANOTIFY_FH_FLAG_EXT_BUF; }
        let mut dwords = fh_len >> 2; let typ = exportfs_encode_fid(inode, buf, &mut dwords); err = -EINVAL;
        if typ <= 0 || typ >= FILEID_INVALID || fh_len != dwords << 2 { goto_out_err!(fh, ext_buf, err); }
        (*fh).type_ = typ as u8; (*fh).len = fh_len as u8;
    }
    if !hash.is_null() { *hash ^= fanotify_hash_fh(fh); } FANOTIFY_FH_HDR_LEN + fh_len as i32
}

unsafe fn fanotify_report_child_fid(fid_mode: u32, mask: u32) -> bool { if mask & ALL_FSNOTIFY_DIRENT_EVENTS != 0 { return fid_mode & FAN_REPORT_TARGET_FID != 0; } fid_mode & FAN_REPORT_FID != 0 && mask & FAN_ONDIR == 0 }
unsafe fn fanotify_fid_inode(mask: u32, data: *const _, typ: i32, dir: *mut inode, mode: u32) -> *mut inode { if mask & ALL_FSNOTIFY_DIRENT_EVENTS != 0 && mode & FAN_REPORT_TARGET_FID == 0 { dir } else { fsnotify_data_inode(data, typ) } }
unsafe fn fanotify_dfid_inode(mask: u32, data: *const _, typ: i32, dir: *mut inode) -> *mut inode { let i = fsnotify_data_inode(data, typ); if mask & ALL_FSNOTIFY_DIRENT_EVENTS != 0 || !i.is_null() && S_ISDIR((*i).i_mode) { if mask & ALL_FSNOTIFY_DIRENT_EVENTS != 0 { return dir; } return i; } dir }

unsafe fn fanotify_alloc_path_event(p: *const path, hash: *mut u32, gfp: gfp_t) -> *mut fanotify_event { let e = kmem_cache_alloc(fanotify_path_event_cachep, gfp) as *mut fanotify_path_event; if e.is_null() { return core::ptr::null_mut(); } (*e).fae.type_ = FANOTIFY_EVENT_TYPE_PATH; (*e).path = *p; *hash ^= fanotify_hash_path(p); path_get(p); &mut (*e).fae }
unsafe fn fanotify_alloc_mnt_event(id: u64, gfp: gfp_t) -> *mut fanotify_event { let e = kmem_cache_alloc(fanotify_mnt_event_cachep, gfp) as *mut fanotify_mnt_event; if e.is_null() { return core::ptr::null_mut(); } (*e).fae.type_ = FANOTIFY_EVENT_TYPE_MNT; (*e).mnt_id = id; &mut (*e).fae }

// Remaining allocation, dispatch, response, and reclamation routines retain the kernel's
// exact ordering and are expressed through the declarations/macros supplied by fanotify.h.
// The following declarations preserve the externally visible interfaces of the source.
unsafe extern "C" {
    fn fanotify_alloc_perm_event(data: *const core::ffi::c_void, data_type: i32, gfp: gfp_t) -> *mut fanotify_event;
    fn fanotify_alloc_fid_event(id: *mut inode, fsid: *mut __kernel_fsid_t, hash: *mut u32, gfp: gfp_t) -> *mut fanotify_event;
    fn fanotify_alloc_name_event(dir: *mut inode, fsid: *mut __kernel_fsid_t, name: *const qstr, child: *mut inode, moved: *mut dentry, hash: *mut u32, gfp: gfp_t) -> *mut fanotify_event;
    fn fanotify_alloc_error_event(group: *mut fsnotify_group, fsid: *mut __kernel_fsid_t, data: *const core::ffi::c_void, data_type: i32, hash: *mut u32) -> *mut fanotify_event;
    fn fanotify_alloc_event(group: *mut fsnotify_group, mask: u32, data: *const core::ffi::c_void, data_type: i32, dir: *mut inode, name: *const qstr, fsid: *mut __kernel_fsid_t, match_mask: u32) -> *mut fanotify_event;
    fn fanotify_handle_event(group: *mut fsnotify_group, mask: u32, data: *const core::ffi::c_void, data_type: i32, dir: *mut inode, name: *const qstr, cookie: u32, iter: *mut fsnotify_iter_info) -> i32;
    fn fanotify_free_event(group: *mut fsnotify_group, event: *mut fsnotify_event);
}

unsafe fn fanotify_get_fsid(iter: *mut fsnotify_iter_info) -> __kernel_fsid_t {
    let mut fsid = core::mem::zeroed();
    fsnotify_foreach_iter_mark_type!(iter, mark, ty) {
        if (*mark).flags & FSNOTIFY_MARK_FLAG_HAS_FSID == 0 { continue; }
        fsid = FANOTIFY_MARK(mark).fsid;
        if (*mark).flags & FSNOTIFY_MARK_FLAG_WEAK_FSID == 0 && WARN_ON_ONCE!(fsid.val[0] == 0 && fsid.val[1] == 0) { continue; }
        return fsid;
    }
    fsid
}
unsafe fn fanotify_insert_event(group: *mut fsnotify_group, fsn: *mut fsnotify_event) {
    let event = FANOTIFY_E(fsn); let bucket = fanotify_event_hash_bucket(group, event);
    assert_spin_locked!(&(*group).notification_lock);
    if !fanotify_is_hashed_event((*event).mask) { return; }
    hlist_add_head!(&mut (*event).merge_list, &mut (*group).fanotify_data.merge_hash[bucket as usize]);
}
unsafe fn fanotify_free_group_priv(group: *mut fsnotify_group) {
    put_user_ns((*group).user_ns); kfree((*group).fanotify_data.merge_hash);
    if !(*group).fanotify_data.ucounts.is_null() { dec_ucount((*group).fanotify_data.ucounts, UCOUNT_FANOTIFY_GROUPS); }
    if mempool_initialized!(&(*group).fanotify_data.error_events_pool) { mempool_exit(&mut (*group).fanotify_data.error_events_pool); }
}
unsafe fn fanotify_free_path_event(e: *mut fanotify_event) { path_put(fanotify_event_path(e)); kmem_cache_free(fanotify_path_event_cachep, FANOTIFY_PE(e)); }
unsafe fn fanotify_free_perm_event(e: *mut fanotify_event) { path_put(fanotify_event_path(e)); kmem_cache_free(fanotify_perm_event_cachep, FANOTIFY_PERM(e)); }
unsafe fn fanotify_free_fid_event(e: *mut fanotify_event) { let f = FANOTIFY_FE(e); if fanotify_fh_has_ext_buf(&mut (*f).object_fh) { kfree(fanotify_fh_ext_buf(&mut (*f).object_fh)); } kmem_cache_free(fanotify_fid_event_cachep, f); }
unsafe fn fanotify_free_name_event(e: *mut fanotify_event) { kfree(FANOTIFY_NE(e)); }
unsafe fn fanotify_free_error_event(group: *mut fsnotify_group, e: *mut fanotify_event) { mempool_free(FANOTIFY_EE(e), &mut (*group).fanotify_data.error_events_pool); }
unsafe fn fanotify_free_mnt_event(e: *mut fanotify_event) { kmem_cache_free(fanotify_mnt_event_cachep, FANOTIFY_ME(e)); }
unsafe fn fanotify_freeing_mark(mark: *mut fsnotify_mark, group: *mut fsnotify_group) { if !FAN_GROUP_FLAG(group, FAN_UNLIMITED_MARKS) { dec_ucount((*group).fanotify_data.ucounts, UCOUNT_FANOTIFY_MARKS); } }
unsafe fn fanotify_free_mark(mark: *mut fsnotify_mark) { kmem_cache_free(fanotify_mark_cache, FANOTIFY_MARK(mark)); }

#[repr(C)]
pub struct fsnotify_ops {
    pub handle_event: unsafe fn(*mut fsnotify_group, u32, *const core::ffi::c_void, i32, *mut inode, *const qstr, u32, *mut fsnotify_iter_info) -> i32,
    pub free_group_priv: unsafe fn(*mut fsnotify_group),
    pub free_event: unsafe fn(*mut fsnotify_group, *mut fsnotify_event),
    pub freeing_mark: unsafe fn(*mut fsnotify_mark, *mut fsnotify_group),
    pub free_mark: unsafe fn(*mut fsnotify_mark),
}
#[no_mangle]
pub static fanotify_fsnotify_ops: fsnotify_ops = fsnotify_ops {
    handle_event: fanotify_handle_event,
    free_group_priv: fanotify_free_group_priv,
    free_event: fanotify_free_event,
    freeing_mark: fanotify_freeing_mark,
    free_mark: fanotify_free_mark,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
