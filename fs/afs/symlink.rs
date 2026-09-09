// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS filesystem symbolic link handling
 *
 * Copyright (C) 2026 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel and AFS project dependencies supplied by the surrounding translation.

unsafe fn afs_put_symlink(symlink: *mut afs_symlink) {
    if refcount_dec_and_test(&mut (*symlink).ref_) {
        kfree_rcu(symlink, rcu);
    }
}

unsafe fn afs_replace_symlink(vnode: *mut afs_vnode, symlink: *mut afs_symlink) {
    let old: *mut afs_symlink = rcu_replace_pointer(
        &mut (*vnode).symlink,
        symlink,
        lockdep_is_held(&(*vnode).validate_lock),
    );
    if !old.is_null() {
        afs_put_symlink(old);
    }
}

/*
 * In the event that a third-party update of a symlink occurs, dispose of the
 * copy of the old contents.  Called under ->validate_lock.
 */
pub unsafe fn afs_invalidate_symlink(vnode: *mut afs_vnode) {
    afs_replace_symlink(vnode, core::ptr::null_mut());
}

/*
 * Dispose of a symlink copy during inode deletion.
 */
pub unsafe fn afs_evict_symlink(vnode: *mut afs_vnode) {
    let old: *mut afs_symlink =
        rcu_replace_pointer(&mut (*vnode).symlink, core::ptr::null_mut(), true);
    if !old.is_null() {
        afs_put_symlink(old);
    }
}

/*
 * Set up a locally created symlink inode for immediate write to the cache.
 */
pub unsafe fn afs_init_new_symlink(vnode: *mut afs_vnode, op: *mut afs_operation) {
    let symlink = (*op).create.symlink;
    let mut dsize: usize = 0;
    let size = strlen((*symlink).content) + 1;
    let p: *mut core::ffi::c_char;

    rcu_assign_pointer(&mut (*vnode).symlink, symlink);
    (*op).create.symlink = core::ptr::null_mut();

    if !fscache_cookie_enabled(netfs_i_cookie(&(*vnode).netfs)) {
        return;
    }

    if netfs_alloc_folioq_buffer(
        core::ptr::null_mut(),
        &mut (*vnode).directory,
        &mut dsize,
        size,
        mapping_gfp_mask((*vnode).netfs.inode.i_mapping),
    ) < 0 {
        return;
    }

    (*vnode).directory_size = dsize;
    p = kmap_local_folio(folioq_folio((*vnode).directory, 0), 0);
    memcpy(p, (*symlink).content, size);
    kunmap_local(p);
    netfs_single_mark_inode_dirty(&mut (*vnode).netfs.inode);
}

/* Read a symlink in a single download. */
unsafe fn afs_do_read_symlink(vnode: *mut afs_vnode) -> isize {
    let mut symlink: *mut afs_symlink;
    let mut iter: iov_iter;
    let mut ret: isize;
    let mut i_size: i64;

    i_size = i_size_read(&(*vnode).netfs.inode);
    if i_size > (PAGE_SIZE - 1) as i64 {
        trace_afs_file_error(vnode, -EFBIG, afs_file_error_dir_big);
        return -EFBIG as isize;
    }

    if (*vnode).directory.is_null() {
        let mut cur_size: usize = 0;
        ret = netfs_alloc_folioq_buffer(
            core::ptr::null_mut(),
            &mut (*vnode).directory,
            &mut cur_size,
            PAGE_SIZE,
            mapping_gfp_mask((*vnode).netfs.inode.i_mapping),
        );
        (*vnode).directory_size = PAGE_SIZE - 1;
        if ret < 0 {
            return ret;
        }
    }

    iov_iter_folio_queue(&mut iter, ITER_DEST, (*vnode).directory, 0, 0, PAGE_SIZE);
    ret = netfs_read_single(&mut (*vnode).netfs.inode, core::ptr::null_mut(), &mut iter);
    if ret >= 0 {
        i_size = ret as i64;
        if i_size > (PAGE_SIZE - 1) as i64 {
            trace_afs_file_error(vnode, -EFBIG, afs_file_error_dir_big);
            return -EFBIG as isize;
        }
        (*vnode).directory_size = i_size as usize;

        symlink = kmalloc_flex::<afs_symlink>(i_size as usize + 1, GFP_KERNEL);
        if symlink.is_null() {
            return -ENOMEM as isize;
        }
        refcount_set(&mut (*symlink).ref_, 1);
        (*symlink).content[i_size as usize] = 0;

        let s = kmap_local_folio(folioq_folio((*vnode).directory, 0), 0);
        memcpy((*symlink).content.as_mut_ptr(), s, i_size as usize);
        kunmap_local(s);
        afs_replace_symlink(vnode, symlink);
    }

    if !fscache_cookie_enabled(netfs_i_cookie(&(*vnode).netfs)) {
        netfs_free_folioq_buffer((*vnode).directory);
        (*vnode).directory = core::ptr::null_mut();
        (*vnode).directory_size = 0;
    }
    ret
}

unsafe fn afs_read_symlink(vnode: *mut afs_vnode) -> isize {
    fscache_use_cookie(afs_vnode_cache(vnode), false);
    let ret = afs_do_read_symlink(vnode);
    fscache_unuse_cookie(afs_vnode_cache(vnode), core::ptr::null_mut(), core::ptr::null_mut());
    ret
}

unsafe extern "C" fn afs_put_link(arg: *mut core::ffi::c_void) {
    afs_put_symlink(arg as *mut afs_symlink);
}

pub unsafe fn afs_get_link(
    dentry: *mut dentry,
    inode: *mut inode,
    callback: *mut delayed_call,
) -> *const core::ffi::c_char {
    let mut symlink: *mut afs_symlink;
    let vnode = AFS_FS_I(inode);
    let mut ret: isize;

    if dentry.is_null() {
        symlink = rcu_dereference((*vnode).symlink);
        if symlink.is_null() || !afs_check_validity(vnode) {
            return ERR_PTR(-ECHILD);
        }
        set_delayed_call(callback, None, core::ptr::null_mut());
        return (*symlink).content.as_ptr();
    }

    if !(*vnode).symlink.is_null() {
        ret = afs_validate(vnode, core::ptr::null_mut());
        if ret < 0 { return ERR_PTR(ret); }
        down_read(&(*vnode).validate_lock);
        if !(*vnode).symlink.is_null() { return afs_get_link_good(vnode, callback); }
        up_read(&(*vnode).validate_lock);
    }

    if down_write_killable(&(*vnode).validate_lock) < 0 { return ERR_PTR(-ERESTARTSYS); }
    if (*vnode).symlink.is_null() {
        ret = afs_read_symlink(vnode);
        if ret < 0 {
            up_write(&(*vnode).validate_lock);
            return ERR_PTR(ret);
        }
    }
    downgrade_write(&(*vnode).validate_lock);
    afs_get_link_good(vnode, callback)
}

unsafe fn afs_get_link_good(vnode: *mut afs_vnode, callback: *mut delayed_call) -> *const core::ffi::c_char {
    let symlink = rcu_dereference_protected((*vnode).symlink, lockdep_is_held(&(*vnode).validate_lock));
    refcount_inc(&mut (*symlink).ref_);
    up_read(&(*vnode).validate_lock);
    set_delayed_call(callback, Some(afs_put_link), symlink as *mut core::ffi::c_void);
    (*symlink).content.as_ptr()
}

pub unsafe fn afs_readlink(dentry: *mut dentry, buffer: *mut core::ffi::c_void, buflen: i32) -> i32 {
    let mut done = DELAYED_CALL_INIT;
    let content = afs_get_link(dentry, d_inode(dentry), &mut done);
    if IS_ERR(content) {
        do_delayed_call(&mut done);
        return PTR_ERR(content) as i32;
    }
    let mut len = umin(strlen(content), buflen as usize) as i32;
    if copy_to_user(buffer, content, len as usize) != 0 { len = -EFAULT; }
    do_delayed_call(&mut done);
    len
}

/* Write the symlink contents to the cache as a single blob. */
pub unsafe fn afs_symlink_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32 {
    let vnode = AFS_FS_I((*mapping).host);
    let mut iter: iov_iter;
    let mut ret: i32 = 0;
    if !down_read_trylock(&(*vnode).validate_lock) {
        if (*wbc).sync_mode == WB_SYNC_NONE {
            netfs_single_mark_inode_dirty(&mut (*vnode).netfs.inode);
            return 0;
        }
        down_read(&(*vnode).validate_lock);
    }
    if !(*vnode).directory.is_null() && atomic64_read(&(*vnode).cb_expires_at) != AFS_NO_CB_PROMISE {
        iov_iter_folio_queue(&mut iter, ITER_SOURCE, (*vnode).directory, 0, 0, i_size_read(&(*vnode).netfs.inode) as usize);
        ret = netfs_writeback_single(mapping, wbc, &mut iter);
    }
    if ret == 0 {
        netfs_wb_begin(&mut (*vnode).netfs, false);
        netfs_free_folioq_buffer((*vnode).directory);
        (*vnode).directory = core::ptr::null_mut();
        (*vnode).directory_size = 0;
        netfs_wb_end(&mut (*vnode).netfs);
    } else if ret == 1 { ret = 0; }
    up_read(&(*vnode).validate_lock);
    ret
}

pub static afs_symlink_inode_operations: inode_operations = inode_operations {
    get_link: Some(afs_get_link),
    readlink: Some(afs_readlink),
};

pub static afs_symlink_aops: address_space_operations = address_space_operations {
    writepages: Some(afs_symlink_writepages),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
