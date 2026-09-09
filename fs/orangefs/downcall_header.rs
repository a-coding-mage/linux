/* SPDX-License-Identifier: GPL-2.0 */
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * See COPYING in top-level directory.
 */

/* Definitions of downcalls used in Linux kernel module. */

use core::ffi::c_char;
use core::mem::ManuallyDrop;

/* Sanitized the device-client core interaction for clean 32-64 bit usage. */
#[repr(C)]
pub struct orangefs_io_response {
    pub amt_complete: i64,
}

#[repr(C)]
pub struct orangefs_lookup_response {
    pub refn: ManuallyDrop<orangefs_object_kref>,
}

#[repr(C)]
pub struct orangefs_create_response {
    pub refn: ManuallyDrop<orangefs_object_kref>,
}

#[repr(C)]
pub struct orangefs_symlink_response {
    pub refn: ManuallyDrop<orangefs_object_kref>,
}

#[repr(C)]
pub struct orangefs_getattr_response {
    pub attributes: ManuallyDrop<ORANGEFS_sys_attr_s>,
    pub link_target: [c_char; ORANGEFS_NAME_MAX],
}

#[repr(C)]
pub struct orangefs_mkdir_response {
    pub refn: ManuallyDrop<orangefs_object_kref>,
}

#[repr(C)]
pub struct orangefs_statfs_response {
    pub block_size: i64,
    pub blocks_total: i64,
    pub blocks_avail: i64,
    pub files_total: i64,
    pub files_avail: i64,
}

#[repr(C)]
pub struct orangefs_fs_mount_response {
    pub fs_id: i32,
    pub id: i32,
    pub root_khandle: ManuallyDrop<orangefs_khandle>,
}

/* The getxattr response is the attribute value. */
#[repr(C)]
pub struct orangefs_getxattr_response {
    pub val_sz: i32,
    pub __pad1: i32,
    pub val: [c_char; ORANGEFS_MAX_XATTR_VALUELEN],
}

/* The listxattr response is an array of attribute names. */
#[repr(C)]
pub struct orangefs_listxattr_response {
    pub returned_count: i32,
    pub __pad1: i32,
    pub token: u64,
    pub key: [c_char; ORANGEFS_MAX_XATTR_LISTLEN * ORANGEFS_MAX_XATTR_NAMELEN],
    pub keylen: i32,
    pub __pad2: i32,
    pub lengths: [i32; ORANGEFS_MAX_XATTR_LISTLEN],
}

#[repr(C)]
pub union orangefs_param_response_u {
    pub value64: i64,
    pub value32: [i32; 2],
}

#[repr(C)]
pub struct orangefs_param_response {
    pub u: ManuallyDrop<orangefs_param_response_u>,
}

pub const PERF_COUNT_BUF_SIZE: usize = 4096;

#[repr(C)]
pub struct orangefs_perf_count_response {
    pub buffer: [c_char; PERF_COUNT_BUF_SIZE],
}

pub const FS_KEY_BUF_SIZE: usize = 4096;

#[repr(C)]
pub struct orangefs_fs_key_response {
    pub fs_keylen: i32,
    pub __pad1: i32,
    pub fs_key: [c_char; FS_KEY_BUF_SIZE],
}

/* 2.9.6 */
#[repr(C)]
pub struct orangefs_features_response {
    pub features: u64,
}

#[repr(C)]
pub union orangefs_downcall_s_resp {
    pub io: ManuallyDrop<orangefs_io_response>,
    pub lookup: ManuallyDrop<orangefs_lookup_response>,
    pub create: ManuallyDrop<orangefs_create_response>,
    pub sym: ManuallyDrop<orangefs_symlink_response>,
    pub getattr: ManuallyDrop<orangefs_getattr_response>,
    pub mkdir: ManuallyDrop<orangefs_mkdir_response>,
    pub statfs: ManuallyDrop<orangefs_statfs_response>,
    pub fs_mount: ManuallyDrop<orangefs_fs_mount_response>,
    pub getxattr: ManuallyDrop<orangefs_getxattr_response>,
    pub listxattr: ManuallyDrop<orangefs_listxattr_response>,
    pub param: ManuallyDrop<orangefs_param_response>,
    pub perf_count: ManuallyDrop<orangefs_perf_count_response>,
    pub fs_key: ManuallyDrop<orangefs_fs_key_response>,
    pub features: ManuallyDrop<orangefs_features_response>,
}

#[repr(C)]
pub struct orangefs_downcall_s {
    pub type_: i32,
    pub status: i32,
    /* Currently trailer is used only by readdir. */
    pub trailer_size: i64,
    pub trailer_buf: *mut c_char,
    pub resp: ManuallyDrop<orangefs_downcall_s_resp>,
}

/*
 * The readdir response comes in the trailer. It is followed by the
 * directory entries as described in dir.c.
 */
#[repr(C)]
pub struct orangefs_readdir_response_s {
    pub token: u64,
    pub directory_version: u64,
    pub __pad2: u32,
    pub orangefs_dirent_outcount: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
