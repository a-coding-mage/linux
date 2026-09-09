/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Declarations corresponding to the C header guard:
// __XFS_SCRUB_FINDPARENT_H__

#[repr(C)]
pub struct xrep_parent_scan_info {
    pub sc: *mut xfs_scrub,

    /* Inode scan cursor. */
    pub iscan: xchk_iscan,

    /* Hook to capture directory entry updates. */
    pub dhook: xfs_dir_hook,

    /* Lock protecting parent_ino. */
    pub lock: mutex,

    /* Parent inode that we've found. */
    pub parent_ino: xfs_ino_t,

    pub lookup_parent: bool,
}

extern "C" {
    pub fn __xrep_findparent_scan_start(
        sc: *mut xfs_scrub,
        pscan: *mut xrep_parent_scan_info,
        custom_fn: notifier_fn_t,
    ) -> ::std::os::raw::c_int;

    pub fn xrep_findparent_scan(pscan: *mut xrep_parent_scan_info) -> ::std::os::raw::c_int;
    pub fn xrep_findparent_scan_teardown(pscan: *mut xrep_parent_scan_info);

    pub fn xrep_findparent_scan_finish_early(
        pscan: *mut xrep_parent_scan_info,
        ino: xfs_ino_t,
    );

    pub fn xrep_findparent_confirm(
        sc: *mut xfs_scrub,
        parent_ino: *mut xfs_ino_t,
    ) -> ::std::os::raw::c_int;

    pub fn xrep_findparent_self_reference(sc: *mut xfs_scrub) -> xfs_ino_t;
    pub fn xrep_findparent_from_dcache(sc: *mut xfs_scrub) -> xfs_ino_t;
}

#[inline]
pub unsafe fn xrep_findparent_scan_start(
    sc: *mut xfs_scrub,
    pscan: *mut xrep_parent_scan_info,
) -> ::std::os::raw::c_int {
    __xrep_findparent_scan_start(sc, pscan, ::std::ptr::null_mut())
}

#[inline]
pub unsafe fn xrep_findparent_scan_found(
    pscan: *mut xrep_parent_scan_info,
    ino: xfs_ino_t,
) {
    mutex_lock(&mut (*pscan).lock);
    (*pscan).parent_ino = ino;
    mutex_unlock(&mut (*pscan).lock);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
