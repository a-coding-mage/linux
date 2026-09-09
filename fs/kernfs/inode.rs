// SPDX-License-Identifier: GPL-2.0-only
/*
 * fs/kernfs/inode.c - kernfs inode implementation
 *
 * Copyright (c) 2001-3 Patrick Mochel
 * Copyright (c) 2007 SUSE Linux Products GmbH
 * Copyright (c) 2007, 2013 Tejun Heo <tj@kernel.org>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

static KERNFS_IOPS: inode_operations = inode_operations {
    permission: Some(kernfs_iop_permission),
    setattr: Some(kernfs_iop_setattr),
    getattr: Some(kernfs_iop_getattr),
    listxattr: Some(kernfs_iop_listxattr),
};

unsafe fn __kernfs_iattrs(kn: *mut kernfs_node, alloc: bool) -> *mut kernfs_iattrs {
    let mut attr = READ_ONCE((*kn).iattr);
    if !attr.is_null() || !alloc { return attr; }
    let ret = kmem_cache_zalloc(kernfs_iattrs_cache, GFP_KERNEL);
    if ret.is_null() { return core::ptr::null_mut(); }
    INIT_LIST_HEAD_RCU(&mut (*ret).xattrs);
    (*ret).ia_uid = GLOBAL_ROOT_UID;
    (*ret).ia_gid = GLOBAL_ROOT_GID;
    ktime_get_real_ts64(&mut (*ret).ia_atime);
    (*ret).ia_mtime = (*ret).ia_atime;
    (*ret).ia_ctime = (*ret).ia_atime;
    simple_xattr_limits_init(&mut (*ret).xattr_limits);
    if !try_cmpxchg(&mut (*kn).iattr, &mut attr, ret) { return READ_ONCE((*kn).iattr); }
    ret
}

unsafe fn kernfs_iattrs(kn: *mut kernfs_node) -> *mut kernfs_iattrs { __kernfs_iattrs(kn, true) }
unsafe fn kernfs_iattrs_noalloc(kn: *mut kernfs_node) -> *mut kernfs_iattrs { __kernfs_iattrs(kn, false) }

pub unsafe fn __kernfs_setattr(kn: *mut kernfs_node, iattr: *const iattr) -> i32 {
    let attrs = kernfs_iattrs(kn);
    if attrs.is_null() { return -ENOMEM; }
    let valid = (*iattr).ia_valid;
    if valid & ATTR_UID != 0 { (*attrs).ia_uid = (*iattr).ia_uid; }
    if valid & ATTR_GID != 0 { (*attrs).ia_gid = (*iattr).ia_gid; }
    if valid & ATTR_ATIME != 0 { (*attrs).ia_atime = (*iattr).ia_atime; }
    if valid & ATTR_MTIME != 0 { (*attrs).ia_mtime = (*iattr).ia_mtime; }
    if valid & ATTR_CTIME != 0 { (*attrs).ia_ctime = (*iattr).ia_ctime; }
    if valid & ATTR_MODE != 0 { (*kn).mode = (*iattr).ia_mode; }
    0
}

pub unsafe fn kernfs_setattr(kn: *mut kernfs_node, iattr: *const iattr) -> i32 {
    let root = kernfs_root(kn); down_write(&mut (*root).kernfs_iattr_rwsem);
    let ret = __kernfs_setattr(kn, iattr); up_write(&mut (*root).kernfs_iattr_rwsem); ret
}

pub unsafe fn kernfs_iop_setattr(_idmap: *mut mnt_idmap, dentry: *mut dentry, iattr: *mut iattr) -> i32 {
    let inode = d_inode(dentry); let kn = (*inode).i_private as *mut kernfs_node;
    if kn.is_null() { return -EINVAL; }
    let root = kernfs_root(kn); down_write(&mut (*root).kernfs_iattr_rwsem);
    let mut error = setattr_prepare(&nop_mnt_idmap, dentry, iattr);
    if error == 0 { error = __kernfs_setattr(kn, iattr); }
    if error == 0 { setattr_copy(&nop_mnt_idmap, inode, iattr); }
    up_write(&mut (*root).kernfs_iattr_rwsem); error
}

pub unsafe fn kernfs_iop_listxattr(dentry: *mut dentry, buf: *mut c_char, size: usize) -> isize {
    let kn = kernfs_dentry_node(dentry); let attrs = kernfs_iattrs_noalloc(kn);
    if attrs.is_null() { return 0; }
    simple_xattr_list(d_inode(dentry), &(*attrs).xattrs, buf, size)
}

unsafe fn set_default_inode_attr(inode: *mut inode, mode: umode_t) { (*inode).i_mode = mode; simple_inode_init_ts(inode); }
unsafe fn set_inode_attr(inode: *mut inode, attrs: *mut kernfs_iattrs) {
    (*inode).i_uid = (*attrs).ia_uid; (*inode).i_gid = (*attrs).ia_gid;
    inode_set_atime_to_ts(inode, (*attrs).ia_atime); inode_set_mtime_to_ts(inode, (*attrs).ia_mtime); inode_set_ctime_to_ts(inode, (*attrs).ia_ctime);
}
unsafe fn kernfs_refresh_inode(kn: *mut kernfs_node, inode: *mut inode) {
    (*inode).i_mode = (*kn).mode; let attrs = kernfs_iattrs_noalloc(kn);
    if !attrs.is_null() { set_inode_attr(inode, attrs); }
    if kernfs_type(kn) == KERNFS_DIR && (*kn).flags & KERNFS_REMOVING == 0 { set_nlink(inode, (*kn).dir.subdirs + 2); }
}

pub unsafe fn kernfs_iop_getattr(_idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, _query_flags: u32) -> i32 {
    let inode = d_inode((*path).dentry); let kn = (*inode).i_private as *mut kernfs_node; let root = kernfs_root(kn);
    down_read(&(*root).kernfs_iattr_rwsem); kernfs_refresh_inode(kn, inode); generic_fillattr(&nop_mnt_idmap, request_mask, inode, stat); up_read(&(*root).kernfs_iattr_rwsem); 0
}

unsafe fn kernfs_init_inode(kn: *mut kernfs_node, inode: *mut inode) {
    kernfs_get(kn); (*inode).i_private = kn as *mut c_void; (*inode).i_mapping.a_ops = &ram_aops; (*inode).i_op = &KERNFS_IOPS; (*inode).i_generation = kernfs_gen(kn);
    set_default_inode_attr(inode, (*kn).mode); kernfs_refresh_inode(kn, inode);
    match kernfs_type(kn) { KERNFS_DIR => { (*inode).i_op = &kernfs_dir_iops; (*inode).i_fop = &kernfs_dir_fops; if (*kn).flags & KERNFS_EMPTY_DIR != 0 { make_empty_dir_inode(inode); } }, KERNFS_FILE => { (*inode).i_size = (*kn).attr.size; (*inode).i_fop = &kernfs_file_fops; }, KERNFS_LINK => (*inode).i_op = &kernfs_symlink_iops, _ => BUG() }
    unlock_new_inode(inode);
}

pub unsafe fn kernfs_get_inode(sb: *mut super_block, kn: *mut kernfs_node) -> *mut inode { let inode = iget_locked(sb, kernfs_ino(kn)); if !inode.is_null() && inode_state_read_once(inode) & I_NEW != 0 { kernfs_init_inode(kn, inode); } inode }
pub unsafe fn kernfs_evict_inode(inode: *mut inode) { let kn = (*inode).i_private as *mut kernfs_node; truncate_inode_pages_final(&mut (*inode).i_data); clear_inode(inode); kernfs_put(kn); }
pub unsafe fn kernfs_iop_permission(_idmap: *mut mnt_idmap, inode: *mut inode, mask: i32) -> i32 { if mask & MAY_NOT_BLOCK != 0 { return -ECHILD; } let kn = (*inode).i_private as *mut kernfs_node; let root = kernfs_root(kn); down_read(&(*root).kernfs_iattr_rwsem); kernfs_refresh_inode(kn, inode); let ret = generic_permission(&nop_mnt_idmap, inode, mask); up_read(&(*root).kernfs_iattr_rwsem); ret }

pub unsafe fn kernfs_xattr_get(kn: *mut kernfs_node, name: *const c_char, value: *mut c_void, size: usize) -> i32 { let attrs = kernfs_iattrs_noalloc(kn); if attrs.is_null() { return -ENODATA; } simple_xattr_get(&mut (*kernfs_root(kn)).xa_cache, &(*attrs).xattrs, name, value, size) }
pub unsafe fn kernfs_xattr_set(kn: *mut kernfs_node, name: *const c_char, value: *const c_void, size: usize, flags: i32) -> i32 { let attrs = kernfs_iattrs(kn); if attrs.is_null() { return -ENOMEM; } let old = simple_xattr_set(&mut (*kernfs_root(kn)).xa_cache, &mut (*attrs).xattrs, name, value, size, flags); if IS_ERR(old) { return PTR_ERR(old); } simple_xattr_free_rcu(old); 0 }

unsafe fn kernfs_vfs_xattr_get(handler: *const xattr_handler, _unused: *mut dentry, inode: *mut inode, suffix: *const c_char, value: *mut c_void, size: usize) -> i32 { kernfs_xattr_get((*inode).i_private as *mut kernfs_node, xattr_full_name(handler, suffix), value, size) }
unsafe fn kernfs_vfs_xattr_set(handler: *const xattr_handler, _idmap: *mut mnt_idmap, _unused: *mut dentry, inode: *mut inode, suffix: *const c_char, value: *const c_void, size: usize, flags: i32) -> i32 { kernfs_xattr_set((*inode).i_private as *mut kernfs_node, xattr_full_name(handler, suffix), value, size, flags) }
unsafe fn kernfs_vfs_user_xattr_set(handler: *const xattr_handler, _idmap: *mut mnt_idmap, _unused: *mut dentry, inode: *mut inode, suffix: *const c_char, value: *const c_void, size: usize, flags: i32) -> i32 {
    let kn = (*inode).i_private as *mut kernfs_node; if (*kernfs_root(kn)).flags & KERNFS_ROOT_SUPPORT_USER_XATTR == 0 { return -EOPNOTSUPP; }
    let attrs = kernfs_iattrs(kn); if attrs.is_null() { return -ENOMEM; }
    simple_xattr_set_limited(&mut (*kernfs_root(kn)).xa_cache, &mut (*attrs).xattrs, &mut (*attrs).xattr_limits, xattr_full_name(handler, suffix), value, size, flags)
}

static KERNFS_TRUSTED_XATTR_HANDLER: xattr_handler = xattr_handler { prefix: XATTR_TRUSTED_PREFIX, get: Some(kernfs_vfs_xattr_get), set: Some(kernfs_vfs_xattr_set) };
static KERNFS_SECURITY_XATTR_HANDLER: xattr_handler = xattr_handler { prefix: XATTR_SECURITY_PREFIX, get: Some(kernfs_vfs_xattr_get), set: Some(kernfs_vfs_xattr_set) };
static KERNFS_USER_XATTR_HANDLER: xattr_handler = xattr_handler { prefix: XATTR_USER_PREFIX, get: Some(kernfs_vfs_xattr_get), set: Some(kernfs_vfs_user_xattr_set) };
pub static KERNFS_XATTR_HANDLERS: [*const xattr_handler; 4] = [&KERNFS_TRUSTED_XATTR_HANDLER, &KERNFS_SECURITY_XATTR_HANDLER, &KERNFS_USER_XATTR_HANDLER, core::ptr::null()];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
