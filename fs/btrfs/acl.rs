// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2007 Red Hat.  All rights reserved.
 */

// Translated from acl.c.  Linux and Btrfs dependencies are supplied by other
// translation units.

use core::ffi::{c_char, c_int, c_uint, c_void};

pub unsafe fn btrfs_get_acl(
    inode: *mut inode,
    acl_type: c_int,
    rcu: bool,
) -> *mut posix_acl {
    let mut size: c_int;
    let name: *const c_char;
    let mut value: *mut c_char = core::ptr::null_mut();
    let acl: *mut posix_acl;

    if rcu {
        return ERR_PTR(-ECHILD);
    }

    match acl_type {
        ACL_TYPE_ACCESS => {
            name = XATTR_NAME_POSIX_ACL_ACCESS;
        }
        ACL_TYPE_DEFAULT => {
            name = XATTR_NAME_POSIX_ACL_DEFAULT;
        }
        _ => return ERR_PTR(-EINVAL),
    }

    size = btrfs_getxattr(inode, name, core::ptr::null_mut(), 0);
    if size > 0 {
        value = kzalloc(size as usize, GFP_KERNEL) as *mut c_char;
        if value.is_null() {
            return ERR_PTR(-ENOMEM);
        }
        size = btrfs_getxattr(inode, name, value as *mut c_void, size as usize) as c_int;
    }
    if size > 0 {
        acl = posix_acl_from_xattr(&init_user_ns, value as *const c_void, size as usize);
    } else if size == -ENODATA || size == 0 {
        acl = core::ptr::null_mut();
    } else {
        acl = ERR_PTR(size);
    }

    acl
}

pub unsafe fn __btrfs_set_acl(
    trans: *mut btrfs_trans_handle,
    inode: *mut inode,
    acl: *mut posix_acl,
    acl_type: c_int,
) -> c_int {
    let ret: c_int;
    let mut size: usize = 0;
    let name: *const c_char;
    let mut value: *mut c_char = core::ptr::null_mut();

    match acl_type {
        ACL_TYPE_ACCESS => {
            name = XATTR_NAME_POSIX_ACL_ACCESS;
        }
        ACL_TYPE_DEFAULT => {
            if !S_ISDIR((*inode).i_mode) {
                return if !acl.is_null() { -EINVAL } else { 0 };
            }
            name = XATTR_NAME_POSIX_ACL_DEFAULT;
        }
        _ => return -EINVAL,
    }

    if !acl.is_null() {
        let nofs_flag: c_uint;

        /*
         * We're holding a transaction handle, so use a NOFS memory
         * allocation context to avoid deadlock if reclaim happens.
         */
        nofs_flag = memalloc_nofs_save();
        value = posix_acl_to_xattr(&init_user_ns, acl, &mut size, GFP_KERNEL);
        memalloc_nofs_restore(nofs_flag);
        if value.is_null() {
            return -ENOMEM;
        }
    }

    if !trans.is_null() {
        ret = btrfs_setxattr(trans, inode, name, value, size, 0);
    } else {
        ret = btrfs_setxattr_trans(inode, name, value, size, 0);
    }
    if ret < 0 {
        return ret;
    }

    set_cached_acl(inode, acl_type, acl);
    0
}

pub unsafe fn btrfs_set_acl(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    acl: *mut posix_acl,
    acl_type: c_int,
) -> c_int {
    let ret: c_int;
    let inode: *mut inode = d_inode(dentry);
    let old_mode: umode_t = (*inode).i_mode;

    if btrfs_root_readonly((*BTRFS_I(inode)).root) {
        return -EROFS;
    }

    if acl_type == ACL_TYPE_ACCESS && !acl.is_null() {
        ret = posix_acl_update_mode(idmap, inode, &mut (*inode).i_mode, &mut (acl as *mut *mut posix_acl));
        if ret != 0 {
            return ret;
        }
    }
    ret = __btrfs_set_acl(core::ptr::null_mut(), inode, acl, acl_type);
    if ret != 0 {
        (*inode).i_mode = old_mode;
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
