// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

#[repr(C)]
pub struct xchk_stats_run {
    pub scrub_ns: u64,
    pub repair_ns: u64,
    pub retries: ::core::ffi::c_uint,
    pub repair_attempted: bool,
    pub repair_succeeded: bool,
}

// CONFIG_XFS_ONLINE_SCRUB_STATS selects the implementation of these declarations.
pub struct xchk_stats {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS")]
extern "C" {
    pub fn xchk_global_stats_setup(parent: *mut dentry) -> ::core::ffi::c_int;
    pub fn xchk_global_stats_teardown();

    pub fn xchk_mount_stats_alloc(mp: *mut xfs_mount) -> ::core::ffi::c_int;
    pub fn xchk_mount_stats_free(mp: *mut xfs_mount);

    pub fn xchk_stats_register(cs: *mut xchk_stats, parent: *mut dentry);
    pub fn xchk_stats_unregister(cs: *mut xchk_stats);

    pub fn xchk_stats_merge(
        mp: *mut xfs_mount,
        sm: *const xfs_scrub_metadata,
        run: *const xchk_stats_run,
    );

    pub fn ktime_get_ns() -> u64;
}

#[cfg(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS")]
#[inline]
pub unsafe fn xchk_stats_now() -> u64 {
    ktime_get_ns()
}

#[cfg(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS")]
#[inline]
pub unsafe fn xchk_stats_elapsed_ns(since: u64) -> u64 {
    let now = xchk_stats_now();

    /*
     * If the system doesn't have a high enough resolution clock, charge at
     * least one nanosecond so that our stats don't report instantaneous
     * runtimes.
     */
    if now == since {
        return 1;
    }

    now - since
}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS"))]
#[inline]
pub fn xchk_global_stats_setup(_parent: *mut dentry) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS"))]
#[inline]
pub fn xchk_global_stats_teardown() {}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS"))]
#[inline]
pub fn xchk_mount_stats_alloc(_mp: *mut xfs_mount) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS"))]
#[inline]
pub fn xchk_mount_stats_free(_mp: *mut xfs_mount) {}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS"))]
#[inline]
pub fn xchk_stats_register(_cs: *mut xchk_stats, _parent: *mut dentry) {}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS"))]
#[inline]
pub fn xchk_stats_unregister(_cs: *mut xchk_stats) {}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS"))]
#[inline]
pub fn xchk_stats_now() -> u64 {
    0
}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS"))]
#[inline]
pub fn xchk_stats_elapsed_ns<T>(_x: T) -> u64 {
    0
}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_SCRUB_STATS"))]
#[inline]
pub fn xchk_stats_merge(
    _mp: *mut xfs_mount,
    _sm: *const xfs_scrub_metadata,
    _run: *const xchk_stats_run,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
