// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) International Business Machines Corp., 2000-2002
 *   Portions Copyright (C) Christoph Hellwig, 2001-2002
 */

// Linux and JFS headers supply the types, constants, macros, and functions
// referenced below.

pub unsafe fn jfs_fsync(
    file: *mut file,
    start: loff_t,
    end: loff_t,
    datasync: c_int,
) -> c_int {
    let inode = (*(*file).f_mapping).host;
    let mut rc: c_int = 0;

    rc = file_write_and_wait_range(file, start, end);
    if rc != 0 {
        return rc;
    }

    inode_lock(inode);
    if (inode_state_read_once(inode) & I_DIRTY_ALL) == 0
        || (datasync != 0 && (inode_state_read_once(inode) & I_DIRTY_DATASYNC) == 0)
    {
        /* Make sure committed changes hit the disk */
        jfs_flush_journal((*JFS_SBI((*inode).i_sb)).log, 1);
        inode_unlock(inode);
        return rc;
    }

    rc |= jfs_commit_inode(inode, 1);
    inode_unlock(inode);

    if rc != 0 { -EIO } else { 0 }
}

unsafe fn jfs_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut rc: c_int;

    if S_ISREG((*inode).i_mode) && (*inode).i_size < 0 {
        return -EIO;
    }

    rc = dquot_file_open(inode, file);
    if rc != 0 {
        return rc;
    }

    /*
     * We attempt to allow only one "active" file open per aggregate
     * group.  Otherwise, appending to files in parallel can cause
     * fragmentation within the files.
     *
     * If the file is empty, it was probably just created and going
     * to be written to.  If it has a size, we'll hold off until the
     * file is actually grown.
     */
    if S_ISREG((*inode).i_mode) && ((*file).f_mode & FMODE_WRITE) != 0 && (*inode).i_size == 0 {
        let ji = JFS_IP(inode);
        spin_lock_irq(&mut (*ji).ag_lock);
        if (*ji).active_ag == -1 {
            let jfs_sb = JFS_SBI((*inode).i_sb);
            (*ji).active_ag = BLKTOAG(addressPXD(&(*ji).ixpxd), jfs_sb);
            atomic_inc(&mut (*(*jfs_sb).bmap).db_active[(*ji).active_ag as usize]);
        }
        spin_unlock_irq(&mut (*ji).ag_lock);
    }

    0
}

unsafe fn jfs_release(inode: *mut inode, _file: *mut file) -> c_int {
    let ji = JFS_IP(inode);

    spin_lock_irq(&mut (*ji).ag_lock);
    if (*ji).active_ag != -1 {
        let bmap = (*JFS_SBI((*inode).i_sb)).bmap;
        atomic_dec(&mut (*bmap).db_active[(*ji).active_ag as usize]);
        (*ji).active_ag = -1;
    }
    spin_unlock_irq(&mut (*ji).ag_lock);

    0
}

pub unsafe fn jfs_setattr(
    _idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    iattr: *mut iattr,
) -> c_int {
    let inode = d_inode(dentry);
    let mut rc: c_int;

    rc = setattr_prepare(&nop_mnt_idmap, dentry, iattr);
    if rc != 0 {
        return rc;
    }

    if is_quota_modification(&nop_mnt_idmap, inode, iattr) {
        rc = dquot_initialize(inode);
        if rc != 0 {
            return rc;
        }
    }
    if (((*iattr).ia_valid & ATTR_UID) != 0 && !uid_eq((*iattr).ia_uid, (*inode).i_uid))
        || (((*iattr).ia_valid & ATTR_GID) != 0 && !gid_eq((*iattr).ia_gid, (*inode).i_gid))
    {
        rc = dquot_transfer(&nop_mnt_idmap, inode, iattr);
        if rc != 0 {
            return rc;
        }
    }

    if ((*iattr).ia_valid & ATTR_SIZE) != 0 && (*iattr).ia_size != i_size_read(inode) {
        inode_dio_wait(inode);

        rc = inode_newsize_ok(inode, (*iattr).ia_size);
        if rc != 0 {
            return rc;
        }

        truncate_setsize(inode, (*iattr).ia_size);
        jfs_truncate(inode);
    }

    setattr_copy(&nop_mnt_idmap, inode, iattr);
    mark_inode_dirty(inode);

    if ((*iattr).ia_valid & ATTR_MODE) != 0 {
        rc = posix_acl_chmod(&nop_mnt_idmap, dentry, (*inode).i_mode);
    }
    rc
}

pub static jfs_file_inode_operations: inode_operations = inode_operations {
    listxattr: Some(jfs_listxattr),
    setattr: Some(jfs_setattr),
    fileattr_get: Some(jfs_fileattr_get),
    fileattr_set: Some(jfs_fileattr_set),
    /* CONFIG_JFS_POSIX_ACL conditionally supplies get_inode_acl and set_acl. */
};

pub static jfs_file_operations: file_operations = file_operations {
    open: Some(jfs_open),
    llseek: Some(generic_file_llseek),
    read_iter: Some(generic_file_read_iter),
    write_iter: Some(generic_file_write_iter),
    mmap_prepare: Some(generic_file_mmap_prepare),
    splice_read: Some(filemap_splice_read),
    splice_write: Some(iter_file_splice_write),
    fsync: Some(jfs_fsync),
    release: Some(jfs_release),
    unlocked_ioctl: Some(jfs_ioctl),
    compat_ioctl: Some(compat_ptr_ioctl),
    setlease: Some(generic_setlease),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
