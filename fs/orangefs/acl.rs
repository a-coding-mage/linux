// SPDX-License-Identifier: GPL-2.0
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * See COPYING in top-level directory.
 */

// Dependencies supplied by protocol.h, orangefs-kernel.h,
// orangefs-bufmap.h, and linux/posix_acl_xattr.h remain external.

pub unsafe fn orangefs_get_acl(
    inode: *mut inode,
    r#type: i32,
    rcu: bool,
) -> *mut posix_acl {
    let mut acl: *mut posix_acl;
    let ret: i32;
    let mut key: *mut i8 = core::ptr::null_mut();
    let mut value: *mut i8 = core::ptr::null_mut();

    if rcu {
        return ERR_PTR(-ECHILD);
    }

    match r#type {
        ACL_TYPE_ACCESS => {
            key = XATTR_NAME_POSIX_ACL_ACCESS as *mut i8;
        }
        ACL_TYPE_DEFAULT => {
            key = XATTR_NAME_POSIX_ACL_DEFAULT as *mut i8;
        }
        _ => {
            gossip_err(c_str!("orangefs_get_acl: bogus value of type %d\n"), r#type);
            return ERR_PTR(-EINVAL);
        }
    }
    /*
     * Rather than incurring a network call just to determine the exact
     * length of the attribute, I just allocate a max length to save on
     * the network call. Conceivably, we could pass NULL to
     * orangefs_inode_getxattr() to probe the length of the value, but
     * I don't do that for now.
     */
    value = kmalloc(ORANGEFS_MAX_XATTR_VALUELEN, GFP_KERNEL);
    if value.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    gossip_debug(
        GOSSIP_ACL_DEBUG,
        c_str!("inode %pU, key %s, type %d\n"),
        get_khandle_from_ino(inode),
        key,
        r#type,
    );
    ret = orangefs_inode_getxattr(inode, key, value, ORANGEFS_MAX_XATTR_VALUELEN);
    /* if the key exists, convert it to an in-memory rep */
    if ret > 0 {
        acl = posix_acl_from_xattr(&init_user_ns, value, ret);
    } else if ret == -ENODATA || ret == -ENOSYS {
        acl = core::ptr::null_mut();
    } else {
        gossip_err(
            c_str!("inode %pU retrieving acl's failed with error %d\n"),
            get_khandle_from_ino(inode),
            ret,
        );
        acl = ERR_PTR(ret);
    }
    /* kfree(NULL) is safe, so don't worry if value ever got used */
    kfree(value);
    acl
}

pub unsafe fn __orangefs_set_acl(
    inode: *mut inode,
    acl: *mut posix_acl,
    r#type: i32,
) -> i32 {
    let mut error: i32 = 0;
    let mut value: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut size: usize = 0;
    let mut name: *const i8 = core::ptr::null();

    match r#type {
        ACL_TYPE_ACCESS => name = XATTR_NAME_POSIX_ACL_ACCESS,
        ACL_TYPE_DEFAULT => name = XATTR_NAME_POSIX_ACL_DEFAULT,
        _ => {
            gossip_err(c_str!("__orangefs_set_acl: invalid type %d!\n"), r#type);
            return -EINVAL;
        }
    }

    gossip_debug(
        GOSSIP_ACL_DEBUG,
        c_str!("__orangefs_set_acl: inode %pU, key %s type %d\n"),
        get_khandle_from_ino(inode),
        name,
        r#type,
    );

    if !acl.is_null() {
        value = posix_acl_to_xattr(&init_user_ns, acl, &mut size, GFP_KERNEL);
        if value.is_null() {
            return -ENOMEM;
        }
    }

    gossip_debug(
        GOSSIP_ACL_DEBUG,
        c_str!("__orangefs_set_acl: name %s, value %p, size %zd, acl %p\n"),
        name,
        value,
        size,
        acl,
    );
    /*
     * Go ahead and set the extended attribute now. NOTE: Suppose acl
     * was NULL, then value will be NULL and size will be 0 and that
     * will xlate to a removexattr. However, we don't want removexattr
     * complain if attributes does not exist.
     */
    error = orangefs_inode_setxattr(inode, name, value, size, 0);

    kfree(value);
    if error == 0 {
        set_cached_acl(inode, r#type, acl);
    }
    error
}

pub unsafe fn orangefs_set_acl(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    mut acl: *mut posix_acl,
    r#type: i32,
) -> i32 {
    let mut error: i32;
    let mut iattr: iattr = core::mem::zeroed();
    let mut rc: i32;
    let inode: *mut inode = d_inode(dentry);

    let _ = idmap;

    if r#type == ACL_TYPE_ACCESS && !acl.is_null() {
        /*
         * posix_acl_update_mode checks to see if the permissions
         * described by the ACL can be encoded into the object's mode.
         * If so, it sets "acl" to NULL and "mode" to the new desired
         * value. It is up to us to propagate the new mode back to the
         * server...
         */
        error = posix_acl_update_mode(
            &nop_mnt_idmap,
            inode,
            &mut iattr.ia_mode,
            &mut acl,
        );
        if error != 0 {
            gossip_err(c_str!("orangefs_set_acl: posix_acl_update_mode err: %d\n"), error);
            return error;
        }

        if (*inode).i_mode != iattr.ia_mode {
            iattr.ia_valid = ATTR_MODE;
        }
    }

    rc = __orangefs_set_acl(inode, acl, r#type);

    if rc == 0 && iattr.ia_valid == ATTR_MODE {
        rc = __orangefs_setattr_mode(dentry, &mut iattr);
    }

    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
