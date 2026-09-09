// SPDX-License-Identifier: GPL-2.0
/*
 * FUSE: Filesystem in Userspace
 * Copyright (C) 2016 Canonical Ltd. <seth.forshee@canonical.com>
 */

// Dependencies supplied by the surrounding kernel/FUSE translation unit.

unsafe fn __fuse_get_acl(
    fc: *mut fuse_conn,
    inode: *mut inode,
    type_: i32,
    rcu: bool,
) -> *mut posix_acl {
    let mut size: i32;
    let name: *const core::ffi::c_char;
    let mut value: *mut core::ffi::c_void = core::ptr::null_mut();
    let acl: *mut posix_acl;

    if rcu {
        return ERR_PTR(-ECHILD);
    }

    if fuse_is_bad(inode) {
        return ERR_PTR(-EIO);
    }

    if (*fc).no_getxattr {
        return core::ptr::null_mut();
    }

    if type_ == ACL_TYPE_ACCESS {
        name = XATTR_NAME_POSIX_ACL_ACCESS;
    } else if type_ == ACL_TYPE_DEFAULT {
        name = XATTR_NAME_POSIX_ACL_DEFAULT;
    } else {
        return ERR_PTR(-EOPNOTSUPP);
    }

    value = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if value.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    size = fuse_getxattr(inode, name, value, PAGE_SIZE);
    if size > 0 {
        acl = posix_acl_from_xattr((*fc).user_ns, value, size);
    } else if (size == 0) || (size == -ENODATA) ||
        (size == -EOPNOTSUPP && (*fc).no_getxattr) {
        acl = core::ptr::null_mut();
    } else if size == -ERANGE {
        acl = ERR_PTR(-E2BIG);
    } else {
        acl = ERR_PTR(size);
    }

    kfree(value);
    acl
}

#[inline]
unsafe fn fuse_no_acl(fc: *const fuse_conn, inode: *const inode) -> bool {
    /*
     * Refuse interacting with POSIX ACLs for daemons that
     * don't support FUSE_POSIX_ACL and are not mounted on
     * the host to retain backwards compatibility.
     */
    !(*fc).posix_acl && (i_user_ns(inode) != &init_user_ns)
}

pub unsafe fn fuse_get_acl(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    type_: i32,
) -> *mut posix_acl {
    let inode = d_inode(dentry);
    let fc = get_fuse_conn(inode);

    if fuse_no_acl(fc, inode) {
        return ERR_PTR(-EOPNOTSUPP);
    }

    __fuse_get_acl(fc, inode, type_, false)
}

pub unsafe fn fuse_get_inode_acl(
    inode: *mut inode,
    type_: i32,
    rcu: bool,
) -> *mut posix_acl {
    let fc = get_fuse_conn(inode);

    /*
     * FUSE daemons before FUSE_POSIX_ACL was introduced could get and set
     * POSIX ACLs without them being used for permission checking by the
     * vfs. Retain that behavior for backwards compatibility as there are
     * filesystems that do all permission checking for acls in the daemon
     * and not in the kernel.
     */
    if !(*fc).posix_acl {
        return core::ptr::null_mut();
    }
    __fuse_get_acl(fc, inode, type_, rcu)
}

pub unsafe fn fuse_set_acl(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    acl: *mut posix_acl,
    type_: i32,
) -> i32 {
    let inode = d_inode(dentry);
    let fc = get_fuse_conn(inode);
    let name: *const core::ffi::c_char;
    let ret: i32;

    if fuse_is_bad(inode) {
        return -EIO;
    }

    if (*fc).no_setxattr || fuse_no_acl(fc, inode) {
        return -EOPNOTSUPP;
    }

    if type_ == ACL_TYPE_ACCESS {
        name = XATTR_NAME_POSIX_ACL_ACCESS;
    } else if type_ == ACL_TYPE_DEFAULT {
        name = XATTR_NAME_POSIX_ACL_DEFAULT;
    } else {
        return -EINVAL;
    }

    if !acl.is_null() {
        let mut extra_flags: u32 = 0;
        /*
         * Fuse userspace is responsible for updating access
         * permissions in the inode, if needed. fuse_setxattr
         * invalidates the inode attributes, which will force
         * them to be refreshed the next time they are used,
         * and it also updates i_ctime.
         */
        let mut size: usize = 0;
        let value: *mut core::ffi::c_void;

        value = posix_acl_to_xattr((*fc).user_ns, acl, &mut size, GFP_KERNEL);
        if value.is_null() {
            return -ENOMEM;
        }

        if size > PAGE_SIZE {
            kfree(value);
            return -E2BIG;
        }

        /*
         * Fuse daemons without FUSE_POSIX_ACL never changed the passed
         * through POSIX ACLs. Such daemons don't expect setgid bits to
         * be stripped.
         */
        if (*fc).posix_acl &&
            !in_group_or_capable(idmap, inode, i_gid_into_vfsgid(idmap, inode)) {
            extra_flags |= FUSE_SETXATTR_ACL_KILL_SGID;
        }

        ret = fuse_setxattr(inode, name, value, size, 0, extra_flags);
        kfree(value);
    } else {
        ret = fuse_removexattr(inode, name);
    }

    if (*fc).posix_acl {
        /*
         * Fuse daemons without FUSE_POSIX_ACL never cached POSIX ACLs
         * and didn't invalidate attributes. Retain that behavior.
         */
        forget_all_cached_acls(inode);
        fuse_invalidate_attr(inode);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
