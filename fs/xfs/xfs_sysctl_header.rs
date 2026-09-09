// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2001-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependency: the C header includes <linux/sysctl.h>.

/*
 * Tunable xfs parameters
 */

#[repr(C)]
pub struct xfs_sysctl_val_t {
    pub min: ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
    pub max: ::core::ffi::c_int,
}

#[repr(C)]
pub struct xfs_param_t {
    pub panic_mask: xfs_sysctl_val_t, // bitmask to cause panic on errors.
    pub error_level: xfs_sysctl_val_t, // Degree of reporting for problems
    pub syncd_timer: xfs_sysctl_val_t, // Interval between xfssyncd wakeups
    pub stats_clear: xfs_sysctl_val_t, // Reset all XFS statistics to zero.
    pub inherit_sync: xfs_sysctl_val_t, // Inherit the "sync" inode flag.
    pub inherit_nodump: xfs_sysctl_val_t, // Inherit the "nodump" inode flag.
    pub inherit_noatim: xfs_sysctl_val_t, // Inherit the "noatime" inode flag.
    pub inherit_nosym: xfs_sysctl_val_t, // Inherit the "nosymlinks" flag.
    pub rotorstep: xfs_sysctl_val_t, // inode32 AG rotoring control knob
    pub inherit_nodfrg: xfs_sysctl_val_t, // Inherit the "nodefrag" inode flag.
    pub fstrm_timer: xfs_sysctl_val_t, // Filestream dir-AG assoc'n timeout.
    pub blockgc_timer: xfs_sysctl_val_t, // Interval between blockgc scans
}

/*
 * xfs_error_level:
 *
 * How much error reporting will be done when internal problems are
 * encountered.  These problems normally return an EFSCORRUPTED to their
 * caller, with no other information reported.
 *
 * 0 No error reports
 * 1 Report EFSCORRUPTED errors that will cause a filesystem shutdown
 * 5 Report all EFSCORRUPTED errors (all of the above errors, plus any
 *   additional errors that are known to not cause shutdowns)
 *
 * xfs_panic_mask bit 0x8 turns the error reports into panics
 */

pub const XFS_SGID_INHERIT: ::core::ffi::c_int = 4;
pub const XFS_SYMLINK_MODE: ::core::ffi::c_int = 5;
pub const XFS_PANIC_MASK: ::core::ffi::c_int = 6;
pub const XFS_ERRLEVEL: ::core::ffi::c_int = 7;
pub const XFS_SYNCD_TIMER: ::core::ffi::c_int = 8;
pub const XFS_STATS_CLEAR: ::core::ffi::c_int = 12;
pub const XFS_INHERIT_SYNC: ::core::ffi::c_int = 13;
pub const XFS_INHERIT_NODUMP: ::core::ffi::c_int = 14;
pub const XFS_INHERIT_NOATIME: ::core::ffi::c_int = 15;
pub const XFS_BUF_TIMER: ::core::ffi::c_int = 16;
pub const XFS_BUF_AGE: ::core::ffi::c_int = 17;
pub const XFS_INHERIT_NOSYM: ::core::ffi::c_int = 19;
pub const XFS_ROTORSTEP: ::core::ffi::c_int = 20;
pub const XFS_INHERIT_NODFRG: ::core::ffi::c_int = 21;
pub const XFS_FILESTREAM_TIMER: ::core::ffi::c_int = 22;

unsafe extern "C" {
    pub static mut xfs_params: xfs_param_t;
}

#[repr(C)]
pub struct xfs_globals {
    // Build-time DEBUG condition from the original header.
    #[cfg(feature = "DEBUG")]
    pub pwork_threads: ::core::ffi::c_int, // parallel workqueue threads
    #[cfg(feature = "DEBUG")]
    pub larp: bool, // log attribute replay
    pub bload_leaf_slack: ::core::ffi::c_int, // btree bulk load leaf slack
    pub bload_node_slack: ::core::ffi::c_int, // btree bulk load node slack
    pub log_recovery_delay: ::core::ffi::c_int, // log recovery delay (secs)
    pub mount_delay: ::core::ffi::c_int, // mount setup delay (secs)
    pub bug_on_assert: bool, // BUG() the kernel on assert failure
    pub always_cow: bool, // use COW fork for all overwrites
}

unsafe extern "C" {
    pub static mut xfs_globals: xfs_globals;
}

// CONFIG_SYSCTL is a build-time condition from the original header.
#[cfg(feature = "CONFIG_SYSCTL")]
unsafe extern "C" {
    pub fn xfs_sysctl_register() -> ::core::ffi::c_int;
    pub fn xfs_sysctl_unregister();
}

#[cfg(not(feature = "CONFIG_SYSCTL"))]
#[inline]
pub const fn xfs_sysctl_register() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_SYSCTL"))]
#[inline]
pub fn xfs_sysctl_unregister() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
