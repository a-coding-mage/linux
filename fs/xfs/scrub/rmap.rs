// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C dependencies supplied by the surrounding XFS translation.

pub unsafe fn xchk_setup_ag_rmapbt(sc: *mut xfs_scrub) -> i32 {
    if xchk_need_intent_drain(sc) { xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN); }
    if xchk_could_repair(sc) {
        let error = xrep_setup_ag_rmapbt(sc);
        if error != 0 { return error; }
    }
    xchk_setup_ag_btree(sc, false)
}

#[repr(C)]
pub struct xchk_rmap {
    pub overlap_rec: xfs_rmap_irec,
    pub prev_rec: xfs_rmap_irec,
    pub fs_owned: xagb_bitmap,
    pub log_owned: xagb_bitmap,
    pub ag_owned: xagb_bitmap,
    pub inobt_owned: xagb_bitmap,
    pub refcbt_owned: xagb_bitmap,
    pub bitmaps_complete: bool,
}

unsafe fn xchk_rmapbt_xref_refc(sc: *mut xfs_scrub, irec: *mut xfs_rmap_irec) {
    if (*sc).sa.refc_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let non_inode = XFS_RMAP_NON_INODE_OWNER((*irec).rm_owner);
    let is_bmbt = ((*irec).rm_flags & XFS_RMAP_BMBT_BLOCK) != 0;
    let is_attr = ((*irec).rm_flags & XFS_RMAP_ATTR_FORK) != 0;
    let is_unwritten = ((*irec).rm_flags & XFS_RMAP_UNWRITTEN) != 0;
    let mut fbno = 0;
    let mut flen = 0;
    let mut error = xfs_refcount_find_shared((*sc).sa.refc_cur,
        (*irec).rm_startblock, (*irec).rm_blockcount, &mut fbno, &mut flen, false);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.refc_cur) { return; }
    if flen != 0 && (non_inode || is_attr || is_bmbt || is_unwritten) {
        xchk_btree_xref_set_corrupt(sc, (*sc).sa.refc_cur, 0);
    }
}

unsafe fn xchk_rmapbt_xref(sc: *mut xfs_scrub, irec: *mut xfs_rmap_irec) {
    let agbno = (*irec).rm_startblock;
    let len = (*irec).rm_blockcount;
    if ((*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 { return; }
    xchk_xref_is_used_space(sc, agbno, len);
    if (*irec).rm_owner == XFS_RMAP_OWN_INODES { xchk_xref_is_inode_chunk(sc, agbno, len); }
    else { xchk_xref_is_not_inode_chunk(sc, agbno, len); }
    if (*irec).rm_owner == XFS_RMAP_OWN_COW {
        xchk_xref_is_cow_staging(sc, (*irec).rm_startblock, (*irec).rm_blockcount);
    } else { xchk_rmapbt_xref_refc(sc, irec); }
}

unsafe fn xchk_rmapbt_check_unwritten_in_keyflags(bs: *mut xchk_btree) {
    let sc = (*bs).sc; let cur = (*bs).cur;
    if ((*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_PREEN) != 0 { return; }
    let badflag = cpu_to_be64(XFS_RMAP_OFF_UNWRITTEN);
    for level in 1..(*cur).bc_nlevels {
        let mut bp = core::ptr::null_mut();
        if (*cur).bc_levels[level].ptr > 1 { continue; }
        let keyblock = xfs_btree_get_block(cur, level, &mut bp);
        for ptr in 1..=be16_to_cpu((*keyblock).bb_numrecs) {
            let lkey = xfs_btree_key_addr(cur, ptr, keyblock);
            if (*lkey).rmap.rm_offset & badflag != 0 { xchk_btree_set_preen(sc, cur, level); break; }
            let hkey = xfs_btree_high_key_addr(cur, ptr, keyblock);
            if (*hkey).rmap.rm_offset & badflag != 0 { xchk_btree_set_preen(sc, cur, level); break; }
        }
    }
}

unsafe fn xchk_rmapbt_is_shareable(sc: *mut xfs_scrub, irec: *const xfs_rmap_irec) -> bool {
    if !xfs_has_reflink((*sc).mp) || XFS_RMAP_NON_INODE_OWNER((*irec).rm_owner) { return false; }
    ((*irec).rm_flags & (XFS_RMAP_BMBT_BLOCK | XFS_RMAP_ATTR_FORK | XFS_RMAP_UNWRITTEN)) == 0
}

unsafe fn xchk_rmapbt_check_overlapping(bs: *mut xchk_btree, cr: *mut xchk_rmap, irec: *const xfs_rmap_irec) {
    if ((*(*(*bs).sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 { return; }
    if (*cr).overlap_rec.rm_blockcount == 0 { (*cr).overlap_rec = *irec; return; }
    let pnext = (*cr).overlap_rec.rm_startblock + (*cr).overlap_rec.rm_blockcount;
    if pnext <= (*irec).rm_startblock { (*cr).overlap_rec = *irec; return; }
    if !xchk_rmapbt_is_shareable((*bs).sc, &(*cr).overlap_rec) || !xchk_rmapbt_is_shareable((*bs).sc, irec) {
        xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0);
    }
    let inext = (*irec).rm_startblock + (*irec).rm_blockcount;
    if pnext <= inext { (*cr).overlap_rec = *irec; }
}

unsafe fn xchk_rmap_mergeable(cr: *mut xchk_rmap, r2: *const xfs_rmap_irec) -> bool {
    let r1 = &(*cr).prev_rec;
    if r1.rm_blockcount == 0 || r1.rm_owner != (*r2).rm_owner ||
       r1.rm_startblock + r1.rm_blockcount != (*r2).rm_startblock ||
       (r1.rm_blockcount as u64) + (*r2).rm_blockcount as u64 > XFS_RMAP_LEN_MAX { return false; }
    if XFS_RMAP_NON_INODE_OWNER((*r2).rm_owner) { return true; }
    if r1.rm_flags != (*r2).rm_flags { return false; }
    if r1.rm_flags & XFS_RMAP_BMBT_BLOCK != 0 { return true; }
    r1.rm_offset + r1.rm_blockcount == (*r2).rm_offset
}

unsafe fn xchk_rmapbt_check_mergeable(bs: *mut xchk_btree, cr: *mut xchk_rmap, irec: *const xfs_rmap_irec) {
    if ((*(*(*bs).sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 { return; }
    if xchk_rmap_mergeable(cr, irec) { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); }
    (*cr).prev_rec = *irec;
}

unsafe fn xchk_rmapbt_mark_bitmap(bs: *mut xchk_btree, cr: *mut xchk_rmap, irec: *const xfs_rmap_irec) -> i32 {
    let sc = (*bs).sc;
    if ((*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 || !(*cr).bitmaps_complete { return 0; }
    let bmp: *mut xagb_bitmap = match (*irec).rm_owner {
        XFS_RMAP_OWN_FS => &mut (*cr).fs_owned, XFS_RMAP_OWN_LOG => &mut (*cr).log_owned,
        XFS_RMAP_OWN_AG => &mut (*cr).ag_owned, XFS_RMAP_OWN_INOBT => &mut (*cr).inobt_owned,
        XFS_RMAP_OWN_REFC => &mut (*cr).refcbt_owned, _ => core::ptr::null_mut(),
    };
    if bmp.is_null() { return 0; }
    let mut fsbcount = (*irec).rm_blockcount;
    if xagb_bitmap_test(bmp, (*irec).rm_startblock, &mut fsbcount) {
        if fsbcount < (*irec).rm_blockcount { xchk_btree_xref_set_corrupt(sc, (*sc).sa.rmap_cur, 0); }
    } else { xchk_btree_xref_set_corrupt(sc, (*sc).sa.rmap_cur, 0); }
    xagb_bitmap_clear(bmp, (*irec).rm_startblock, (*irec).rm_blockcount)
}

pub unsafe fn xchk_rmapbt_rec(bs: *mut xchk_btree, rec: *const xfs_btree_rec) -> i32 {
    let cr = (*bs).private as *mut xchk_rmap;
    let mut irec = core::mem::zeroed::<xfs_rmap_irec>();
    if !xfs_rmap_btrec_to_irec(rec, &mut irec).is_null() || !xfs_rmap_check_irec(to_perag((*bs).cur.bc_group), &mut irec).is_null() {
        xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); return 0;
    }
    xchk_rmapbt_check_unwritten_in_keyflags(bs); xchk_rmapbt_check_mergeable(bs, cr, &irec);
    xchk_rmapbt_check_overlapping(bs, cr, &irec); xchk_rmapbt_xref((*bs).sc, &mut irec);
    xchk_rmapbt_mark_bitmap(bs, cr, &irec)
}

unsafe fn xchk_rmapbt_walk_agfl(_mp: *mut xfs_mount, agbno: xfs_agblock_t, priv_: *mut core::ffi::c_void) -> i32 {
    xagb_bitmap_set(priv_ as *mut xagb_bitmap, agbno, 1)
}

unsafe fn xchk_rmapbt_walk_ag_metadata(sc: *mut xfs_scrub, cr: *mut xchk_rmap) -> i32 {
    let mp = (*sc).mp; let agf = (*(*sc).sa.agf_bp).b_addr as *mut xfs_agf; let mut error;
    error = xagb_bitmap_set(&mut (*cr).fs_owned, XFS_SB_BLOCK(mp), XFS_AGFL_BLOCK(mp) - XFS_SB_BLOCK(mp) + 1); if error != 0 { return 0; }
    if xfs_ag_contains_log(mp, pag_agno((*sc).sa.pag)) { error = xagb_bitmap_set(&mut (*cr).log_owned, XFS_FSB_TO_AGBNO(mp, (*mp).m_sb.sb_logstart), (*mp).m_sb.sb_logblocks); if error != 0 { return 0; } }
    let mut cur = (*sc).sa.bno_cur; if cur.is_null() { cur = xfs_bnobt_init_cursor(mp, (*sc).tp, (*sc).sa.agf_bp, (*sc).sa.pag); }
    error = xagb_bitmap_set_btblocks(&mut (*cr).ag_owned, cur); if cur != (*sc).sa.bno_cur { xfs_btree_del_cursor(cur, error); } if error != 0 { return 0; }
    cur = (*sc).sa.cnt_cur; if cur.is_null() { cur = xfs_cntbt_init_cursor(mp, (*sc).tp, (*sc).sa.agf_bp, (*sc).sa.pag); }
    error = xagb_bitmap_set_btblocks(&mut (*cr).ag_owned, cur); if cur != (*sc).sa.cnt_cur { xfs_btree_del_cursor(cur, error); } if error != 0 { return 0; }
    error = xagb_bitmap_set_btblocks(&mut (*cr).ag_owned, (*sc).sa.rmap_cur); if error != 0 { return 0; }
    let mut agfl_bp = core::ptr::null_mut(); error = xfs_alloc_read_agfl((*sc).sa.pag, (*sc).tp, &mut agfl_bp); if error != 0 { return 0; }
    error = xfs_agfl_walk(mp, agf, agfl_bp, xchk_rmapbt_walk_agfl, &mut (*cr).ag_owned as *mut _ as *mut _); xfs_trans_brelse((*sc).tp, agfl_bp); if error != 0 { return 0; }
    cur = (*sc).sa.ino_cur; if cur.is_null() { cur = xfs_inobt_init_cursor((*sc).sa.pag, (*sc).tp, (*sc).sa.agi_bp); }
    error = xagb_bitmap_set_btblocks(&mut (*cr).inobt_owned, cur); if cur != (*sc).sa.ino_cur { xfs_btree_del_cursor(cur, error); } if error != 0 { return 0; }
    if xfs_has_finobt(mp) { cur = (*sc).sa.fino_cur; if cur.is_null() { cur = xfs_finobt_init_cursor((*sc).sa.pag, (*sc).tp, (*sc).sa.agi_bp); } error = xagb_bitmap_set_btblocks(&mut (*cr).inobt_owned, cur); if cur != (*sc).sa.fino_cur { xfs_btree_del_cursor(cur, error); } if error != 0 { return 0; } }
    if xfs_has_reflink(mp) { cur = (*sc).sa.refc_cur; if cur.is_null() { cur = xfs_refcountbt_init_cursor(mp, (*sc).tp, (*sc).sa.agf_bp, (*sc).sa.pag); } error = xagb_bitmap_set_btblocks(&mut (*cr).refcbt_owned, cur); if cur != (*sc).sa.refc_cur { xfs_btree_del_cursor(cur, error); } if error != 0 { return 0; } }
    (*cr).bitmaps_complete = true; 0
}

unsafe fn xchk_rmapbt_check_bitmaps(sc: *mut xfs_scrub, cr: *mut xchk_rmap) {
    if ((*(*sc).sm).sm_flags & (XFS_SCRUB_OFLAG_CORRUPT | XFS_SCRUB_OFLAG_XFAIL)) != 0 || (*sc).sa.rmap_cur.is_null() { return; }
    let cur = (*sc).sa.rmap_cur; let level = (*cur).bc_nlevels - 1;
    for bmp in [&(*cr).fs_owned, &(*cr).log_owned, &(*cr).ag_owned, &(*cr).inobt_owned, &(*cr).refcbt_owned] { if xagb_bitmap_hweight(bmp) != 0 { xchk_btree_xref_set_corrupt(sc, cur, level); } }
}

pub unsafe fn xchk_rmapbt(sc: *mut xfs_scrub) -> i32 {
    let cr = kzalloc_obj::<xchk_rmap>(XCHK_GFP_FLAGS); if cr.is_null() { return -ENOMEM; }
    xagb_bitmap_init(&mut (*cr).fs_owned); xagb_bitmap_init(&mut (*cr).log_owned); xagb_bitmap_init(&mut (*cr).ag_owned); xagb_bitmap_init(&mut (*cr).inobt_owned); xagb_bitmap_init(&mut (*cr).refcbt_owned);
    let mut error = xchk_rmapbt_walk_ag_metadata(sc, cr); if error == 0 { error = xchk_btree(sc, (*sc).sa.rmap_cur, xchk_rmapbt_rec, &XFS_RMAP_OINFO_AG, cr); } if error == 0 { xchk_rmapbt_check_bitmaps(sc, cr); }
    xagb_bitmap_destroy(&mut (*cr).refcbt_owned); xagb_bitmap_destroy(&mut (*cr).inobt_owned); xagb_bitmap_destroy(&mut (*cr).ag_owned); xagb_bitmap_destroy(&mut (*cr).log_owned); xagb_bitmap_destroy(&mut (*cr).fs_owned); kfree(cr as *mut _); error
}

pub unsafe fn xchk_xref_is_only_owned_by(sc: *mut xfs_scrub, bno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info) {
    if (*sc).sa.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; } let mut res = core::mem::zeroed(); let mut error = xfs_rmap_count_owners((*sc).sa.rmap_cur, bno, len, oinfo, &mut res); if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.rmap_cur) { return; } if res.matches != 1 || res.bad_non_owner_matches || res.non_owner_matches { xchk_btree_xref_set_corrupt(sc, (*sc).sa.rmap_cur, 0); }
}

pub unsafe fn xchk_xref_is_not_owned_by(sc: *mut xfs_scrub, bno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info) {
    if (*sc).sa.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; } let mut res = core::mem::zeroed(); let mut error = xfs_rmap_count_owners((*sc).sa.rmap_cur, bno, len, oinfo, &mut res); if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.rmap_cur) { return; } if res.matches != 0 || res.bad_non_owner_matches { xchk_btree_xref_set_corrupt(sc, (*sc).sa.rmap_cur, 0); }
}

pub unsafe fn xchk_xref_has_no_owner(sc: *mut xfs_scrub, bno: xfs_agblock_t, len: xfs_extlen_t) {
    if (*sc).sa.rmap_cur.is_null() || xchk_skip_xref((*sc).sm) { return; } let mut outcome = core::mem::zeroed(); let mut error = xfs_rmap_has_records((*sc).sa.rmap_cur, bno, len, &mut outcome); if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.rmap_cur) { return; } if outcome != XBTREE_RECPACKING_EMPTY { xchk_btree_xref_set_corrupt(sc, (*sc).sa.rmap_cur, 0); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
