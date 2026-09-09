// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2008 Christoph Hellwig.
 * Portions Copyright (C) 2000-2008 Silicon Graphics, Inc.
 */

// Dependencies supplied by the corresponding XFS and Linux headers are
// intentionally left external to this translation unit.

unsafe fn xfs_attr_grab_log_assist(mp: *mut xfs_mount) -> c_int {
    let mut error: c_int = 0;

    /* xattr update log intent items are already enabled */
    if xfs_is_using_logged_xattrs(mp) {
        return 0;
    }

    /*
     * Check if the filesystem featureset is new enough to set this log
     * incompat feature bit.  Strictly speaking, the minimum requirement is
     * a V5 filesystem for the superblock field, but we'll require rmap
     * or reflink to avoid having to deal with really old kernels.
     */
    if !xfs_has_reflink(mp) && !xfs_has_rmapbt(mp) {
        return -EOPNOTSUPP;
    }

    /* Enable log-assisted xattrs. */
    error = xfs_add_incompat_log_feature(mp, XFS_SB_FEAT_INCOMPAT_LOG_XATTRS);
    if error != 0 {
        return error;
    }
    xfs_set_using_logged_xattrs(mp);

    xfs_warn_experimental(mp, XFS_EXPERIMENTAL_LARP);

    0
}

unsafe fn xfs_attr_want_log_assist(mp: *mut xfs_mount) -> bool {
    // C conditional: this branch is present only when DEBUG is enabled.
    #[cfg(DEBUG)]
    {
        /* Logged xattrs require a V5 super for log_incompat */
        return xfs_has_crc(mp) && xfs_globals.larp;
    }
    #[cfg(not(DEBUG))]
    {
        let _ = mp;
        false
    }
}

/*
 * Set or remove an xattr, having grabbed the appropriate logging resources
 * prior to calling libxfs.  Callers of this function are only required to
 * initialize the inode, attr_filter, name, namelen, value, and valuelen fields
 * of @args.
 */
pub unsafe fn xfs_attr_change(args: *mut xfs_da_args, op: xfs_attr_update) -> c_int {
    let mp = (*(*args).dp).i_mount;
    let mut error: c_int;

    if xfs_is_shutdown(mp) {
        return -EIO;
    }

    error = xfs_qm_dqattach((*args).dp);
    if error != 0 {
        return error;
    }

    /*
     * We have no control over the attribute names that userspace passes us
     * to remove, so we have to allow the name lookup prior to attribute
     * removal to fail as well.
     */
    (*args).op_flags = XFS_DA_OP_OKNOENT;

    if xfs_attr_want_log_assist(mp) {
        error = xfs_attr_grab_log_assist(mp);
        if error != 0 {
            return error;
        }

        (*args).op_flags |= XFS_DA_OP_LOGGED;
    }

    (*args).owner = I_INO((*args).dp);
    (*args).geo = (*mp).m_attr_geo;
    (*args).whichfork = XFS_ATTR_FORK;
    xfs_attr_sethash(args);

    /*
     * Some xattrs must be resistant to allocation failure at ENOSPC, e.g.
     * creating an inode with ACLs or security attributes requires the
     * allocation of the xattr holding that information to succeed. Hence
     * we allow xattrs in the VFS TRUSTED, SYSTEM, POSIX_ACL and SECURITY
     * (LSM xattr) namespaces to dip into the reserve block pool to allow
     * manipulation of these xattrs when at ENOSPC. These VFS xattr
     * namespaces translate to the XFS_ATTR_ROOT and XFS_ATTR_SECURE on-disk
     * namespaces.
     *
     * For most of these cases, these special xattrs will fit in the inode
     * itself and so consume no extra space or only require temporary extra
     * space while an overwrite is being made. Hence the use of the reserved
     * pool is largely to avoid the worst case reservation from preventing
     * the xattr from being created at ENOSPC.
     */
    xfs_attr_set(args, op, (*args).attr_filter & (XFS_ATTR_ROOT | XFS_ATTR_SECURE))
}

unsafe fn xfs_xattr_get(
    handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    value: *mut c_void,
    size: usize,
) -> isize {
    let mut args = xfs_da_args {
        dp: XFS_I(inode),
        attr_filter: (*handler).flags,
        name,
        namelen: strlen(name),
        value,
        valuelen: size,
    };

    if xfs_ifork_zapped(XFS_I(inode), XFS_ATTR_FORK) {
        return -EIO as isize;
    }

    let error = xfs_attr_get(&mut args);
    if error != 0 {
        return error as isize;
    }
    args.valuelen as isize
}

unsafe fn xfs_xattr_flags_to_op(flags: c_int, value: *const c_void) -> xfs_attr_update {
    if value.is_null() {
        return XFS_ATTRUPDATE_REMOVE;
    }
    if flags & XATTR_CREATE != 0 {
        return XFS_ATTRUPDATE_CREATE;
    }
    if flags & XATTR_REPLACE != 0 {
        return XFS_ATTRUPDATE_REPLACE;
    }
    XFS_ATTRUPDATE_UPSERT
}

unsafe fn xfs_xattr_set(
    handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    value: *const c_void,
    size: usize,
    flags: c_int,
) -> c_int {
    let mut args = xfs_da_args {
        dp: XFS_I(inode),
        attr_filter: (*handler).flags,
        name,
        namelen: strlen(name),
        value: value as *mut c_void,
        valuelen: size,
    };

    let error = xfs_attr_change(&mut args, xfs_xattr_flags_to_op(flags, value));
    if error == 0 && (*handler).flags & XFS_ATTR_ROOT != 0 {
        xfs_forget_acl(inode, name);
    }
    error
}

static xfs_xattr_user_handler: xattr_handler = xattr_handler {
    prefix: XATTR_USER_PREFIX,
    flags: 0, /* no flags implies user namespace */
    get: xfs_xattr_get,
    set: xfs_xattr_set,
};

static xfs_xattr_trusted_handler: xattr_handler = xattr_handler {
    prefix: XATTR_TRUSTED_PREFIX,
    flags: XFS_ATTR_ROOT,
    get: xfs_xattr_get,
    set: xfs_xattr_set,
};

static xfs_xattr_security_handler: xattr_handler = xattr_handler {
    prefix: XATTR_SECURITY_PREFIX,
    flags: XFS_ATTR_SECURE,
    get: xfs_xattr_get,
    set: xfs_xattr_set,
};

pub static xfs_xattr_handlers: [*const xattr_handler; 4] = [
    &xfs_xattr_user_handler,
    &xfs_xattr_trusted_handler,
    &xfs_xattr_security_handler,
    core::ptr::null(),
];

unsafe fn __xfs_xattr_put_listent(
    context: *mut xfs_attr_list_context,
    prefix: *mut c_char,
    prefix_len: c_int,
    name: *mut u8,
    namelen: c_int,
) {
    let mut offset: *mut c_char;
    let arraytop: c_int;

    if (*context).count < 0 || (*context).seen_enough {
        return;
    }

    if (*context).buffer.is_null() {
        (*context).count += prefix_len + namelen + 1;
        return;
    }

    arraytop = (*context).count + prefix_len + namelen + 1;
    if arraytop > (*context).firstu {
        (*context).count = -1; /* insufficient space */
        (*context).seen_enough = true;
        return;
    }
    offset = (*context).buffer.add((*context).count as usize);
    memcpy(offset as *mut c_void, prefix as *const c_void, prefix_len as usize);
    offset = offset.add(prefix_len as usize);
    memcpy(offset as *mut c_void, name as *const c_void, namelen as usize); /* real name */
    offset = offset.add(namelen as usize);
    *offset = 0;

    (*context).count += prefix_len + namelen + 1;
}

unsafe fn xfs_xattr_put_listent(
    context: *mut xfs_attr_list_context,
    flags: c_int,
    name: *mut u8,
    namelen: c_int,
    _value: *mut c_void,
    _valuelen: c_int,
) {
    let prefix: *mut c_char;
    let prefix_len: c_int;

    ASSERT((*context).count >= 0);

    /* Don't expose private xattr namespaces. */
    if flags & XFS_ATTR_PRIVATE_NSP_MASK != 0 {
        return;
    }

    if flags & XFS_ATTR_ROOT != 0 {
        // C conditional: this ACL namespace mapping is present only with CONFIG_XFS_POSIX_ACL.
        #[cfg(CONFIG_XFS_POSIX_ACL)]
        {
            if namelen == SGI_ACL_FILE_SIZE && strncmp(name, SGI_ACL_FILE, SGI_ACL_FILE_SIZE as usize) == 0 {
                __xfs_xattr_put_listent(context, XATTR_SYSTEM_PREFIX, XATTR_SYSTEM_PREFIX_LEN, XATTR_POSIX_ACL_ACCESS, strlen(XATTR_POSIX_ACL_ACCESS));
            } else if namelen == SGI_ACL_DEFAULT_SIZE && strncmp(name, SGI_ACL_DEFAULT, SGI_ACL_DEFAULT_SIZE as usize) == 0 {
                __xfs_xattr_put_listent(context, XATTR_SYSTEM_PREFIX, XATTR_SYSTEM_PREFIX_LEN, XATTR_POSIX_ACL_DEFAULT, strlen(XATTR_POSIX_ACL_DEFAULT));
            }
        }

        /* Only show root namespace entries if we are actually allowed to see them. */
        if !capable(CAP_SYS_ADMIN) {
            return;
        }

        prefix = XATTR_TRUSTED_PREFIX;
        prefix_len = XATTR_TRUSTED_PREFIX_LEN;
    } else if flags & XFS_ATTR_SECURE != 0 {
        prefix = XATTR_SECURITY_PREFIX;
        prefix_len = XATTR_SECURITY_PREFIX_LEN;
    } else {
        prefix = XATTR_USER_PREFIX;
        prefix_len = XATTR_USER_PREFIX_LEN;
    }

    __xfs_xattr_put_listent(context, prefix, prefix_len, name, namelen);
}

pub unsafe fn xfs_vn_listxattr(dentry: *mut dentry, data: *mut c_char, size: usize) -> isize {
    let mut context: xfs_attr_list_context = core::mem::zeroed();
    let inode = d_inode(dentry);

    if xfs_ifork_zapped(XFS_I(inode), XFS_ATTR_FORK) {
        return -EIO as isize;
    }

    /* First read the regular on-disk attributes. */
    context.dp = XFS_I(inode);
    context.resynch = 1;
    context.bufsize = size;
    context.buffer = if size != 0 { data } else { core::ptr::null_mut() };
    context.firstu = context.bufsize;
    context.put_listent = xfs_xattr_put_listent;

    let error = xfs_attr_list(&mut context);
    if error != 0 {
        return error as isize;
    }
    if context.count < 0 {
        return -ERANGE as isize;
    }

    context.count as isize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
