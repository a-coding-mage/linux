// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) International Business Machines  Corp., 2002-2004
 *   Copyright (C) Andreas Gruenbacher, 2001
 *   Copyright (C) Linus Torvalds, 1991, 1992
 */

// Dependencies supplied by the surrounding kernel/JFS translation.

pub unsafe fn jfs_get_acl(inode: *mut inode, type_: c_int, rcu: bool) -> *mut posix_acl {
    let mut acl: *mut posix_acl;
    let ea_name: *mut c_char;
    let mut size: c_int;
    let mut value: *mut c_char = core::ptr::null_mut();

    if rcu {
        return ERR_PTR(-ECHILD);
    }

    ea_name = match type_ {
        ACL_TYPE_ACCESS => XATTR_NAME_POSIX_ACL_ACCESS,
        ACL_TYPE_DEFAULT => XATTR_NAME_POSIX_ACL_DEFAULT,
        _ => return ERR_PTR(-EINVAL),
    };

    size = __jfs_getxattr(inode, ea_name, core::ptr::null_mut(), 0);

    if size > 0 {
        value = kmalloc(size as usize, GFP_KERNEL);
        if value.is_null() {
            return ERR_PTR(-ENOMEM);
        }
        size = __jfs_getxattr(inode, ea_name, value, size);
    }

    if size < 0 {
        if size == -ENODATA {
            acl = core::ptr::null_mut();
        } else {
            acl = ERR_PTR(size);
        }
    } else {
        acl = posix_acl_from_xattr(&init_user_ns, value, size as usize);
    }
    kfree(value);
    acl
}

unsafe fn __jfs_set_acl(
    tid: tid_t,
    inode: *mut inode,
    type_: c_int,
    acl: *mut posix_acl,
) -> c_int {
    let ea_name: *mut c_char;
    let rc: c_int;
    let mut size: usize = 0;
    let mut value: *mut c_char = core::ptr::null_mut();

    ea_name = match type_ {
        ACL_TYPE_ACCESS => XATTR_NAME_POSIX_ACL_ACCESS,
        ACL_TYPE_DEFAULT => XATTR_NAME_POSIX_ACL_DEFAULT,
        _ => return -EINVAL,
    };

    if !acl.is_null() {
        value = posix_acl_to_xattr(&init_user_ns, acl, &mut size, GFP_KERNEL);
        if value.is_null() {
            return -ENOMEM;
        }
    }
    rc = __jfs_setxattr(tid, inode, ea_name, value, size, 0);
    kfree(value);

    if rc == 0 {
        set_cached_acl(inode, type_, acl);
    }

    rc
}

pub unsafe fn jfs_set_acl(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    acl: *mut posix_acl,
    type_: c_int,
) -> c_int {
    let rc: c_int;
    let tid: tid_t;
    let mut update_mode: c_int = 0;
    let inode: *mut inode = d_inode(dentry);
    let mut mode: umode_t = (*inode).i_mode;

    let _ = idmap;
    tid = txBegin((*inode).i_sb, 0);
    mutex_lock(&mut (*JFS_IP(inode)).commit_mutex);
    if type_ == ACL_TYPE_ACCESS && !acl.is_null() {
        rc = posix_acl_update_mode(&nop_mnt_idmap, inode, &mut mode, &mut (acl as *mut *mut posix_acl));
        if rc != 0 {
            txEnd(tid);
            mutex_unlock(&mut (*JFS_IP(inode)).commit_mutex);
            return rc;
        }
        if mode != (*inode).i_mode {
            update_mode = 1;
        }
    }
    rc = __jfs_set_acl(tid, inode, type_, acl);
    if rc == 0 {
        if update_mode != 0 {
            (*inode).i_mode = mode;
            inode_set_ctime_current(inode);
            mark_inode_dirty(inode);
        }
        rc = txCommit(tid, 1, &mut (inode as *mut inode), 0);
    }
    txEnd(tid);
    mutex_unlock(&mut (*JFS_IP(inode)).commit_mutex);
    rc
}

pub unsafe fn jfs_init_acl(tid: tid_t, inode: *mut inode, dir: *mut inode) -> c_int {
    let mut default_acl: *mut posix_acl;
    let mut acl: *mut posix_acl;
    let mut rc: c_int = 0;

    rc = posix_acl_create(dir, &mut (*inode).i_mode, &mut default_acl, &mut acl);
    if rc != 0 {
        return rc;
    }

    if !default_acl.is_null() {
        rc = __jfs_set_acl(tid, inode, ACL_TYPE_DEFAULT, default_acl);
        posix_acl_release(default_acl);
    } else {
        (*inode).i_default_acl = core::ptr::null_mut();
    }

    if !acl.is_null() {
        if rc == 0 {
            rc = __jfs_set_acl(tid, inode, ACL_TYPE_ACCESS, acl);
        }
        posix_acl_release(acl);
    } else {
        (*inode).i_acl = core::ptr::null_mut();
    }

    (*JFS_IP(inode)).mode2 = ((*JFS_IP(inode)).mode2 & 0xffff0000) | (*inode).i_mode;

    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
