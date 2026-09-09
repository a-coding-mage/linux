/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Live link count control structure. */
#[repr(C)]
pub struct xchk_nlink_ctrs {
    pub sc: *mut xfs_scrub,

    /* Shadow link count data and its mutex. */
    pub nlinks: *mut xfarray,
    pub lock: mutex,

    /*
     * The collection step uses a separate iscan context from the compare
     * step because the collection iscan coordinates live updates to the
     * observation data while this scanner is running.  The compare iscan
     * is secondary and can be reinitialized as needed.
     */
    pub collect_iscan: xchk_iscan,
    pub compare_iscan: xchk_iscan,

    /*
     * Hook into directory updates so that we can receive live updates
     * from other writer threads.
     */
    pub dhook: xfs_dir_hook,

    /* Orphanage reparenting request. */
    pub adoption: xrep_adoption,

    /* Directory entry name, plus the trailing null. */
    pub xname: xfs_name,
    pub namebuf: [core::ffi::c_char; MAXNAMELEN],
}

/*
 * In-core link counts for a given inode in the filesystem.
 *
 * For an empty rootdir, the directory entries and the field to which they are
 * accounted are as follows:
 *
 * Root directory:
 *
 * . points to self        (root.child)
 * .. points to self       (root.parent)
 * f1 points to a child file   (f1.parent)
 * d1 points to a child dir    (d1.parent, root.child)
 *
 * Subdirectory d1:
 *
 * . points to self        (d1.child)
 * .. points to root dir       (root.backref)
 * f2 points to child file     (f2.parent)
 * f3 points to root.f1        (f1.parent)
 *
 * root.nlink == 3 (root.dot, root.dotdot, root.d1)
 * d1.nlink == 2 (root.d1, d1.dot)
 * f1.nlink == 2 (root.f1, d1.f3)
 * f2.nlink == 1 (d1.f2)
 */
#[repr(C)]
pub struct xchk_nlink {
    /* Count of forward links from parent directories to this file. */
    pub parents: xfs_nlink_t,

    /*
     * Count of back links to this parent directory from child
     * subdirectories.
     */
    pub backrefs: xfs_nlink_t,

    /*
     * Count of forward links from this directory to all child files and
     * the number of dot entries.  Should be zero for non-directories.
     */
    pub children: xfs_nlink_t,

    /* Record state flags */
    pub flags: u32,
}

/*
 * This incore link count has been written at least once.  We never want to
 * store an xchk_nlink that looks uninitialized.
 */
pub const XCHK_NLINK_WRITTEN: u32 = 1u32 << 0;

/* Already checked this link count record. */
pub const XCHK_NLINK_COMPARE_SCANNED: u32 = 1u32 << 1;

/* Already made a repair with this link count record. */
pub const XREP_NLINK_DIRTY: u32 = 1u32 << 2;

/* Compute total link count, using large enough variables to detect overflow. */
pub unsafe fn xchk_nlink_total(ip: *mut xfs_inode, live: *const xchk_nlink) -> u64 {
    let mut ret: u64 = (*live).parents as u64;

    /* Add one link count for the dot entry of any linked directory. */
    if !ip.is_null() && S_ISDIR((*VFS_I(ip)).i_mode) && (*VFS_I(ip)).i_nlink != 0 {
        ret = ret.wrapping_add(1);
    }
    ret.wrapping_add((*live).children as u64)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
