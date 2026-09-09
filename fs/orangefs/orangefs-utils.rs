// SPDX-License-Identifier: GPL-2.0
/*
 * (C) 2001 Clemson University and The University of Chicago
 * Copyright 2018 Omnibond Systems, L.L.C.
 *
 * See COPYING in top-level directory.
 */
// External kernel and OrangeFS dependencies are supplied by other translation units.

pub unsafe fn fsid_of_op(op: *mut orangefs_kernel_op_s) -> __s32 {
    let mut fsid: __s32 = ORANGEFS_FS_ID_NULL;
    if !op.is_null() {
        match (*op).upcall.r#type {
            ORANGEFS_VFS_OP_FILE_IO => fsid = (*op).upcall.req.io.refn.fs_id,
            ORANGEFS_VFS_OP_LOOKUP => fsid = (*op).upcall.req.lookup.parent_refn.fs_id,
            ORANGEFS_VFS_OP_CREATE => fsid = (*op).upcall.req.create.parent_refn.fs_id,
            ORANGEFS_VFS_OP_GETATTR => fsid = (*op).upcall.req.getattr.refn.fs_id,
            ORANGEFS_VFS_OP_REMOVE => fsid = (*op).upcall.req.remove.parent_refn.fs_id,
            ORANGEFS_VFS_OP_MKDIR => fsid = (*op).upcall.req.mkdir.parent_refn.fs_id,
            ORANGEFS_VFS_OP_READDIR => fsid = (*op).upcall.req.readdir.refn.fs_id,
            ORANGEFS_VFS_OP_SETATTR => fsid = (*op).upcall.req.setattr.refn.fs_id,
            ORANGEFS_VFS_OP_SYMLINK => fsid = (*op).upcall.req.sym.parent_refn.fs_id,
            ORANGEFS_VFS_OP_RENAME => fsid = (*op).upcall.req.rename.old_parent_refn.fs_id,
            ORANGEFS_VFS_OP_STATFS => fsid = (*op).upcall.req.statfs.fs_id,
            ORANGEFS_VFS_OP_TRUNCATE => fsid = (*op).upcall.req.truncate.refn.fs_id,
            ORANGEFS_VFS_OP_RA_FLUSH => fsid = (*op).upcall.req.ra_cache_flush.refn.fs_id,
            ORANGEFS_VFS_OP_FS_UMOUNT => fsid = (*op).upcall.req.fs_umount.fs_id,
            ORANGEFS_VFS_OP_GETXATTR => fsid = (*op).upcall.req.getxattr.refn.fs_id,
            ORANGEFS_VFS_OP_SETXATTR => fsid = (*op).upcall.req.setxattr.refn.fs_id,
            ORANGEFS_VFS_OP_LISTXATTR => fsid = (*op).upcall.req.listxattr.refn.fs_id,
            ORANGEFS_VFS_OP_REMOVEXATTR => fsid = (*op).upcall.req.removexattr.refn.fs_id,
            ORANGEFS_VFS_OP_FSYNC => fsid = (*op).upcall.req.fsync.refn.fs_id,
            _ => {}
        }
    }
    fsid
}

unsafe fn orangefs_inode_flags(attrs: *mut ORANGEFS_sys_attr_s) -> i32 {
    let mut flags = 0;
    if (*attrs).flags & ORANGEFS_IMMUTABLE_FL != 0 { flags |= S_IMMUTABLE; } else { flags &= !S_IMMUTABLE; }
    if (*attrs).flags & ORANGEFS_APPEND_FL != 0 { flags |= S_APPEND; } else { flags &= !S_APPEND; }
    if (*attrs).flags & ORANGEFS_NOATIME_FL != 0 { flags |= S_NOATIME; } else { flags &= !S_NOATIME; }
    flags
}

unsafe fn orangefs_inode_perms(attrs: *mut ORANGEFS_sys_attr_s) -> i32 {
    let mut perm_mode = 0;
    if (*attrs).perms & ORANGEFS_O_EXECUTE != 0 { perm_mode |= S_IXOTH; }
    if (*attrs).perms & ORANGEFS_O_WRITE != 0 { perm_mode |= S_IWOTH; }
    if (*attrs).perms & ORANGEFS_O_READ != 0 { perm_mode |= S_IROTH; }
    if (*attrs).perms & ORANGEFS_G_EXECUTE != 0 { perm_mode |= S_IXGRP; }
    if (*attrs).perms & ORANGEFS_G_WRITE != 0 { perm_mode |= S_IWGRP; }
    if (*attrs).perms & ORANGEFS_G_READ != 0 { perm_mode |= S_IRGRP; }
    if (*attrs).perms & ORANGEFS_U_EXECUTE != 0 { perm_mode |= S_IXUSR; }
    if (*attrs).perms & ORANGEFS_U_WRITE != 0 { perm_mode |= S_IWUSR; }
    if (*attrs).perms & ORANGEFS_U_READ != 0 { perm_mode |= S_IRUSR; }
    if (*attrs).perms & ORANGEFS_G_SGID != 0 { perm_mode |= S_ISGID; }
    if (*attrs).perms & ORANGEFS_U_SUID != 0 { perm_mode |= S_ISUID; }
    perm_mode
}

/* NOTE: link_target is intentionally not copied, as in the kernel source. */
unsafe fn copy_attributes_from_inode(inode: *mut inode, attrs: *mut ORANGEFS_sys_attr_s) {
    let orangefs_inode = ORANGEFS_I(inode);
    (*attrs).mask = 0;
    if (*orangefs_inode).attr_valid & ATTR_UID != 0 {
        (*attrs).owner = from_kuid(&init_user_ns, (*inode).i_uid);
        (*attrs).mask |= ORANGEFS_ATTR_SYS_UID;
        gossip_debug(GOSSIP_UTILS_DEBUG, "(UID) %d\n", (*attrs).owner);
    }
    if (*orangefs_inode).attr_valid & ATTR_GID != 0 {
        (*attrs).group = from_kgid(&init_user_ns, (*inode).i_gid);
        (*attrs).mask |= ORANGEFS_ATTR_SYS_GID;
        gossip_debug(GOSSIP_UTILS_DEBUG, "(GID) %d\n", (*attrs).group);
    }
    if (*orangefs_inode).attr_valid & ATTR_ATIME != 0 {
        (*attrs).mask |= ORANGEFS_ATTR_SYS_ATIME;
        if (*orangefs_inode).attr_valid & ATTR_ATIME_SET != 0 {
            (*attrs).atime = inode_get_atime_sec(inode) as time64_t;
            (*attrs).mask |= ORANGEFS_ATTR_SYS_ATIME_SET;
        }
    }
    if (*orangefs_inode).attr_valid & ATTR_MTIME != 0 {
        (*attrs).mask |= ORANGEFS_ATTR_SYS_MTIME;
        if (*orangefs_inode).attr_valid & ATTR_MTIME_SET != 0 {
            (*attrs).mtime = inode_get_mtime_sec(inode) as time64_t;
            (*attrs).mask |= ORANGEFS_ATTR_SYS_MTIME_SET;
        }
    }
    if (*orangefs_inode).attr_valid & ATTR_CTIME != 0 { (*attrs).mask |= ORANGEFS_ATTR_SYS_CTIME; }
    if (*orangefs_inode).attr_valid & ATTR_MODE != 0 {
        (*attrs).perms = ORANGEFS_util_translate_mode((*inode).i_mode);
        (*attrs).mask |= ORANGEFS_ATTR_SYS_PERM;
    }
}

unsafe fn orangefs_inode_type(objtype: orangefs_ds_type) -> i32 {
    if objtype == ORANGEFS_TYPE_METAFILE { S_IFREG }
    else if objtype == ORANGEFS_TYPE_DIRECTORY { S_IFDIR }
    else if objtype == ORANGEFS_TYPE_SYMLINK { S_IFLNK }
    else { -1 }
}

unsafe fn orangefs_make_bad_inode(inode: *mut inode) {
    if is_root_handle(inode) {
        gossip_debug(GOSSIP_UTILS_DEBUG, "*** NOT making bad root inode %pU\n", get_khandle_from_ino(inode));
    } else {
        gossip_debug(GOSSIP_UTILS_DEBUG, "*** making bad inode %pU\n", get_khandle_from_ino(inode));
        make_bad_inode(inode);
    }
}

unsafe fn orangefs_inode_is_stale(inode: *mut inode, attrs: *mut ORANGEFS_sys_attr_s, link_target: *mut c_char) -> i32 {
    let orangefs_inode = ORANGEFS_I(inode);
    let r#type = orangefs_inode_type((*attrs).objtype);
    if r#type == -1 || inode_wrong_type(inode, r#type) {
        orangefs_make_bad_inode(inode);
        return 1;
    }
    if r#type == S_IFLNK && strncmp((*orangefs_inode).link_target, link_target, ORANGEFS_NAME_MAX) != 0 {
        orangefs_make_bad_inode(inode);
        return 1;
    }
    0
}

unsafe fn orangefs_inode_getattr(inode: *mut inode, flags: i32) -> i32 {
    let orangefs_inode = ORANGEFS_I(inode);
    let mut new_op;
    let mut inode_size: loff_t;
    let mut ret: i32;
    let mut r#type: i32;
    gossip_debug(GOSSIP_UTILS_DEBUG, "%s: called on inode %pU flags %d\n", __func__, get_khandle_from_ino(inode), flags);
'again: loop {
        spin_lock(&mut (*inode).i_lock);
        if ((!flags != 0 && time_before(jiffies, (*orangefs_inode).getattr_time)) || (*orangefs_inode).attr_valid != 0 || inode_state_read(inode) & I_DIRTY_PAGES != 0) {
            if (*orangefs_inode).attr_valid != 0 { spin_unlock(&mut (*inode).i_lock); write_inode_now(inode, 1); continue 'again; }
            spin_unlock(&mut (*inode).i_lock); return 0;
        }
        spin_unlock(&mut (*inode).i_lock);
        new_op = op_alloc(ORANGEFS_VFS_OP_GETATTR);
        if new_op.is_null() { return -ENOMEM; }
        (*new_op).upcall.req.getattr.refn = (*orangefs_inode).refn;
        (*new_op).upcall.req.getattr.mask = if flags != 0 { ORANGEFS_ATTR_SYS_ALL_NOHINT } else { ORANGEFS_ATTR_SYS_ALL_NOHINT & !ORANGEFS_ATTR_SYS_SIZE };
        ret = service_operation(new_op, __func__, get_interruptible_flag(inode));
        if ret != 0 { break; }
        'again2: loop {
            spin_lock(&mut (*inode).i_lock);
            if ((!flags != 0 && time_before(jiffies, (*orangefs_inode).getattr_time)) || (*orangefs_inode).attr_valid != 0 || inode_state_read(inode) & I_DIRTY_PAGES != 0) {
                if (*orangefs_inode).attr_valid != 0 { spin_unlock(&mut (*inode).i_lock); write_inode_now(inode, 1); continue 'again2; }
                if inode_state_read(inode) & I_DIRTY_PAGES != 0 { ret = 0; break 'again2; }
                gossip_debug(GOSSIP_UTILS_DEBUG, "%s: in cache or dirty\n", __func__); ret = 0; break 'again2;
            }
            if flags & ORANGEFS_GETATTR_NEW == 0 {
                ret = orangefs_inode_is_stale(inode, &mut (*new_op).downcall.resp.getattr.attributes, (*new_op).downcall.resp.getattr.link_target);
                if ret != 0 { ret = -ESTALE; break 'again2; }
            }
            r#type = orangefs_inode_type((*new_op).downcall.resp.getattr.attributes.objtype);
            match r#type {
                S_IFREG => { (*inode).i_flags = orangefs_inode_flags(&mut (*new_op).downcall.resp.getattr.attributes); if flags != 0 { inode_size = (*new_op).downcall.resp.getattr.attributes.size as loff_t; (*inode).i_size = inode_size; (*inode).i_blkbits = ffs((*new_op).downcall.resp.getattr.attributes.blksize); (*inode).i_bytes = inode_size; (*inode).i_blocks = (inode_size + 512 - inode_size % 512) / 512; } }
                S_IFDIR => { if flags != 0 { (*inode).i_size = PAGE_SIZE; inode_set_bytes(inode, (*inode).i_size); } set_nlink(inode, 1); }
                S_IFLNK => { if flags & ORANGEFS_GETATTR_NEW != 0 { (*inode).i_size = strlen((*new_op).downcall.resp.getattr.link_target) as loff_t; ret = strscpy((*orangefs_inode).link_target, (*new_op).downcall.resp.getattr.link_target, ORANGEFS_NAME_MAX); if ret == -E2BIG { ret = -EIO; break 'again2; } (*inode).i_link = (*orangefs_inode).link_target; } }
                _ => { orangefs_make_bad_inode(inode); ret = -ESTALE; break 'again2; }
            }
            (*inode).i_uid = make_kuid(&init_user_ns, (*new_op).downcall.resp.getattr.attributes.owner);
            (*inode).i_gid = make_kgid(&init_user_ns, (*new_op).downcall.resp.getattr.attributes.group);
            inode_set_atime(inode, (*new_op).downcall.resp.getattr.attributes.atime as time64_t, 0);
            inode_set_mtime(inode, (*new_op).downcall.resp.getattr.attributes.mtime as time64_t, 0);
            inode_set_ctime(inode, (*new_op).downcall.resp.getattr.attributes.ctime as time64_t, 0);
            (*inode).i_mode = r#type | if is_root_handle(inode) { S_ISVTX } else { 0 } | orangefs_inode_perms(&mut (*new_op).downcall.resp.getattr.attributes);
            (*orangefs_inode).getattr_time = jiffies + orangefs_getattr_timeout_msecs * HZ / 1000;
            ret = 0;
            break 'again2;
        }
        spin_unlock(&mut (*inode).i_lock);
        break;
    }
    op_release(new_op);
    ret
}

unsafe fn orangefs_inode_check_changed(inode: *mut inode) -> i32 {
    let orangefs_inode = ORANGEFS_I(inode);
    let new_op = op_alloc(ORANGEFS_VFS_OP_GETATTR);
    if new_op.is_null() { return -ENOMEM; }
    (*new_op).upcall.req.getattr.refn = (*orangefs_inode).refn;
    (*new_op).upcall.req.getattr.mask = ORANGEFS_ATTR_SYS_TYPE | ORANGEFS_ATTR_SYS_LNK_TARGET;
    let mut ret = service_operation(new_op, __func__, get_interruptible_flag(inode));
    if ret == 0 { ret = orangefs_inode_is_stale(inode, &mut (*new_op).downcall.resp.getattr.attributes, (*new_op).downcall.resp.getattr.link_target); }
    op_release(new_op); ret
}

unsafe fn orangefs_inode_setattr(inode: *mut inode) -> i32 {
    let orangefs_inode = ORANGEFS_I(inode);
    let new_op = op_alloc(ORANGEFS_VFS_OP_SETATTR);
    if new_op.is_null() { return -ENOMEM; }
    spin_lock(&mut (*inode).i_lock);
    (*new_op).upcall.uid = from_kuid(&init_user_ns, (*orangefs_inode).attr_uid);
    (*new_op).upcall.gid = from_kgid(&init_user_ns, (*orangefs_inode).attr_gid);
    (*new_op).upcall.req.setattr.refn = (*orangefs_inode).refn;
    copy_attributes_from_inode(inode, &mut (*new_op).upcall.req.setattr.attributes);
    (*orangefs_inode).attr_valid = 0;
    if (*new_op).upcall.req.setattr.attributes.mask == 0 { spin_unlock(&mut (*inode).i_lock); op_release(new_op); return 0; }
    spin_unlock(&mut (*inode).i_lock);
    let ret = service_operation(new_op, __func__, get_interruptible_flag(inode) | ORANGEFS_OP_WRITEBACK);
    gossip_debug(GOSSIP_UTILS_DEBUG, "orangefs_inode_setattr: returning %d\n", ret);
    if ret != 0 { orangefs_make_bad_inode(inode); }
    op_release(new_op);
    if ret == 0 { (*orangefs_inode).getattr_time = jiffies - 1; }
    ret
}

static PINT_errno_mapping: [i32; 60] = [
    0, EPERM, ENOENT, EINTR, EIO, ENXIO, EBADF, EAGAIN, ENOMEM, EFAULT, EBUSY, EEXIST, ENODEV, ENOTDIR, EISDIR, EINVAL, EMFILE,
    EFBIG, ENOSPC, EROFS, EMLINK, EPIPE, EDEADLK, ENAMETOOLONG, ENOLCK, ENOSYS, ENOTEMPTY, ELOOP, EWOULDBLOCK, ENOMSG, EUNATCH,
    EBADR, EDEADLOCK, ENODATA, ETIME, ENONET, EREMOTE, ECOMM, EPROTO, EBADMSG, EOVERFLOW, ERESTART, EMSGSIZE, EPROTOTYPE,
    ENOPROTOOPT, EPROTONOSUPPORT, EOPNOTSUPP, EADDRINUSE, EADDRNOTAVAIL, ENETDOWN, ENETUNREACH, ENETRESET, ENOBUFS, ETIMEDOUT,
    ECONNREFUSED, EHOSTDOWN, EHOSTUNREACH, EALREADY, EACCES, ECONNRESET, ERANGE
];

pub fn orangefs_normalize_to_errno(mut error_code: __s32) -> i32 {
    if error_code == 0 { return 0; }
    if error_code > 0 { gossip_err("orangefs: error status received.\n"); gossip_err("orangefs: assuming error code is inverted.\n"); error_code = -error_code; }
    if (-error_code) & ORANGEFS_NON_ERRNO_ERROR_BIT != 0 {
        if ((-error_code) & (ORANGEFS_ERROR_NUMBER_BITS | ORANGEFS_NON_ERRNO_ERROR_BIT | ORANGEFS_ERROR_BIT)) == ORANGEFS_ECANCEL { error_code = -ETIMEDOUT; }
        else { gossip_err("%s: bad error code :%d:.\n", __func__, error_code); error_code = -EINVAL; }
    } else if (-error_code) & ORANGEFS_ERROR_BIT != 0 {
        let i = ((-error_code) & !(ORANGEFS_ERROR_BIT | ORANGEFS_ERROR_CLASS_BITS)) as usize;
        error_code = if i < PINT_errno_mapping.len() { -PINT_errno_mapping[i] } else { -EINVAL };
    } else { gossip_err("%s: unknown error code.\n", __func__); error_code = -EINVAL; }
    error_code
}

pub unsafe fn ORANGEFS_util_translate_mode(mode: i32) -> __s32 {
    let modes = [S_IXOTH, S_IWOTH, S_IROTH, S_IXGRP, S_IWGRP, S_IRGRP, S_IXUSR, S_IWUSR, S_IRUSR, S_ISGID, S_ISUID];
    let orangefs_modes = [ORANGEFS_O_EXECUTE, ORANGEFS_O_WRITE, ORANGEFS_O_READ, ORANGEFS_G_EXECUTE, ORANGEFS_G_WRITE, ORANGEFS_G_READ, ORANGEFS_U_EXECUTE, ORANGEFS_U_WRITE, ORANGEFS_U_READ, ORANGEFS_G_SGID, ORANGEFS_U_SUID];
    let mut ret = 0;
    for i in 0..11 { if mode & modes[i] != 0 { ret |= orangefs_modes[i]; } }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
