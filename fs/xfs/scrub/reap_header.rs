// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2022-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Forward declarations supplied by the surrounding XFS translation.
#[repr(C)]
pub struct xagb_bitmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfsb_bitmap {
    _private: [u8; 0],
}

// CONFIG_XFS_RT controls whether the realtime implementation is available.
#[cfg(feature = "CONFIG_XFS_RT")]
extern "C" {
    pub fn xrep_reap_rtblocks(
        sc: *mut xfs_scrub,
        bitmap: *mut xrtb_bitmap,
        oinfo: *const xfs_owner_info,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
macro_rules! xrep_reap_rtblocks {
    ($($arg:tt)*) => { -EOPNOTSUPP };
}

/* Buffer cache scan context. */
#[repr(C)]
pub struct xrep_bufscan {
    /* Disk address for the buffers we want to scan. */
    pub daddr: xfs_daddr_t,

    /* Maximum number of sectors to scan. */
    pub max_sectors: xfs_daddr_t,

    /* Each round, increment the search length by this number of sectors. */
    pub daddr_step: xfs_daddr_t,

    /* Internal scan state; initialize to zero. */
    pub __sector_count: xfs_daddr_t,
}

extern "C" {
    pub fn xrep_reap_agblocks(
        sc: *mut xfs_scrub,
        bitmap: *mut xagb_bitmap,
        oinfo: *const xfs_owner_info,
        type_: xfs_ag_resv_type,
    ) -> ::core::ffi::c_int;

    pub fn xrep_reap_fsblocks(
        sc: *mut xfs_scrub,
        bitmap: *mut xfsb_bitmap,
        oinfo: *const xfs_owner_info,
    ) -> ::core::ffi::c_int;

    pub fn xrep_reap_ifork(
        sc: *mut xfs_scrub,
        ip: *mut xfs_inode,
        whichfork: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn xrep_reap_metadir_fsblocks(
        sc: *mut xfs_scrub,
        bitmap: *mut xfsb_bitmap,
    ) -> ::core::ffi::c_int;

    pub fn xrep_bufscan_max_sectors(
        mp: *mut xfs_mount,
        fsblocks: xfs_extlen_t,
    ) -> xfs_daddr_t;

    pub fn xrep_bufscan_advance(
        mp: *mut xfs_mount,
        scan: *mut xrep_bufscan,
    ) -> *mut xfs_buf;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
