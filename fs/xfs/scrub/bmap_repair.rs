// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/*
 * Inode Fork Block Mapping (BMBT) Repair
 * ======================================
 *
 * Gather all the rmap records for the inode and fork we're fixing, reset the
 * incore fork, then recreate the btree.
 */

#[repr(C)]
pub enum reflink_scan_state {
    RLS_IRRELEVANT = -1,
    RLS_UNKNOWN,
    RLS_SET_IFLAG,
}

#[repr(C)]
pub struct xrep_bmap {
    pub old_bmbt_blocks: xfsb_bitmap,
    pub new_bmapbt: xrep_newbt,
    pub bmap_records: *mut xfarray,
    pub sc: *mut xfs_scrub,
    pub nblocks: xfs_rfsblock_t,
    pub old_bmbt_block_count: xfs_rfsblock_t,
    pub array_cur: xfarray_idx_t,
    pub real_mappings: u64,
    pub whichfork: i32,
    pub reflink_scan: reflink_scan_state,
    pub allow_unwritten: bool,
}

unsafe fn xrep_bmap_discover_shared(rb: *mut xrep_bmap, startblock: xfs_fsblock_t,
        blockcount: xfs_filblks_t) -> i32 {
    let sc = (*rb).sc;
    let (agbno, cur) = if XFS_IS_REALTIME_INODE((*sc).ip) {
        (xfs_rtb_to_rgbno((*sc).mp, startblock), (*sc).sr.refc_cur)
    } else {
        (XFS_FSB_TO_AGBNO((*sc).mp, startblock), (*sc).sa.refc_cur)
    };
    let mut fbno = 0;
    let mut flen = 0;
    let error = xfs_refcount_find_shared(cur, agbno, blockcount, &mut fbno, &mut flen, false);
    if error != 0 { return error; }
    if fbno != NULLAGBLOCK { (*rb).reflink_scan = reflink_scan_state::RLS_SET_IFLAG; }
    0
}

unsafe fn xrep_bmap_from_rmap(rb: *mut xrep_bmap, mut startoff: xfs_fileoff_t,
        mut startblock: xfs_fsblock_t, mut blockcount: xfs_filblks_t, unwritten: bool) -> i32 {
    let sc = (*rb).sc;
    let mut irec = xfs_bmbt_irec { br_startoff: startoff, br_startblock: startblock,
        br_blockcount: 0, br_state: if unwritten { XFS_EXT_UNWRITTEN } else { XFS_EXT_NORM } };
    let mut rbe = xfs_bmbt_rec::default();
    let mut error = 0;
    if matches!((*rb).reflink_scan, reflink_scan_state::RLS_UNKNOWN) && !unwritten {
        error = xrep_bmap_discover_shared(rb, startblock, blockcount);
        if error != 0 { return error; }
    }
    while blockcount > 0 {
        irec.br_blockcount = min_t::<xfs_filblks_t>(blockcount, XFS_MAX_BMBT_EXTLEN);
        let fa = xfs_bmap_validate_extent((*sc).ip, (*rb).whichfork, &mut irec);
        if !fa.is_null() { return -EFSCORRUPTED; }
        xfs_bmbt_disk_set_all(&mut rbe, &irec);
        trace_xrep_bmap_found((*sc).ip, (*rb).whichfork, &irec);
        if xchk_should_terminate(sc, &mut error) { return error; }
        error = xfarray_append((*rb).bmap_records, &rbe);
        if error != 0 { return error; }
        (*rb).real_mappings += 1;
        startblock = startblock.wrapping_add(irec.br_blockcount);
        startoff = startoff.wrapping_add(irec.br_blockcount);
        blockcount -= irec.br_blockcount;
    }
    0
}

unsafe fn xrep_bmap_check_fork_rmap(rb: *mut xrep_bmap, cur: *mut xfs_btree_cur,
        rec: *const xfs_rmap_irec) -> i32 {
    let sc = (*rb).sc;
    if XFS_IS_REALTIME_INODE((*sc).ip) && ((*rec).rm_flags & (XFS_RMAP_ATTR_FORK | XFS_RMAP_BMBT_BLOCK)) == 0 { return -EFSCORRUPTED; }
    if !xfs_verify_agbext(to_perag((*cur).bc_group), (*rec).rm_startblock, (*rec).rm_blockcount) { return -EFSCORRUPTED; }
    if ((*rec).rm_flags & XFS_RMAP_BMBT_BLOCK) == 0 && !xfs_verify_fileext((*sc).mp, (*rec).rm_offset, (*rec).rm_blockcount) { return -EFSCORRUPTED; }
    if (((*rec).rm_flags & (XFS_RMAP_ATTR_FORK | XFS_RMAP_BMBT_BLOCK)) != 0) && ((*rec).rm_flags & XFS_RMAP_UNWRITTEN) != 0 { return -EFSCORRUPTED; }
    let mut outcome = 0;
    let mut error = xfs_alloc_has_records((*sc).sa.bno_cur, (*rec).rm_startblock, (*rec).rm_blockcount, &mut outcome);
    if error != 0 { return error; }
    if outcome != XBTREE_RECPACKING_EMPTY { return -EFSCORRUPTED; }
    error = xfs_ialloc_has_inodes_at_extent((*sc).sa.ino_cur, (*rec).rm_startblock, (*rec).rm_blockcount, &mut outcome);
    if error != 0 { return error; }
    if outcome != XBTREE_RECPACKING_EMPTY { return -EFSCORRUPTED; }
    0
}

unsafe extern "C" fn xrep_bmap_walk_rmap(cur: *mut xfs_btree_cur, rec: *const xfs_rmap_irec, priv_: *mut core::ffi::c_void) -> i32 {
    let rb = priv_ as *mut xrep_bmap;
    let mut error = 0;
    if xchk_should_terminate((*rb).sc, &mut error) { return error; }
    if (*rec).rm_owner != I_INO((*(*rb).sc).ip) { return 0; }
    error = xrep_bmap_check_fork_rmap(rb, cur, rec); if error != 0 { return error; }
    (*rb).nblocks += (*rec).rm_blockcount;
    if ((*rb).whichfork == XFS_DATA_FORK && ((*rec).rm_flags & XFS_RMAP_ATTR_FORK) != 0) ||
       ((*rb).whichfork == XFS_ATTR_FORK && ((*rec).rm_flags & XFS_RMAP_ATTR_FORK) == 0) { return 0; }
    if ((*rec).rm_flags & XFS_RMAP_UNWRITTEN) != 0 && !(*rb).allow_unwritten { return -EFSCORRUPTED; }
    let fsbno = xfs_agbno_to_fsb(to_perag((*cur).bc_group), (*rec).rm_startblock);
    if ((*rec).rm_flags & XFS_RMAP_BMBT_BLOCK) != 0 {
        (*rb).old_bmbt_block_count += (*rec).rm_blockcount;
        return xfsb_bitmap_set(&mut (*rb).old_bmbt_blocks, fsbno, (*rec).rm_blockcount);
    }
    xrep_bmap_from_rmap(rb, (*rec).rm_offset, fsbno, (*rec).rm_blockcount, (*rec).rm_flags & XFS_RMAP_UNWRITTEN != 0)
}

unsafe fn xrep_bmap_extent_cmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let ao = xfs_bmbt_disk_get_startoff(a as *const xfs_bmbt_rec);
    let bo = xfs_bmbt_disk_get_startoff(b as *const xfs_bmbt_rec);
    if ao > bo { 1 } else if ao < bo { -1 } else { 0 }
}

unsafe fn xrep_bmap_sort_records(rb: *mut xrep_bmap) -> i32 {
    let mut error = 0;
    error = xfarray_sort((*rb).bmap_records, Some(xrep_bmap_extent_cmp), XFARRAY_SORT_KILLABLE);
    if error != 0 { return error; }
    let mut next_off = 0;
    foreach_xfarray_idx((*rb).bmap_records, array_cur) {
        let mut rec = xfs_bmbt_rec::default();
        if xchk_should_terminate((*rb).sc, &mut error) { return error; }
        error = xfarray_load((*rb).bmap_records, array_cur, &mut rec); if error != 0 { return error; }
        let mut irec = xfs_bmbt_irec::default(); xfs_bmbt_disk_get_all(&rec, &mut irec);
        if irec.br_startoff < next_off { return -EFSCORRUPTED; }
        next_off = irec.br_startoff + irec.br_blockcount;
    }
    0
}

unsafe fn xrep_bmap_scan_ag(rb: *mut xrep_bmap, pag: *mut xfs_perag) -> i32 {
    let sc = (*rb).sc; let mut error = xrep_ag_init(sc, pag, &mut (*sc).sa);
    if error == 0 { error = xfs_rmap_query_all((*sc).sa.rmap_cur, Some(xrep_bmap_walk_rmap), rb as *mut _); }
    xchk_ag_free(sc, &mut (*sc).sa); error
}

#[cfg(feature = "CONFIG_XFS_RT")]
unsafe fn xrep_bmap_scan_rtgroup(rb: *mut xrep_bmap, rtg: *mut xfs_rtgroup) -> i32 {
    let sc = (*rb).sc; if !xfs_has_rtrmapbt((*sc).mp) { return 0; }
    let mut error = xrep_rtgroup_init(sc, rtg, &mut (*sc).sr, XFS_RTGLOCK_RMAP | XFS_RTGLOCK_REFCOUNT | XFS_RTGLOCK_BITMAP_SHARED);
    if error == 0 { error = xfs_rmap_query_all((*sc).sr.rmap_cur, Some(xrep_bmap_walk_rtrmap), rb as *mut _); }
    xchk_rtgroup_btcur_free(&mut (*sc).sr); xchk_rtgroup_free(sc, &mut (*sc).sr); error
}
#[cfg(not(feature = "CONFIG_XFS_RT"))]
unsafe fn xrep_bmap_scan_rtgroup(_rb: *mut xrep_bmap, _rtg: *mut xfs_rtgroup) -> i32 { -EFSCORRUPTED }

#[cfg(feature = "CONFIG_XFS_RT")]
unsafe extern "C" fn xrep_bmap_walk_rtrmap(cur: *mut xfs_btree_cur, rec: *const xfs_rmap_irec, priv_: *mut core::ffi::c_void) -> i32 {
    let rb = priv_ as *mut xrep_bmap; let mut error = 0;
    if xchk_should_terminate((*rb).sc, &mut error) { return error; }
    if (*rec).rm_owner != I_INO((*(*rb).sc).ip) { return 0; }
    if ((*rec).rm_flags & XFS_RMAP_ATTR_FORK) != 0 || ((*rec).rm_flags & XFS_RMAP_BMBT_BLOCK) != 0 || !XFS_IS_REALTIME_INODE((*(*rb).sc).ip) { return -EFSCORRUPTED; }
    if !xfs_verify_fileext((*(*rb).sc).mp, (*rec).rm_offset, (*rec).rm_blockcount) || !xfs_verify_rgbext(to_rtg((*cur).bc_group), (*rec).rm_startblock, (*rec).rm_blockcount) { return -EFSCORRUPTED; }
    error = xrep_require_rtext_inuse((*rb).sc, (*rec).rm_startblock, (*rec).rm_blockcount); if error != 0 { return error; }
    (*rb).nblocks += (*rec).rm_blockcount;
    if ((*rb).whichfork == XFS_DATA_FORK && ((*rec).rm_flags & XFS_RMAP_ATTR_FORK) != 0) || (*rb).whichfork == XFS_ATTR_FORK { return 0; }
    xrep_bmap_from_rmap(rb, (*rec).rm_offset, xfs_rgbno_to_rtb(to_rtg((*cur).bc_group), (*rec).rm_startblock), (*rec).rm_blockcount, (*rec).rm_flags & XFS_RMAP_UNWRITTEN != 0)
}

/* The remaining declarations retain the source-level repair pipeline. */
unsafe fn xrep_bmap_find_delalloc(rb: *mut xrep_bmap) -> i32 { let ip = (*(*rb).sc).ip; if (*rb).whichfork == XFS_ATTR_FORK || (*ip).i_delayed_blks == 0 { return 0; } let ifp = xfs_ifork_ptr(ip, (*rb).whichfork); let mut icur = xfs_iext_cursor::default(); let mut irec = xfs_bmbt_irec::default(); let mut rbe = xfs_bmbt_rec::default(); let mut error = 0; for_each_xfs_iext(ifp, &mut icur, &mut irec) { if !isnullstartblock(irec.br_startblock) { continue; } xfs_bmbt_disk_set_all(&mut rbe, &irec); trace_xrep_bmap_found(ip, (*rb).whichfork, &irec); if xchk_should_terminate((*rb).sc, &mut error) { return error; } error = xfarray_append((*rb).bmap_records, &rbe); if error != 0 { return error; } } 0 }

unsafe fn xrep_bmap_find_mappings(rb: *mut xrep_bmap) -> i32 { let sc = (*rb).sc; let mut rtg = core::ptr::null_mut(); if !xfs_is_metadir_inode((*sc).ip) { while { rtg = xfs_rtgroup_next((*sc).mp, rtg); !rtg.is_null() } { let e = xrep_bmap_scan_rtgroup(rb, rtg); if e != 0 { xfs_rtgroup_rele(rtg); return e; } } } let mut pag = core::ptr::null_mut(); while { pag = xfs_perag_next((*sc).mp, pag); !pag.is_null() } { let e = xrep_bmap_scan_ag(rb, pag); if e != 0 { xfs_perag_rele(pag); return e; } } xrep_bmap_find_delalloc(rb) }

unsafe fn xrep_bmap_get_records(cur: *mut xfs_btree_cur, mut idx: u32, block: *mut xfs_btree_block, nr_wanted: u32, priv_: *mut core::ffi::c_void) -> i32 { let rb = priv_ as *mut xrep_bmap; let irec = &mut (*cur).bc_rec.b; let mut loaded = 0; while loaded < nr_wanted { let mut rec = xfs_bmbt_rec::default(); loop { let e = xfarray_load((*rb).bmap_records, (*rb).array_cur, &mut rec); (*rb).array_cur += 1; if e != 0 { return e; } xfs_bmbt_disk_get_all(&rec, irec); if !isnullstartblock(irec.br_startblock) { break; } } let block_rec = xfs_btree_rec_addr(cur, idx, block); ((*cur).bc_ops).init_rec_from_cur(cur, block_rec); loaded += 1; idx += 1; } loaded as i32 }
unsafe fn xrep_bmap_claim_block(cur: *mut xfs_btree_cur, ptr: *mut xfs_btree_ptr, priv_: *mut core::ffi::c_void) -> i32 { xrep_newbt_claim_block(cur, &mut (*(priv_ as *mut xrep_bmap)).new_bmapbt, ptr) }
unsafe fn xrep_bmap_iroot_size(cur: *mut xfs_btree_cur, _level: u32, nr_this_level: u32, _priv_: *mut core::ffi::c_void) -> usize { ASSERT(_level > 0); xfs_bmap_broot_space_calc((*cur).bc_mp, nr_this_level) }

unsafe fn xrep_bmap_reset_counters(rb: *mut xrep_bmap) -> i32 { let sc = (*rb).sc; let ifake = &mut (*rb).new_bmapbt.ifake; if matches!((*rb).reflink_scan, reflink_scan_state::RLS_SET_IFLAG) { (*(*sc).ip).i_diflags2 |= XFS_DIFLAG2_REFLINK; } let delta = ifake.if_blocks - (*rb).old_bmbt_block_count; (*(*sc).ip).i_nblocks = (*rb).nblocks + delta; xfs_trans_log_inode((*sc).tp, (*sc).ip, XFS_ILOG_CORE); xfs_trans_mod_dquot_byino((*sc).tp, (*sc).ip, XFS_TRANS_DQ_BCOUNT, delta); 0 }
unsafe fn xrep_bmap_extents_load(rb: *mut xrep_bmap) -> i32 { let ifp = (*rb).new_bmapbt.ifake.if_fork; ASSERT((*ifp).if_bytes == 0); let mut icur = xfs_iext_cursor::default(); let mut error = 0; xfs_iext_first(ifp, &mut icur); foreach_xfarray_idx((*rb).bmap_records, array_cur) { let mut rec = xfs_bmbt_rec::default(); error = xfarray_load((*rb).bmap_records, array_cur, &mut rec); if error != 0 { return error; } let mut irec = xfs_bmbt_irec::default(); xfs_bmbt_disk_get_all(&rec, &mut irec); xfs_iext_insert_raw(ifp, &mut icur, &irec); if !isnullstartblock(irec.br_startblock) { (*ifp).if_nextents += 1; } xfs_iext_next(ifp, &mut icur); } xrep_ino_ensure_extent_count((*rb).sc, (*rb).whichfork, (*ifp).if_nextents) }

/* External repair stages are declared here to preserve the implementation interface. */
unsafe fn xrep_bmap_build_new_fork(rb: *mut xrep_bmap) -> i32 { let e = xrep_bmap_sort_records(rb); if e != 0 { return e; } let sc = (*rb).sc; let mut oinfo = xfs_owner_info::default(); xfs_rmap_inode_bmbt_owner(&mut oinfo, (*sc).ip, (*rb).whichfork); let e = xrep_newbt_init_inode(&mut (*rb).new_bmapbt, sc, (*rb).whichfork, &oinfo); if e != 0 { return e; } let mut bmap_cur = xfs_bmbt_init_cursor((*sc).mp, core::ptr::null_mut(), (*sc).ip, XFS_STAGING_FORK); let ifake = &mut (*rb).new_bmapbt.ifake; xfs_btree_stage_ifakeroot(bmap_cur, ifake); let e = if (*rb).real_mappings <= XFS_IFORK_MAXEXT((*sc).ip, (*rb).whichfork) { (*ifake).if_fork.if_format = XFS_DINODE_FMT_EXTENTS; xrep_bmap_extents_load(rb) } else { (*ifake).if_fork.if_format = XFS_DINODE_FMT_BTREE; xrep_bmap_btree_load(rb, bmap_cur) }; if e != 0 { xfs_btree_del_cursor(bmap_cur, e); xrep_newbt_cancel(&mut (*rb).new_bmapbt); return e; } xfs_bmbt_commit_staged_btree(bmap_cur, (*sc).tp, (*rb).whichfork); xfs_btree_del_cursor(bmap_cur, 0); let e = xrep_bmap_reset_counters(rb); if e != 0 { xrep_newbt_cancel(&mut (*rb).new_bmapbt); return e; } let e = xrep_newbt_commit(&mut (*rb).new_bmapbt); if e != 0 { return e; } xrep_roll_trans(sc) }
unsafe fn xrep_bmap_btree_load(rb: *mut xrep_bmap, cur: *mut xfs_btree_cur) -> i32 { let sc = (*rb).sc; let mut error = xfs_btree_bload_compute_geometry(cur, &mut (*rb).new_bmapbt.bload, (*rb).real_mappings); if error != 0 { return error; } if xchk_should_terminate(sc, &mut error) { return error; } error = xfs_trans_reserve_more_inode((*sc).tp, (*sc).ip, (*rb).new_bmapbt.bload.nr_blocks, 0, true); if error != 0 { return error; } error = xrep_newbt_alloc_blocks(&mut (*rb).new_bmapbt, (*rb).new_bmapbt.bload.nr_blocks); if error != 0 { return error; } (*rb).array_cur = XFARRAY_CURSOR_INIT; error = xfs_btree_bload(cur, &mut (*rb).new_bmapbt.bload, rb as *mut _); if error != 0 { return error; } xrep_bmap_extents_load(rb) }
unsafe fn xrep_bmap_remove_old_tree(rb: *mut xrep_bmap) -> i32 { let sc = (*rb).sc; let mut oinfo = xfs_owner_info::default(); xfs_rmap_inode_bmbt_owner(&mut oinfo, (*sc).ip, (*rb).whichfork); xrep_reap_fsblocks(sc, &mut (*rb).old_bmbt_blocks, &oinfo) }

unsafe fn xrep_bmap_check_inputs(sc: *mut xfs_scrub, whichfork: i32) -> i32 { ASSERT(whichfork == XFS_DATA_FORK || whichfork == XFS_ATTR_FORK); if !xfs_has_rmapbt((*sc).mp) { return -EOPNOTSUPP; } let ifp = xfs_ifork_ptr((*sc).ip, whichfork); if ifp.is_null() { return -ECANCELED; } match (*ifp).if_format { XFS_DINODE_FMT_DEV | XFS_DINODE_FMT_LOCAL | XFS_DINODE_FMT_UUID | XFS_DINODE_FMT_META_BTREE => -ECANCELED, XFS_DINODE_FMT_EXTENTS | XFS_DINODE_FMT_BTREE => { if whichfork == XFS_ATTR_FORK { 0 } else { match VFS_I((*sc).ip).i_mode & S_IFMT { S_IFREG | S_IFDIR | S_IFLNK => 0, _ => -EINVAL } } }, _ => -EFSCORRUPTED } }
unsafe fn xrep_bmap_init_reflink_scan(sc: *mut xfs_scrub, whichfork: i32) -> reflink_scan_state { if !xfs_has_reflink((*sc).mp) || xfs_is_reflink_inode((*sc).ip) || !S_ISREG(VFS_I((*sc).ip).i_mode) || whichfork != XFS_DATA_FORK { if xfs_is_reflink_inode((*sc).ip) { return reflink_scan_state::RLS_SET_IFLAG; } return reflink_scan_state::RLS_IRRELEVANT; } reflink_scan_state::RLS_UNKNOWN }

pub unsafe fn xrep_bmap(sc: *mut xfs_scrub, whichfork: i32, allow_unwritten: bool) -> i32 { let mut error = xrep_bmap_check_inputs(sc, whichfork); if error == -ECANCELED { return 0; } if error != 0 { return error; } let rb = kzalloc_obj::<xrep_bmap>(XCHK_GFP_FLAGS); if rb.is_null() { return -ENOMEM; } (*rb).sc = sc; (*rb).whichfork = whichfork; (*rb).reflink_scan = xrep_bmap_init_reflink_scan(sc, whichfork); (*rb).allow_unwritten = allow_unwritten; let large = xfs_has_large_extent_counts((*sc).mp); let max = xfs_iext_max_nextents(large, whichfork); error = xfarray_create("fork mapping records", max, core::mem::size_of::<xfs_bmbt_rec>(), &mut (*rb).bmap_records); if error == 0 { xfsb_bitmap_init(&mut (*rb).old_bmbt_blocks); error = xrep_bmap_find_mappings(rb); } if error == 0 { xfs_trans_ijoin((*sc).tp, (*sc).ip, 0); error = xrep_bmap_build_new_fork(rb); } if error == 0 { error = xrep_bmap_remove_old_tree(rb); } xfsb_bitmap_destroy(&mut (*rb).old_bmbt_blocks); xfarray_destroy((*rb).bmap_records); kfree(rb); error }
pub unsafe fn xrep_bmap_data(sc: *mut xfs_scrub) -> i32 { xrep_bmap(sc, XFS_DATA_FORK, true) }
pub unsafe fn xrep_bmap_attr(sc: *mut xfs_scrub) -> i32 { xrep_bmap(sc, XFS_ATTR_FORK, false) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
