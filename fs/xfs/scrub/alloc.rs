// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C dependencies supplied by the surrounding translation unit.

/*
 * Set us up to scrub free space btrees.
 */
pub unsafe fn xchk_setup_ag_allocbt(sc: *mut xfs_scrub) -> i32 {
    let error: i32;

    if xchk_need_intent_drain(sc) {
        xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN);
    }

    error = xchk_setup_ag_btree(sc, false);
    if error != 0 {
        return error;
    }

    if xchk_could_repair(sc) {
        return xrep_setup_ag_allocbt(sc);
    }

    0
}

/* Free space btree scrubber. */

#[repr(C)]
pub struct xchk_alloc {
    /* Previous free space extent. */
    pub prev: xfs_alloc_rec_incore,
}

/*
 * Ensure there's a corresponding cntbt/bnobt record matching this
 * bnobt/cntbt record, respectively.
 */
unsafe fn xchk_allocbt_xref_other(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t) {
    let pcur: *mut *mut xfs_btree_cur;
    let mut fbno: xfs_agblock_t = 0;
    let mut flen: xfs_extlen_t = 0;
    let mut has_otherrec: i32 = 0;
    let mut error: i32;

    if (*(*sc).sm).sm_type == XFS_SCRUB_TYPE_BNOBT {
        pcur = &mut (*(*sc).sa).cnt_cur;
    } else {
        pcur = &mut (*(*sc).sa).bno_cur;
    }
    if (*pcur).is_null() || xchk_skip_xref((*sc).sm) {
        return;
    }

    error = xfs_alloc_lookup_le(*pcur, agbno, len, &mut has_otherrec);
    if !xchk_should_check_xref(sc, &mut error, pcur) {
        return;
    }
    if has_otherrec == 0 {
        xchk_btree_xref_set_corrupt(sc, *pcur, 0);
        return;
    }

    error = xfs_alloc_get_rec(*pcur, &mut fbno, &mut flen, &mut has_otherrec);
    if !xchk_should_check_xref(sc, &mut error, pcur) {
        return;
    }
    if has_otherrec == 0 {
        xchk_btree_xref_set_corrupt(sc, *pcur, 0);
        return;
    }

    if fbno != agbno || flen != len {
        xchk_btree_xref_set_corrupt(sc, *pcur, 0);
    }
}

/* Cross-reference with the other btrees. */
unsafe fn xchk_allocbt_xref(sc: *mut xfs_scrub, irec: *const xfs_alloc_rec_incore) {
    let agbno = (*irec).ar_startblock;
    let len = (*irec).ar_blockcount;

    if (*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 {
        return;
    }

    xchk_allocbt_xref_other(sc, agbno, len);
    xchk_xref_is_not_inode_chunk(sc, agbno, len);
    xchk_xref_has_no_owner(sc, agbno, len);
    xchk_xref_is_not_shared(sc, agbno, len);
    xchk_xref_is_not_cow_staging(sc, agbno, len);
}

/* Flag failures for records that could be merged. */
unsafe fn xchk_allocbt_mergeable(
    bs: *mut xchk_btree,
    ca: *mut xchk_alloc,
    irec: *const xfs_alloc_rec_incore,
) {
    if (*(*(*bs).sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 {
        return;
    }

    if (*ca).prev.ar_blockcount > 0
        && (*ca).prev.ar_startblock + (*ca).prev.ar_blockcount == (*irec).ar_startblock
        && (*ca).prev.ar_blockcount + (*irec).ar_blockcount < u32::MAX as _
    {
        xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0);
    }

    core::ptr::copy_nonoverlapping(irec, &mut (*ca).prev, 1);
}

/* Scrub a bnobt/cntbt record. */
unsafe fn xchk_allocbt_rec(bs: *mut xchk_btree, rec: *const xfs_btree_rec) -> i32 {
    let mut irec: xfs_alloc_rec_incore = core::mem::zeroed();
    let ca = (*bs).private as *mut xchk_alloc;

    xfs_alloc_btrec_to_irec(rec, &mut irec);
    if !xfs_alloc_check_irec(to_perag((*(*bs).cur).bc_group), &irec).is_null() {
        xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0);
        return 0;
    }

    if (*(*(*bs).sc).sm).sm_type == XFS_SCRUB_TYPE_BNOBT {
        xchk_allocbt_mergeable(bs, ca, &irec);
    }
    xchk_allocbt_xref((*bs).sc, &irec);

    0
}

/* Scrub one of the freespace btrees for some AG. */
pub unsafe fn xchk_allocbt(sc: *mut xfs_scrub) -> i32 {
    let mut ca: xchk_alloc = core::mem::zeroed();
    let cur: *mut xfs_btree_cur;

    match (*(*sc).sm).sm_type {
        XFS_SCRUB_TYPE_BNOBT => cur = (*(*sc).sa).bno_cur,
        XFS_SCRUB_TYPE_CNTBT => cur = (*(*sc).sa).cnt_cur,
        _ => {
            ASSERT(false);
            return -EIO;
        }
    }

    xchk_btree(sc, cur, xchk_allocbt_rec, &XFS_RMAP_OINFO_AG, &mut ca)
}

/* xref check that the extent is not free */
pub unsafe fn xchk_xref_is_used_space(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t) {
    let mut outcome: xbtree_recpacking = core::mem::zeroed();
    let mut error: i32;

    if (*(*sc).sa).bno_cur.is_null() || xchk_skip_xref((*sc).sm) {
        return;
    }

    error = xfs_alloc_has_records((*(*sc).sa).bno_cur, agbno, len, &mut outcome);
    if !xchk_should_check_xref(sc, &mut error, &mut (*(*sc).sa).bno_cur) {
        return;
    }
    if outcome != XBTREE_RECPACKING_EMPTY {
        xchk_btree_xref_set_corrupt(sc, (*(*sc).sa).bno_cur, 0);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
