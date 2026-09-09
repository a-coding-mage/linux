/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * filecheck.h
 *
 * Online file check.
 *
 * Copyright (C) 2016 SuSE.  All rights reserved.
 */

/* File check errno */
pub const OCFS2_FILECHECK_ERR_SUCCESS: i32 = 0; /* Success */
pub const OCFS2_FILECHECK_ERR_FAILED: i32 = 1000; /* Other failure */
pub const OCFS2_FILECHECK_ERR_INPROGRESS: i32 = 1001; /* In progress */
pub const OCFS2_FILECHECK_ERR_READONLY: i32 = 1002; /* Read only */
pub const OCFS2_FILECHECK_ERR_INJBD: i32 = 1003; /* Buffer in jbd */
pub const OCFS2_FILECHECK_ERR_INVALIDINO: i32 = 1004; /* Invalid ino */
pub const OCFS2_FILECHECK_ERR_BLOCKECC: i32 = 1005; /* Block ecc */
pub const OCFS2_FILECHECK_ERR_BLOCKNO: i32 = 1006; /* Block number */
pub const OCFS2_FILECHECK_ERR_VALIDFLAG: i32 = 1007; /* Inode valid flag */
pub const OCFS2_FILECHECK_ERR_GENERATION: i32 = 1008; /* Inode generation */
pub const OCFS2_FILECHECK_ERR_UNSUPPORTED: i32 = 1009; /* Unsupported */

pub const OCFS2_FILECHECK_ERR_START: i32 = OCFS2_FILECHECK_ERR_FAILED;
pub const OCFS2_FILECHECK_ERR_END: i32 = OCFS2_FILECHECK_ERR_UNSUPPORTED;

#[repr(C)]
pub struct ocfs2_filecheck {
    pub fc_head: list_head, /* File check entry list head */
    pub fc_lock: spinlock_t,
    pub fc_max: ::core::ffi::c_uint, /* Maximum number of entry in list */
    pub fc_size: ::core::ffi::c_uint, /* Current entry count in list */
    pub fc_done: ::core::ffi::c_uint, /* Finished entry count in list */
}

pub const OCFS2_FILECHECK_MAXSIZE: ::core::ffi::c_uint = 100;
pub const OCFS2_FILECHECK_MINSIZE: ::core::ffi::c_uint = 10;

/* File check operation type */
pub const OCFS2_FILECHECK_TYPE_CHK: i32 = 0; /* Check a file(inode) */
pub const OCFS2_FILECHECK_TYPE_FIX: i32 = 1; /* Fix a file(inode) */
pub const OCFS2_FILECHECK_TYPE_SET: i32 = 100; /* Set entry list maximum size */

#[repr(C)]
pub struct ocfs2_filecheck_sysfs_entry { /* sysfs entry per partition */
    pub fs_kobj: kobject,
    pub fs_kobj_unregister: completion,
    pub fs_fcheck: *mut ocfs2_filecheck,
}

unsafe extern "C" {
    pub fn ocfs2_filecheck_create_sysfs(osb: *mut ocfs2_super) -> i32;
    pub fn ocfs2_filecheck_remove_sysfs(osb: *mut ocfs2_super);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
