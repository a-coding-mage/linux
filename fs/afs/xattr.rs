// SPDX-License-Identifier: GPL-2.0-or-later
/* Extended attribute handling for AFS.  We use xattrs to get and set metadata
 * instead of providing pioctl().
 *
 * Copyright (C) 2017 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel/AFS translation.

/* Deal with the result of a successful fetch ACL operation. */
unsafe fn afs_acl_success(op: *mut afs_operation) {
    afs_vnode_commit_status(op, &mut (*op).file[0]);
}

unsafe fn afs_acl_put(op: *mut afs_operation) {
    kfree((*op).acl);
}

static afs_fetch_acl_operation: afs_operation_ops = afs_operation_ops {
    issue_afs_rpc: Some(afs_fs_fetch_acl),
    success: Some(afs_acl_success),
    put: Some(afs_acl_put),
};

/* Get a file's ACL. */
unsafe fn afs_xattr_get_acl(
    _handler: *const xattr_handler, _dentry: *mut dentry, inode: *mut inode,
    _name: *const c_char, buffer: *mut c_void, size: usize,
) -> c_int {
    let vnode = AFS_FS_I(inode);
    let mut op = afs_alloc_operation(core::ptr::null_mut(), (*vnode).volume);
    if IS_ERR(op) { return -ENOMEM; }

    afs_op_set_vnode(op, 0, vnode);
    (*op).ops = &afs_fetch_acl_operation;
    afs_begin_vnode_operation(op);
    afs_wait_for_operation(op);
    let acl = (*op).acl;
    (*op).acl = core::ptr::null_mut();
    let mut ret = afs_put_operation(op);

    if ret == 0 {
        ret = (*acl).size as c_int;
        if size > 0 {
            if (*acl).size <= size {
                memcpy(buffer, (*acl).data.as_ptr() as *const c_void, (*acl).size);
            } else { ret = -ERANGE; }
        }
    }
    kfree(acl);
    ret
}

unsafe fn afs_make_acl(op: *mut afs_operation, buffer: *const c_void, size: usize) -> bool {
    let acl = kmalloc_flex::<afs_acl>(size);
    if acl.is_null() { afs_op_nomem(op); return false; }
    (*acl).size = size;
    memcpy((*acl).data.as_mut_ptr() as *mut c_void, buffer, size);
    (*op).acl = acl;
    true
}

static afs_store_acl_operation: afs_operation_ops = afs_operation_ops {
    issue_afs_rpc: Some(afs_fs_store_acl), success: Some(afs_acl_success), put: Some(afs_acl_put),
};

/* Set a file's AFS3 ACL. */
unsafe fn afs_xattr_set_acl(
    _handler: *const xattr_handler, _idmap: *mut mnt_idmap, _dentry: *mut dentry,
    inode: *mut inode, _name: *const c_char, buffer: *const c_void, size: usize, flags: c_int,
) -> c_int {
    let vnode = AFS_FS_I(inode);
    if flags == XATTR_CREATE { return -EINVAL; }
    let mut op = afs_alloc_operation(core::ptr::null_mut(), (*vnode).volume);
    if IS_ERR(op) { return -ENOMEM; }
    afs_op_set_vnode(op, 0, vnode);
    if !afs_make_acl(op, buffer, size) { return afs_put_operation(op); }
    (*op).ops = &afs_store_acl_operation;
    afs_do_sync_operation(op)
}

static afs_xattr_afs_acl_handler: xattr_handler = xattr_handler {
    name: "afs.acl\0", prefix: core::ptr::null(), get: Some(afs_xattr_get_acl), set: Some(afs_xattr_set_acl),
};

static yfs_fetch_opaque_acl_operation: afs_operation_ops = afs_operation_ops {
    issue_yfs_rpc: Some(yfs_fs_fetch_opaque_acl), success: Some(afs_acl_success),
    // Don't free op->yacl in .put here.
    put: None,
};

/* Get a file's YFS ACL. */
unsafe fn afs_xattr_get_yfs(
    _handler: *const xattr_handler, _dentry: *mut dentry, inode: *mut inode,
    name: *const c_char, buffer: *mut c_void, size: usize,
) -> c_int {
    let vnode = AFS_FS_I(inode);
    let mut buf = [0i8; 16];
    let mut which = 0;
    let mut ret = -ENOMEM;
    if strcmp(name, c"acl".as_ptr()) == 0 { which = 0; }
    else if strcmp(name, c"acl_inherited".as_ptr()) == 0 { which = 1; }
    else if strcmp(name, c"acl_num_cleaned".as_ptr()) == 0 { which = 2; }
    else if strcmp(name, c"vol_acl".as_ptr()) == 0 { which = 3; }
    else { return -EOPNOTSUPP; }

    let yacl = kzalloc_obj::<yfs_acl>();
    if yacl.is_null() { return ret; }
    if which == 0 { (*yacl).flags |= YFS_ACL_WANT_ACL; }
    else if which == 3 { (*yacl).flags |= YFS_ACL_WANT_VOL_ACL; }
    let op = afs_alloc_operation(core::ptr::null_mut(), (*vnode).volume);
    if IS_ERR(op) { yfs_free_opaque_acl(yacl); return ret; }
    afs_op_set_vnode(op, 0, vnode); (*op).yacl = yacl; (*op).ops = &yfs_fetch_opaque_acl_operation;
    afs_begin_vnode_operation(op); afs_wait_for_operation(op); ret = afs_put_operation(op);
    if ret == 0 {
        let (data, dsize) = match which {
            0 => ((*yacl).acl.data.as_ptr(), (*yacl).acl.size as c_int),
            1 => (buf.as_ptr(), scnprintf(buf.as_mut_ptr(), 16, c"%u".as_ptr(), (*yacl).inherit_flag)),
            2 => (buf.as_ptr(), scnprintf(buf.as_mut_ptr(), 16, c"%u".as_ptr(), (*yacl).num_cleaned)),
            3 => ((*yacl).vol_acl.data.as_ptr(), (*yacl).vol_acl.size as c_int),
            _ => { yfs_free_opaque_acl(yacl); return -EOPNOTSUPP; }
        };
        ret = dsize; if size > 0 { if (dsize as usize) <= size { memcpy(buffer, data as *const c_void, dsize as usize); } else { ret = -ERANGE; } }
    } else if ret == -ENOTSUPP { ret = -ENODATA; }
    yfs_free_opaque_acl(yacl); ret
}

static yfs_store_opaque_acl2_operation: afs_operation_ops = afs_operation_ops {
    issue_yfs_rpc: Some(yfs_fs_store_opaque_acl2), success: Some(afs_acl_success), put: Some(afs_acl_put),
};

/* Set a file's YFS ACL. */
unsafe fn afs_xattr_set_yfs(
    _handler: *const xattr_handler, _idmap: *mut mnt_idmap, _dentry: *mut dentry,
    inode: *mut inode, name: *const c_char, buffer: *const c_void, size: usize, flags: c_int,
) -> c_int {
    let vnode = AFS_FS_I(inode);
    if flags == XATTR_CREATE || strcmp(name, c"acl".as_ptr()) != 0 { return -EINVAL; }
    let mut op = afs_alloc_operation(core::ptr::null_mut(), (*vnode).volume);
    if IS_ERR(op) { return -ENOMEM; }
    afs_op_set_vnode(op, 0, vnode);
    if !afs_make_acl(op, buffer, size) { return afs_put_operation(op); }
    (*op).ops = &yfs_store_opaque_acl2_operation;
    let mut ret = afs_do_sync_operation(op); if ret == -ENOTSUPP { ret = -ENODATA; } ret
}

static afs_xattr_yfs_handler: xattr_handler = xattr_handler {
    name: core::ptr::null(), prefix: "afs.yfs.\0", get: Some(afs_xattr_get_yfs), set: Some(afs_xattr_set_yfs),
};

/* Get the name of the cell on which a file resides. */
unsafe fn afs_xattr_get_cell(_handler: *const xattr_handler, _dentry: *mut dentry, inode: *mut inode, _name: *const c_char, buffer: *mut c_void, size: usize) -> c_int {
    let cell = (*(*AFS_FS_I(inode)).volume).cell; let namelen = (*cell).name_len;
    if size == 0 { return namelen as c_int; } if namelen > size { return -ERANGE; }
    memcpy(buffer, (*cell).name as *const c_void, namelen); namelen as c_int
}

static afs_xattr_afs_cell_handler: xattr_handler = xattr_handler { name: "afs.cell\0", prefix: core::ptr::null(), get: Some(afs_xattr_get_cell), set: None };

/* Get the volume ID, vnode ID and vnode uniquifier as hex numbers separated by colons. */
unsafe fn afs_xattr_get_fid(_handler: *const xattr_handler, _dentry: *mut dentry, inode: *mut inode, _name: *const c_char, buffer: *mut c_void, size: usize) -> c_int {
    let fid = &(*AFS_FS_I(inode)).fid; let mut text = [0i8; 16 + 1 + 24 + 1 + 8 + 1];
    let mut len = scnprintf(text.as_mut_ptr(), text.len(), c"%llx:".as_ptr(), fid.vid);
    if fid.vnode_hi != 0 { len += scnprintf(text.as_mut_ptr().add(len), text.len()-len, c"%x%016llx".as_ptr(), fid.vnode_hi, fid.vnode); }
    else { len += scnprintf(text.as_mut_ptr().add(len), text.len()-len, c"%llx".as_ptr(), fid.vnode); }
    len += scnprintf(text.as_mut_ptr().add(len), text.len()-len, c":%x".as_ptr(), fid.unique);
    if size == 0 { return len as c_int; } if len > size { return -ERANGE; }
    memcpy(buffer, text.as_ptr() as *const c_void, len); len as c_int
}

static afs_xattr_afs_fid_handler: xattr_handler = xattr_handler { name: "afs.fid\0", prefix: core::ptr::null(), get: Some(afs_xattr_get_fid), set: None };

/* Get the name of the volume on which a file resides. */
unsafe fn afs_xattr_get_volume(_handler: *const xattr_handler, _dentry: *mut dentry, inode: *mut inode, _name: *const c_char, buffer: *mut c_void, size: usize) -> c_int {
    let volname = (*(*AFS_FS_I(inode)).volume).name; let namelen = strlen(volname);
    if size == 0 { return namelen as c_int; } if namelen > size { return -ERANGE; }
    memcpy(buffer, volname as *const c_void, namelen); namelen as c_int
}

static afs_xattr_afs_volume_handler: xattr_handler = xattr_handler { name: "afs.volume\0", prefix: core::ptr::null(), get: Some(afs_xattr_get_volume), set: None };

static afs_xattr_handlers: [*const xattr_handler; 6] = [
    &afs_xattr_afs_acl_handler, &afs_xattr_afs_cell_handler, &afs_xattr_afs_fid_handler,
    &afs_xattr_afs_volume_handler, &afs_xattr_yfs_handler, core::ptr::null(),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
