/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// C dependency: #include "incore.h"

unsafe extern "C" {
    pub static mut gfs2_freeze_wq: *mut workqueue_struct;

    pub static gfs2_meta_glops: gfs2_glock_operations;
    pub static gfs2_inode_glops: gfs2_glock_operations;
    pub static gfs2_rgrp_glops: gfs2_glock_operations;
    pub static gfs2_freeze_glops: gfs2_glock_operations;
    pub static gfs2_iopen_glops: gfs2_glock_operations;
    pub static gfs2_flock_glops: gfs2_glock_operations;
    pub static gfs2_nondisk_glops: gfs2_glock_operations;
    pub static gfs2_quota_glops: gfs2_glock_operations;
    pub static gfs2_journal_glops: gfs2_glock_operations;
    pub static gfs2_glops_list: *const *const gfs2_glock_operations;

    pub fn gfs2_inode_metasync(gl: *mut gfs2_glock) -> ::core::ffi::c_int;
    pub fn gfs2_ail_flush(gl: *mut gfs2_glock, fsync: bool);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
