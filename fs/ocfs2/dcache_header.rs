/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * dcache.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// Original C header guard: OCFS2_DCACHE_H

extern "C" {
    pub static ocfs2_dentry_ops: dentry_operations;
}

#[repr(C)]
pub struct ocfs2_dentry_lock {
    pub dl_count: ::core::ffi::c_uint,
    pub dl_parent_blkno: u64,

    /*
     * The ocfs2_dentry_lock keeps an inode reference until
     * dl_lockres has been destroyed. This is usually done in
     * ->d_iput() anyway, so there should be minimal impact.
     */
    pub dl_inode: *mut inode,
    pub dl_lockres: ocfs2_lock_res,
}

extern "C" {
    pub fn ocfs2_dentry_attach_lock(
        dentry: *mut dentry,
        inode: *mut inode,
        parent_blkno: u64,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_dentry_lock_put(
        osb: *mut ocfs2_super,
        dl: *mut ocfs2_dentry_lock,
    );

    pub fn ocfs2_find_local_alias(
        inode: *mut inode,
        parent_blkno: u64,
        skip_unhashed: ::core::ffi::c_int,
    ) -> *mut dentry;

    pub fn ocfs2_dentry_move(
        dentry: *mut dentry,
        target: *mut dentry,
        old_dir: *mut inode,
        new_dir: *mut inode,
    );

    pub static mut dentry_attach_lock: spinlock_t;

    pub fn ocfs2_dentry_attach_gen(dentry: *mut dentry);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
