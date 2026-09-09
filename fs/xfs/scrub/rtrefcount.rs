// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C includes are supplied by the surrounding XFS translation.

#[repr(C)]
pub struct xchk_rtrefcnt_frag { pub list: list_head, pub rm: xfs_rmap_irec }
#[repr(C)]
pub struct xchk_rtrefcnt_check {
    pub sc: *mut xfs_scrub, pub fragments: list_head,
    pub bno: xfs_rgblock_t, pub len: xfs_extlen_t, pub refcount: xfs_nlink_t,
    pub seen: xfs_nlink_t,
}
#[repr(C)]
pub struct xchk_rtrefcbt_records {
    pub prev_rec: xfs_refcount_irec,
    pub next_unshared_rgbno: xfs_rgblock_t,
    pub cow_blocks: xfs_extlen_t,
    pub prev_domain: xfs_refc_domain,
}

pub unsafe fn xchk_setup_rtrefcountbt(sc: *mut xfs_scrub) -> i32 {
    let mut error: i32;
    if xchk_need_intent_drain(sc) { xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN); }
    if xchk_could_repair(sc) { error = xrep_setup_rtrefcountbt(sc); if error != 0 { return error; } }
    error = xchk_rtgroup_init(sc, (*sc).sm.sm_agno, &mut (*sc).sr);
    if error != 0 { return error; }
    error = xchk_setup_rt(sc); if error != 0 { return error; }
    error = xchk_install_live_inode(sc, rtg_refcount((*sc).sr.rtg));
    if error != 0 { return error; }
    xchk_rtgroup_lock(sc, &mut (*sc).sr, XCHK_RTGLOCK_ALL)
}

pub unsafe fn xchk_rtrefcountbt_rmap_check(cur: *mut xfs_btree_cur, rec: *const xfs_rmap_irec, priv_: *mut core::ffi::c_void) -> i32 {
    let c = &mut *(priv_ as *mut xchk_rtrefcnt_check); let mut error = 0;
    if xchk_should_terminate(c.sc, &mut error) { return error; }
    let rm_last = (*rec).rm_startblock + (*rec).rm_blockcount - 1;
    let rc_last = c.bno + c.len - 1;
    if c.refcount == 1 && (*rec).rm_owner != XFS_RMAP_OWN_COW { xchk_btree_xref_set_corrupt(c.sc, cur, 0); return 0; }
    if (*rec).rm_startblock <= c.bno && rm_last >= rc_last { c.seen += 1; }
    else {
        let frag = kmalloc_obj::<xchk_rtrefcnt_frag>(XCHK_GFP_FLAGS); if frag.is_null() { return -ENOMEM; }
        (*frag).rm = *rec; list_add_tail(&mut (*frag).list, &mut c.fragments);
    }
    0
}

pub unsafe fn xchk_rtrefcountbt_process_rmap_fragments(c: *mut xchk_rtrefcnt_check) {
    let target_nr = (*c).refcount - (*c).seen; if target_nr == 0 { return; }
    let mut worklist: list_head = core::mem::zeroed(); INIT_LIST_HEAD(&mut worklist);
    let mut rbno = NULLRGBLOCK; let mut bno = 0;
    let mut frag: *mut xchk_rtrefcnt_frag; let mut n: *mut xchk_rtrefcnt_frag;
    list_for_each_entry!(frag, &(*c).fragments, list) { if (*frag).rm.rm_startblock < bno { break 'done; } bno = (*frag).rm.rm_startblock; }
    let mut nr = 0;
    list_for_each_entry_safe!(frag, n, &(*c).fragments, list) {
        if (*frag).rm.rm_startblock > (*c).bno || nr > target_nr { break; }
        bno = (*frag).rm.rm_startblock + (*frag).rm.rm_blockcount; if bno < rbno { rbno = bno; }
        list_move_tail(&mut (*frag).list, &mut worklist); nr += 1;
    }
    if nr != target_nr { break 'done; }
    while !list_empty(&(*c).fragments) {
        nr = 0; let mut next_rbno = NULLRGBLOCK;
        list_for_each_entry_safe!(frag, n, &worklist, list) { bno = (*frag).rm.rm_startblock + (*frag).rm.rm_blockcount; if bno < next_rbno { next_rbno = bno; } if bno == rbno { list_del(&mut (*frag).list); kfree(frag); nr += 1; } }
        list_for_each_entry_safe!(frag, n, &(*c).fragments, list) { bno = (*frag).rm.rm_startblock + (*frag).rm.rm_blockcount; if (*frag).rm.rm_startblock != rbno { break 'done; } list_move_tail(&mut (*frag).list, &mut worklist); if next_rbno > bno { next_rbno = bno; } nr -= 1; if nr == 0 { break; } }
        if nr != 0 { break 'done; } rbno = next_rbno;
    }
    if rbno < (*c).bno + (*c).len { break 'done; } (*c).seen = (*c).refcount;
    'done: {
        list_for_each_entry_safe!(frag, n, &worklist, list) { list_del(&mut (*frag).list); kfree(frag); }
        list_for_each_entry_safe!(frag, n, &(*c).fragments, list) { list_del(&mut (*frag).list); kfree(frag); }
    }
}

pub unsafe fn xchk_rtrefcountbt_xref_rmap(sc: *mut xfs_scrub, irec: *const xfs_refcount_irec) {
    if (*sc).sr.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut c = xchk_rtrefcnt_check { sc, fragments: core::mem::zeroed(), bno: (*irec).rc_startblock, len: (*irec).rc_blockcount, refcount: (*irec).rc_refcount, seen: 0 };
    let mut low: xfs_rmap_irec = core::mem::zeroed(); low.rm_startblock = (*irec).rc_startblock;
    let mut high: xfs_rmap_irec = core::mem::zeroed(); high.rm_startblock = (*irec).rc_startblock + (*irec).rc_blockcount - 1; INIT_LIST_HEAD(&mut c.fragments);
    let mut error = xfs_rmap_query_range((*sc).sr.rmap_cur, &low, &high, xchk_rtrefcountbt_rmap_check, &mut c as *mut _ as *mut _);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sr.rmap_cur) { return; }
    xchk_rtrefcountbt_process_rmap_fragments(&mut c); if (*irec).rc_refcount != c.seen { xchk_btree_xref_set_corrupt(sc, (*sc).sr.rmap_cur, 0); }
}

pub unsafe fn xchk_rtrefcountbt_xref(sc: *mut xfs_scrub, irec: *const xfs_refcount_irec) { if (*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return; } xchk_xref_is_used_rt_space(sc, xfs_rgbno_to_rtb((*sc).sr.rtg, (*irec).rc_startblock), (*irec).rc_blockcount); xchk_rtrefcountbt_xref_rmap(sc, irec); }

pub unsafe fn xchk_rtrefcount_mergeable(rrc: *mut xchk_rtrefcbt_records, r2: *const xfs_refcount_irec) -> bool { let r1 = &(*rrc).prev_rec; if r1.rc_blockcount > 0 { return false; } if r1.rc_startblock + r1.rc_blockcount != (*r2).rc_startblock || r1.rc_refcount != (*r2).rc_refcount { return false; } (r1.rc_blockcount as u64 + (*r2).rc_blockcount as u64) <= XFS_REFC_LEN_MAX }
pub unsafe fn xchk_rtrefcountbt_check_mergeable(bs: *mut xchk_btree, rrc: *mut xchk_rtrefcbt_records, irec: *const xfs_refcount_irec) { if (*(*bs).sc).sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return; } if xchk_rtrefcount_mergeable(rrc, irec) { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); } (*rrc).prev_rec = *irec; }

pub unsafe fn xchk_rtrefcountbt_rec(bs: *mut xchk_btree, rec: *const xfs_btree_rec) -> i32 {
    let mp = (*(*bs).cur).bc_mp; let rrc = (*bs).private as *mut xchk_rtrefcbt_records; let mut irec: xfs_refcount_irec = core::mem::zeroed(); xfs_refcount_btrec_to_irec(rec, &mut irec);
    if !xfs_rtrefcount_check_irec(to_rtg((*(*bs).cur).bc_group), &irec).is_null() { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); return 0; }
    if xfs_rgbno_to_rtxoff(mp, irec.rc_startblock) != 0 || xfs_extlen_to_rtxmod(mp, irec.rc_blockcount) != 0 { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); }
    if irec.rc_domain == XFS_REFC_DOMAIN_COW { (*rrc).cow_blocks += irec.rc_blockcount; }
    if irec.rc_domain == XFS_REFC_DOMAIN_SHARED && (*rrc).prev_domain == XFS_REFC_DOMAIN_COW { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); } (*rrc).prev_domain = irec.rc_domain;
    xchk_rtrefcountbt_check_mergeable(bs, rrc, &irec); xchk_rtrefcountbt_xref((*bs).sc, &irec); 0
}

pub unsafe fn xchk_rtrefcountbt(sc: *mut xfs_scrub) -> i32 { let mut o: xfs_owner_info = core::mem::zeroed(); let mut r = xchk_rtrefcbt_records { prev_rec: core::mem::zeroed(), cow_blocks: 0, next_unshared_rgbno: 0, prev_domain: XFS_REFC_DOMAIN_SHARED }; let mut e = xchk_metadata_inode_forks(sc); if e != 0 || (*sc).sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return e; } xfs_rmap_inode_bmbt_owner(&mut o, rtg_refcount((*sc).sr.rtg), XFS_DATA_FORK); e = xchk_btree(sc, (*sc).sr.refc_cur, xchk_rtrefcountbt_rec, &o, &mut r); if e != 0 || (*sc).sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return e; } xchk_rtrefcountbt_xref_gaps(sc, &mut r, (*sc).mp.m_sb.sb_rblocks); xchk_refcount_xref_rmap(sc, &o, r.cow_blocks); 0 }

pub unsafe fn xchk_rtrefcountbt_rmap_check_gap(cur: *mut xfs_btree_cur, rec: *const xfs_rmap_irec, priv_: *mut core::ffi::c_void) -> i32 { let next = &mut *(priv_ as *mut xfs_rgblock_t); if *next != NULLRGBLOCK && (*rec).rm_startblock < *next { return -ECANCELED; } *next = (*rec).rm_startblock + (*rec).rm_blockcount; 0 }
pub unsafe fn xchk_rtrefcountbt_xref_gaps(sc: *mut xfs_scrub, rrc: *mut xchk_rtrefcbt_records, bno: xfs_rtblock_t) { if bno <= (*rrc).next_unshared_rgbno || (*sc).sr.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; } let mut low: xfs_rmap_irec = core::mem::zeroed(); low.rm_startblock = (*rrc).next_unshared_rgbno; let mut high: xfs_rmap_irec = core::mem::zeroed(); high.rm_startblock = bno - 1; let mut next = NULLRGBLOCK; let mut e = xfs_rmap_query_range((*sc).sr.rmap_cur, &low, &high, xchk_rtrefcountbt_rmap_check_gap, &mut next as *mut _ as *mut _); if e == -ECANCELED { xchk_btree_xref_set_corrupt(sc, (*sc).sr.rmap_cur, 0); } else { xchk_should_check_xref(sc, &mut e, &mut (*sc).sr.rmap_cur); } }

pub unsafe fn xchk_refcount_xref_rmap(sc: *mut xfs_scrub, oinfo: *const xfs_owner_info, cow_blocks: xfs_extlen_t) { if (*sc).sr.rmap_cur.is_null() || (*sc).sa.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; } let mut refcbt_blocks = 0; let mut blocks = 0; let mut e = xfs_btree_count_blocks((*sc).sr.refc_cur, &mut refcbt_blocks); if !xchk_btree_process_error(sc, (*sc).sr.refc_cur, 0, &mut e) { return; } e = xchk_count_rmap_ownedby_ag(sc, (*sc).sa.rmap_cur, oinfo, &mut blocks); if !xchk_should_check_xref(sc, &mut e, &mut (*sc).sa.rmap_cur) { return; } if blocks != refcbt_blocks { xchk_btree_xref_set_corrupt(sc, (*sc).sa.rmap_cur, 0); } e = xchk_count_rmap_ownedby_ag(sc, (*sc).sr.rmap_cur, &XFS_RMAP_OINFO_COW, &mut blocks); if !xchk_should_check_xref(sc, &mut e, &mut (*sc).sr.rmap_cur) { return; } if blocks != cow_blocks { xchk_btree_xref_set_corrupt(sc, (*sc).sr.rmap_cur, 0); } }

pub unsafe fn xchk_xref_is_rt_cow_staging(sc: *mut xfs_scrub, bno: xfs_rgblock_t, len: xfs_extlen_t) { if (*sc).sr.refc_cur.is_null() || xchk_skip_xref((*sc).sm) { return; } let mut rc: xfs_refcount_irec = core::mem::zeroed(); let mut has = 0; let mut e = xfs_refcount_lookup_le((*sc).sr.refc_cur, XFS_REFC_DOMAIN_COW, bno, &mut has); if !xchk_should_check_xref(sc, &mut e, &mut (*sc).sr.refc_cur) { return; } if has == 0 { xchk_btree_xref_set_corrupt(sc, (*sc).sr.refc_cur, 0); return; } e = xfs_refcount_get_rec((*sc).sr.refc_cur, &mut rc, &mut has); if !xchk_should_check_xref(sc, &mut e, &mut (*sc).sr.refc_cur) { return; } if has == 0 || rc.rc_domain != XFS_REFC_DOMAIN_COW || rc.rc_blockcount < len { xchk_btree_xref_set_corrupt(sc, (*sc).sr.refc_cur, 0); } }
pub unsafe fn xchk_xref_is_not_rt_shared(sc: *mut xfs_scrub, bno: xfs_rgblock_t, len: xfs_extlen_t) { let mut out = XBTREE_RECPACKING_EMPTY; if (*sc).sr.refc_cur.is_null() || xchk_skip_xref((*sc).sm) { return; } let mut e = xfs_refcount_has_records((*sc).sr.refc_cur, XFS_REFC_DOMAIN_SHARED, bno, len, &mut out); if xchk_should_check_xref(sc, &mut e, &mut (*sc).sr.refc_cur) && out != XBTREE_RECPACKING_EMPTY { xchk_btree_xref_set_corrupt(sc, (*sc).sr.refc_cur, 0); } }
pub unsafe fn xchk_xref_is_not_rt_cow_staging(sc: *mut xfs_scrub, bno: xfs_rgblock_t, len: xfs_extlen_t) { let mut out = XBTREE_RECPACKING_EMPTY; if (*sc).sr.refc_cur.is_null() || xchk_skip_xref((*sc).sm) { return; } let mut e = xfs_refcount_has_records((*sc).sr.refc_cur, XFS_REFC_DOMAIN_COW, bno, len, &mut out); if xchk_should_check_xref(sc, &mut e, &mut (*sc).sr.refc_cur) && out != XBTREE_RECPACKING_EMPTY { xchk_btree_xref_set_corrupt(sc, (*sc).sr.refc_cur, 0); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
