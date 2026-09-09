// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Dependencies are supplied by the surrounding XFS implementation. */

/* Set us up with the realtime metadata locked. */
pub unsafe fn xchk_setup_rtbitmap(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp;
    let mut rtb: *mut xchk_rtbitmap;
    let mut error: i32;

    if xchk_need_intent_drain(sc) {
        xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN);
    }

    rtb = kzalloc_flex(xchk_rtbitmap_wordcnt(sc), XCHK_GFP_FLAGS);
    if rtb.is_null() {
        return -ENOMEM;
    }
    (*sc).buf = rtb as *mut core::ffi::c_void;
    (*rtb).sc = sc;

    error = xchk_rtgroup_init(sc, (*(*sc).sm).sm_agno, &mut (*sc).sr);
    if error != 0 { return error; }

    if xchk_could_repair(sc) {
        error = xrep_setup_rtbitmap(sc, rtb);
        if error != 0 { return error; }
    }

    error = xchk_trans_alloc(sc, (*rtb).resblks);
    if error != 0 { return error; }
    error = xchk_install_live_inode(sc, rtg_bitmap((*sc).sr.rtg));
    if error != 0 { return error; }
    error = xchk_ino_dqattach(sc);
    if error != 0 { return error; }
    error = xchk_rtgroup_lock(sc, &mut (*sc).sr, XCHK_RTGLOCK_ALL);
    if error != 0 { return error; }

    if (*mp).m_sb.sb_rblocks != 0 {
        (*rtb).rextents = xfs_blen_to_rtbxlen(mp, (*mp).m_sb.sb_rblocks);
        (*rtb).rextslog = xfs_compute_rextslog((*rtb).rextents);
        (*rtb).rbmblocks = xfs_rtbitmap_blockcount(mp);
    }
    0
}

/* Per-rtgroup bitmap contents. */
/* Cross-reference rtbitmap entries with other metadata. */
unsafe fn xchk_rtbitmap_xref(rtb: *mut xchk_rtbitmap, startblock: xfs_rtblock_t, blockcount: xfs_rtblock_t) {
    let sc = (*rtb).sc;
    let rgbno = xfs_rtb_to_rgbno((*sc).mp, startblock);
    if ((*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 || (*(*sc).sr).rmap_cur.is_null() { return; }
    xchk_xref_has_no_rt_owner(sc, rgbno, blockcount);
    xchk_xref_is_not_rt_shared(sc, rgbno, blockcount);
    xchk_xref_is_not_rt_cow_staging(sc, rgbno, blockcount);
    if (*rtb).next_free_rgbno < rgbno { xchk_xref_has_rt_owner(sc, (*rtb).next_free_rgbno, rgbno - (*rtb).next_free_rgbno); }
    (*rtb).next_free_rgbno = rgbno + blockcount;
}

/* Scrub a free extent record from the realtime bitmap. */
unsafe fn xchk_rtbitmap_rec(rtg: *mut xfs_rtgroup, _tp: *mut xfs_trans, rec: *const xfs_rtalloc_rec, priv_: *mut core::ffi::c_void) -> i32 {
    let rtb = priv_ as *mut xchk_rtbitmap;
    let sc = (*rtb).sc;
    let startblock = xfs_rtx_to_rtb(rtg, (*rec).ar_startext);
    let blockcount = xfs_rtxlen_to_extlen(rtg_mount(rtg), (*rec).ar_extcount);
    if !xfs_verify_rtbext(rtg_mount(rtg), startblock, blockcount) { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); }
    xchk_rtbitmap_xref(rtb, startblock, blockcount);
    if ((*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 { return -ECANCELED; }
    0
}

/* Make sure the entire rtbitmap file is mapped with written extents. */
unsafe fn xchk_rtbitmap_check_extents(sc: *mut xfs_scrub) -> i32 {
    let mut map: xfs_bmbt_irec = core::mem::zeroed();
    let mut icur: xfs_iext_cursor = core::mem::zeroed();
    let mp = (*sc).mp;
    let ip = (*sc).ip;
    let mut off: xfs_fileoff_t = 0;
    let endoff = XFS_B_TO_FSB(mp, (*ip).i_disk_size);
    let mut error: i32 = 0;
    if xfs_iext_lookup_extent(ip, &mut (*ip).i_df, endoff, &mut icur, &mut map) { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, endoff); return 0; }
    while off < endoff {
        let mut nmap = 1;
        if xchk_should_terminate(sc, &mut error) || ((*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 { break; }
        error = xfs_bmapi_read(ip, off, endoff - off, &mut map, &mut nmap, XFS_DATA_FORK);
        if !xchk_fblock_process_error(sc, XFS_DATA_FORK, off, &mut error) { break; }
        if nmap != 1 || !xfs_bmap_is_written_extent(&map) { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, off); break; }
        off += map.br_blockcount;
    }
    error
}

/* Scrub this group's realtime bitmap. */
pub unsafe fn xchk_rtbitmap(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp;
    let rtg = (*sc).sr.rtg;
    let rbmip = rtg_bitmap(rtg);
    let rtb = (*sc).buf as *mut xchk_rtbitmap;
    if (*mp).m_sb.sb_rextents != (*rtb).rextents || (*mp).m_sb.sb_rextslog != (*rtb).rextslog || (*rtb).rbmblocks > U32_MAX || (*mp).m_sb.sb_rbmblocks != (*rtb).rbmblocks || ((*rbmip).i_disk_size & (*mp).m_blockmask) != 0 || (*rbmip).i_disk_size < XFS_FSB_TO_B(mp, (*rtb).rbmblocks) { xchk_ip_set_corrupt(sc, rbmip); return 0; }
    let mut error = xchk_metadata_inode_forks(sc);
    if error != 0 || ((*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 { return error; }
    error = xchk_rtbitmap_check_extents(sc);
    if error != 0 || ((*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 { return error; }
    (*rtb).next_free_rgbno = 0;
    error = xfs_rtalloc_query_all(rtg, (*sc).tp, Some(xchk_rtbitmap_rec), rtb as *mut core::ffi::c_void);
    if !xchk_fblock_process_error(sc, XFS_DATA_FORK, 0, &mut error) { return error; }
    let last_rgbno = (*rtg).rtg_extents * (*mp).m_sb.sb_rextsize;
    if (*rtb).next_free_rgbno < last_rgbno { xchk_xref_has_rt_owner(sc, (*rtb).next_free_rgbno, last_rgbno - (*rtb).next_free_rgbno); }
    0
}

/* xref check that the extent is not free in the rtbitmap */
pub unsafe fn xchk_xref_is_used_rt_space(sc: *mut xfs_scrub, rtbno: xfs_rtblock_t, len: xfs_extlen_t) {
    let rtg = (*sc).sr.rtg;
    if xchk_skip_xref((*sc).sm) { return; }
    if xfs_has_zoned((*sc).mp) {
        if !xfs_zone_rgbno_is_valid(rtg, xfs_rtb_to_rgbno((*sc).mp, rtbno) + len - 1) { xchk_ip_xref_set_corrupt(sc, rtg_rmap(rtg)); }
        return;
    }
    let startext = xfs_rtb_to_rtx((*sc).mp, rtbno);
    let endext = xfs_rtb_to_rtx((*sc).mp, rtbno + len - 1);
    let mut is_free = false;
    let mut error = xfs_rtalloc_extent_is_free(rtg, (*sc).tp, startext, endext - startext + 1, &mut is_free);
    if !xchk_should_check_xref(sc, &mut error, core::ptr::null_mut()) { return; }
    if is_free { xchk_ip_xref_set_corrupt(sc, rtg_bitmap(rtg)); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
