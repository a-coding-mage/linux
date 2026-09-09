// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// The declarations below are present only when CONFIG_XFS_ONLINE_REPAIR is
// enabled in the C build.

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
extern "C" {
    pub fn xrep_orphanage_create(sc: *mut xfs_scrub) -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
#[inline]
pub unsafe fn xrep_orphanage_try_create(sc: *mut xfs_scrub) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int;

    assert!((*(*sc).sm).sm_flags & XFS_SCRUB_IFLAG_REPAIR != 0);

    error = xrep_orphanage_create(sc);
    match error {
        0 | -ENOENT | -ENOTDIR | -ENOSPC => {
            /*
             * If the orphanage can't be found or isn't a directory, we'll
             * keep going, but we won't be able to attach the file to the
             * orphanage if we can't find the parent.
             */
            0
        }
        _ => error,
    }
}

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
extern "C" {
    pub fn xrep_orphanage_iolock_two(sc: *mut xfs_scrub) -> ::core::ffi::c_int;

    pub fn xrep_orphanage_ilock(sc: *mut xfs_scrub, ilock_flags: ::core::ffi::c_uint);
    pub fn xrep_orphanage_ilock_nowait(
        sc: *mut xfs_scrub,
        ilock_flags: ::core::ffi::c_uint,
    ) -> bool;
    pub fn xrep_orphanage_iunlock(sc: *mut xfs_scrub, ilock_flags: ::core::ffi::c_uint);

    pub fn xrep_orphanage_rele(sc: *mut xfs_scrub);
}

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
#[repr(C)]
pub struct xrep_adoption {
    pub sc: *mut xfs_scrub,

    /* Name used for the adoption. */
    pub xname: *mut xfs_name,

    /* Parent pointer context tracking */
    pub ppargs: xfs_parent_args,

    /* Block reservations for orphanage and child (if directory). */
    pub orphanage_blkres: ::core::ffi::c_uint,
    pub child_blkres: ::core::ffi::c_uint,

    /*
     * Does the caller want us to bump the child link count?  This is not
     * needed when reattaching files that have become disconnected but have
     * nlink > 1.  It is necessary when changing the directory tree
     * structure.
     */
    pub bump_child_nlink: bool,
}

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
extern "C" {
    pub fn xrep_orphanage_can_adopt(sc: *mut xfs_scrub) -> bool;

    pub fn xrep_adoption_trans_alloc(
        sc: *mut xfs_scrub,
        adopt: *mut xrep_adoption,
    ) -> ::core::ffi::c_int;
    pub fn xrep_adoption_compute_name(
        adopt: *mut xrep_adoption,
        xname: *mut xfs_name,
    ) -> ::core::ffi::c_int;
    pub fn xrep_adoption_move(adopt: *mut xrep_adoption) -> ::core::ffi::c_int;
    pub fn xrep_adoption_trans_roll(adopt: *mut xrep_adoption) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
#[repr(C)]
pub struct xrep_adoption {
    _empty: [u8; 0],
}

#[cfg(not(feature = "CONFIG_XFS_ONLINE_REPAIR"))]
#[inline]
pub fn xrep_orphanage_rele(_sc: *mut xfs_scrub) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
