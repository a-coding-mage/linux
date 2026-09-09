// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Linux and GFS2 dependencies supplied by the surrounding translation unit.

unsafe fn gfs2_acl_name(type_: core::ffi::c_int) -> *const core::ffi::c_char {
    match type_ {
        ACL_TYPE_ACCESS => XATTR_POSIX_ACL_ACCESS,
        ACL_TYPE_DEFAULT => XATTR_POSIX_ACL_DEFAULT,
        _ => core::ptr::null(),
    }
}

unsafe fn __gfs2_get_acl(
    inode: *mut inode,
    type_: core::ffi::c_int,
) -> *mut posix_acl {
    let ip = GFS2_I(inode);
    let mut acl: *mut posix_acl;
    let name: *const core::ffi::c_char;
    let mut data: *mut core::ffi::c_char = core::ptr::null_mut();
    let len: core::ffi::c_int;

    if !(*ip).i_eattr {
        return core::ptr::null_mut();
    }

    name = gfs2_acl_name(type_);
    len = gfs2_xattr_acl_get(ip, name, &mut data);
    if len <= 0 {
        return ERR_PTR(len as isize);
    }
    acl = posix_acl_from_xattr(&init_user_ns, data, len as usize);
    kfree(data as *mut core::ffi::c_void);
    acl
}

pub unsafe fn gfs2_get_acl(
    inode: *mut inode,
    type_: core::ffi::c_int,
    rcu: bool,
) -> *mut posix_acl {
    let ip = GFS2_I(inode);
    let mut gh: gfs2_holder;
    let mut need_unlock = false;
    let acl: *mut posix_acl;

    if rcu {
        return ERR_PTR(-ECHILD as isize);
    }

    if !gfs2_glock_is_locked_by_me((*ip).i_gl) {
        let ret = gfs2_glock_nq_init(
            (*ip).i_gl,
            LM_ST_SHARED,
            LM_FLAG_ANY,
            &mut gh,
        );
        if ret != 0 {
            return ERR_PTR(ret as isize);
        }
        need_unlock = true;
    }
    acl = __gfs2_get_acl(inode, type_);
    if need_unlock {
        gfs2_glock_dq_uninit(&mut gh);
    }
    acl
}

pub unsafe fn __gfs2_set_acl(
    inode: *mut inode,
    acl: *mut posix_acl,
    type_: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut error: core::ffi::c_int;
    let mut len: usize = 0;
    let mut data: *mut core::ffi::c_char = core::ptr::null_mut();
    let name = gfs2_acl_name(type_);

    if !acl.is_null() {
        data = posix_acl_to_xattr(&init_user_ns, acl, &mut len, GFP_NOFS);
        if data.is_null() {
            return -ENOMEM;
        }
    }

    error = __gfs2_xattr_set(inode, name, data, len, 0, GFS2_EATYPE_SYS);
    if error != 0 {
        kfree(data as *mut core::ffi::c_void);
        return error;
    }
    set_cached_acl(inode, type_, acl);
    kfree(data as *mut core::ffi::c_void);
    error
}

pub unsafe fn gfs2_set_acl(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    acl: *mut posix_acl,
    type_: core::ffi::c_int,
) -> core::ffi::c_int {
    let inode = d_inode(dentry);
    let ip = GFS2_I(inode);
    let mut gh: gfs2_holder;
    let mut need_unlock = false;
    let mut ret: core::ffi::c_int;
    let mut mode: umode_t;

    if !acl.is_null() && (*acl).a_count > GFS2_ACL_MAX_ENTRIES(GFS2_SB(inode)) {
        return -E2BIG;
    }

    ret = gfs2_qa_get(ip);
    if ret != 0 {
        return ret;
    }

    if !gfs2_glock_is_locked_by_me((*ip).i_gl) {
        ret = gfs2_glock_nq_init((*ip).i_gl, LM_ST_EXCLUSIVE, 0, &mut gh);
        if ret != 0 {
            gfs2_qa_put(ip);
            return ret;
        }
        need_unlock = true;
    }

    mode = (*inode).i_mode;
    if type_ == ACL_TYPE_ACCESS && !acl.is_null() {
        ret = posix_acl_update_mode(&nop_mnt_idmap, inode, &mut mode, &mut (acl as *mut posix_acl));
        if ret != 0 {
            if need_unlock {
                gfs2_glock_dq_uninit(&mut gh);
            }
            gfs2_qa_put(ip);
            return ret;
        }
    }

    ret = __gfs2_set_acl(inode, acl, type_);
    if ret == 0 && mode != (*inode).i_mode {
        inode_set_ctime_current(inode);
        (*inode).i_mode = mode;
        mark_inode_dirty(inode);
    }
    if need_unlock {
        gfs2_glock_dq_uninit(&mut gh);
    }
    gfs2_qa_put(ip);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
