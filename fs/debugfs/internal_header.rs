/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  internal.h - declarations internal to debugfs
 *
 *  Copyright (C) 2016 Nicolai Stange <nicstange@gmail.com>
 */

// Dependency supplied by the surrounding kernel translation: linux/list.h

extern "C" {
    pub static debugfs_noop_file_operations: file_operations;
    pub static debugfs_open_proxy_file_operations: file_operations;
    pub static debugfs_full_proxy_file_operations: file_operations;
    pub static debugfs_full_short_proxy_file_operations: file_operations;
}

#[repr(C)]
pub struct debugfs_inode_info {
    pub vfs_inode: inode,
    pub _bindgen_anon_1: debugfs_inode_info__bindgen_ty_1,
    pub aux: *mut core::ffi::c_void,
}

#[repr(C)]
pub union debugfs_inode_info__bindgen_ty_1 {
    pub raw: *const core::ffi::c_void,
    pub real_fops: *const file_operations,
    pub short_fops: *const debugfs_short_fops,
    pub automount: debugfs_automount_t,
}

#[inline]
pub unsafe fn DEBUGFS_I(inode: *mut inode) -> *mut debugfs_inode_info {
    (inode as *mut u8).sub(core::mem::offset_of!(debugfs_inode_info, vfs_inode))
        as *mut debugfs_inode_info
}

#[repr(C)]
pub struct debugfs_fsdata {
    pub real_fops: *const file_operations,
    pub short_fops: *const debugfs_short_fops,
    pub active_users: refcount_t,
    pub active_users_drained: completion,
    /* protect cancellations */
    pub cancellations_mtx: mutex,
    pub cancellations: list_head,
    pub methods: core::ffi::c_uint,
}

pub const HAS_READ: core::ffi::c_int = 1;
pub const HAS_WRITE: core::ffi::c_int = 2;
pub const HAS_LSEEK: core::ffi::c_int = 4;
pub const HAS_POLL: core::ffi::c_int = 8;
pub const HAS_IOCTL: core::ffi::c_int = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
