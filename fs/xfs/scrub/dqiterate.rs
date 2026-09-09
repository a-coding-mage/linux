// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Dependencies supplied by the corresponding XFS headers and scrub sources.

/* Initialize a dquot iteration cursor. */
pub unsafe fn xchk_dqiter_init(
    cursor: *mut xchk_dqiter,
    sc: *mut xfs_scrub,
    dqtype: xfs_dqtype_t,
) {
    (*cursor).sc = sc;
    (*cursor).bmap.br_startoff = NULLFILEOFF;
    (*cursor).dqtype = dqtype & XFS_DQTYPE_REC_MASK;
    (*cursor).quota_ip = xfs_quota_inode((*sc).mp, (*cursor).dqtype);
    (*cursor).id = 0;
}

/*
 * Ensure that the cached data fork mapping for the dqiter cursor is fresh and
 * covers the dquot pointed to by the scan cursor.
 */
unsafe fn xchk_dquot_iter_revalidate_bmap(cursor: *mut xchk_dqiter) -> i32 {
    let qi = (*(*cursor).sc).mp.m_quotainfo;
    let ifp = xfs_ifork_ptr((*cursor).quota_ip, XFS_DATA_FORK);
    let mut fileoff: xfs_fileoff_t;
    let this_id: xfs_dqid_t = (*cursor).id;
    let mut nmaps: i32 = 1;
    let error: i32;

    fileoff = this_id / (*qi).qi_dqperchunk;

    /*
     * If we have a mapping for cursor->id and it's still fresh, there's
     * no need to reread the bmbt.
     */
    if (*cursor).bmap.br_startoff != NULLFILEOFF
        && (*cursor).if_seq == (*ifp).if_seq
        && (*cursor).bmap.br_startoff + (*cursor).bmap.br_blockcount > fileoff
    {
        return 0;
    }

    /* Look up the data fork mapping for the dquot id of interest. */
    error = xfs_bmapi_read(
        (*cursor).quota_ip,
        fileoff,
        XFS_MAX_FILEOFF - fileoff,
        &mut (*cursor).bmap,
        &mut nmaps,
        0,
    );
    if error != 0 {
        return error;
    }
    if nmaps == 0 {
        ASSERT(nmaps > 0);
        return -EFSCORRUPTED;
    }
    if (*cursor).bmap.br_startoff > fileoff {
        ASSERT((*cursor).bmap.br_startoff == fileoff);
        return -EFSCORRUPTED;
    }

    (*cursor).if_seq = (*ifp).if_seq;
    trace_xchk_dquot_iter_revalidate_bmap(cursor, (*cursor).id);
    0
}

/* Advance the dqiter cursor to the next non-sparse region of the quota file. */
unsafe fn xchk_dquot_iter_advance_bmap(
    cursor: *mut xchk_dqiter,
    next_ondisk_id: *mut u64,
) -> i32 {
    let qi = (*(*cursor).sc).mp.m_quotainfo;
    let ifp = xfs_ifork_ptr((*cursor).quota_ip, XFS_DATA_FORK);
    let mut fileoff: xfs_fileoff_t;
    let mut next_id: u64;
    let mut nmaps: i32 = 1;
    let error: i32;

    /* Find the dquot id for the next non-hole mapping. */
    loop {
        fileoff = (*cursor).bmap.br_startoff + (*cursor).bmap.br_blockcount;
        if fileoff > XFS_DQ_ID_MAX / (*qi).qi_dqperchunk {
            /* The hole goes beyond the max dquot id, we're done */
            *next_ondisk_id = u64::MAX;
            return 0;
        }

        error = xfs_bmapi_read(
            (*cursor).quota_ip,
            fileoff,
            XFS_MAX_FILEOFF - fileoff,
            &mut (*cursor).bmap,
            &mut nmaps,
            0,
        );
        if error != 0 {
            return error;
        }
        if nmaps == 0 {
            /* Must have reached the end of the mappings. */
            *next_ondisk_id = u64::MAX;
            return 0;
        }
        if (*cursor).bmap.br_startoff > fileoff {
            ASSERT((*cursor).bmap.br_startoff == fileoff);
            return -EFSCORRUPTED;
        }
        if xfs_bmap_is_real_extent(&(*cursor).bmap) {
            break;
        }
    }

    next_id = (*cursor).bmap.br_startoff * (*qi).qi_dqperchunk;
    if next_id > XFS_DQ_ID_MAX {
        /* The hole goes beyond the max dquot id, we're done */
        *next_ondisk_id = u64::MAX;
        return 0;
    }

    /* Propose jumping forward to the dquot in the next allocated block. */
    *next_ondisk_id = next_id;
    (*cursor).if_seq = (*ifp).if_seq;
    trace_xchk_dquot_iter_advance_bmap(cursor, *next_ondisk_id);
    0
}

/*
 * Find the id of the next highest incore dquot.  Normally this will correspond
 * exactly with the quota file block mappings, but repair might have erased a
 * mapping because it was crosslinked; in that case, we need to re-allocate the
 * space so that we can reset q_blkno.
 */
unsafe fn xchk_dquot_iter_advance_incore(
    cursor: *mut xchk_dqiter,
    next_incore_id: *mut u64,
) {
    let qi = (*(*cursor).sc).mp.m_quotainfo;
    let tree = xfs_dquot_tree(qi, (*cursor).dqtype);
    let mut dq: *mut xfs_dquot = core::ptr::null_mut();
    let nr_found: u32;

    *next_incore_id = u64::MAX;

    mutex_lock(&mut (*qi).qi_tree_lock);
    nr_found = radix_tree_gang_lookup(tree, &mut dq, (*cursor).id, 1);
    if nr_found != 0 {
        *next_incore_id = (*dq).q_id;
    }
    mutex_unlock(&mut (*qi).qi_tree_lock);

    trace_xchk_dquot_iter_advance_incore(cursor, *next_incore_id);
}

/*
 * Walk all incore dquots of this filesystem.  Caller must set *@cursorp to
 * zero before the first call, and must not hold the quota file ILOCK.
 * Returns 1 and a valid *@dqpp; 0 and *@dqpp == NULL when there are no more
 * dquots to iterate; or a negative errno.
 */
pub unsafe fn xchk_dquot_iter(
    cursor: *mut xchk_dqiter,
    dqpp: *mut *mut xfs_dquot,
) -> i32 {
    let mp = (*(*cursor).sc).mp;
    let mut dq: *mut xfs_dquot = core::ptr::null_mut();
    let mut next_ondisk: u64;
    let mut next_incore: u64 = u64::MAX;
    let lock_mode: u32;
    let mut error: i32 = 0;

    if (*cursor).id > XFS_DQ_ID_MAX {
        return 0;
    }
    next_ondisk = (*cursor).id;

    /* Revalidate and/or advance the cursor. */
    lock_mode = xfs_ilock_data_map_shared((*cursor).quota_ip);
    error = xchk_dquot_iter_revalidate_bmap(cursor);
    if error == 0 && !xfs_bmap_is_real_extent(&(*cursor).bmap) {
        error = xchk_dquot_iter_advance_bmap(cursor, &mut next_ondisk);
    }
    xfs_iunlock((*cursor).quota_ip, lock_mode);
    if error != 0 {
        return error;
    }

    if next_ondisk > (*cursor).id {
        xchk_dquot_iter_advance_incore(cursor, &mut next_incore);
    }

    /* Pick the next dquot in the sequence and return it. */
    (*cursor).id = core::cmp::min(next_ondisk, next_incore);
    if (*cursor).id > XFS_DQ_ID_MAX {
        return 0;
    }

    trace_xchk_dquot_iter(cursor, (*cursor).id);

    error = xfs_qm_dqget(mp, (*cursor).id, (*cursor).dqtype, false, &mut dq);
    if error != 0 {
        return error;
    }

    (*cursor).id = (*dq).q_id as u64 + 1;
    *dqpp = dq;
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
