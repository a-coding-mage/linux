// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Linux and local header dependencies are supplied by other translation units.

/**
 * gfs2_drevalidate - Check directory lookup consistency
 * @dir: expected parent directory inode
 * @name: expexted name
 * @dentry: dentry to check
 * @flags: lookup flags
 *
 * Check to make sure the lookup necessary to arrive at this inode from its
 * parent is still good.
 *
 * Returns: 1 if the dentry is ok, 0 if it isn't
 */
unsafe fn gfs2_drevalidate(
    dir: *mut inode,
    name: *const qstr,
    dentry: *mut dentry,
    flags: c_uint,
) -> c_int {
    let sdp: *mut gfs2_sbd = GFS2_SB(dir);
    let dip: *mut gfs2_inode = GFS2_I(dir);
    let mut inode: *mut inode;
    let mut d_gh: gfs2_holder;
    let mut ip: *mut gfs2_inode = core::ptr::null_mut();
    let error: c_int;
    let valid: c_int;
    let mut had_lock: c_int = 0;

    if flags & LOOKUP_RCU != 0 {
        return -ECHILD;
    }

    inode = d_inode(dentry);

    if !inode.is_null() {
        if is_bad_inode(inode) != 0 {
            return 0;
        }
        ip = GFS2_I(inode);
    }

    if (*(*sdp).sd_lockstruct.ls_ops).lm_mount.is_none() {
        return 1;
    }

    had_lock = (gfs2_glock_is_locked_by_me((*dip).i_gl) != core::ptr::null_mut()) as c_int;
    if had_lock == 0 {
        error = gfs2_glock_nq_init((*dip).i_gl, LM_ST_SHARED, 0, &mut d_gh);
        if error != 0 {
            return 0;
        }
    }

    error = gfs2_dir_check(dir, name, ip);
    valid = if !inode.is_null() { (error == 0) as c_int } else { (error == -ENOENT) as c_int };

    if had_lock == 0 {
        gfs2_glock_dq_uninit(&mut d_gh);
    }
    valid
}

unsafe fn gfs2_dhash(dentry: *const dentry, str_: *mut qstr) -> c_int {
    (*str_).hash = gfs2_disk_hash((*str_).name, (*str_).len);
    0
}

unsafe fn gfs2_dentry_delete(dentry: *const dentry) -> c_int {
    let ginode: *mut gfs2_inode;

    if d_really_is_negative(dentry) != 0 {
        return 0;
    }

    ginode = GFS2_I(d_inode(dentry));
    if gfs2_holder_initialized(&(*ginode).i_iopen_gh) == 0 {
        return 0;
    }

    if test_bit(GLF_DEMOTE, &(*(*ginode).i_iopen_gh.gh_gl).gl_flags) != 0 {
        return 1;
    }

    0
}

#[no_mangle]
pub static gfs2_dops: dentry_operations = dentry_operations {
    d_revalidate: Some(gfs2_drevalidate),
    d_hash: Some(gfs2_dhash),
    d_delete: Some(gfs2_dentry_delete),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
