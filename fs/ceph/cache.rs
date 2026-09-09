// SPDX-License-Identifier: GPL-2.0-only
/*
 * Ceph cache definitions.
 *
 *  Copyright (C) 2013 by Adfin Solutions, Inc. All Rights Reserved.
 *  Written by Milosz Tanski (milosz@adfin.com)
 */

// Dependencies supplied by the surrounding kernel/Ceph translation unit:
// linux/ceph/ceph_debug.h, linux/fs_context.h, super.h, and cache.h

pub unsafe fn ceph_fscache_register_inode_cookie(inode: *mut inode) {
    let ci: *mut ceph_inode_info = ceph_inode(inode);
    let fsc: *mut ceph_fs_client = ceph_inode_to_fs_client(inode);

    /* No caching for filesystem? */
    if (*fsc).fscache.is_null() {
        return;
    }

    /* Regular files only */
    if !s_isreg((*inode).i_mode) {
        return;
    }

    /* Only new inodes! */
    if inode_state_read_once(inode) & i_new == 0 {
        return;
    }

    warn_on_once(!(*ci).netfs.cache.is_null());

    (*ci).netfs.cache = fscache_acquire_cookie(
        (*fsc).fscache,
        0,
        &(*ci).i_vino as *const _ as *const core::ffi::c_void,
        core::mem::size_of_val(&(*ci).i_vino),
        &(*ci).i_version as *const _ as *const core::ffi::c_void,
        core::mem::size_of_val(&(*ci).i_version),
        i_size_read(inode),
    );
    if !(*ci).netfs.cache.is_null() {
        mapping_set_release_always((*inode).i_mapping);
    }
}

pub unsafe fn ceph_fscache_unregister_inode_cookie(ci: *mut ceph_inode_info) {
    fscache_relinquish_cookie(ceph_fscache_cookie(ci), false);
}

pub unsafe fn ceph_fscache_use_cookie(inode: *mut inode, will_modify: bool) {
    let ci: *mut ceph_inode_info = ceph_inode(inode);

    fscache_use_cookie(ceph_fscache_cookie(ci), will_modify);
}

pub unsafe fn ceph_fscache_unuse_cookie(inode: *mut inode, update: bool) {
    let ci: *mut ceph_inode_info = ceph_inode(inode);

    if update {
        let mut i_size: loff_t = i_size_read(inode);

        fscache_unuse_cookie(
            ceph_fscache_cookie(ci),
            &mut (*ci).i_version,
            &mut i_size,
        );
    } else {
        fscache_unuse_cookie(ceph_fscache_cookie(ci), core::ptr::null_mut(), core::ptr::null_mut());
    }
}

pub unsafe fn ceph_fscache_update(inode: *mut inode) {
    let ci: *mut ceph_inode_info = ceph_inode(inode);
    let mut i_size: loff_t = i_size_read(inode);

    fscache_update_cookie(ceph_fscache_cookie(ci), &mut (*ci).i_version, &mut i_size);
}

pub unsafe fn ceph_fscache_invalidate(inode: *mut inode, dio_write: bool) {
    let ci: *mut ceph_inode_info = ceph_inode(inode);

    fscache_invalidate(
        ceph_fscache_cookie(ci),
        &mut (*ci).i_version,
        i_size_read(inode),
        if dio_write { fscache_inval_dio_write } else { 0 },
    );
}

pub unsafe fn ceph_fscache_register_fs(
    fsc: *mut ceph_fs_client,
    fc: *mut fs_context,
) -> i32 {
    let fsid: *const ceph_fsid = &(*(*fsc).client).fsid;
    let fscache_uniq: *const core::ffi::c_char = (*(*fsc).mount_options).fscache_uniq;
    let uniq_len: usize = if !fscache_uniq.is_null() {
        strlen(fscache_uniq)
    } else {
        0
    };
    let mut name: *mut core::ffi::c_char;
    let mut err: i32 = 0;

    name = kasprintf(
        gfp_kernel,
        c"ceph,%pU%s%s".as_ptr(),
        fsid,
        if uniq_len != 0 { c",".as_ptr() } else { c"".as_ptr() },
        if uniq_len != 0 { fscache_uniq } else { c"".as_ptr() },
    );
    if name.is_null() {
        return -enomem;
    }

    (*fsc).fscache = fscache_acquire_volume(name, core::ptr::null(), core::ptr::null(), 0);
    if is_err_or_null((*fsc).fscache) {
        errorfc(fc, c"Unable to register fscache cookie for %s".as_ptr(), name);
        err = if !(*fsc).fscache.is_null() {
            ptr_err((*fsc).fscache)
        } else {
            -eopnotsupp
        };
        (*fsc).fscache = core::ptr::null_mut();
    }
    kfree(name as *mut core::ffi::c_void);
    err
}

pub unsafe fn ceph_fscache_unregister_fs(fsc: *mut ceph_fs_client) {
    fscache_relinquish_volume((*fsc).fscache, core::ptr::null_mut(), false);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
