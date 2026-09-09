// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C dependencies supplied by other translation units are intentionally external.

#[repr(C)]
pub struct xrep_refc {
    pub refcount_records: *mut xfarray,
    pub new_btree: xrep_newbt,
    pub old_refcountbt_blocks: xagb_bitmap,
    pub sc: *mut xfs_scrub,
    pub array_cur: xfarray_idx_t,
    pub btblocks: xfs_extlen_t,
}

pub unsafe fn xrep_setup_ag_refcountbt(sc: *mut xfs_scrub) -> i32 {
    xrep_setup_xfbtree(sc, c"rmap record bag".as_ptr())
}

unsafe fn xrep_refc_check_ext(sc: *mut xfs_scrub, rec: *const xfs_refcount_irec) -> i32 {
    let mut outcome: xbtree_recpacking;
    let mut error: i32;
    if !xfs_refcount_check_irec((*sc).sa.pag, rec).is_null() { return -EFSCORRUPTED; }
    error = xfs_alloc_has_records((*sc).sa.bno_cur, (*rec).rc_startblock, (*rec).rc_blockcount, &mut outcome);
    if error != 0 { return error; }
    if outcome != XBTREE_RECPACKING_EMPTY { return -EFSCORRUPTED; }
    error = xfs_ialloc_has_inodes_at_extent((*sc).sa.ino_cur, (*rec).rc_startblock, (*rec).rc_blockcount, &mut outcome);
    if error != 0 { return error; }
    if outcome != XBTREE_RECPACKING_EMPTY { return -EFSCORRUPTED; }
    0
}

unsafe fn xrep_refc_stash(rr: *mut xrep_refc, domain: xfs_refc_domain, agbno: xfs_agblock_t, len: xfs_extlen_t, refcount: u64) -> i32 {
    let mut irec = xfs_refcount_irec { rc_startblock: agbno, rc_blockcount: len, rc_domain: domain, rc_refcount: 0 };
    let sc = (*rr).sc;
    let mut error = 0;
    if xchk_should_terminate(sc, &mut error) { return error; }
    irec.rc_refcount = core::cmp::min(XFS_REFC_REFCOUNT_MAX, refcount);
    error = xrep_refc_check_ext(sc, &irec);
    if error != 0 { return error; }
    trace_xrep_refc_found(pag_group((*sc).sa.pag), &irec);
    xfarray_append((*rr).refcount_records, &irec)
}

unsafe fn xrep_refc_stash_cow(rr: *mut xrep_refc, agbno: xfs_agblock_t, len: xfs_extlen_t) -> i32 {
    xrep_refc_stash(rr, XFS_REFC_DOMAIN_COW, agbno, len, 1)
}

unsafe fn xrep_refc_rmap_shareable(mp: *mut xfs_mount, rmap: *const xfs_rmap_irec) -> bool {
    if XFS_RMAP_NON_INODE_OWNER((*rmap).rm_owner) { return false; }
    if xfs_is_sb_inum(mp, (*rmap).rm_owner) { return false; }
    if ((*rmap).rm_flags & (XFS_RMAP_ATTR_FORK | XFS_RMAP_BMBT_BLOCK | XFS_RMAP_UNWRITTEN)) != 0 { return false; }
    true
}

unsafe fn xrep_refc_walk_rmaps(rr: *mut xrep_refc, rmap: *mut xfs_rmap_irec, have_rec: *mut bool) -> i32 {
    let cur = (*(*rr).sc).sa.rmap_cur;
    let mp = (*cur).bc_mp;
    let mut have_gt: i32;
    let mut error = 0;
    *have_rec = false;
    loop {
        if xchk_should_terminate((*rr).sc, &mut error) { return error; }
        error = xfs_btree_increment(cur, 0, &mut have_gt); if error != 0 { return error; }
        if have_gt == 0 { return 0; }
        error = xfs_rmap_get_rec(cur, rmap, &mut have_gt); if error != 0 { return error; }
        if XFS_IS_CORRUPT(mp, have_gt == 0) { xfs_btree_mark_sick(cur); return -EFSCORRUPTED; }
        if (*rmap).rm_owner == XFS_RMAP_OWN_COW { error = xrep_refc_stash_cow(rr, (*rmap).rm_startblock, (*rmap).rm_blockcount); if error != 0 { return error; } }
        else if (*rmap).rm_owner == XFS_RMAP_OWN_REFC { (*rr).btblocks += (*rmap).rm_blockcount; error = xagb_bitmap_set(&mut (*rr).old_refcountbt_blocks, (*rmap).rm_startblock, (*rmap).rm_blockcount); if error != 0 { return error; } }
        if xrep_refc_rmap_shareable(mp, rmap) { break; }
    }
    *have_rec = true; 0
}

unsafe fn xrep_refc_encode_startblock(irec: *const xfs_refcount_irec) -> u32 {
    let mut start = (*irec).rc_startblock & !XFS_REFC_COWFLAG;
    if (*irec).rc_domain == XFS_REFC_DOMAIN_COW { start |= XFS_REFC_COWFLAG; }
    start
}

unsafe fn xrep_refc_extent_cmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let sa = xrep_refc_encode_startblock(a as *const xfs_refcount_irec);
    let sb = xrep_refc_encode_startblock(b as *const xfs_refcount_irec);
    if sa > sb { 1 } else if sa < sb { -1 } else { 0 }
}

unsafe fn xrep_refc_sort_records(rr: *mut xrep_refc) -> i32 {
    let mut irec: xfs_refcount_irec = core::mem::zeroed();
    let mut cur: xfarray_idx_t;
    let mut dom = XFS_REFC_DOMAIN_SHARED;
    let mut next_agbno: xfs_agblock_t = 0;
    let mut error = xfarray_sort((*rr).refcount_records, xrep_refc_extent_cmp, XFARRAY_SORT_KILLABLE);
    if error != 0 { return error; }
    foreach_xfarray_idx!((*rr).refcount_records, cur, {
        if xchk_should_terminate((*rr).sc, &mut error) { return error; }
        error = xfarray_load((*rr).refcount_records, cur, &mut irec); if error != 0 { return error; }
        if dom == XFS_REFC_DOMAIN_SHARED && irec.rc_domain == XFS_REFC_DOMAIN_COW { dom = irec.rc_domain; next_agbno = 0; }
        if dom != irec.rc_domain || irec.rc_startblock < next_agbno { return -EFSCORRUPTED; }
        next_agbno = irec.rc_startblock + irec.rc_blockcount;
    });
    error
}

unsafe fn xrep_refc_push_rmaps_at(rr: *mut xrep_refc, rcstack: *mut rcbag, bno: xfs_agblock_t, rmap: *mut xfs_rmap_irec, have: *mut bool) -> i32 {
    let sc = (*rr).sc; let mut have_gt: i32; let mut error: i32;
    while *have && (*rmap).rm_startblock == bno { error = rcbag_add(rcstack, (*sc).tp, rmap); if error != 0 { return error; } error = xrep_refc_walk_rmaps(rr, rmap, have); if error != 0 { return error; } }
    error = xfs_btree_decrement((*sc).sa.rmap_cur, 0, &mut have_gt); if error != 0 { return error; }
    if XFS_IS_CORRUPT((*sc).mp, have_gt == 0) { xfs_btree_mark_sick((*sc).sa.rmap_cur); return -EFSCORRUPTED; } 0
}

unsafe fn xrep_refc_find_refcounts(rr: *mut xrep_refc) -> i32 {
    let sc = (*rr).sc; let mut rcstack: *mut rcbag = core::ptr::null_mut(); let mut old_stack_height: u64; let mut sbno; let mut cbno; let mut nbno; let mut have = false; let mut error: i32;
    xrep_ag_btcur_init(sc, &mut (*sc).sa);
    error = rcbag_init((*sc).mp, (*sc).xmbtp, &mut rcstack); if error != 0 { return error; }
    error = xfs_btree_goto_left_edge((*sc).sa.rmap_cur); if error != 0 { rcbag_free(&mut rcstack); return error; }
    while xfs_btree_has_more_records((*sc).sa.rmap_cur) {
        let mut rmap: xfs_rmap_irec = core::mem::zeroed();
        error = xrep_refc_walk_rmaps(rr, &mut rmap, &mut have); if error != 0 { break; } if !have { break; }
        sbno = rmap.rm_startblock; cbno = sbno; error = xrep_refc_push_rmaps_at(rr, rcstack, sbno, &mut rmap, &mut have); if error != 0 { break; }
        error = rcbag_next_edge(rcstack, (*sc).tp, &rmap, have, &mut nbno); if error != 0 { break; }
        old_stack_height = rcbag_count(rcstack);
        while rcbag_count(rcstack) > 0 {
            error = rcbag_remove_ending_at(rcstack, (*sc).tp, nbno); if error != 0 { break; }
            error = xrep_refc_walk_rmaps(rr, &mut rmap, &mut have); if error != 0 { break; }
            if have { error = xrep_refc_push_rmaps_at(rr, rcstack, nbno, &mut rmap, &mut have); if error != 0 { break; } }
            if rcbag_count(rcstack) != old_stack_height { if old_stack_height > 1 { error = xrep_refc_stash(rr, XFS_REFC_DOMAIN_SHARED, cbno, nbno - cbno, old_stack_height); if error != 0 { break; } } cbno = nbno; }
            if rcbag_count(rcstack) == 0 { break; }
            old_stack_height = rcbag_count(rcstack); sbno = nbno; error = rcbag_next_edge(rcstack, (*sc).tp, &rmap, have, &mut nbno); if error != 0 { break; }
        }
        if error != 0 { break; }
    }
    rcbag_free(&mut rcstack); xchk_ag_btcur_free(&mut (*sc).sa); error
}

unsafe fn xrep_refc_get_records(cur: *mut xfs_btree_cur, mut idx: u32, block: *mut xfs_btree_block, nr_wanted: u32, priv_: *mut core::ffi::c_void) -> i32 {
    let irec = &mut (*cur).bc_rec.rc; let rr = priv_ as *mut xrep_refc; let mut loaded = 0; while loaded < nr_wanted { let block_rec = xfs_btree_rec_addr(cur, idx, block); let error = xfarray_load((*rr).refcount_records, (*rr).array_cur, irec); if error != 0 { return error; } (*rr).array_cur += 1; (*cur).bc_ops.init_rec_from_cur(cur, block_rec); loaded += 1; idx += 1; } loaded as i32
}

unsafe fn xrep_refc_claim_block(cur: *mut xfs_btree_cur, ptr: *mut xfs_btree_ptr, priv_: *mut core::ffi::c_void) -> i32 { xrep_newbt_claim_block(cur, &mut (*(priv_ as *mut xrep_refc)).new_btree, ptr) }

unsafe fn xrep_refc_reset_counters(rr: *mut xrep_refc) -> i32 { let sc = (*rr).sc; let pag = (*sc).sa.pag; (*pag).pagf_repair_refcount_level = (*pag).pagf_refcount_level; xrep_reinit_pagf(sc) }

unsafe fn xrep_refc_build_new_tree(rr: *mut xrep_refc) -> i32 {
    let sc = (*rr).sc; let pag = (*sc).sa.pag; let mut refc_cur: *mut xfs_btree_cur; let mut error = xrep_refc_sort_records(rr); if error != 0 { return error; }
    xrep_newbt_init_ag(&mut (*rr).new_btree, sc, &XFS_RMAP_OINFO_REFC, xfs_agbno_to_fsb(pag, xfs_refc_block((*sc).mp)), XFS_AG_RESV_METADATA);
    (*rr).new_btree.bload.get_records = Some(xrep_refc_get_records); (*rr).new_btree.bload.claim_block = Some(xrep_refc_claim_block);
    refc_cur = xfs_refcountbt_init_cursor((*sc).mp, core::ptr::null_mut(), core::ptr::null_mut(), pag); xfs_btree_stage_afakeroot(refc_cur, &mut (*rr).new_btree.afake);
    error = xfs_btree_bload_compute_geometry(refc_cur, &mut (*rr).new_btree.bload, xfarray_length((*rr).refcount_records)); if error != 0 { xfs_btree_del_cursor(refc_cur, error); xrep_newbt_cancel(&mut (*rr).new_btree); return error; }
    if xchk_should_terminate(sc, &mut error) { xfs_btree_del_cursor(refc_cur, error); xrep_newbt_cancel(&mut (*rr).new_btree); return error; }
    error = xrep_newbt_alloc_blocks(&mut (*rr).new_btree, (*rr).new_btree.bload.nr_blocks); if error != 0 { xfs_btree_del_cursor(refc_cur, error); xrep_newbt_cancel(&mut (*rr).new_btree); return error; }
    (*pag).pagf_repair_refcount_level = (*rr).new_btree.bload.btree_height; (*rr).array_cur = XFARRAY_CURSOR_INIT;
    error = xfs_btree_bload(refc_cur, &mut (*rr).new_btree.bload, rr); if error != 0 { (*pag).pagf_repair_refcount_level = 0; xfs_btree_del_cursor(refc_cur, error); xrep_newbt_cancel(&mut (*rr).new_btree); return error; }
    xfs_refcountbt_commit_staged_btree(refc_cur, (*sc).tp, (*sc).sa.agf_bp); xfs_btree_del_cursor(refc_cur, 0); error = xrep_refc_reset_counters(rr); if error != 0 { xrep_newbt_cancel(&mut (*rr).new_btree); return error; } error = xrep_newbt_commit(&mut (*rr).new_btree); if error != 0 { return error; } xrep_roll_ag_trans(sc)
}

unsafe fn xrep_refc_remove_old_tree(rr: *mut xrep_refc) -> i32 { let sc = (*rr).sc; let pag = (*sc).sa.pag; let error = xrep_reap_agblocks(sc, &mut (*rr).old_refcountbt_blocks, &XFS_RMAP_OINFO_REFC, XFS_AG_RESV_METADATA); if error != 0 { return error; } (*pag).pagf_repair_refcount_level = 0; (*sc).flags |= XREP_RESET_PERAG_RESV; 0 }

pub unsafe fn xrep_refcountbt(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp; if !xfs_has_rmapbt(mp) { return -EOPNOTSUPP; }
    let rr = kzalloc_obj::<xrep_refc>(XCHK_GFP_FLAGS); if rr.is_null() { return -ENOMEM; } (*rr).sc = sc;
    let mut error = xfarray_create(c"reference count records".as_ptr(), (*mp).m_sb.sb_agblocks, core::mem::size_of::<xfs_refcount_irec>(), &mut (*rr).refcount_records); if error != 0 { kfree(rr); return error; }
    xagb_bitmap_init(&mut (*rr).old_refcountbt_blocks); error = xrep_refc_find_refcounts(rr); if error == 0 { error = xrep_refc_build_new_tree(rr); } if error == 0 { error = xrep_refc_remove_old_tree(rr); }
    xagb_bitmap_destroy(&mut (*rr).old_refcountbt_blocks); xfarray_destroy((*rr).refcount_records); kfree(rr); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
