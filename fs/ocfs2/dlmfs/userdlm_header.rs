/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * userdlm.h
 *
 * Userspace dlm defines
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

/* C dependencies: linux/module.h, linux/fs.h, linux/types.h,
 * linux/workqueue.h.  Their declarations are supplied externally. */

/* user_lock_res->l_flags flags. */
pub const USER_LOCK_ATTACHED: i32 = 0x00000001; /* we have initialized the lvb */
pub const USER_LOCK_BUSY: i32 = 0x00000002; /* we are currently in dlm_lock */
pub const USER_LOCK_BLOCKED: i32 = 0x00000004; /* blocked waiting to downconvert */
pub const USER_LOCK_IN_TEARDOWN: i32 = 0x00000008; /* we're currently destroying this lock */
pub const USER_LOCK_QUEUED: i32 = 0x00000010; /* lock is on the workqueue */
pub const USER_LOCK_IN_CANCEL: i32 = 0x00000020;

pub const USER_DLM_LOCK_ID_MAX_LEN: usize = 32;

#[repr(C)]
pub struct user_lock_res {
    pub l_lock: spinlock_t,
    pub l_flags: i32,
    pub l_name: [core::ffi::c_char; USER_DLM_LOCK_ID_MAX_LEN],
    pub l_namelen: i32,
    pub l_level: i32,
    pub l_ro_holders: u32,
    pub l_ex_holders: u32,
    pub l_lksb: ocfs2_dlm_lksb,
    pub l_requested: i32,
    pub l_blocking: i32,
    pub l_event: wait_queue_head_t,
    pub l_work: work_struct,
}

extern "C" {
    pub static mut user_dlm_worker: *mut workqueue_struct;

    pub fn user_dlm_lock_res_init(lockres: *mut user_lock_res, dentry: *mut dentry);
    pub fn user_dlm_destroy_lock(lockres: *mut user_lock_res) -> i32;
    pub fn user_dlm_cluster_lock(
        lockres: *mut user_lock_res,
        level: i32,
        lkm_flags: i32,
    ) -> i32;
    pub fn user_dlm_cluster_unlock(lockres: *mut user_lock_res, level: i32);
    pub fn user_dlm_write_lvb(
        inode: *mut inode,
        val: *const core::ffi::c_char,
        len: u32,
    );
    pub fn user_dlm_read_lvb(inode: *mut inode, val: *mut core::ffi::c_char) -> bool;
    pub fn user_dlm_register(name: *const qstr) -> *mut ocfs2_cluster_connection;
    pub fn user_dlm_unregister(conn: *mut ocfs2_cluster_connection);
    pub fn user_dlm_set_locking_protocol();
}

#[repr(C)]
pub struct dlmfs_inode_private {
    pub ip_conn: *mut ocfs2_cluster_connection,
    pub ip_lockres: user_lock_res, /* unused for directories. */
    pub ip_parent: *mut inode,
    pub ip_vfs_inode: inode,
}

#[inline]
pub unsafe fn DLMFS_I(inode: *mut inode) -> *mut dlmfs_inode_private {
    (inode as *mut u8).sub(core::mem::offset_of!(dlmfs_inode_private, ip_vfs_inode))
        as *mut dlmfs_inode_private
}

#[repr(C)]
pub struct dlmfs_filp_private {
    pub fp_lock_level: i32,
}

pub const DLMFS_MAGIC: u32 = 0x76a9f425;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
