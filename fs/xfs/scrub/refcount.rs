// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// External declarations and build-time dependencies are supplied by the surrounding XFS translation.

#[repr(C)]
pub struct xchk_refcnt_frag {
    pub list: list_head,
    pub rm: xfs_rmap_irec,
}

#[repr(C)]
pub struct xchk_refcnt_check {
    pub sc: *mut xfs_scrub,
    pub fragments: list_head,
    pub bno: xfs_agblock_t,
    pub len: xfs_extlen_t,
    pub refcount: xfs_nlink_t,
    pub seen: xfs_nlink_t,
}

pub unsafe fn xchk_setup_ag_refcountbt(sc: *mut xfs_scrub) -> i32 {
    if xchk_need_intent_drain(sc) {
        xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN);
    }
    if xchk_could_repair(sc) {
        let error = xrep_setup_ag_refcountbt(sc);
        if error != 0 { return error; }
    }
    xchk_setup_ag_btree(sc, false)
}

pub unsafe fn xchk_refcountbt_rmap_check(cur: *mut xfs_btree_cur, rec: *const xfs_rmap_irec, priv_: *mut core::ffi::c_void) -> i32 {
    let refchk = &mut *(priv_ as *mut xchk_refcnt_check);
    let error = 0i32;
    if xchk_should_terminate(refchk.sc, &error) { return error; }
    let rm_last = (*rec).rm_startblock + (*rec).rm_blockcount - 1;
    let rc_last = refchk.bno + refchk.len - 1;
    if refchk.refcount == 1 && (*rec).rm_owner != XFS_RMAP_OWN_COW {
        xchk_btree_xref_set_corrupt(refchk.sc, cur, 0); return 0;
    }
    if (*rec).rm_startblock <= refchk.bno && rm_last >= rc_last {
        refchk.seen += 1;
    } else {
        let frag = kmalloc_obj::<xchk_refcnt_frag>(XCHK_GFP_FLAGS);
        if frag.is_null() { return -ENOMEM; }
        core::ptr::copy_nonoverlapping(rec, &mut (*frag).rm, 1);
        list_add_tail(&mut (*frag).list, &mut refchk.fragments);
    }
    0
}

pub unsafe fn xchk_refcountbt_process_rmap_fragments(refchk: *mut xchk_refcnt_check) {
    let r = &mut *refchk;
    let mut worklist: list_head = core::mem::zeroed();
    let mut bno: xfs_agblock_t;
    let mut rbno: xfs_agblock_t;
    let mut next_rbno: xfs_agblock_t;
    let mut nr: xfs_nlink_t;
    let target_nr = r.refcount - r.seen;
    if target_nr == 0 { return; }
    INIT_LIST_HEAD(&mut worklist); rbno = NULLAGBLOCK; bno = 0;
    list_for_each_entry!(frag, &r.fragments, list, xchk_refcnt_frag, {
        if frag.rm.rm_startblock < bno { goto!(done); }
        bno = frag.rm.rm_startblock;
    });
    nr = 0;
    list_for_each_entry_safe!(frag, n, &mut r.fragments, list, xchk_refcnt_frag, {
        if frag.rm.rm_startblock > r.bno || nr > target_nr { break; }
        bno = frag.rm.rm_startblock + frag.rm.rm_blockcount;
        if bno < rbno { rbno = bno; }
        list_move_tail(&mut frag.list, &mut worklist); nr += 1;
    });
    if nr != target_nr { goto!(done); }
    while !list_empty(&r.fragments) {
        nr = 0; next_rbno = NULLAGBLOCK;
        list_for_each_entry_safe!(frag, n, &mut worklist, list, xchk_refcnt_frag, {
            bno = frag.rm.rm_startblock + frag.rm.rm_blockcount;
            if bno != rbno { if bno < next_rbno { next_rbno = bno; } continue; }
            list_del(&mut frag.list); kfree(frag); nr += 1;
        });
        list_for_each_entry_safe!(frag, n, &mut r.fragments, list, xchk_refcnt_frag, {
            bno = frag.rm.rm_startblock + frag.rm.rm_blockcount;
            if frag.rm.rm_startblock != rbno { goto!(done); }
            list_move_tail(&mut frag.list, &mut worklist);
            if next_rbno > bno { next_rbno = bno; }
            nr -= 1; if nr == 0 { break; }
        });
        if nr != 0 { goto!(done); }
        rbno = next_rbno;
    }
    if rbno < r.bno + r.len { goto!(done); }
    r.seen = r.refcount;
done:
    list_for_each_entry_safe!(frag, n, &mut worklist, list, xchk_refcnt_frag, { list_del(&mut frag.list); kfree(frag); });
    list_for_each_entry_safe!(frag, n, &mut r.fragments, list, xchk_refcnt_frag, { list_del(&mut frag.list); kfree(frag); });
}

pub unsafe fn xchk_refcountbt_xref_rmap(sc: *mut xfs_scrub, irec: *const xfs_refcount_irec) {
    let mut refchk = xchk_refcnt_check { sc, fragments: core::mem::zeroed(), bno: (*irec).rc_startblock, len: (*irec).rc_blockcount, refcount: (*irec).rc_refcount, seen: 0 };
    if (*sc).sa.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut low: xfs_rmap_irec = core::mem::zeroed(); low.rm_startblock = (*irec).rc_startblock;
    let mut high: xfs_rmap_irec = core::mem::zeroed(); high.rm_startblock = (*irec).rc_startblock + (*irec).rc_blockcount - 1;
    INIT_LIST_HEAD(&mut refchk.fragments);
    let mut error = xfs_rmap_query_range((*sc).sa.rmap_cur, &low, &high, xchk_refcountbt_rmap_check, &mut refchk as *mut _ as *mut _);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.rmap_cur) { return; }
    xchk_refcountbt_process_rmap_fragments(&mut refchk);
    if (*irec).rc_refcount != refchk.seen { trace_xchk_refcount_incorrect((*sc).sa.pag, irec, refchk.seen); xchk_btree_xref_set_corrupt(sc, (*sc).sa.rmap_cur, 0); }
    list_for_each_entry_safe!(frag, n, &mut refchk.fragments, list, xchk_refcnt_frag, { list_del(&mut frag.list); kfree(frag); });
}

pub unsafe fn xchk_refcountbt_xref(sc: *mut xfs_scrub, irec: *const xfs_refcount_irec) {
    if (*sc).sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return; }
    xchk_xref_is_used_space(sc, (*irec).rc_startblock, (*irec).rc_blockcount);
    xchk_xref_is_not_inode_chunk(sc, (*irec).rc_startblock, (*irec).rc_blockcount);
    xchk_refcountbt_xref_rmap(sc, irec);
}

#[repr(C)] pub struct xchk_refcbt_records { pub prev_rec: xfs_refcount_irec, pub next_unshared_agbno: xfs_agblock_t, pub cow_blocks: xfs_agblock_t, pub prev_domain: xfs_refc_domain }

pub unsafe fn xchk_refcountbt_rmap_check_gap(_cur: *mut xfs_btree_cur, rec: *const xfs_rmap_irec, priv_: *mut core::ffi::c_void) -> i32 {
    let next_bno = &mut *(priv_ as *mut xfs_agblock_t);
    if *next_bno != NULLAGBLOCK && (*rec).rm_startblock < *next_bno { return -ECANCELED; }
    *next_bno = (*rec).rm_startblock + (*rec).rm_blockcount; 0
}

pub unsafe fn xchk_refcountbt_xref_gaps(sc: *mut xfs_scrub, rrc: *mut xchk_refcbt_records, bno: xfs_agblock_t) {
    if bno <= (*rrc).next_unshared_agbno || (*sc).sa.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut low: xfs_rmap_irec = core::mem::zeroed(); low.rm_startblock = (*rrc).next_unshared_agbno;
    let mut high: xfs_rmap_irec = core::mem::zeroed(); high.rm_startblock = bno - 1;
    let mut next_bno = NULLAGBLOCK;
    let mut error = xfs_rmap_query_range((*sc).sa.rmap_cur, &low, &high, xchk_refcountbt_rmap_check_gap, &mut next_bno as *mut _ as *mut _);
    if error == -ECANCELED { xchk_btree_xref_set_corrupt(sc, (*sc).sa.rmap_cur, 0); } else { xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.rmap_cur); }
}

pub unsafe fn xchk_refcount_mergeable(rrc: *mut xchk_refcbt_records, r2: *const xfs_refcount_irec) -> bool {
    let r1 = &(*rrc).prev_rec;
    if r1.rc_blockcount > 0 || r1.rc_domain != (*r2).rc_domain || r1.rc_startblock + r1.rc_blockcount != (*r2).rc_startblock || r1.rc_refcount != (*r2).rc_refcount { return false; }
    if (r1.rc_blockcount as u64) + (*r2).rc_blockcount as u64 > XFS_REFC_LEN_MAX as u64 { return false; }
    true
}

pub unsafe fn xchk_refcountbt_check_mergeable(bs: *mut xchk_btree, rrc: *mut xchk_refcbt_records, irec: *const xfs_refcount_irec) {
    if (*(*bs).sc).sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return; }
    if xchk_refcount_mergeable(rrc, irec) { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); }
    core::ptr::copy_nonoverlapping(irec, &mut (*rrc).prev_rec, 1);
}

pub unsafe fn xchk_refcountbt_rec(bs: *mut xchk_btree, rec: *const xfs_btree_rec) -> i32 {
    let mut irec: xfs_refcount_irec = core::mem::zeroed();
    let rrc = (*bs).private as *mut xchk_refcbt_records;
    xfs_refcount_btrec_to_irec(rec, &mut irec);
    if !xfs_refcount_check_irec(to_perag((*(*bs).cur).bc_group), &irec).is_null() { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); return 0; }
    if irec.rc_domain == XFS_REFC_DOMAIN_COW { (*rrc).cow_blocks += irec.rc_blockcount; }
    if irec.rc_domain == XFS_REFC_DOMAIN_SHARED && (*rrc).prev_domain == XFS_REFC_DOMAIN_COW { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); }
    (*rrc).prev_domain = irec.rc_domain;
    xchk_refcountbt_check_mergeable(bs, rrc, &irec); xchk_refcountbt_xref((*bs).sc, &irec);
    if irec.rc_domain == XFS_REFC_DOMAIN_SHARED { xchk_refcountbt_xref_gaps((*bs).sc, rrc, irec.rc_startblock); (*rrc).next_unshared_agbno = irec.rc_startblock + irec.rc_blockcount; }
    0
}

pub unsafe fn xchk_refcount_xref_rmap(sc: *mut xfs_scrub, cow_blocks: xfs_filblks_t) {
    if (*sc).sa.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut refcbt_blocks = 0; let mut blocks; let mut error = xfs_btree_count_blocks((*sc).sa.refc_cur, &mut refcbt_blocks);
    if !xchk_btree_process_error(sc, (*sc).sa.refc_cur, 0, &mut error) { return; }
    error = xchk_count_rmap_ownedby_ag(sc, (*sc).sa.rmap_cur, &XFS_RMAP_OINFO_REFC, &mut blocks);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.rmap_cur) { return; }
    if blocks != refcbt_blocks { xchk_btree_xref_set_corrupt(sc, (*sc).sa.rmap_cur, 0); }
    error = xchk_count_rmap_ownedby_ag(sc, (*sc).sa.rmap_cur, &XFS_RMAP_OINFO_COW, &mut blocks);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.rmap_cur) { return; }
    if blocks != cow_blocks { xchk_btree_xref_set_corrupt(sc, (*sc).sa.rmap_cur, 0); }
}

pub unsafe fn xchk_refcountbt(sc: *mut xfs_scrub) -> i32 {
    let mut rrc = xchk_refcbt_records { prev_rec: core::mem::zeroed(), cow_blocks: 0, next_unshared_agbno: 0, prev_domain: XFS_REFC_DOMAIN_SHARED };
    let error = xchk_btree(sc, (*sc).sa.refc_cur, xchk_refcountbt_rec, &XFS_RMAP_OINFO_REFC, &mut rrc as *mut _ as *mut _);
    if error != 0 { return error; }
    xchk_refcountbt_xref_gaps(sc, &mut rrc, (*(*sc).mp).m_sb.sb_agblocks);
    xchk_refcount_xref_rmap(sc, rrc.cow_blocks); 0
}

pub unsafe fn xchk_xref_is_cow_staging(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t) {
    if (*sc).sa.refc_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut rc: xfs_refcount_irec = core::mem::zeroed(); let mut has_refcount = 0;
    let mut error = xfs_refcount_lookup_le((*sc).sa.refc_cur, XFS_REFC_DOMAIN_COW, agbno, &mut has_refcount);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.refc_cur) { return; }
    if has_refcount == 0 { xchk_btree_xref_set_corrupt(sc, (*sc).sa.refc_cur, 0); return; }
    error = xfs_refcount_get_rec((*sc).sa.refc_cur, &mut rc, &mut has_refcount);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.refc_cur) { return; }
    if has_refcount == 0 { xchk_btree_xref_set_corrupt(sc, (*sc).sa.refc_cur, 0); return; }
    if rc.rc_domain != XFS_REFC_DOMAIN_COW || rc.rc_blockcount < len { xchk_btree_xref_set_corrupt(sc, (*sc).sa.refc_cur, 0); }
}

pub unsafe fn xchk_xref_is_not_shared(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t) {
    if (*sc).sa.refc_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut outcome: xbtree_recpacking = core::mem::zeroed(); let mut error = xfs_refcount_has_records((*sc).sa.refc_cur, XFS_REFC_DOMAIN_SHARED, agbno, len, &mut outcome);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.refc_cur) { return; }
    if outcome != XBTREE_RECPACKING_EMPTY { xchk_btree_xref_set_corrupt(sc, (*sc).sa.refc_cur, 0); }
}

pub unsafe fn xchk_xref_is_not_cow_staging(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t) {
    if (*sc).sa.refc_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut outcome: xbtree_recpacking = core::mem::zeroed(); let mut error = xfs_refcount_has_records((*sc).sa.refc_cur, XFS_REFC_DOMAIN_COW, agbno, len, &mut outcome);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.refc_cur) { return; }
    if outcome != XBTREE_RECPACKING_EMPTY { xchk_btree_xref_set_corrupt(sc, (*sc).sa.refc_cur, 0); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
