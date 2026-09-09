/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

#[repr(C)]
pub struct xchk_iscan {
    pub sc: *mut xfs_scrub,

    /* Lock to protect the scan cursor. */
    pub lock: mutex,

    /*
     * This is the first inode in the inumber address space that we
     * examined.  When the scan wraps around back to here, the scan is
     * finished.
     */
    pub scan_start_ino: xfs_ino_t,

    /* This is the inode that will be examined next. */
    pub cursor_ino: xfs_ino_t,

    /* If nonzero and non-NULL, skip this inode when scanning. */
    pub skip_ino: xfs_ino_t,

    /*
     * This is the last inode that we've successfully scanned, either
     * because the caller scanned it, or we moved the cursor past an empty
     * part of the inode address space.  Scan callers should only use the
     * xchk_iscan_visit function to modify this.
     */
    pub __visited_ino: xfs_ino_t,

    /* Operational state of the livescan. */
    pub __opstate: libc::c_ulong,

    /* Give up on iterating @cursor_ino if we can't iget it by this time. */
    pub __iget_deadline: libc::c_ulong,

    /* Amount of time (in ms) that we will try to iget an inode. */
    pub iget_timeout: libc::c_uint,

    /* Wait this many ms to retry an iget. */
    pub iget_retry_delay: libc::c_uint,

    /*
     * The scan grabs batches of inodes and stashes them here before
     * handing them out with _iter.  Unallocated inodes are set in the
     * mask so that all updates to that inode are selected for live
     * update propagation.
     */
    pub __batch_ino: xfs_ino_t,
    pub __skipped_inomask: xfs_inofree_t,
    pub __inodes: [*mut xfs_inode; XFS_INODES_PER_CHUNK],
}

/* Set if the scan has been aborted due to some event in the fs. */
pub const XCHK_ISCAN_OPSTATE_ABORTED: libc::c_ulong = 1;

/* Use trylock to acquire the AGI */
pub const XCHK_ISCAN_OPSTATE_TRYLOCK_AGI: libc::c_ulong = 2;

#[inline]
pub unsafe fn xchk_iscan_aborted(iscan: *const xchk_iscan) -> bool {
    test_bit(XCHK_ISCAN_OPSTATE_ABORTED, &(*iscan).__opstate)
}

#[inline]
pub unsafe fn xchk_iscan_abort(iscan: *mut xchk_iscan) {
    set_bit(XCHK_ISCAN_OPSTATE_ABORTED, &mut (*iscan).__opstate);
}

#[inline]
pub unsafe fn xchk_iscan_agi_needs_trylock(iscan: *const xchk_iscan) -> bool {
    test_bit(XCHK_ISCAN_OPSTATE_TRYLOCK_AGI, &(*iscan).__opstate)
}

#[inline]
pub unsafe fn xchk_iscan_set_agi_trylock(iscan: *mut xchk_iscan) {
    set_bit(XCHK_ISCAN_OPSTATE_TRYLOCK_AGI, &mut (*iscan).__opstate);
}

extern "C" {
    pub fn xchk_iscan_start(
        sc: *mut xfs_scrub,
        iget_timeout: libc::c_uint,
        iget_retry_delay: libc::c_uint,
        iscan: *mut xchk_iscan,
    );
    pub fn xchk_iscan_finish_early(iscan: *mut xchk_iscan);
    pub fn xchk_iscan_teardown(iscan: *mut xchk_iscan);

    pub fn xchk_iscan_iter(iscan: *mut xchk_iscan, ipp: *mut *mut xfs_inode) -> libc::c_int;
    pub fn xchk_iscan_iter_finish(iscan: *mut xchk_iscan);

    pub fn xchk_iscan_mark_visited(iscan: *mut xchk_iscan, ip: *mut xfs_inode);
    pub fn xchk_iscan_want_live_update(iscan: *mut xchk_iscan, ino: xfs_ino_t) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
