// SPDX-License-Identifier: LGPL-2.1
/*
 * Copyright IBM Corporation, 2010
 * Author Aneesh Kumar K.V <aneesh.kumar@linux.vnet.ibm.com>
 */

// Kernel and local headers from acl.c provide the types, constants, macros,
// globals, and extern functions referenced below.

unsafe fn v9fs_fid_get_acl(fid: *mut p9_fid, name: *const c_char) -> *mut posix_acl {
    let mut size: ssize_t;
    let mut value: *mut c_void = core::ptr::null_mut();
    let mut acl: *mut posix_acl = core::ptr::null_mut();

    size = v9fs_fid_xattr_get(fid, name, core::ptr::null_mut(), 0);
    if size < 0 { return ERR_PTR(size); }
    if size == 0 { return ERR_PTR(-ENODATA); }

    value = kzalloc(size as usize, GFP_NOFS);
    if value.is_null() { return ERR_PTR(-ENOMEM); }

    size = v9fs_fid_xattr_get(fid, name, value, size);
    if size < 0 {
        acl = ERR_PTR(size);
    } else if size == 0 {
        acl = ERR_PTR(-ENODATA);
    } else {
        acl = posix_acl_from_xattr(&init_user_ns, value, size);
    }
    kfree(value);
    acl
}

unsafe fn v9fs_acl_get(dentry: *mut dentry, name: *const c_char) -> *mut posix_acl {
    let fid = v9fs_fid_lookup(dentry);
    if IS_ERR(fid) { return ERR_CAST(fid); }
    let acl = v9fs_fid_get_acl(fid, name);
    p9_fid_put(fid);
    acl
}

unsafe fn __v9fs_get_acl(fid: *mut p9_fid, name: *const c_char) -> *mut posix_acl {
    let acl = v9fs_fid_get_acl(fid, name);
    if !IS_ERR(acl) { return acl; }
    let retval = PTR_ERR(acl);
    if retval == -ENODATA || retval == -ENOSYS || retval == -EOPNOTSUPP { return core::ptr::null_mut(); }
    // map everything else to -EIO
    ERR_PTR(-EIO)
}

pub unsafe fn v9fs_get_acl(inode: *mut inode, fid: *mut p9_fid) -> c_int {
    let mut retval: c_int = 0;
    let v9ses = v9fs_inode2v9ses(inode);
    if ((unsafe { (*v9ses).flags } & V9FS_ACCESS_MASK) != V9FS_ACCESS_CLIENT) ||
       ((unsafe { (*v9ses).flags } & V9FS_ACL_MASK) != V9FS_POSIX_ACL) {
        set_cached_acl(inode, ACL_TYPE_DEFAULT, core::ptr::null_mut());
        set_cached_acl(inode, ACL_TYPE_ACCESS, core::ptr::null_mut());
        return 0;
    }
    let dacl = __v9fs_get_acl(fid, XATTR_NAME_POSIX_ACL_DEFAULT);
    let pacl = __v9fs_get_acl(fid, XATTR_NAME_POSIX_ACL_ACCESS);
    if !IS_ERR(dacl) && !IS_ERR(pacl) {
        set_cached_acl(inode, ACL_TYPE_DEFAULT, dacl);
        set_cached_acl(inode, ACL_TYPE_ACCESS, pacl);
    } else { retval = -EIO; }
    if !IS_ERR(dacl) { posix_acl_release(dacl); }
    if !IS_ERR(pacl) { posix_acl_release(pacl); }
    retval
}

unsafe fn v9fs_get_cached_acl(inode: *mut inode, type_: c_int) -> *mut posix_acl {
    // 9p Always cache the acl value when instantiating the inode (v9fs_inode_from_fid)
    let acl = get_cached_acl(inode, type_);
    BUG_ON(is_uncached_acl(acl));
    acl
}

pub unsafe fn v9fs_iop_get_inode_acl(inode: *mut inode, type_: c_int, rcu: bool) -> *mut posix_acl {
    if rcu { return ERR_PTR(-ECHILD); }
    let v9ses = v9fs_inode2v9ses(inode);
    if ((unsafe { (*v9ses).flags } & V9FS_ACCESS_MASK) != V9FS_ACCESS_CLIENT) ||
       ((unsafe { (*v9ses).flags } & V9FS_ACL_MASK) != V9FS_POSIX_ACL) { return core::ptr::null_mut(); }
    v9fs_get_cached_acl(inode, type_)
}

pub unsafe fn v9fs_iop_get_acl(_idmap: *mut mnt_idmap, dentry: *mut dentry, type_: c_int) -> *mut posix_acl {
    let v9ses = v9fs_dentry2v9ses(dentry);
    if (unsafe { (*v9ses).flags } & V9FS_ACCESS_MASK) != V9FS_ACCESS_CLIENT {
        return v9fs_acl_get(dentry, posix_acl_xattr_name(type_));
    }
    v9fs_get_cached_acl(d_inode(dentry), type_)
}

pub unsafe fn v9fs_iop_set_acl(_idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut posix_acl, type_: c_int) -> c_int {
    let inode = d_inode(dentry);
    let mut retval: c_int;
    let mut size: usize = 0;
    let mut value: *mut c_void = core::ptr::null_mut();
    if !acl.is_null() {
        retval = posix_acl_valid((*inode).i_sb.as_ref().unwrap().s_user_ns, acl);
        if retval != 0 { return retval; }
        value = posix_acl_to_xattr(&init_user_ns, acl, &mut size, GFP_NOFS);
        if value.is_null() { return -ENOMEM; }
    }
    let acl_name = posix_acl_xattr_name(type_);
    let v9ses = v9fs_dentry2v9ses(dentry);
    if ((*v9ses).flags & V9FS_ACCESS_MASK) != V9FS_ACCESS_CLIENT {
        retval = v9fs_xattr_set(dentry, acl_name, value, size, 0); kfree(value); return retval;
    }
    if S_ISLNK((*inode).i_mode) { kfree(value); return -EOPNOTSUPP; }
    if !inode_owner_or_capable(&nop_mnt_idmap, inode) { kfree(value); return -EPERM; }
    match type_ {
        ACL_TYPE_ACCESS => if !acl.is_null() {
            let mut iattr: iattr = core::mem::zeroed();
            let mut acl_mode = acl;
            retval = posix_acl_update_mode(&nop_mnt_idmap, inode, &mut iattr.ia_mode, &mut acl_mode);
            if retval != 0 { kfree(value); return retval; }
            if acl_mode.is_null() { kfree(value); value = core::ptr::null_mut(); size = 0; }
            iattr.ia_valid = ATTR_MODE;
            v9fs_vfs_setattr_dotl(&nop_mnt_idmap, dentry, &mut iattr);
        },
        ACL_TYPE_DEFAULT => if !S_ISDIR((*inode).i_mode) { retval = if !acl.is_null() { -EINVAL } else { 0 }; kfree(value); return retval; },
        _ => {}
    }
    retval = v9fs_xattr_set(dentry, acl_name, value, size, 0);
    if retval == 0 { set_cached_acl(inode, type_, acl); }
    kfree(value); retval
}

unsafe fn v9fs_set_acl(fid: *mut p9_fid, type_: c_int, acl: *mut posix_acl) -> c_int {
    if acl.is_null() { return 0; }
    let mut size: usize = 0;
    let buffer = posix_acl_to_xattr(&init_user_ns, acl, &mut size, GFP_KERNEL);
    if buffer.is_null() { return -ENOMEM; }
    let name = match type_ { ACL_TYPE_ACCESS => XATTR_NAME_POSIX_ACL_ACCESS, ACL_TYPE_DEFAULT => XATTR_NAME_POSIX_ACL_DEFAULT, _ => { BUG(); core::ptr::null() } };
    let retval = v9fs_fid_xattr_set(fid, name, buffer, size, 0);
    kfree(buffer); retval
}

pub unsafe fn v9fs_acl_chmod(inode: *mut inode, fid: *mut p9_fid) -> c_int {
    if S_ISLNK((*inode).i_mode) { return -EOPNOTSUPP; }
    let acl = v9fs_get_cached_acl(inode, ACL_TYPE_ACCESS);
    if acl.is_null() { return 0; }
    let mut aclp = acl;
    let mut retval = __posix_acl_chmod(&mut aclp, GFP_KERNEL, (*inode).i_mode);
    if retval != 0 { return retval; }
    set_cached_acl(inode, ACL_TYPE_ACCESS, aclp);
    retval = v9fs_set_acl(fid, ACL_TYPE_ACCESS, aclp);
    posix_acl_release(aclp); retval
}

pub unsafe fn v9fs_set_create_acl(inode: *mut inode, fid: *mut p9_fid, dacl: *mut posix_acl, acl: *mut posix_acl) -> c_int {
    set_cached_acl(inode, ACL_TYPE_DEFAULT, dacl); set_cached_acl(inode, ACL_TYPE_ACCESS, acl);
    v9fs_set_acl(fid, ACL_TYPE_DEFAULT, dacl); v9fs_set_acl(fid, ACL_TYPE_ACCESS, acl); 0
}

pub unsafe fn v9fs_put_acl(dacl: *mut posix_acl, acl: *mut posix_acl) { posix_acl_release(dacl); posix_acl_release(acl); }

pub unsafe fn v9fs_acl_mode(dir: *mut inode, modep: *mut umode_t, dpacl: *mut *mut posix_acl, pacl: *mut *mut posix_acl) -> c_int {
    let mut mode = *modep;
    let mut acl: *mut posix_acl = core::ptr::null_mut();
    if !S_ISLNK(mode) { acl = v9fs_get_cached_acl(dir, ACL_TYPE_DEFAULT); if IS_ERR(acl) { return PTR_ERR(acl); } if acl.is_null() { mode &= !current_umask(); } }
    if !acl.is_null() {
        if S_ISDIR(mode) { *dpacl = posix_acl_dup(acl); }
        let retval = __posix_acl_create(&mut acl, GFP_NOFS, &mut mode);
        if retval < 0 { return retval; }
        if retval > 0 { *pacl = acl; } else { posix_acl_release(acl); }
    }
    *modep = mode; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
