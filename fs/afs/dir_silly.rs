// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS silly rename handling
 *
 * Copyright (C) 2019 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 * - Derived from NFS's sillyrename.
 */

// Kernel and project dependencies supplied externally.

unsafe fn afs_silly_rename_success(op: *mut afs_operation) {
    _enter!("op=%08x", (*op).debug_id);

    afs_check_dir_conflict(op, &mut (*op).file[0]);
    afs_vnode_commit_status(op, &mut (*op).file[0]);
}

unsafe fn afs_silly_rename_edit_dir(op: *mut afs_operation) {
    let dvp = &mut (*op).file[0];
    let dvnode = dvp.vnode;
    let vnode = AFS_FS_I(d_inode((*op).dentry));
    let old = (*op).dentry;
    let new = (*op).dentry_2;

    spin_lock(&mut (*old).d_lock);
    (*old).d_flags |= DCACHE_NFSFS_RENAMED;
    spin_unlock(&mut (*old).d_lock);
    if (*dvnode).silly_key != (*op).key {
        key_put((*dvnode).silly_key);
        (*dvnode).silly_key = key_get((*op).key);
    }

    down_write(&mut (*dvnode).validate_lock);
    if test_bit(AFS_VNODE_DIR_VALID, &(*dvnode).flags)
        && (*dvnode).status.data_version == dvp.dv_before + dvp.dv_delta
    {
        afs_edit_dir_remove(dvnode, &(*old).d_name, afs_edit_dir_for_silly_0);
        afs_edit_dir_add(
            dvnode,
            &(*new).d_name,
            &(*vnode).fid,
            afs_edit_dir_for_silly_1,
        );
    }
    up_write(&mut (*dvnode).validate_lock);
}

static afs_silly_rename_operation: afs_operation_ops = afs_operation_ops {
    issue_afs_rpc: afs_fs_rename,
    issue_yfs_rpc: yfs_fs_rename,
    success: afs_silly_rename_success,
    edit_dir: afs_silly_rename_edit_dir,
};

/*
 * Actually perform the silly rename step.
 */
unsafe fn afs_do_silly_rename(
    dvnode: *mut afs_vnode,
    vnode: *mut afs_vnode,
    old: *mut dentry,
    new: *mut dentry,
    key: *mut key,
) -> c_int {
    _enter!("%pd,%pd", old, new);

    let op = afs_alloc_operation(key, (*dvnode).volume);
    if IS_ERR(op) {
        return PTR_ERR(op);
    }

    (*op).more_files = kvzalloc_objs::<afs_vnode_param>(2);
    if (*op).more_files.is_null() {
        afs_put_operation(op);
        return -ENOMEM;
    }

    afs_op_set_vnode(op, 0, dvnode);
    afs_op_set_vnode(op, 1, dvnode);
    (*op).file[0].dv_delta = 1;
    (*op).file[1].dv_delta = 1;
    (*op).file[0].modification = true;
    (*op).file[1].modification = true;
    (*op).file[0].update_ctime = true;
    (*op).file[1].update_ctime = true;
    (*op).more_files[0].vnode = AFS_FS_I(d_inode(old));
    (*op).more_files[0].speculative = true;
    (*op).more_files[1].vnode = AFS_FS_I(d_inode(new));
    (*op).more_files[1].speculative = true;
    (*op).nr_files = 4;

    (*op).dentry = old;
    (*op).dentry_2 = new;
    (*op).ops = &afs_silly_rename_operation;

    trace_afs_silly_rename(vnode, false);
    afs_do_sync_operation(op)
}

/*
 * Perform silly-rename of a dentry.
 *
 * AFS is stateless and the server doesn't know when the client is holding a
 * file open.  To prevent application problems when a file is unlinked while
 * it's still open, the client performs a "silly-rename".  That is, it renames
 * the file to a hidden file in the same directory, and only performs the
 * unlink once the last reference to it is put.
 *
 * The final cleanup is done during dentry_iput.
 */
unsafe fn afs_sillyrename(
    dvnode: *mut afs_vnode,
    vnode: *mut afs_vnode,
    dentry: *mut dentry,
    key: *mut key,
) -> c_int {
    static mut SILLYCOUNTER: c_uint = 0;
    let mut sdentry: *mut dentry = core::ptr::null_mut();
    let mut silly = [0u8; 16];
    let mut ret = -EBUSY;

    _enter!("");

    /* We don't allow a dentry to be silly-renamed twice. */
    if (*dentry).d_flags & DCACHE_NFSFS_RENAMED != 0 {
        return -EBUSY;
    }

    sdentry = core::ptr::null_mut();
    loop {
        dput(sdentry);
        SILLYCOUNTER = SILLYCOUNTER.wrapping_add(1);

        /* Create a silly name.  Note that the "..__afs" prefix is
         * understood by the salvager and must not be changed.
         */
        scnprintf(silly.as_mut_ptr(), silly.len(), ".__afs%04X", SILLYCOUNTER);
        sdentry = lookup_noperm(&QSTR(silly), (*dentry).d_parent);

        /* N.B. Better to return EBUSY here ... it could be dangerous
         * to delete the file while it's in use.
         */
        if IS_ERR(sdentry) {
            break;
        }
        if d_is_negative(sdentry) == 0 {
            break;
        }
    }

    if IS_ERR(sdentry) {
        _leave!(" = %d", ret);
        return ret;
    }

    ihold(&mut (*vnode).netfs.inode);

    ret = afs_do_silly_rename(dvnode, vnode, dentry, sdentry, key);
    match ret {
        0 => {
            /* The rename succeeded. */
            set_bit(AFS_VNODE_SILLY_DELETED, &mut (*vnode).flags);
            d_move(dentry, sdentry);
        }
        -ERESTARTSYS => {
            /* The result of the rename is unknown. Play it safe by forcing
             * a new lookup.
             */
            d_drop(dentry);
            d_drop(sdentry);
        }
        _ => {}
    }

    iput(&mut (*vnode).netfs.inode);
    dput(sdentry);
    _leave!(" = %d", ret);
    ret
}

unsafe fn afs_silly_unlink_success(op: *mut afs_operation) {
    _enter!("op=%08x", (*op).debug_id);
    afs_check_dir_conflict(op, &mut (*op).file[0]);
    afs_vnode_commit_status(op, &mut (*op).file[0]);
    afs_vnode_commit_status(op, &mut (*op).file[1]);
    afs_update_dentry_version(op, &mut (*op).file[0], (*op).dentry);
}

unsafe fn afs_silly_unlink_edit_dir(op: *mut afs_operation) {
    let dvp = &mut (*op).file[0];
    let dvnode = dvp.vnode;

    _enter!("op=%08x", (*op).debug_id);
    down_write(&mut (*dvnode).validate_lock);
    if test_bit(AFS_VNODE_DIR_VALID, &(*dvnode).flags)
        && (*dvnode).status.data_version == dvp.dv_before + dvp.dv_delta
    {
        afs_edit_dir_remove(dvnode, &(*(*op).dentry).d_name, afs_edit_dir_for_unlink);
    }
    up_write(&mut (*dvnode).validate_lock);
}

static afs_silly_unlink_operation: afs_operation_ops = afs_operation_ops {
    issue_afs_rpc: afs_fs_remove_file,
    issue_yfs_rpc: yfs_fs_remove_file,
    success: afs_silly_unlink_success,
    aborted: afs_check_for_remote_deletion,
    edit_dir: afs_silly_unlink_edit_dir,
};

/*
 * Tell the server to remove a sillyrename file.
 */
unsafe fn afs_do_silly_unlink(
    dvnode: *mut afs_vnode,
    vnode: *mut afs_vnode,
    dentry: *mut dentry,
    _key: *mut key,
) -> c_int {
    _enter!("");

    let op = afs_alloc_operation(core::ptr::null_mut(), (*dvnode).volume);
    if IS_ERR(op) {
        return PTR_ERR(op);
    }

    afs_op_set_vnode(op, 0, dvnode);
    afs_op_set_vnode(op, 1, vnode);
    (*op).file[0].dv_delta = 1;
    (*op).file[0].modification = true;
    (*op).file[0].update_ctime = true;
    (*op).file[1].op_unlinked = true;
    (*op).file[1].update_ctime = true;

    (*op).dentry = dentry;
    (*op).ops = &afs_silly_unlink_operation;

    trace_afs_silly_rename(vnode, true);
    afs_begin_vnode_operation(op);
    afs_wait_for_operation(op);

    /* If there was a conflict with a third party, check the status of the
     * unlinked vnode.
     */
    if (*op).cumul_error.error == 0 && ((*op).flags & AFS_OPERATION_DIR_CONFLICT) != 0 {
        (*op).file[1].update_ctime = false;
        (*op).fetch_status.which = 1;
        (*op).ops = &afs_fetch_status_operation;
        afs_begin_vnode_operation(op);
        afs_wait_for_operation(op);
    }

    afs_put_operation(op)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
