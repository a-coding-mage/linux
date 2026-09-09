/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Quota counters for live quotacheck. */
#[repr(C)]
pub struct xqcheck_dquot {
    /* block usage count */
    pub bcount: i64,

    /* inode usage count */
    pub icount: i64,

    /* realtime block usage count */
    pub rtbcount: i64,

    /* Record state */
    pub flags: u32,
}

/*
 * This incore dquot record has been written at least once.  We never want to
 * store an xqcheck_dquot that looks uninitialized.
 */
pub const XQCHECK_DQUOT_WRITTEN: u32 = 1u32 << 0;

/* Already checked this dquot. */
pub const XQCHECK_DQUOT_COMPARE_SCANNED: u32 = 1u32 << 1;

/* Already repaired this dquot. */
pub const XQCHECK_DQUOT_REPAIR_SCANNED: u32 = 1u32 << 2;

/* Live quotacheck control structure. */
#[repr(C)]
pub struct xqcheck {
    pub sc: *mut xfs_scrub,

    /* Shadow dquot counter data. */
    pub ucounts: *mut xfarray,
    pub gcounts: *mut xfarray,
    pub pcounts: *mut xfarray,

    /* Lock protecting quotacheck count observations */
    pub lock: mutex,

    pub iscan: xchk_iscan,

    /* Hooks into the quota code. */
    pub qhook: xfs_dqtrx_hook,

    /* Shadow quota delta tracking structure. */
    pub shadow_dquot_acct: rhashtable,
}

/* Return the incore counter array for a given quota type. */
#[inline]
pub unsafe fn xqcheck_counters_for(
    xqc: *mut xqcheck,
    dqtype: xfs_dqtype_t,
) -> *mut xfarray {
    match dqtype {
        XFS_DQTYPE_USER => (*xqc).ucounts,
        XFS_DQTYPE_GROUP => (*xqc).gcounts,
        XFS_DQTYPE_PROJ => (*xqc).pcounts,
        _ => {
            /* ASSERT(0); */
            core::ptr::null_mut()
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
