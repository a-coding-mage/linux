// SPDX-License-Identifier: GPL-2.0-or-later
/* FS-Cache interface to CacheFiles
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel/cachefiles translation unit.

static mut cachefiles_object_debug_id: atomic_t = atomic_t { counter: 0 };

/*
 * Allocate a cache object record.
 */
unsafe fn cachefiles_alloc_object(cookie: *mut fscache_cookie) -> *mut cachefiles_object {
    let vcookie = (*cookie).volume;
    let volume = (*vcookie).cache_priv;
    let object: *mut cachefiles_object;

    _enter!("{%s},%x,", (*vcookie).key, (*cookie).debug_id);

    object = kmem_cache_zalloc(cachefiles_object_jar, GFP_KERNEL);
    if object.is_null() {
        return core::ptr::null_mut();
    }

    refcount_set(&mut (*object).ref_, 1);
    spin_lock_init(&mut (*object).lock);
    INIT_LIST_HEAD(&mut (*object).cache_link);
    (*object).volume = volume;
    (*object).debug_id = atomic_inc_return(&mut cachefiles_object_debug_id);
    (*object).cookie = fscache_get_cookie(cookie, fscache_cookie_get_attach_object);

    fscache_count_object((*vcookie).cache);
    trace_cachefiles_ref((*object).debug_id, (*cookie).debug_id, 1,
                         cachefiles_obj_new);
    object
}

/*
 * Note that an object has been seen.
 */
unsafe fn cachefiles_see_object(object: *mut cachefiles_object,
                                why: cachefiles_obj_ref_trace) {
    trace_cachefiles_ref((*object).debug_id, (*(*object).cookie).debug_id,
                         refcount_read(&(*object).ref_), why);
}

/*
 * Increment the usage count on an object;
 */
unsafe fn cachefiles_grab_object(object: *mut cachefiles_object,
                                 why: cachefiles_obj_ref_trace) -> *mut cachefiles_object {
    let mut r = 0;
    __refcount_inc(&mut (*object).ref_, &mut r);
    trace_cachefiles_ref((*object).debug_id, (*(*object).cookie).debug_id, r, why);
    object
}

/*
 * dispose of a reference to an object
 */
unsafe fn cachefiles_put_object(object: *mut cachefiles_object,
                                why: cachefiles_obj_ref_trace) {
    let object_debug_id = (*object).debug_id;
    let cookie_debug_id = (*(*object).cookie).debug_id;
    let mut cache: *mut fscache_cache;
    let mut r = 0;

    let done = __refcount_dec_and_test(&mut (*object).ref_, &mut r);
    trace_cachefiles_ref(object_debug_id, cookie_debug_id, r, why);
    if done {
        _debug!("- kill object OBJ%x", object_debug_id);
        ASSERTCMP!((*object).file, ==, core::ptr::null_mut());
        kfree((*object).d_name);
        cache = (*(*object).volume).cache->cache;
        fscache_put_cookie((*object).cookie, fscache_cookie_put_object);
        (*object).cookie = core::ptr::null_mut();
        kmem_cache_free(cachefiles_object_jar, object);
        fscache_uncount_object(cache);
    }
    _leave!("");
}

/*
 * Adjust the size of a cache file if necessary to match the DIO size.  We keep
 * the EOF marker a multiple of DIO blocks so that we don't fall back to doing
 * non-DIO for a partial block straddling the EOF, but we also have to be
 * careful of someone expanding the file and accidentally accreting the
 * padding.
 */
unsafe fn cachefiles_adjust_size(object: *mut cachefiles_object) -> i32 {
    let mut newattrs: iattr = core::mem::zeroed();
    let file = (*object).file;
    let mut ni_size = round_up((*(*object).cookie).object_size, CACHEFILES_DIO_BLOCK_SIZE);
    let oi_size: loff_t;
    let mut ret: i32;

    _enter!("{OBJ%x},[%llu]", (*object).debug_id, ni_size as u64);
    if file.is_null() { return -ENOBUFS; }
    oi_size = i_size_read(file_inode(file));
    if oi_size == ni_size { return 0; }
    inode_lock(file_inode(file));
    if (oi_size & !PAGE_MASK) != 0 && ni_size > oi_size {
        _debug!("discard tail %llx", oi_size);
        newattrs.ia_valid = ATTR_SIZE;
        newattrs.ia_size = oi_size & PAGE_MASK;
        ret = cachefiles_inject_remove_error();
        if ret == 0 { ret = notify_change(&nop_mnt_idmap, (*file).f_path.dentry, &mut newattrs, core::ptr::null_mut()); }
        if ret < 0 { return cachefiles_adjust_size_truncate_failed(file, object, ret); }
    }
    newattrs.ia_valid = ATTR_SIZE;
    newattrs.ia_size = ni_size;
    ret = cachefiles_inject_write_error();
    if ret == 0 { ret = notify_change(&nop_mnt_idmap, (*file).f_path.dentry, &mut newattrs, core::ptr::null_mut()); }
    inode_unlock(file_inode(file));
    if ret < 0 { trace_cachefiles_io_error(core::ptr::null_mut(), file_inode(file), ret, cachefiles_trace_notify_change_error); }
    if ret == -EIO { cachefiles_io_error_obj(object, "Size set failed"); ret = -ENOBUFS; }
    _leave!(" = %d", ret);
    ret
}

unsafe fn cachefiles_adjust_size_truncate_failed(file: *mut file, object: *mut cachefiles_object, ret: i32) -> i32 {
    inode_unlock(file_inode(file));
    if ret < 0 { trace_cachefiles_io_error(core::ptr::null_mut(), file_inode(file), ret, cachefiles_trace_notify_change_error); }
    if ret == -EIO { cachefiles_io_error_obj(object, "Size set failed"); return -ENOBUFS; }
    _leave!(" = %d", ret);
    ret
}

unsafe fn cachefiles_lookup_cookie(cookie: *mut fscache_cookie) -> bool {
    let object = cachefiles_alloc_object(cookie);
    if object.is_null() { return false; }
    if !cachefiles_cook_key(object) { cachefiles_put_object(object, cachefiles_obj_put_alloc_fail); return false; }
    (*cookie).cache_priv = object;
    let cache = (*(*cookie).volume).cache->cache_priv;
    let mut saved_cred: *const cred = core::ptr::null();
    cachefiles_begin_secure(cache, &mut saved_cred);
    if !cachefiles_look_up_object(object) {
        cachefiles_end_secure(cache, saved_cred); cachefiles_see_object(object, cachefiles_obj_see_lookup_failed);
        fscache_caching_failed(cookie); return false;
    }
    cachefiles_see_object(object, cachefiles_obj_see_lookup_cookie);
    spin_lock(&mut (*cache).object_list_lock); list_add(&mut (*object).cache_link, &mut (*cache).object_list); spin_unlock(&mut (*cache).object_list_lock);
    cachefiles_adjust_size(object); cachefiles_end_secure(cache, saved_cred); true
}

unsafe fn cachefiles_shorten_object(object: *mut cachefiles_object, file: *mut file, new_size: loff_t) -> bool {
    let cache = (*(*object).volume).cache; let inode = file_inode(file); let dio_size = round_up(new_size, CACHEFILES_DIO_BLOCK_SIZE); let i_size = i_size_read(inode);
    trace_cachefiles_trunc(object, inode, i_size, dio_size, cachefiles_trunc_shrink);
    let mut ret = cachefiles_inject_remove_error(); if ret == 0 { ret = vfs_truncate(&(*file).f_path, dio_size); }
    if ret < 0 { trace_cachefiles_io_error(object, inode, ret, cachefiles_trace_trunc_error); cachefiles_io_error_obj(object, "Trunc-to-size failed %d", ret); cachefiles_remove_object_xattr(cache, object, (*file).f_path.dentry); return false; }
    if new_size < dio_size { trace_cachefiles_trunc(object, inode, dio_size, new_size, cachefiles_trunc_dio_adjust); ret = cachefiles_inject_write_error(); if ret == 0 { ret = vfs_fallocate(file, FALLOC_FL_ZERO_RANGE, new_size, dio_size - new_size); } if ret < 0 { trace_cachefiles_io_error(object, inode, ret, cachefiles_trace_fallocate_error); cachefiles_io_error_obj(object, "Trunc-to-dio-size failed %d", ret); cachefiles_remove_object_xattr(cache, object, (*file).f_path.dentry); return false; } }
    true
}

unsafe fn cachefiles_resize_cookie(cres: *mut netfs_cache_resources, new_size: loff_t) { let object = cachefiles_cres_object(cres); let cookie = (*object).cookie; let old_size = (*cookie).object_size; if new_size < old_size { let cache = (*(*object).volume).cache; let file = cachefiles_cres_file(cres); let mut c: *const cred = core::ptr::null(); cachefiles_begin_secure(cache, &mut c); cachefiles_shorten_object(object, file, new_size); cachefiles_end_secure(cache, c); (*cookie).object_size = new_size; } else { (*cookie).object_size = new_size; } }
unsafe fn cachefiles_commit_object(object: *mut cachefiles_object, cache: *mut cachefiles_cache) { let mut update = false; if test_and_clear_bit(FSCACHE_COOKIE_LOCAL_WRITE, &mut (*(*object).cookie).flags) { update = true; } if test_and_clear_bit(FSCACHE_COOKIE_NEEDS_UPDATE, &mut (*(*object).cookie).flags) { update = true; } if update { cachefiles_set_object_xattr(object); } if test_bit(CACHEFILES_OBJECT_USING_TMPFILE, &(*object).flags) { cachefiles_commit_tmpfile(cache, object); } }
unsafe fn cachefiles_clean_up_object(object: *mut cachefiles_object, cache: *mut cachefiles_cache) { if test_bit(FSCACHE_COOKIE_RETIRED, &(*(*object).cookie).flags) && !test_bit(CACHEFILES_OBJECT_USING_TMPFILE, &(*object).flags) { cachefiles_delete_object(object, FSCACHE_OBJECT_WAS_RETIRED); } else if !test_bit(FSCACHE_COOKIE_RETIRED, &(*(*object).cookie).flags) { cachefiles_commit_object(object, cache); } cachefiles_unmark_inode_in_use(object, (*object).file); spin_lock(&mut (*object).lock); let file = (*object).file; (*object).file = core::ptr::null_mut(); spin_unlock(&mut (*object).lock); if !file.is_null() { fput(file); } }
unsafe fn cachefiles_withdraw_cookie(cookie: *mut fscache_cookie) { let object = (*cookie).cache_priv; let cache = (*(*object).volume).cache; if !list_empty(&(*object).cache_link) { spin_lock(&mut (*cache).object_list_lock); list_del_init(&mut (*object).cache_link); spin_unlock(&mut (*cache).object_list_lock); } if !(*object).file.is_null() { let mut c: *const cred = core::ptr::null(); cachefiles_begin_secure(cache, &mut c); cachefiles_clean_up_object(object, cache); cachefiles_end_secure(cache, c); } (*cookie).cache_priv = core::ptr::null_mut(); cachefiles_put_object(object, cachefiles_obj_put_detach); }
unsafe fn cachefiles_invalidate_cookie(cookie: *mut fscache_cookie) -> bool { let object = (*cookie).cache_priv; if (*object).file.is_null() { fscache_resume_after_invalidation(cookie); return true; } let new_file = cachefiles_create_tmpfile(object); if IS_ERR(new_file) { return false; } spin_lock(&mut (*object).lock); let old_file = (*object).file; (*object).file = new_file; (*object).content_info = CACHEFILES_CONTENT_NO_DATA; set_bit(CACHEFILES_OBJECT_USING_TMPFILE, &mut (*object).flags); set_bit(FSCACHE_COOKIE_NEEDS_UPDATE, &mut (*(*object).cookie).flags); spin_unlock(&mut (*object).lock); fscache_resume_after_invalidation(cookie); if !old_file.is_null() { fput(old_file); } true }

const cachefiles_cache_ops: fscache_cache_ops = fscache_cache_ops {
    name: b"cachefiles\0".as_ptr() as *const i8,
    acquire_volume: Some(cachefiles_acquire_volume),
    free_volume: Some(cachefiles_free_volume),
    lookup_cookie: Some(cachefiles_lookup_cookie),
    withdraw_cookie: Some(cachefiles_withdraw_cookie),
    invalidate_cookie: Some(cachefiles_invalidate_cookie),
    begin_operation: Some(cachefiles_begin_operation),
    resize_cookie: Some(cachefiles_resize_cookie),
    prepare_to_write: Some(cachefiles_prepare_to_write),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
