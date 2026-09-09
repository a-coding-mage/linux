// SPDX-License-Identifier: GPL-2.0-only
/*
 * fs/kernfs/symlink.c - kernfs symlink implementation
 *
 * Copyright (c) 2001-3 Patrick Mochel
 * Copyright (c) 2007 SUSE Linux Products GmbH
 * Copyright (c) 2007, 2013 Tejun Heo <tj@kernel.org>
 */

// Dependencies supplied by the surrounding kernel translation.

/// kernfs_create_link - create a symlink
/// @parent: directory to create the symlink in
/// @name: name of the symlink
/// @target: target node for the symlink to point to
///
/// Return: the created node on success, ERR_PTR() value on error.
/// Ownership of the link matches ownership of the target.
pub unsafe fn kernfs_create_link(
    parent: *mut kernfs_node,
    name: *const core::ffi::c_char,
    target: *mut kernfs_node,
) -> *mut kernfs_node {
    let mut kn: *mut kernfs_node;
    let mut error: i32;
    let mut uid: kuid_t = GLOBAL_ROOT_UID;
    let mut gid: kgid_t = GLOBAL_ROOT_GID;

    if !(*target).iattr.is_null() {
        uid = (*(*target).iattr).ia_uid;
        gid = (*(*target).iattr).ia_gid;
    }

    kn = kernfs_new_node(parent, name, S_IFLNK | 0o777, uid, gid, KERNFS_LINK);
    if kn.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    if kernfs_ns_enabled(parent) {
        (*kn).ns = (*target).ns;
    }
    (*kn).symlink.target_kn = target;
    kernfs_get(target); /* ref owned by symlink */

    error = kernfs_add_one(kn);
    if error == 0 {
        return kn;
    }

    kernfs_put(kn);
    ERR_PTR(error)
}

unsafe fn kernfs_get_target_path(
    parent: *mut kernfs_node,
    target: *mut kernfs_node,
    path: *mut core::ffi::c_char,
) -> i32 {
    let mut base: *mut kernfs_node;
    let mut kn: *mut kernfs_node;
    let mut s = path;
    let mut len: i32 = 0;

    /* go up to the root, stop at the base */
    base = parent;
    while !kernfs_parent(base).is_null() {
        kn = kernfs_parent(target);
        while !kernfs_parent(kn).is_null() && base != kn {
            kn = kernfs_parent(kn);
        }

        if base == kn {
            break;
        }

        if (s.offset_from(path) + 3) >= PATH_MAX as isize {
            return -ENAMETOOLONG;
        }

        core::ptr::copy_nonoverlapping(b"../\0".as_ptr() as *const core::ffi::c_char, s, 4);
        s = s.add(3);
        base = kernfs_parent(base);
    }

    /* determine end of target string for reverse fillup */
    kn = target;
    while !kernfs_parent(kn).is_null() && kn != base {
        len += strlen(kernfs_rcu_name(kn)) as i32 + 1;
        kn = kernfs_parent(kn);
    }

    /* check limits */
    if len < 2 {
        return -EINVAL;
    }
    len -= 1;
    if (s.offset_from(path) + len as isize) >= PATH_MAX as isize {
        return -ENAMETOOLONG;
    }

    /* reverse fillup of target string from target to base */
    kn = target;
    while !kernfs_parent(kn).is_null() && kn != base {
        let name = kernfs_rcu_name(kn);
        let slen = strlen(name) as i32;

        len -= slen;
        memcpy(s.add(len as usize), name, slen as usize);
        if len != 0 {
            len -= 1;
            *s.add(len as usize) = b'/' as core::ffi::c_char;
        }

        kn = kernfs_parent(kn);
    }

    0
}

unsafe fn kernfs_getlink(inode: *mut inode, path: *mut core::ffi::c_char) -> i32 {
    let kn = (*inode).i_private as *mut kernfs_node;
    let parent: *mut kernfs_node;
    let target = (*kn).symlink.target_kn;
    let root = kernfs_root(kn);
    let error: i32;

    down_read(&mut (*root).kernfs_rwsem);
    parent = kernfs_parent(kn);
    error = kernfs_get_target_path(parent, target, path);
    up_read(&mut (*root).kernfs_rwsem);

    error
}

unsafe fn kernfs_iop_get_link(
    dentry: *mut dentry,
    inode: *mut inode,
    done: *mut delayed_call,
) -> *const core::ffi::c_char {
    let body: *mut core::ffi::c_char;
    let error: i32;

    if dentry.is_null() {
        return ERR_PTR(-ECHILD);
    }
    body = kzalloc(PAGE_SIZE, GFP_KERNEL);
    if body.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    error = kernfs_getlink(inode, body);
    if error < 0 {
        kfree(body as *mut core::ffi::c_void);
        return ERR_PTR(error);
    }
    set_delayed_call(done, kfree_link, body);
    body
}

pub static kernfs_symlink_iops: inode_operations = inode_operations {
    listxattr: Some(kernfs_iop_listxattr),
    get_link: Some(kernfs_iop_get_link),
    setattr: Some(kernfs_iop_setattr),
    getattr: Some(kernfs_iop_getattr),
    permission: Some(kernfs_iop_permission),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
