// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// __XFS_SCRUB_QUOTA_H__

extern "C" {
    pub fn xchk_quota_to_dqtype(sc: *mut xfs_scrub) -> xfs_dqtype_t;
}

/* dquot iteration code */

#[repr(C)]
pub struct xchk_dqiter {
    pub sc: *mut xfs_scrub,

    /* Quota file that we're walking. */
    pub quota_ip: *mut xfs_inode,

    /* Cached data fork mapping for the dquot. */
    pub bmap: xfs_bmbt_irec,

    /* The next dquot to scan. */
    pub id: u64,

    /* Quota type (user/group/project). */
    pub dqtype: xfs_dqtype_t,

    /* Data fork sequence number to detect stale mappings. */
    pub if_seq: u32,
}

extern "C" {
    pub fn xchk_dqiter_init(
        cursor: *mut xchk_dqiter,
        sc: *mut xfs_scrub,
        dqtype: xfs_dqtype_t,
    );
    pub fn xchk_dquot_iter(cursor: *mut xchk_dqiter, dqpp: *mut *mut xfs_dquot) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
