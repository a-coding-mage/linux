// SPDX-License-Identifier: GPL-2.0
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * See COPYING in top-level directory.
 */

/* Linux VFS namei operations. */

/* Dependencies are supplied by the surrounding OrangeFS kernel bindings. */

/* Get a newly allocated inode to go with a negative dentry. */
unsafe fn orangefs_create(
    idmap: *mut mnt_idmap,
    dir: *mut inode,
    dentry: *mut dentry,
    mode: umode_t,
) -> c_int {
    let parent = ORANGEFS_I(dir);
    let mut new_op: *mut orangefs_kernel_op_s;
    let mut reference: orangefs_object_kref;
    let mut inode: *mut inode;
    let mut iattr: iattr;
    let mut ret: c_int;

    gossip_debug(GOSSIP_NAME_DEBUG, "%s: %pd\n", __func__, dentry);

    new_op = op_alloc(ORANGEFS_VFS_OP_CREATE);
    if new_op.is_null() { return -ENOMEM; }

    (*new_op).upcall.req.create.parent_refn = (*parent).refn;
    fill_default_sys_attrs((*new_op).upcall.req.create.attributes, mode);
    strscpy((*new_op).upcall.req.create.d_name, (*dentry).d_name.name);

    ret = service_operation(new_op, __func__, get_interruptible_flag(dir));
    gossip_debug(GOSSIP_NAME_DEBUG,
        "%s: %pd: handle:%pU: fsid:%d: new_op:%p: ret:%d:\n",
        __func__, dentry, &(*new_op).downcall.resp.create.refn.khandle,
        (*new_op).downcall.resp.create.refn.fs_id, new_op, ret);
    if ret < 0 { op_release(new_op); return ret; }

    reference = (*new_op).downcall.resp.create.refn;
    inode = orangefs_new_inode((*dir).i_sb, dir, S_IFREG | mode, 0, &reference);
    if IS_ERR(inode) {
        gossip_err("%s: Failed to allocate inode for file :%pd:\n", __func__, dentry);
        ret = PTR_ERR(inode);
        op_release(new_op);
        return ret;
    }
    gossip_debug(GOSSIP_NAME_DEBUG, "%s: Assigned inode :%pU: for file :%pd:\n",
        __func__, get_khandle_from_ino(inode), dentry);
    d_instantiate_new(dentry, inode);
    orangefs_set_timeout(dentry);
    gossip_debug(GOSSIP_NAME_DEBUG, "%s: dentry instantiated for %pd\n", __func__, dentry);
    memset(&mut iattr, 0, core::mem::size_of::<iattr>());
    iattr.ia_valid |= ATTR_MTIME | ATTR_CTIME;
    iattr.ia_mtime = current_time(dir);
    iattr.ia_ctime = iattr.ia_mtime;
    __orangefs_setattr(dir, &mut iattr);
    ret = 0;
    op_release(new_op);
    gossip_debug(GOSSIP_NAME_DEBUG, "%s: %pd: returning %d\n", __func__, dentry, ret);
    ret
}

/* Attempt to resolve an object name, parent handle, and fsid into a handle. */
unsafe fn orangefs_lookup(dir: *mut inode, dentry: *mut dentry, flags: c_uint) -> *mut dentry {
    let parent = ORANGEFS_I(dir);
    let new_op = op_alloc(ORANGEFS_VFS_OP_LOOKUP);
    if (*dentry).d_name.len > (ORANGEFS_NAME_MAX - 1) { return ERR_PTR(-ENAMETOOLONG); }
    if new_op.is_null() { return ERR_PTR(-ENOMEM); }
    (*new_op).upcall.req.lookup.sym_follow = ORANGEFS_LOOKUP_LINK_NO_FOLLOW;
    (*new_op).upcall.req.lookup.parent_refn = (*parent).refn;
    strscpy((*new_op).upcall.req.lookup.d_name, (*dentry).d_name.name);
    let ret = service_operation(new_op, __func__, get_interruptible_flag(dir));
    let inode = if ret == 0 {
        orangefs_set_timeout(dentry);
        orangefs_iget((*dir).i_sb, &(*new_op).downcall.resp.lookup.refn)
    } else if ret == -ENOENT { core::ptr::null_mut() } else { ERR_PTR(ret) };
    op_release(new_op);
    d_splice_alias(inode, dentry)
}

/* return 0 on success; non-zero otherwise */
unsafe fn orangefs_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let inode = (*dentry).d_inode;
    let parent = ORANGEFS_I(dir);
    let new_op = op_alloc(ORANGEFS_VFS_OP_REMOVE);
    if new_op.is_null() { return -ENOMEM; }
    (*new_op).upcall.req.remove.parent_refn = (*parent).refn;
    strscpy((*new_op).upcall.req.remove.d_name, (*dentry).d_name.name);
    let ret = service_operation(new_op, "orangefs_unlink", get_interruptible_flag(inode));
    op_release(new_op);
    if ret == 0 {
        drop_nlink(inode);
        let mut iattr: iattr = core::mem::zeroed();
        iattr.ia_valid |= ATTR_MTIME | ATTR_CTIME;
        iattr.ia_mtime = current_time(dir);
        iattr.ia_ctime = iattr.ia_mtime;
        __orangefs_setattr(dir, &mut iattr);
    }
    ret
}

unsafe fn orangefs_symlink(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, symname: *const c_char) -> c_int {
    if symname.is_null() { return -EINVAL; }
    if strlen(symname) + 1 > ORANGEFS_NAME_MAX { return -ENAMETOOLONG; }
    let parent = ORANGEFS_I(dir);
    let new_op = op_alloc(ORANGEFS_VFS_OP_SYMLINK);
    if new_op.is_null() { return -ENOMEM; }
    (*new_op).upcall.req.sym.parent_refn = (*parent).refn;
    fill_default_sys_attrs((*new_op).upcall.req.sym.attributes, 0o755);
    strscpy((*new_op).upcall.req.sym.entry_name, (*dentry).d_name.name);
    strscpy((*new_op).upcall.req.sym.target, symname);
    let mut ret = service_operation(new_op, __func__, get_interruptible_flag(dir));
    if ret >= 0 {
        let reference = (*new_op).downcall.resp.sym.refn;
        let inode = orangefs_new_inode((*dir).i_sb, dir, S_IFLNK | 0o755, 0, &reference);
        if IS_ERR(inode) { ret = PTR_ERR(inode); } else {
            (*inode).i_size = strlen(symname) as _;
            d_instantiate_new(dentry, inode);
            orangefs_set_timeout(dentry);
            let mut iattr: iattr = core::mem::zeroed();
            iattr.ia_valid |= ATTR_MTIME | ATTR_CTIME;
            iattr.ia_mtime = current_time(dir);
            iattr.ia_ctime = iattr.ia_mtime;
            __orangefs_setattr(dir, &mut iattr);
            ret = 0;
        }
    }
    op_release(new_op);
    ret
}

unsafe fn orangefs_mkdir(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    let parent = ORANGEFS_I(dir);
    let new_op = op_alloc(ORANGEFS_VFS_OP_MKDIR);
    if new_op.is_null() { return ERR_PTR(-ENOMEM); }
    (*new_op).upcall.req.mkdir.parent_refn = (*parent).refn;
    fill_default_sys_attrs((*new_op).upcall.req.mkdir.attributes, mode);
    strscpy((*new_op).upcall.req.mkdir.d_name, (*dentry).d_name.name);
    let ret = service_operation(new_op, __func__, get_interruptible_flag(dir));
    if ret >= 0 {
        let reference = (*new_op).downcall.resp.mkdir.refn;
        let inode = orangefs_new_inode((*dir).i_sb, dir, mode, 0, &reference);
        if !IS_ERR(inode) {
            d_instantiate_new(dentry, inode);
            orangefs_set_timeout(dentry);
            let mut iattr: iattr = core::mem::zeroed();
            iattr.ia_valid |= ATTR_MTIME | ATTR_CTIME;
            iattr.ia_mtime = current_time(dir);
            iattr.ia_ctime = iattr.ia_mtime;
            __orangefs_setattr(dir, &mut iattr);
        }
    }
    op_release(new_op);
    if ret != 0 { ERR_PTR(ret) } else { core::ptr::null_mut() }
}

unsafe fn orangefs_rename(idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: c_uint) -> c_int {
    if flags != 0 { return -EINVAL; }
    let mut iattr: iattr = core::mem::zeroed();
    iattr.ia_valid |= ATTR_MTIME | ATTR_CTIME;
    iattr.ia_mtime = current_time(new_dir);
    iattr.ia_ctime = iattr.ia_mtime;
    __orangefs_setattr(new_dir, &mut iattr);
    let new_op = op_alloc(ORANGEFS_VFS_OP_RENAME);
    if new_op.is_null() { return -EINVAL; }
    (*new_op).upcall.req.rename.old_parent_refn = (*ORANGEFS_I(old_dir)).refn;
    (*new_op).upcall.req.rename.new_parent_refn = (*ORANGEFS_I(new_dir)).refn;
    strscpy((*new_op).upcall.req.rename.d_old_name, (*old_dentry).d_name.name);
    strscpy((*new_op).upcall.req.rename.d_new_name, (*new_dentry).d_name.name);
    let ret = service_operation(new_op, "orangefs_rename", get_interruptible_flag((*old_dentry).d_inode));
    if !(*new_dentry).d_inode.is_null() { inode_set_ctime_current(d_inode((*new_dentry).d_inode)); }
    op_release(new_op);
    ret
}

/* ORANGEFS implementation of VFS inode operations for directories. */
pub static orangefs_dir_inode_operations: inode_operations = inode_operations {
    lookup: Some(orangefs_lookup), get_inode_acl: Some(orangefs_get_acl),
    set_acl: Some(orangefs_set_acl), create: Some(orangefs_create),
    unlink: Some(orangefs_unlink), symlink: Some(orangefs_symlink),
    mkdir: Some(orangefs_mkdir), rmdir: Some(orangefs_unlink),
    rename: Some(orangefs_rename), setattr: Some(orangefs_setattr),
    getattr: Some(orangefs_getattr), listxattr: Some(orangefs_listxattr),
    permission: Some(orangefs_permission), update_time: Some(orangefs_update_time),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
