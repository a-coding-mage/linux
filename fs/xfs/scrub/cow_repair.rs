// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2022-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* CoW Fork Mapping Repair
 * =======================
 *
 * Although CoW staging extents are owned by incore CoW inode forks, on disk
 * they are owned by the refcount btree.  The ondisk metadata does not record
 * any ownership information, which limits what we can do to repair the
 * mappings in the CoW fork.  At most, we can replace ifork mappings that lack
 * an entry in the refcount btree or are described by a reverse mapping record
 * whose owner is not OWN_COW.
 *
 * Replacing extents is also tricky -- we can't touch written CoW fork extents
 * since they are undergoing writeback, and delalloc extents do not require
 * repair since they only exist incore.  Hence the most we can do is find the
 * bad parts of unwritten mappings, allocate a replacement set of blocks, and
 * replace the incore mapping.  We use the regular reaping process to unmap
 * or free the discarded blocks, as appropriate.
 */
#[repr(C)]
pub struct xrep_cow {
    pub sc: *mut xfs_scrub,
    pub bad_fileoffs: xoff_bitmap,
    pub old_cowfork: xrep_cow_old_cowfork,
    pub irec: xfs_bmbt_irec,
    pub irec_startbno: u32,
    pub next_bno: u32,
}

#[repr(C)]
pub union xrep_cow_old_cowfork {
    pub old_cowfork_fsblocks: xfsb_bitmap,
    pub old_cowfork_rtblocks: xrtb_bitmap,
}

unsafe fn xrep_cow_mark_file_range(xc: *mut xrep_cow, startblock: xfs_fsblock_t, blockcount: xfs_filblks_t) -> i32 {
    let startoff = (*xc).irec.br_startoff + (startblock - (*xc).irec.br_startblock);
    trace_xrep_cow_mark_file_range((*(*xc).sc).ip, startblock, startoff, blockcount);
    xoff_bitmap_set(&mut (*xc).bad_fileoffs, startoff, blockcount)
}

unsafe fn xrep_cow_trim_refcount(xc: *mut xrep_cow, dst: *mut xfs_refcount_irec, src: *const xfs_refcount_irec) {
    *dst = *src;
    if (*dst).rc_startblock < (*xc).irec_startbno {
        let adj = (*xc).irec_startbno - (*dst).rc_startblock;
        (*dst).rc_blockcount -= adj;
        (*dst).rc_startblock += adj;
    }
    if (*dst).rc_startblock + (*dst).rc_blockcount > (*xc).irec_startbno + (*xc).irec.br_blockcount {
        let adj = (*dst).rc_startblock + (*dst).rc_blockcount - ((*xc).irec_startbno + (*xc).irec.br_blockcount);
        (*dst).rc_blockcount -= adj;
    }
}

unsafe fn xrep_cow_mark_shared_staging(cur: *mut xfs_btree_cur, rec: *const xfs_refcount_irec, priv_: *mut core::ffi::c_void) -> i32 {
    let xc = priv_ as *mut xrep_cow;
    let mut rrec = core::mem::zeroed::<xfs_refcount_irec>();
    if !xfs_refcount_check_domain(rec) || (*rec).rc_domain != XFS_REFC_DOMAIN_SHARED { return -EFSCORRUPTED; }
    xrep_cow_trim_refcount(xc, &mut rrec, rec);
    xrep_cow_mark_file_range(xc, xfs_gbno_to_fsb((*cur).bc_group, rrec.rc_startblock), rrec.rc_blockcount)
}

unsafe fn xrep_cow_mark_missing_staging(cur: *mut xfs_btree_cur, rec: *const xfs_refcount_irec, priv_: *mut core::ffi::c_void) -> i32 {
    let xc = priv_ as *mut xrep_cow;
    let mut rrec = core::mem::zeroed::<xfs_refcount_irec>();
    if !xfs_refcount_check_domain(rec) || (*rec).rc_domain != XFS_REFC_DOMAIN_COW { return -EFSCORRUPTED; }
    xrep_cow_trim_refcount(xc, &mut rrec, rec);
    if (*xc).next_bno < rrec.rc_startblock {
        let error = xrep_cow_mark_file_range(xc, xfs_gbno_to_fsb((*cur).bc_group, (*xc).next_bno), rrec.rc_startblock - (*xc).next_bno);
        if error != 0 { return error; }
    }
    (*xc).next_bno = rrec.rc_startblock + rrec.rc_blockcount;
    0
}

unsafe fn xrep_cow_mark_missing_staging_rmap(cur: *mut xfs_btree_cur, rec: *const xfs_rmap_irec, priv_: *mut core::ffi::c_void) -> i32 {
    let xc = priv_ as *mut xrep_cow;
    if (*rec).rm_owner == XFS_RMAP_OWN_COW { return 0; }
    let mut rec_bno = (*rec).rm_startblock;
    let mut rec_len = (*rec).rm_blockcount;
    if rec_bno < (*xc).irec_startbno { let adj = (*xc).irec_startbno - rec_bno; rec_len -= adj; rec_bno += adj; }
    if rec_bno + rec_len > (*xc).irec_startbno + (*xc).irec.br_blockcount { rec_len -= rec_bno + rec_len - ((*xc).irec_startbno + (*xc).irec.br_blockcount); }
    xrep_cow_mark_file_range(xc, xfs_gbno_to_fsb((*cur).bc_group, rec_bno), rec_len)
}

unsafe fn xrep_cow_debug_replacement(xc: *mut xrep_cow) -> i32 {
    let mut fsbno = (*xc).irec.br_startblock;
    let mut len = (*xc).irec.br_blockcount;
    let mut trim = if len > 4 { get_random_u32_below(len / 4) } else { 0 };
    len -= trim;
    trim = if len > 4 { get_random_u32_below(len / 4) } else { 0 };
    fsbno += trim; len -= trim;
    xrep_cow_mark_file_range(xc, fsbno, len)
}

/* Find any part of the CoW fork mapping that is not a single-owner staging extent. */
unsafe fn xrep_cow_find_bad(xc: *mut xrep_cow) -> i32 {
    let sc = (*xc).sc;
    let agno = XFS_FSB_TO_AGNO((*sc).mp, (*xc).irec.br_startblock);
    (*xc).irec_startbno = XFS_FSB_TO_AGBNO((*sc).mp, (*xc).irec.br_startblock);
    let pag = xfs_perag_get((*sc).mp, agno); if pag.is_null() { return -EFSCORRUPTED; }
    let mut error = xrep_ag_init(sc, pag, &mut (*sc).sa); if error != 0 { xfs_perag_put(pag); return error; }
    let mut low: xfs_refcount_irec = core::mem::zeroed(); let mut high: xfs_refcount_irec = core::mem::zeroed();
    low.rc_startblock = (*xc).irec_startbno; high.rc_startblock = low.rc_startblock + (*xc).irec.br_blockcount - 1; low.rc_domain = XFS_REFC_DOMAIN_SHARED; high.rc_domain = XFS_REFC_DOMAIN_SHARED;
    error = xfs_refcount_query_range((*sc).sa.refc_cur, &low, &high, xrep_cow_mark_shared_staging, xc as *mut _ as *mut core::ffi::c_void);
    if error == 0 { low.rc_domain = XFS_REFC_DOMAIN_COW; high.rc_domain = XFS_REFC_DOMAIN_COW; (*xc).next_bno = (*xc).irec_startbno; error = xfs_refcount_query_range((*sc).sa.refc_cur, &low, &high, xrep_cow_mark_missing_staging, xc as *mut _ as *mut core::ffi::c_void); }
    if error == 0 && (*xc).next_bno < (*xc).irec_startbno + (*xc).irec.br_blockcount { error = xrep_cow_mark_file_range(xc, xfs_agbno_to_fsb(pag, (*xc).next_bno), (*xc).irec_startbno + (*xc).irec.br_blockcount - (*xc).next_bno); }
    if error == 0 { let mut rl: xfs_rmap_irec = core::mem::zeroed(); let mut rh: xfs_rmap_irec = core::mem::zeroed(); rl.rm_startblock = (*xc).irec_startbno; rh.rm_startblock = (*xc).irec_startbno + (*xc).irec.br_blockcount - 1; rh = core::mem::zeroed(); rh.rm_startblock = (*xc).irec_startbno + (*xc).irec.br_blockcount - 1; error = xfs_rmap_query_range((*sc).sa.rmap_cur, &rl, &rh, xrep_cow_mark_missing_staging_rmap, xc as *mut _ as *mut core::ffi::c_void); }
    if error == 0 { if XFS_TEST_ERROR((*sc).mp, XFS_ERRTAG_FORCE_SCRUB_REPAIR) { error = xrep_cow_debug_replacement(xc); } else if (*(*sc).sm).sm_flags & XFS_SCRUB_IFLAG_FORCE_REBUILD != 0 { error = xrep_cow_mark_file_range(xc, (*xc).irec.br_startblock, (*xc).irec.br_blockcount); } }
    xchk_ag_free(sc, &mut (*sc).sa); xfs_perag_put(pag); error
}

/* Realtime variant of the mapping scan. */
unsafe fn xrep_cow_find_bad_rt(xc: *mut xrep_cow) -> i32 {
    let sc = (*xc).sc; (*xc).irec_startbno = xfs_rtb_to_rgbno((*sc).mp, (*xc).irec.br_startblock);
    let rtg = xfs_rtgroup_get((*sc).mp, xfs_rtb_to_rgno((*sc).mp, (*xc).irec.br_startblock)); if rtg.is_null() { return -EFSCORRUPTED; }
    let mut error = xrep_rtgroup_init(sc, rtg, &mut (*sc).sr, XFS_RTGLOCK_RMAP | XFS_RTGLOCK_REFCOUNT); if error == 0 { error = xrep_cow_find_bad(xc); }
    xchk_rtgroup_btcur_free(&mut (*sc).sr); xchk_rtgroup_free(sc, &mut (*sc).sr); xfs_rtgroup_put(rtg); error
}

/* Allocate and replace the bad mappings. */
unsafe fn xrep_cow_alloc(sc: *mut xfs_scrub, del: *mut xfs_bmbt_irec) -> i32 {
    let mut args: xfs_alloc_arg = core::mem::zeroed(); args.tp = (*sc).tp; args.mp = (*sc).mp; args.oinfo = XFS_RMAP_OINFO_SKIP_UPDATE; args.minlen = 1; args.maxlen = (*del).br_blockcount; args.prod = 1; args.resv = XFS_AG_RESV_NONE; args.datatype = XFS_ALLOC_USERDATA;
    let mut error = xfs_trans_reserve_more((*sc).tp, (*del).br_blockcount, 0); if error != 0 { return error; }
    error = xfs_alloc_vextent_start_ag(&mut args, XFS_INODE_TO_FSB((*sc).ip)); if error != 0 { return error; }
    if args.fsbno == NULLFSBLOCK { return -ENOSPC; }
    xfs_refcount_alloc_cow_extent((*sc).tp, false, args.fsbno, args.len); (*del).br_startblock = args.fsbno; (*del).br_blockcount = args.len; 0
}

unsafe fn xrep_cow_alloc_rt(sc: *mut xfs_scrub, del: *mut xfs_bmbt_irec) -> i32 {
    let maxrtx = core::cmp::min(U32_MAX, xfs_blen_to_rtbxlen((*sc).mp, (*del).br_blockcount)); let mut fsbno = 0; let mut len = 0;
    let error = xfs_trans_reserve_more((*sc).tp, 0, maxrtx); if error != 0 { return error; }
    let error = xfs_rtallocate_rtgs((*sc).tp, NULLRTBLOCK, 1, maxrtx, 1, false, false, &mut fsbno, &mut len); if error != 0 { return error; }
    xfs_refcount_alloc_cow_extent((*sc).tp, true, fsbno, len); (*del).br_startblock = fsbno; (*del).br_blockcount = len; 0
}

unsafe fn xrep_cow_replace_range(xc: *mut xrep_cow, startoff: xfs_fileoff_t, blockcount: *mut xfs_extlen_t) -> i32 {
    let sc = (*xc).sc; let mut icur: xfs_iext_cursor = core::mem::zeroed(); let mut got: xfs_bmbt_irec = core::mem::zeroed(); let mut rep = got;
    if !xfs_iext_lookup_extent((*sc).ip, xfs_ifork_ptr((*sc).ip, XFS_COW_FORK), startoff, &mut icur, &mut got) { ASSERT(0); return -EFSCORRUPTED; }
    rep = got; if got.br_startoff > startoff || isnullstartblock(got.br_startblock) || xfs_bmap_is_written_extent(&got) { ASSERT(0); return -EFSCORRUPTED; }
    if got.br_startoff < startoff { let d = startoff - got.br_startoff; rep.br_blockcount -= d; rep.br_startoff += d; rep.br_startblock += d; }
    if got.br_startoff + got.br_blockcount > startoff + *blockcount { rep.br_blockcount -= got.br_startoff + got.br_blockcount - (startoff + *blockcount); }
    if got.br_blockcount == 0 { ASSERT(0); return -EFSCORRUPTED; }
    let old = rep.br_startblock; let error = if XFS_IS_REALTIME_INODE((*sc).ip) { xrep_cow_alloc_rt(sc, &mut rep) } else { xrep_cow_alloc(sc, &mut rep) }; if error != 0 { return error; }
    xfs_bmap_replace_cow_mapping((*sc).ip, &mut icur, &mut got, &mut rep); xfs_inode_set_cowblocks_tag((*sc).ip); let error = xfs_defer_finish(&mut (*sc).tp); if error != 0 { return error; }
    let error = if XFS_IS_REALTIME_INODE((*sc).ip) { xrtb_bitmap_set(&mut (*xc).old_cowfork.old_cowfork_rtblocks, old, rep.br_blockcount) } else { xfsb_bitmap_set(&mut (*xc).old_cowfork.old_cowfork_fsblocks, old, rep.br_blockcount) }; if error != 0 { return error; } *blockcount = rep.br_blockcount; 0
}

unsafe fn xrep_cow_replace(startoff: u64, mut blockcount: u64, priv_: *mut core::ffi::c_void) -> i32 {
    let xc = priv_ as *mut xrep_cow; let mut error = 0;
    while blockcount > 0 { let mut len = core::cmp::min(blockcount, XFS_MAX_BMBT_EXTLEN as u64) as xfs_extlen_t; error = xrep_cow_replace_range(xc, startoff, &mut len); if error != 0 { break; } blockcount -= len as u64; }
    error
}

pub unsafe fn xrep_bmap_cow(sc: *mut xfs_scrub) -> i32 {
    let ifp = xfs_ifork_ptr((*sc).ip, XFS_COW_FORK); let mut error = 0;
    if !xfs_has_rmapbt((*sc).mp) || !xfs_has_reflink((*sc).mp) { return -EOPNOTSUPP; }
    if ifp.is_null() { return 0; }
    if xfs_inode_has_bigrtalloc((*sc).ip) || (xfs_is_metadir_inode((*sc).ip) && XFS_IS_REALTIME_INODE((*sc).ip)) { return -EOPNOTSUPP; }
    if (*ifp).if_format != XFS_DINODE_FMT_EXTENTS { (*ifp).if_format = XFS_DINODE_FMT_EXTENTS; (*ifp).if_nextents = 0; return 0; }
    let xc = kzalloc_obj::<xrep_cow>(XCHK_GFP_FLAGS); if xc.is_null() { return -ENOMEM; }
    xfs_trans_ijoin((*sc).tp, (*sc).ip, 0); (*xc).sc = sc; xoff_bitmap_init(&mut (*xc).bad_fileoffs);
    if XFS_IS_REALTIME_INODE((*sc).ip) { xrtb_bitmap_init(&mut (*xc).old_cowfork.old_cowfork_rtblocks); } else { xfsb_bitmap_init(&mut (*xc).old_cowfork.old_cowfork_fsblocks); }
    let mut icur: xfs_iext_cursor = core::mem::zeroed();
    for_each_xfs_iext(ifp, &mut icur, &mut (*xc).irec) {
        if xchk_should_terminate(sc, &mut error) { break; }
        if isnullstartblock((*xc).irec.br_startblock) || xfs_bmap_is_written_extent(&(*xc).irec) { continue; }
        error = if XFS_IS_REALTIME_INODE((*sc).ip) { xrep_cow_find_bad_rt(xc) } else { xrep_cow_find_bad(xc) };
        if error != 0 { break; }
    }
    if error != 0 { if XFS_IS_REALTIME_INODE((*sc).ip) { xrtb_bitmap_destroy(&mut (*xc).old_cowfork.old_cowfork_rtblocks); } else { xfsb_bitmap_destroy(&mut (*xc).old_cowfork.old_cowfork_fsblocks); } xoff_bitmap_destroy(&mut (*xc).bad_fileoffs); kfree(xc as *mut _); return error; }
    error = xoff_bitmap_walk(&(*xc).bad_fileoffs, xrep_cow_replace, xc as *mut _ as *mut core::ffi::c_void);
    if XFS_IS_REALTIME_INODE((*sc).ip) { error = xrep_reap_rtblocks(sc, &(*xc).old_cowfork.old_cowfork_rtblocks, &XFS_RMAP_OINFO_COW); } else { error = xrep_reap_fsblocks(sc, &(*xc).old_cowfork.old_cowfork_fsblocks, &XFS_RMAP_OINFO_COW); }
    if XFS_IS_REALTIME_INODE((*sc).ip) { xrtb_bitmap_destroy(&mut (*xc).old_cowfork.old_cowfork_rtblocks); } else { xfsb_bitmap_destroy(&mut (*xc).old_cowfork.old_cowfork_fsblocks); }
    xoff_bitmap_destroy(&mut (*xc).bad_fileoffs); kfree(xc as *mut _); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
