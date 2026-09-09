// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependency intent: declarations from <linux/exportfs.h> and configuration
// symbols supplied by the surrounding kernel translation are external.

#[cfg(feature = "CONFIG_XFS_QUOTA")]
extern "C" {
    pub fn xfs_qm_init() -> ::core::ffi::c_int;
    pub fn xfs_qm_exit();
}

#[cfg(feature = "CONFIG_XFS_QUOTA")]
pub const XFS_QUOTA_STRING: &str = "quota, ";
#[cfg(not(feature = "CONFIG_XFS_QUOTA"))]
pub const XFS_QUOTA_STRING: &str = "";

#[cfg(feature = "CONFIG_XFS_POSIX_ACL")]
pub const XFS_ACL_STRING: &str = "ACLs, ";
#[cfg(not(feature = "CONFIG_XFS_POSIX_ACL"))]
pub const XFS_ACL_STRING: &str = "";

#[cfg(feature = "CONFIG_XFS_POSIX_ACL")]
#[macro_export]
macro_rules! set_posix_acl_flag {
    ($sb:expr) => {{ ($sb).s_flags |= SB_POSIXACL; }};
}
#[cfg(not(feature = "CONFIG_XFS_POSIX_ACL"))]
#[macro_export]
macro_rules! set_posix_acl_flag {
    ($sb:expr) => {{ }};
}

pub const XFS_SECURITY_STRING: &str = "security attributes, ";

#[cfg(feature = "CONFIG_XFS_RT")]
pub const XFS_REALTIME_STRING: &str = "realtime, ";
#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub const XFS_REALTIME_STRING: &str = "";

#[cfg(feature = "CONFIG_XFS_ONLINE_SCRUB")]
pub const XFS_SCRUB_STRING: &str = "scrub, ";
#[cfg(not(feature = "CONFIG_XFS_ONLINE_SCRUB"))]
pub const XFS_SCRUB_STRING: &str = "";

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
pub const XFS_REPAIR_STRING: &str = "repair, ";
#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
pub const XFS_REPAIR_STRING: &str = "";

#[cfg(feature = "CONFIG_XFS_WARN")]
pub const XFS_WARN_STRING: &str = "verbose warnings, ";
#[cfg(not(feature = "CONFIG_XFS_WARN"))]
pub const XFS_WARN_STRING: &str = "";

#[cfg(feature = "CONFIG_XFS_ASSERT_FATAL")]
pub const XFS_ASSERT_FATAL_STRING: &str = "fatal assert, ";
#[cfg(not(feature = "CONFIG_XFS_ASSERT_FATAL"))]
pub const XFS_ASSERT_FATAL_STRING: &str = "";

#[cfg(feature = "DEBUG")]
pub const XFS_DBG_STRING: &str = "debug";
#[cfg(not(feature = "DEBUG"))]
pub const XFS_DBG_STRING: &str = "no debug";

pub const XFS_VERSION_STRING: &str = "SGI XFS";
// XFS_BUILD_OPTIONS is assembled from the configuration-dependent strings;
// const string concatenation is left to the consuming translation unit.
pub const XFS_BUILD_OPTIONS: [&str; 9] = [
    XFS_ACL_STRING,
    XFS_SECURITY_STRING,
    XFS_REALTIME_STRING,
    XFS_SCRUB_STRING,
    XFS_REPAIR_STRING,
    XFS_WARN_STRING,
    XFS_QUOTA_STRING,
    XFS_ASSERT_FATAL_STRING,
    XFS_DBG_STRING, // DBG must be last
];

#[cfg(feature = "DEBUG")]
#[macro_export]
macro_rules! XFS_WQFLAGS {
    ($wqflags:expr) => {{ WQ_SYSFS | ($wqflags) }};
}
#[cfg(not(feature = "DEBUG"))]
#[macro_export]
macro_rules! XFS_WQFLAGS {
    ($wqflags:expr) => {{ $wqflags }};
}

#[repr(C)]
pub struct xfs_inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_buftarg {
    _private: [u8; 0],
}
#[repr(C)]
pub struct block_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn xfs_flush_inodes(mp: *mut xfs_mount);
    pub fn xfs_set_inode_alloc(mp: *mut xfs_mount, agcount: xfs_agnumber_t) -> xfs_agnumber_t;
    pub static xfs_export_operations: export_operations;
    pub static xfs_quotactl_operations: quotactl_ops;
    pub fn xfs_reinit_percpu_counters(mp: *mut xfs_mount);
    pub static mut xfs_discard_wq: *mut workqueue_struct;
    pub fn xfs_debugfs_mkdir(name: *const ::core::ffi::c_char, parent: *mut dentry) -> *mut dentry;
}

// External types supplied by other translated headers.
pub type xfs_agnumber_t = u32;
#[repr(C)]
pub struct export_operations { _private: [u8; 0] }
#[repr(C)]
pub struct quotactl_ops { _private: [u8; 0] }
#[repr(C)]
pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }

#[macro_export]
macro_rules! XFS_M {
    ($sb:expr) => {{ ($sb).s_fs_info as *mut xfs_mount }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
