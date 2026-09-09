// SPDX-License-Identifier: GPL-2.0-or-later
/* Manage high-level VFS aspects of a cache.
 *
 * Copyright (C) 2007, 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel dependencies are supplied by the surrounding translation unit.

pub unsafe fn cachefiles_add_cache(cache: *mut cachefiles_cache) -> i32 {
    let mut cache_cookie: *mut fscache_cache;
    let mut path: path;
    let mut stats: kstatfs;
    let mut graveyard: *mut dentry;
    let mut cachedir: *mut dentry;
    let mut root: *mut dentry;
    let mut saved_cred: *const cred;
    let mut ret: i32;

    _enter!("");
    cache_cookie = fscache_acquire_cache((*cache).tag);
    if IS_ERR!(cache_cookie) { return PTR_ERR!(cache_cookie); }
    ret = cachefiles_get_security_ID(cache);
    if ret < 0 { goto!(error_getsec); }
    cachefiles_begin_secure(cache, &mut saved_cred);
    ret = kern_path((*cache).rootdirname, LOOKUP_DIRECTORY, &mut path);
    if ret < 0 { goto!(error_open_root); }
    (*cache).mnt = path.mnt;
    root = path.dentry;
    ret = -EINVAL;
    if is_idmapped_mnt(path.mnt) {
        pr_warn!("File cache on idmapped mounts not supported");
        goto!(error_unsupported);
    }
    ret = -EOPNOTSUPP;
    if d_is_negative(root) || (*d_backing_inode(root)).i_op.lookup.is_none() ||
       (*d_backing_inode(root)).i_op.mkdir.is_none() ||
       (*d_backing_inode(root)).i_op.tmpfile.is_none() ||
       ((*d_backing_inode(root)).i_opflags & IOP_XATTR) == 0 ||
       (*(*root).d_sb).s_op.statfs.is_none() ||
       (*(*root).d_sb).s_op.sync_fs.is_none() || (*(*root).d_sb).s_blocksize > PAGE_SIZE {
        goto!(error_unsupported);
    }
    ret = -EROFS;
    if sb_rdonly((*root).d_sb) { goto!(error_unsupported); }
    ret = cachefiles_determine_cache_security(cache, root, &mut saved_cred);
    if ret < 0 { goto!(error_unsupported); }
    ret = vfs_statfs(&path, &mut stats);
    if ret < 0 { goto!(error_unsupported); }
    ret = -ERANGE;
    if stats.f_bsize <= 0 { goto!(error_unsupported); }
    ret = -EOPNOTSUPP;
    if stats.f_bsize > PAGE_SIZE { goto!(error_unsupported); }
    (*cache).bsize = stats.f_bsize;
    (*cache).bshift = ilog2(stats.f_bsize);
    _debug!("blksize {} (shift {})", (*cache).bsize, (*cache).bshift);
    _debug!("size {}, avail {}", stats.f_blocks, stats.f_bavail);
    stats.f_files /= 100;
    (*cache).fstop = stats.f_files * (*cache).fstop_percent;
    (*cache).fcull = stats.f_files * (*cache).fcull_percent;
    (*cache).frun = stats.f_files * (*cache).frun_percent;
    _debug!("limits {{{},{},{}}} files", (*cache).frun, (*cache).fcull, (*cache).fstop);
    stats.f_blocks /= 100;
    (*cache).bstop = stats.f_blocks * (*cache).bstop_percent;
    (*cache).bcull = stats.f_blocks * (*cache).bcull_percent;
    (*cache).brun = stats.f_blocks * (*cache).brun_percent;
    _debug!("limits {{{},{},{}}} blocks", (*cache).brun, (*cache).bcull, (*cache).bstop);
    cachedir = cachefiles_get_directory(cache, root, "cache".as_ptr() as *const i8, core::ptr::null_mut());
    if IS_ERR!(cachedir) { ret = PTR_ERR!(cachedir); goto!(error_unsupported); }
    (*cache).store = cachedir;
    graveyard = cachefiles_get_directory(cache, root, "graveyard".as_ptr() as *const i8, core::ptr::null_mut());
    if IS_ERR!(graveyard) { ret = PTR_ERR!(graveyard); goto!(error_unsupported); }
    (*cache).graveyard = graveyard;
    (*cache).cache = cache_cookie;
    ret = fscache_add_cache(cache_cookie, &cachefiles_cache_ops, cache);
    if ret < 0 { goto!(error_add_cache); }
    set_bit!(CACHEFILES_READY, &mut (*cache).flags);
    dput(root);
    pr_info!("File cache on {} registered\n", (*cache_cookie).name);
    cachefiles_has_space(cache, 0, 0, cachefiles_has_space_check);
    cachefiles_end_secure(cache, saved_cred);
    _leave!(" = 0 [%px]", (*cache).cache);
    return 0;

error_add_cache:
    cachefiles_put_directory((*cache).graveyard); (*cache).graveyard = core::ptr::null_mut();
error_unsupported:
    cachefiles_put_directory((*cache).store); (*cache).store = core::ptr::null_mut();
    mntput((*cache).mnt); (*cache).mnt = core::ptr::null_mut(); dput(root);
error_open_root:
    cachefiles_end_secure(cache, saved_cred); put_cred((*cache).cache_cred);
    (*cache).cache_cred = core::ptr::null();
error_getsec:
    fscache_relinquish_cache(cache_cookie); (*cache).cache = core::ptr::null_mut();
    pr_err!("Failed to register: {}\n", ret); ret
}

pub unsafe fn cachefiles_has_space(cache: *mut cachefiles_cache, fnr: u32, bnr: u32,
                                   reason: cachefiles_has_space_for) -> i32 {
    let mut stats: kstatfs = core::mem::zeroed();
    let mut b_avail: u64;
    let b_writing: u64;
    let path = path { mnt: (*cache).mnt, dentry: (*(*cache).mnt).mnt_root };
    let mut ret = vfs_statfs(&path, &mut stats);
    if ret < 0 { trace_cachefiles_vfs_error(core::ptr::null_mut(), d_inode(path.dentry), ret, cachefiles_trace_statfs_error); if ret == -EIO { cachefiles_io_error(cache, "statfs failed"); } _leave!(" = {}", ret); return ret; }
    b_avail = stats.f_bavail; b_writing = atomic_long_read(&(*cache).b_writing);
    b_avail = b_avail.saturating_sub(b_writing);
    stats.f_ffree = stats.f_ffree.saturating_sub(fnr as _); b_avail = b_avail.saturating_sub(bnr as _);
    ret = -ENOBUFS;
    if stats.f_ffree < (*cache).fstop || b_avail < (*cache).bstop {
        match reason { cachefiles_has_space_for_write => fscache_count_no_write_space(), cachefiles_has_space_for_create => fscache_count_no_create_space(), _ => {} }
        if !test_and_set_bit!(CACHEFILES_CULLING, &mut (*cache).flags) { _debug!("### CULL CACHE ###"); cachefiles_state_changed(cache); }
        _leave!(" = {}", ret); return ret;
    }
    ret = 0;
    if stats.f_ffree < (*cache).fcull || b_avail < (*cache).bcull {
        if !test_and_set_bit!(CACHEFILES_CULLING, &mut (*cache).flags) { _debug!("### CULL CACHE ###"); cachefiles_state_changed(cache); }
        _leave!(" = {}", ret); return ret;
    }
    if test_bit!(CACHEFILES_CULLING, &(*cache).flags) && stats.f_ffree >= (*cache).frun && b_avail >= (*cache).brun && test_and_clear_bit!(CACHEFILES_CULLING, &mut (*cache).flags) { _debug!("cease culling"); cachefiles_state_changed(cache); }
    ret
}

unsafe fn cachefiles_withdraw_objects(cache: *mut cachefiles_cache) {
    let mut count: u32 = 0; _enter!(""); spin_lock(&mut (*cache).object_list_lock);
    while !list_empty(&(*cache).object_list) { let object = list_first_entry!(&(*cache).object_list, cachefiles_object, cache_link); cachefiles_see_object(object, cachefiles_obj_see_withdrawal); list_del_init(&mut (*object).cache_link); fscache_withdraw_cookie((*object).cookie); count += 1; if count & 63 == 0 { spin_unlock(&mut (*cache).object_list_lock); cond_resched(); spin_lock(&mut (*cache).object_list_lock); } }
    spin_unlock(&mut (*cache).object_list_lock); _leave!(" [{} objs]", count);
}

unsafe fn cachefiles_withdraw_fscache_volumes(cache: *mut cachefiles_cache) {
    _enter!("");
    'retry: loop { spin_lock(&mut (*cache).object_list_lock); let mut cur = (*cache).volumes.next; while cur != &mut (*cache).volumes { let volume = list_entry!(cur, cachefiles_volume, cache_link); if atomic_read(&(*(*volume).vcookie).n_accesses) != 0 { let vcookie = fscache_try_get_volume((*volume).vcookie, fscache_volume_get_withdraw); if !vcookie.is_null() { spin_unlock(&mut (*cache).object_list_lock); fscache_withdraw_volume(vcookie); fscache_put_volume(vcookie, fscache_volume_put_withdraw); continue 'retry; } } cur = (*cur).next; } spin_unlock(&mut (*cache).object_list_lock); break; } _leave!("");
}

unsafe fn cachefiles_withdraw_volumes(cache: *mut cachefiles_cache) {
    _enter!(""); loop { spin_lock(&mut (*cache).object_list_lock); if !list_empty(&(*cache).volumes) { let volume = list_first_entry!(&(*cache).volumes, cachefiles_volume, cache_link); let vcookie = fscache_try_get_volume((*volume).vcookie, fscache_volume_get_withdraw); if vcookie.is_null() { spin_unlock(&mut (*cache).object_list_lock); cpu_relax(); continue; } list_del_init(&mut (*volume).cache_link); spin_unlock(&mut (*cache).object_list_lock); cachefiles_withdraw_volume(volume); fscache_put_volume(vcookie, fscache_volume_put_withdraw); } else { spin_unlock(&mut (*cache).object_list_lock); break; } } _leave!("");
}

unsafe fn cachefiles_sync_cache(cache: *mut cachefiles_cache) {
    let mut saved_cred: *const cred; _enter!("{}", (*(*cache).cache).name); cachefiles_begin_secure(cache, &mut saved_cred); down_read(&mut (*(*(*cache).mnt).mnt_sb).s_umount); let ret = sync_filesystem((*cache).mnt); up_read(&mut (*(*(*cache).mnt).mnt_sb).s_umount); cachefiles_end_secure(cache, saved_cred); if ret == -EIO { cachefiles_io_error(cache, "Attempt to sync backing fs superblock returned error %d", ret); }
}

pub unsafe fn cachefiles_withdraw_cache(cache: *mut cachefiles_cache) {
    let fscache = (*cache).cache; pr_info!("File cache on {} unregistering\n", (*fscache).name); fscache_withdraw_cache(fscache); cachefiles_withdraw_fscache_volumes(cache); cachefiles_withdraw_objects(cache); fscache_wait_for_objects(fscache); cachefiles_withdraw_volumes(cache); cachefiles_sync_cache(cache); (*cache).cache = core::ptr::null_mut(); fscache_relinquish_cache(fscache);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
