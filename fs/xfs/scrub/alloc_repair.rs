// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// XFS headers and symbols are supplied by the surrounding translation unit.

/*
 * Free Space Btree Repair
 * =======================
 *
 * The reverse mappings are supposed to record all space usage for the entire
 * AG. Therefore, we can recreate the free extent records in an AG by looking
 * for gaps in the physical extents recorded in the rmapbt.
 *
 * We use the prefix 'xrep_abt' here because we regenerate both free space
 * allocation btrees at the same time.
 */

#[repr(C)]
pub struct xrep_abt {
    pub not_allocbt_blocks: xagb_bitmap,
    pub old_allocbt_blocks: xagb_bitmap,
    pub new_bnobt: xrep_newbt,
    pub new_cntbt: xrep_newbt,
    pub free_records: *mut xfarray,
    pub sc: *mut xfs_scrub,
    pub nr_real_records: u64,
    pub array_cur: xfarray_idx_t,
    pub next_agbno: xfs_agblock_t,
    pub nr_blocks: xfs_agblock_t,
    pub longest: xfs_agblock_t,
}

pub unsafe fn xrep_setup_ag_allocbt(sc: *mut xfs_scrub) -> i32 {
    let xg = pag_group((*sc).sa.pag);
    let mut busy_gen: u32 = 0;
    if xfs_extent_busy_list_empty(xg, &mut busy_gen) { 0 }
    else { xfs_extent_busy_flush((*sc).tp, xg, busy_gen, 0) }
}

unsafe fn xrep_abt_check_free_ext(sc: *mut xfs_scrub, rec: *const xfs_alloc_rec_incore) -> i32 {
    let mut outcome: xbtree_recpacking = XBTREE_RECPACKING_EMPTY;
    if !xfs_alloc_check_irec((*sc).sa.pag, rec).is_null() { return -EFSCORRUPTED; }
    let mut error = xfs_ialloc_has_inodes_at_extent((*sc).sa.ino_cur,
        (*rec).ar_startblock, (*rec).ar_blockcount, &mut outcome);
    if error != 0 { return error; }
    if outcome != XBTREE_RECPACKING_EMPTY { return -EFSCORRUPTED; }
    if !(*sc).sa.refc_cur.is_null() {
        error = xfs_refcount_has_records((*sc).sa.refc_cur, XFS_REFC_DOMAIN_SHARED,
            (*rec).ar_startblock, (*rec).ar_blockcount, &mut outcome);
        if error != 0 { return error; }
        if outcome != XBTREE_RECPACKING_EMPTY { return -EFSCORRUPTED; }
        error = xfs_refcount_has_records((*sc).sa.refc_cur, XFS_REFC_DOMAIN_COW,
            (*rec).ar_startblock, (*rec).ar_blockcount, &mut outcome);
        if error != 0 { return error; }
        if outcome != XBTREE_RECPACKING_EMPTY { return -EFSCORRUPTED; }
    }
    0
}

unsafe fn xrep_abt_stash(ra: *mut xrep_abt, end: xfs_agblock_t) -> i32 {
    let arec = xfs_alloc_rec_incore { ar_startblock: (*ra).next_agbno,
        ar_blockcount: end - (*ra).next_agbno };
    let mut error = 0;
    if xchk_should_terminate((*ra).sc, &mut error) { return error; }
    error = xrep_abt_check_free_ext((*ra).sc, &arec);
    if error != 0 { return error; }
    trace_xrep_abt_found((*ra).sc.sa.pag, &arec);
    error = xfarray_append((*ra).free_records, &arec);
    if error != 0 { return error; }
    (*ra).nr_blocks += arec.ar_blockcount;
    0
}

pub unsafe fn xrep_abt_walk_rmap(cur: *mut xfs_btree_cur, rec: *const xfs_rmap_irec, priv_: *mut core::ffi::c_void) -> i32 {
    let ra = priv_ as *mut xrep_abt;
    let mut error;
    if (*rec).rm_owner == XFS_RMAP_OWN_AG {
        error = xagb_bitmap_set(&mut (*ra).old_allocbt_blocks, (*rec).rm_startblock, (*rec).rm_blockcount);
        if error != 0 { return error; }
    }
    error = xagb_bitmap_set_btcur_path(&mut (*ra).not_allocbt_blocks, cur);
    if error != 0 { return error; }
    if (*rec).rm_startblock > (*ra).next_agbno {
        error = xrep_abt_stash(ra, (*rec).rm_startblock);
        if error != 0 { return error; }
    }
    (*ra).next_agbno = core::cmp::max((*ra).next_agbno,
        (*rec).rm_startblock + (*rec).rm_blockcount);
    0
}

unsafe fn xrep_abt_walk_agfl(_mp: *mut xfs_mount, agbno: xfs_agblock_t, priv_: *mut core::ffi::c_void) -> i32 {
    xagb_bitmap_set(&mut (*(priv_ as *mut xrep_abt)).not_allocbt_blocks, agbno, 1)
}

unsafe fn xrep_bnobt_extent_cmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let ap = &*(a as *const xfs_alloc_rec_incore); let bp = &*(b as *const xfs_alloc_rec_incore);
    if ap.ar_startblock > bp.ar_startblock { 1 } else if ap.ar_startblock < bp.ar_startblock { -1 } else { 0 }
}

unsafe fn xrep_bnobt_sort_records(ra: *mut xrep_abt) -> i32 {
    let mut cur = XFARRAY_CURSOR_INIT; let mut arec = core::mem::zeroed(); let mut next = 0;
    let mut error = xfarray_sort((*ra).free_records, xrep_bnobt_extent_cmp, 0);
    if error != 0 { return error; }
    while { error = xfarray_iter((*ra).free_records, &mut cur, &mut arec); error == 1 } {
        if arec.ar_startblock < next { return -EFSCORRUPTED; }
        next = arec.ar_startblock + arec.ar_blockcount;
    }
    error
}

unsafe fn xrep_cntbt_extent_cmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let ap = &*(a as *const xfs_alloc_rec_incore); let bp = &*(b as *const xfs_alloc_rec_incore);
    if ap.ar_blockcount > bp.ar_blockcount { 1 } else if ap.ar_blockcount < bp.ar_blockcount { -1 } else { xrep_bnobt_extent_cmp(a,b) }
}

unsafe fn xrep_cntbt_sort_records(ra: *mut xrep_abt, is_resort: bool) -> i32 {
    xfarray_sort((*ra).free_records, xrep_cntbt_extent_cmp, if is_resort { 0 } else { XFARRAY_SORT_KILLABLE })
}

// The remaining routines are a literal unsafe translation of the source-level
// repair workflow; external XFS declarations are intentionally unresolved.
unsafe fn xrep_abt_find_freespace(ra: *mut xrep_abt) -> i32 {
    let sc = (*ra).sc; let mp = (*sc).mp; let agf = (*sc).sa.agf_bp.b_addr as *mut xfs_agf;
    let mut agfl_bp = core::ptr::null_mut(); let mut error;
    xagb_bitmap_init(&mut (*ra).not_allocbt_blocks); xrep_ag_btcur_init(sc, &mut (*sc).sa);
    error = xfs_rmap_query_all((*sc).sa.rmap_cur, xrep_abt_walk_rmap, ra as *mut _);
    if error != 0 { xchk_ag_btcur_free(&mut (*sc).sa); xagb_bitmap_destroy(&mut (*ra).not_allocbt_blocks); return error; }
    let agend = be32_to_cpu((*agf).agf_length);
    if (*ra).next_agbno < agend { error = xrep_abt_stash(ra, agend); if error != 0 { xchk_ag_btcur_free(&mut (*sc).sa); xagb_bitmap_destroy(&mut (*ra).not_allocbt_blocks); return error; } }
    error = xfs_alloc_read_agfl((*sc).sa.pag, (*sc).tp, &mut agfl_bp);
    if error == 0 { error = xfs_agfl_walk(mp, agf, agfl_bp, xrep_abt_walk_agfl, ra as *mut _); }
    if error == 0 { error = xagb_bitmap_disunion(&mut (*ra).old_allocbt_blocks, &mut (*ra).not_allocbt_blocks); }
    if error == 0 { (*ra).nr_real_records = xfarray_length((*ra).free_records); }
    if !agfl_bp.is_null() { xfs_trans_brelse((*sc).tp, agfl_bp); }
    xchk_ag_btcur_free(&mut (*sc).sa); xagb_bitmap_destroy(&mut (*ra).not_allocbt_blocks); error
}

// Complex bulk-loader helpers and the public entry points retain the original
// ordering and side effects.  These declarations are supplied by the adjacent
// translated implementation where the corresponding helper bodies reside.
extern "C" {
    fn xrep_abt_build_new_trees(ra: *mut xrep_abt) -> i32;
    fn xrep_abt_remove_old_trees(ra: *mut xrep_abt) -> i32;
}
pub unsafe fn xrep_allocbt(sc: *mut xfs_scrub) -> i32 {
    if !xfs_has_rmapbt((*sc).mp) { return -EOPNOTSUPP; }
    let ra = kzalloc_obj::<xrep_abt>(XCHK_GFP_FLAGS); if ra.is_null() { return -ENOMEM; }
    (*ra).sc = sc; (*sc).sick_mask = XFS_SICK_AG_BNOBT | XFS_SICK_AG_CNTBT;
    let mut busy_gen = 0; let mut error = 0;
    if !xfs_extent_busy_list_empty(pag_group((*sc).sa.pag), &mut busy_gen) { error = -EDEADLOCK; }
    else { error = xfarray_create("free space records", (*sc).mp.m_sb.sb_agblocks / 2, core::mem::size_of::<xfs_alloc_rec_incore>(), &mut (*ra).free_records); }
    if error == 0 { xagb_bitmap_init(&mut (*ra).old_allocbt_blocks); error = xrep_abt_find_freespace(ra); }
    if error == 0 { error = xrep_abt_build_new_trees(ra); }
    if error == 0 { error = xrep_abt_remove_old_trees(ra); }
    xagb_bitmap_destroy(&mut (*ra).old_allocbt_blocks); xfarray_destroy((*ra).free_records); kfree(ra); error
}

pub unsafe fn xrep_revalidate_allocbt(sc: *mut xfs_scrub) -> i32 {
    let old_type = (*sc).sm.sm_type; (*sc).sm.sm_type = XFS_SCRUB_TYPE_BNOBT;
    let mut error = xchk_allocbt(sc);
    if error == 0 && ((*sc).sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT) == 0 {
        (*sc).sm.sm_type = XFS_SCRUB_TYPE_CNTBT;
        if (*sc).sa.cnt_cur.is_null() { xchk_set_incomplete(sc); } else { error = xchk_allocbt(sc); }
    }
    (*sc).sm.sm_type = old_type; error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
