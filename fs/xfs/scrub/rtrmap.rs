// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// C dependencies supplied by the surrounding translation unit.

/* Set us up with the realtime metadata locked. */
pub unsafe fn xchk_setup_rtrmapbt(sc: *mut xfs_scrub) -> i32 {
    let mut error: i32;

    if xchk_need_intent_drain(sc) {
        xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN);
    }

    if xchk_could_repair(sc) {
        error = xrep_setup_rtrmapbt(sc);
        if error != 0 { return error; }
    }

    error = xchk_rtgroup_init(sc, (*(*sc).sm).sm_agno, &mut (*sc).sr);
    if error != 0 { return error; }
    error = xchk_setup_rt(sc);
    if error != 0 { return error; }
    error = xchk_install_live_inode(sc, rtg_rmap((*sc).sr.rtg));
    if error != 0 { return error; }
    xchk_rtgroup_lock(sc, &mut (*sc).sr, XCHK_RTGLOCK_ALL)
}

/* Realtime reverse mapping. */
#[repr(C)]
pub struct xchk_rtrmap {
    /* The furthest-reaching processed rmapbt record. */
    pub overlap_rec: xfs_rmap_irec,
    /* The previous rmapbt record. */
    pub prev_rec: xfs_rmap_irec,
}

#[inline]
pub unsafe fn xchk_rtrmapbt_is_shareable(sc: *mut xfs_scrub, irec: *const xfs_rmap_irec) -> bool {
    if !xfs_has_rtreflink((*sc).mp) { return false; }
    if (*irec).rm_flags & XFS_RMAP_UNWRITTEN != 0 { return false; }
    if (*irec).rm_owner == XFS_RMAP_OWN_COW || (*irec).rm_owner == XFS_RMAP_OWN_FS { return false; }
    true
}

/* Flag failures for records that overlap but cannot. */
pub unsafe fn xchk_rtrmapbt_check_overlapping(bs: *mut xchk_btree, cr: *mut xchk_rtrmap, irec: *const xfs_rmap_irec) {
    if (*(*bs).sc).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return; }
    if (*cr).overlap_rec.rm_blockcount == 0 { (*cr).overlap_rec = *irec; return; }
    let pnext = (*cr).overlap_rec.rm_startblock + (*cr).overlap_rec.rm_blockcount;
    if pnext <= (*irec).rm_startblock { (*cr).overlap_rec = *irec; return; }
    if !xchk_rtrmapbt_is_shareable((*bs).sc, &(*cr).overlap_rec) || !xchk_rtrmapbt_is_shareable((*bs).sc, irec) {
        xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0);
    }
    let inext = (*irec).rm_startblock + (*irec).rm_blockcount;
    if pnext > inext { return; }
    (*cr).overlap_rec = *irec;
}

#[inline]
pub unsafe fn xchk_rtrmap_mergeable(cr: *mut xchk_rtrmap, r2: *const xfs_rmap_irec) -> bool {
    let r1 = &(*cr).prev_rec;
    if r1.rm_blockcount == 0 || r1.rm_owner != (*r2).rm_owner { return false; }
    if r1.rm_startblock + r1.rm_blockcount != (*r2).rm_startblock { return false; }
    if (r1.rm_blockcount as u64).wrapping_add((*r2).rm_blockcount as u64) > XFS_RMAP_LEN_MAX { return false; }
    if r1.rm_flags != (*r2).rm_flags { return false; }
    if r1.rm_owner == XFS_RMAP_OWN_COW || r1.rm_owner == XFS_RMAP_OWN_FS { return true; }
    r1.rm_offset + r1.rm_blockcount == (*r2).rm_offset
}

/* Flag failures for records that could be merged. */
pub unsafe fn xchk_rtrmapbt_check_mergeable(bs: *mut xchk_btree, cr: *mut xchk_rtrmap, irec: *const xfs_rmap_irec) {
    if (*(*bs).sc).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return; }
    if xchk_rtrmap_mergeable(cr, irec) { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); }
    (*cr).prev_rec = *irec;
}

/* Cross-reference a rmap against the refcount btree. */
pub unsafe fn xchk_rtrmapbt_xref_rtrefc(sc: *mut xfs_scrub, irec: *mut xfs_rmap_irec) {
    if (*sc).sr.refc_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let is_inode = !XFS_RMAP_NON_INODE_OWNER((*irec).rm_owner);
    let is_bmbt = (*irec).rm_flags & XFS_RMAP_BMBT_BLOCK != 0;
    let is_attr = (*irec).rm_flags & XFS_RMAP_ATTR_FORK != 0;
    let is_unwritten = (*irec).rm_flags & XFS_RMAP_UNWRITTEN != 0;
    let mut fbno = 0; let mut flen = 0;
    let mut error = xfs_refcount_find_shared((*sc).sr.refc_cur, (*irec).rm_startblock, (*irec).rm_blockcount, &mut fbno, &mut flen, false);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sr.refc_cur) { return; }
    if flen != 0 && (!is_inode || is_attr || is_bmbt || is_unwritten) { xchk_btree_xref_set_corrupt(sc, (*sc).sr.refc_cur, 0); }
}

/* Cross-reference with other metadata. */
pub unsafe fn xchk_rtrmapbt_xref(sc: *mut xfs_scrub, irec: *mut xfs_rmap_irec) {
    if (*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return; }
    xchk_xref_is_used_rt_space(sc, xfs_rgbno_to_rtb((*sc).sr.rtg, (*irec).rm_startblock), (*irec).rm_blockcount);
    if (*irec).rm_owner == XFS_RMAP_OWN_COW { xchk_xref_is_rt_cow_staging(sc, (*irec).rm_startblock, (*irec).rm_blockcount); } else { xchk_rtrmapbt_xref_rtrefc(sc, irec); }
}

/* Scrub a realtime rmapbt record. */
pub unsafe fn xchk_rtrmapbt_rec(bs: *mut xchk_btree, rec: *const xfs_btree_rec) -> i32 {
    let cr = (*bs).private as *mut xchk_rtrmap;
    let mut irec = core::mem::zeroed::<xfs_rmap_irec>();
    if !xfs_rmap_btrec_to_irec(rec, &mut irec).is_null() || !xfs_rtrmap_check_irec(to_rtg((*(*bs).cur).bc_group), &mut irec).is_null() { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); return 0; }
    if (*(*bs).sc).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return 0; }
    xchk_rtrmapbt_check_mergeable(bs, cr, &irec); xchk_rtrmapbt_check_overlapping(bs, cr, &irec); xchk_rtrmapbt_xref((*bs).sc, &mut irec); 0
}

/* Scrub the realtime rmap btree. */
pub unsafe fn xchk_rtrmapbt(sc: *mut xfs_scrub) -> i32 {
    let mut oinfo = core::mem::zeroed::<xfs_owner_info>();
    let mut cr = core::mem::zeroed::<xchk_rtrmap>();
    let error = xchk_metadata_inode_forks(sc);
    if error != 0 || (*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return error; }
    xfs_rmap_inode_bmbt_owner(&mut oinfo, rtg_rmap((*sc).sr.rtg), XFS_DATA_FORK);
    xchk_btree(sc, (*sc).sr.rmap_cur, xchk_rtrmapbt_rec, &mut oinfo, &mut cr)
}

/* xref check that the extent has no realtime reverse mapping at all */
pub unsafe fn xchk_xref_has_no_rt_owner(sc: *mut xfs_scrub, bno: xfs_rgblock_t, len: xfs_extlen_t) {
    if (*sc).sr.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut outcome = core::mem::zeroed::<xbtree_recpacking>();
    let mut error = xfs_rmap_has_records((*sc).sr.rmap_cur, bno, len, &mut outcome);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sr.rmap_cur) { return; }
    if outcome != XBTREE_RECPACKING_EMPTY { xchk_btree_xref_set_corrupt(sc, (*sc).sr.rmap_cur, 0); }
}

/* xref check that the extent is completely mapped */
pub unsafe fn xchk_xref_has_rt_owner(sc: *mut xfs_scrub, bno: xfs_rgblock_t, len: xfs_extlen_t) {
    if (*sc).sr.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut outcome = core::mem::zeroed::<xbtree_recpacking>(); let mut error = xfs_rmap_has_records((*sc).sr.rmap_cur, bno, len, &mut outcome);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sr.rmap_cur) { return; }
    if outcome != XBTREE_RECPACKING_FULL { xchk_btree_xref_set_corrupt(sc, (*sc).sr.rmap_cur, 0); }
}

/* xref check that the extent is only owned by a given owner */
pub unsafe fn xchk_xref_is_only_rt_owned_by(sc: *mut xfs_scrub, bno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info) {
    if (*sc).sr.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut res = core::mem::zeroed::<xfs_rmap_matches>(); let mut error = xfs_rmap_count_owners((*sc).sr.rmap_cur, bno, len, oinfo, &mut res);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sr.rmap_cur) { return; }
    if res.matches != 1 || res.bad_non_owner_matches || res.non_owner_matches { xchk_btree_xref_set_corrupt(sc, (*sc).sr.rmap_cur, 0); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
