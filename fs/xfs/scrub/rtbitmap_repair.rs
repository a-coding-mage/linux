// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2020-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Dependencies are supplied by the surrounding XFS Rust bindings. */

/* rt bitmap content repairs */

/*
 * Reserve enough blocks to write out a completely new bitmap file, plus twice
 * as many blocks as we would need if we can only allocate one block per data
 * fork mapping.  This should cover the preallocation of the temporary file and
 * exchanging the extent mappings.
 *
 * We cannot use xfs_exchmaps_estimate because we have not yet constructed the
 * replacement bitmap and therefore do not know how many extents it will use.
 * By the time we do, we will have a dirty transaction (which we cannot drop
 * because we cannot drop the rtbitmap ILOCK) and cannot ask for more
 * reservation.
 */
#[inline]
unsafe fn xrep_rtbitmap_calc_blocks(mp: *mut xfs_mount, blocks: u64) -> u64 {
    blocks + xfs_bmbt_calc_size(mp, blocks) * 2
}

/* Set up to repair the realtime bitmap for this group. */
unsafe fn xrep_setup_rtbitmap(sc: *mut xfs_scrub, rtb: *mut xchk_rtbitmap) -> i32 {
    let mp = (*sc).mp;
    let mut blocks = (*mp).m_sb.sb_rbmblocks;
    let mut error: i32;

    error = xrep_tempfile_create(sc, S_IFREG);
    if error != 0 { return error; }

    /* Create an xfile to hold our reconstructed bitmap. */
    error = xfile_create(
        "realtime bitmap file",
        blocks * (*mp).m_sb.sb_blocksize,
        &mut (*sc).xfile,
    );
    if error != 0 { return error; }

    blocks = xrep_rtbitmap_calc_blocks(mp, (*mp).m_sb.sb_rbmblocks);
    if blocks > UINT_MAX as u64 { return -EOPNOTSUPP; }

    (*rtb).resblks += blocks;
    0
}

#[inline]
unsafe fn rtx_to_wordoff(_mp: *mut xfs_mount, rtx: xfs_rtxnum_t) -> xrep_wordoff_t {
    rtx >> XFS_NBWORDLOG
}

#[inline]
unsafe fn rtxlen_to_wordcnt(rtxlen: xfs_rtxlen_t) -> xrep_wordcnt_t {
    rtxlen >> XFS_NBWORDLOG
}

/* Helper functions to record rtwords in an xfile. */

#[inline]
unsafe fn xfbmp_load(rtb: *mut xchk_rtbitmap, wordoff: xrep_wordoff_t, word: *mut xfs_rtword_t) -> i32 {
    let mut urk: xfs_rtword_raw = core::mem::zeroed();
    ASSERT(xfs_has_rtgroups((*(*rtb).sc).mp));
    let error = xfile_load((*(*rtb).sc).xfile, &mut urk as *mut _, core::mem::size_of::<xfs_rtword_raw>(), wordoff << XFS_WORDLOG);
    if error != 0 { return error; }
    *word = be32_to_cpu(urk.rtg);
    0
}

#[inline]
unsafe fn xfbmp_store(rtb: *mut xchk_rtbitmap, wordoff: xrep_wordoff_t, word: xfs_rtword_t) -> i32 {
    let mut urk: xfs_rtword_raw = core::mem::zeroed();
    ASSERT(xfs_has_rtgroups((*(*rtb).sc).mp));
    urk.rtg = cpu_to_be32(word);
    xfile_store((*(*rtb).sc).xfile, &mut urk as *mut _, core::mem::size_of::<xfs_rtword_raw>(), wordoff << XFS_WORDLOG)
}

#[inline]
unsafe fn xfbmp_copyin(rtb: *mut xchk_rtbitmap, wordoff: xrep_wordoff_t, word: *const xfs_rtword_raw, nr_words: xrep_wordcnt_t) -> i32 {
    xfile_store((*(*rtb).sc).xfile, word, nr_words << XFS_WORDLOG, wordoff << XFS_WORDLOG)
}

#[inline]
unsafe fn xfbmp_copyout(rtb: *mut xchk_rtbitmap, wordoff: xrep_wordoff_t, word: *mut xfs_rtword_raw, nr_words: xrep_wordcnt_t) -> i32 {
    xfile_load((*(*rtb).sc).xfile, word, nr_words << XFS_WORDLOG, wordoff << XFS_WORDLOG)
}

/* Perform a logical OR operation on an rtword in the incore bitmap. */
unsafe fn xrep_rtbitmap_or(rtb: *mut xchk_rtbitmap, wordoff: xrep_wordoff_t, mask: xfs_rtword_t) -> i32 {
    let mut word: xfs_rtword_t = 0;
    let error = xfbmp_load(rtb, wordoff, &mut word);
    if error != 0 { return error; }
    trace_xrep_rtbitmap_or((*(*rtb).sc).mp, wordoff, mask, word);
    xfbmp_store(rtb, wordoff, word | mask)
}

/* Mark as free every rt extent between the next rt block we expected to see
 * in the rtrmap records and the given rt block. */
unsafe fn xrep_rtbitmap_mark_free(rtb: *mut xchk_rtbitmap, rgbno: xfs_rgblock_t) -> i32 {
    let mp = (*(*rtb).sc).mp;
    let sr = &mut (*(*rtb).sc).sr;
    let rtg = sr.rtg;
    let mut startrtx: xfs_rtxnum_t;
    let mut nextrtx: xfs_rtxnum_t;
    let mut wordoff: xrep_wordoff_t;
    let mut nextwordoff: xrep_wordoff_t;
    let mut bit: u32;
    let bufwsize: u32;
    let mut mod_: xfs_extlen_t;
    let mut mask: xfs_rtword_t;
    let mut outcome: xbtree_recpacking;
    let mut error: i32;

    if !xfs_verify_rgbext(rtg, (*rtb).next_rgbno, rgbno - (*rtb).next_rgbno) { return -EFSCORRUPTED; }
    startrtx = xfs_rgbno_to_rtx(mp, (*rtb).next_rgbno);
    mod_ = xfs_rgbno_to_rtxoff(mp, (*rtb).next_rgbno);
    if mod_ != 0 { return -EFSCORRUPTED; }
    nextrtx = xfs_rgbno_to_rtx(mp, rgbno - 1) + 1;
    mod_ = xfs_rgbno_to_rtxoff(mp, rgbno - 1);
    if mod_ != (*mp).m_sb.sb_rextsize - 1 { return -EFSCORRUPTED; }

    if !sr.refc_cur.is_null() {
        error = xfs_refcount_has_records(sr.refc_cur, XFS_REFC_DOMAIN_SHARED, (*rtb).next_rgbno, rgbno - (*rtb).next_rgbno, &mut outcome);
        if error != 0 { return error; }
        if outcome != XBTREE_RECPACKING_EMPTY { return -EFSCORRUPTED; }
        error = xfs_refcount_has_records(sr.refc_cur, XFS_REFC_DOMAIN_COW, (*rtb).next_rgbno, rgbno - (*rtb).next_rgbno, &mut outcome);
        if error != 0 { return error; }
        if outcome != XBTREE_RECPACKING_EMPTY { return -EFSCORRUPTED; }
    }
    trace_xrep_rtbitmap_record_free(mp, startrtx, nextrtx - 1);
    bit = startrtx & XREP_RTBMP_WORDMASK;
    if bit != 0 {
        let len = nextrtx - startrtx;
        let lastbit = min(bit + len, XFS_NBWORD);
        mask = (((1 as xfs_rtword_t) << (lastbit - bit)) - 1) << bit;
        error = xrep_rtbitmap_or(rtb, rtx_to_wordoff(mp, startrtx), mask);
        if error != 0 || lastbit - bit == len { return error; }
        startrtx += XFS_NBWORD - bit;
    }
    bit = nextrtx & XREP_RTBMP_WORDMASK;
    if bit != 0 {
        mask = ((1 as xfs_rtword_t) << bit) - 1;
        error = xrep_rtbitmap_or(rtb, rtx_to_wordoff(mp, nextrtx), mask);
        if error != 0 || startrtx + bit == nextrtx { return error; }
        nextrtx -= bit;
    }
    trace_xrep_rtbitmap_record_free_bulk(mp, startrtx, nextrtx - 1);
    wordoff = rtx_to_wordoff(mp, startrtx);
    nextwordoff = rtx_to_wordoff(mp, nextrtx);
    bufwsize = (*mp).m_sb.sb_blocksize >> XFS_WORDLOG;
    while wordoff < nextwordoff {
        let mut wordcnt = min(nextwordoff - wordoff, bufwsize);
        let rem = wordoff & (bufwsize - 1);
        if rem != 0 { wordcnt = min(wordcnt, bufwsize - rem); }
        error = xfbmp_copyin(rtb, wordoff, (*rtb).words, wordcnt);
        if error != 0 { return error; }
        wordoff += wordcnt;
    }
    0
}

/* Set free space in the rtbitmap based on rtrmapbt records. */
unsafe fn xrep_rtbitmap_walk_rtrmap(cur: *mut xfs_btree_cur, rec: *const xfs_rmap_irec, priv_: *mut core::ffi::c_void) -> i32 {
    let rtb = priv_ as *mut xchk_rtbitmap;
    let mut error = 0;
    if xchk_should_terminate((*rtb).sc, &mut error) { return error; }
    if (*rtb).next_rgbno < (*rec).rm_startblock {
        error = xrep_rtbitmap_mark_free(rtb, (*rec).rm_startblock);
        if error != 0 { return error; }
    }
    (*rtb).next_rgbno = max((*rtb).next_rgbno, (*rec).rm_startblock + (*rec).rm_blockcount);
    0
}

/* Walk the rtrmapbt to find all the gaps between records, and mark the gaps
 * in the realtime bitmap that we're computing. */
unsafe fn xrep_rtbitmap_find_freespace(rtb: *mut xchk_rtbitmap) -> i32 {
    let sc = (*rtb).sc;
    let mp = (*sc).mp;
    let rtg = (*sc).sr.rtg;
    let mut error: i32;
    memset((*rtb).words as *mut _, 0xFF, (*mp).m_sb.sb_blocksize as usize);
    xrep_rtgroup_btcur_init(sc, &mut (*sc).sr);
    error = xfs_rmap_query_all((*sc).sr.rmap_cur, xrep_rtbitmap_walk_rtrmap, rtb as *mut _);
    if error != 0 { xchk_rtgroup_btcur_free(&mut (*sc).sr); return error; }
    let blockcount = (*rtg).rtg_extents * (*mp).m_sb.sb_rextsize;
    if (*rtb).next_rgbno < blockcount {
        error = xrep_rtbitmap_mark_free(rtb, blockcount);
        if error != 0 { xchk_rtgroup_btcur_free(&mut (*sc).sr); return error; }
    }
    xchk_rtgroup_btcur_free(&mut (*sc).sr);
    0
}

unsafe fn xrep_rtbitmap_prep_buf(sc: *mut xfs_scrub, bp: *mut xfs_buf, data: *mut core::ffi::c_void) -> i32 {
    let rtb = data as *mut xchk_rtbitmap;
    let mp = (*sc).mp;
    let mut ondisk: *mut xfs_rtword_raw;
    (*rtb).args.mp = (*sc).mp;
    (*rtb).args.tp = (*sc).tp;
    (*rtb).args.rbmbp = bp;
    ondisk = xfs_rbmblock_wordptr(&mut (*rtb).args, 0);
    (*rtb).args.rbmbp = core::ptr::null_mut();
    let error = xfbmp_copyout(rtb, (*rtb).prep_wordoff, ondisk, (*mp).m_blockwsize);
    if error != 0 { return error; }
    if xfs_has_rtgroups((*sc).mp) {
        let hdr = (*bp).b_addr as *mut xfs_rtbuf_blkinfo;
        (*hdr).rt_magic = cpu_to_be32(XFS_RTBITMAP_MAGIC);
        (*hdr).rt_owner = cpu_to_be64(I_INO((*sc).ip));
        (*hdr).rt_blkno = cpu_to_be64(xfs_buf_daddr(bp));
        (*hdr).rt_lsn = 0;
        uuid_copy(&mut (*hdr).rt_uuid, &(*sc).mp.m_sb.sb_meta_uuid);
        (*bp).b_ops = &xfs_rtbitmap_buf_ops;
    } else { (*bp).b_ops = &xfs_rtbuf_ops; }
    (*rtb).prep_wordoff += (*mp).m_blockwsize;
    xfs_trans_buf_set_type((*sc).tp, bp, XFS_BLFT_RTBITMAP_BUF);
    0
}

/* Make sure that the given range of the data fork of the realtime file is
 * mapped to written blocks.  The caller must ensure that the inode is joined
 * to the transaction. */
unsafe fn xrep_rtbitmap_data_mappings(sc: *mut xfs_scrub, len: xfs_filblks_t) -> i32 {
    let mut map: xfs_bmbt_irec = core::mem::zeroed();
    let mut off: xfs_fileoff_t = 0;
    ASSERT(!(*sc).ip.is_null());
    while off < len {
        let mut nmaps = 1;
        let error = xfs_bmapi_read((*sc).ip, off, len - off, &mut map, &mut nmaps, XFS_DATA_FORK);
        if error != 0 { return error; }
        if nmaps == 0 { ASSERT(nmaps != 0); return -EFSCORRUPTED; }
        if xfs_bmap_is_written_extent(&map) || map.br_startblock == HOLESTARTBLOCK {
            off = map.br_startoff + map.br_blockcount;
            continue;
        }
        if map.br_startblock == DELAYSTARTBLOCK { return -EFSCORRUPTED; }
        if map.br_state != XFS_EXT_UNWRITTEN { ASSERT(map.br_state == XFS_EXT_UNWRITTEN); return -EFSCORRUPTED; }
        nmaps = 1;
        let error = xfs_bmapi_write((*sc).tp, (*sc).ip, map.br_startoff, map.br_blockcount, XFS_BMAPI_CONVERT | XFS_BMAPI_ZERO, 0, &mut map, &mut nmaps);
        if error != 0 { return error; }
        let error = xrep_defer_finish(sc);
        if error != 0 { return error; }
        off = map.br_startoff + map.br_blockcount;
    }
    0
}

/* Fix broken rt volume geometry. */
unsafe fn xrep_rtbitmap_geometry(sc: *mut xfs_scrub, rtb: *mut xchk_rtbitmap) -> i32 {
    let mp = (*sc).mp;
    let tp = (*sc).tp;
    if (*mp).m_sb.sb_rextents != (*rtb).rextents { xfs_trans_mod_sb(tp, XFS_TRANS_SB_REXTENTS, (*rtb).rextents - (*mp).m_sb.sb_rextents); }
    if (*mp).m_sb.sb_rbmblocks != (*rtb).rbmblocks { xfs_trans_mod_sb(tp, XFS_TRANS_SB_RBMBLOCKS, (*rtb).rbmblocks - (*mp).m_sb.sb_rbmblocks); }
    if (*mp).m_sb.sb_rextslog != (*rtb).rextslog { xfs_trans_mod_sb(tp, XFS_TRANS_SB_REXTSLOG, (*rtb).rextslog - (*mp).m_sb.sb_rextslog); }
    (*sc).ip.i_disk_size = roundup_64((*sc).ip.i_disk_size, (*mp).m_sb.sb_blocksize);
    if (*sc).ip.i_disk_size < XFS_FSB_TO_B(mp, (*rtb).rbmblocks) { (*sc).ip.i_disk_size = XFS_FSB_TO_B(mp, (*rtb).rbmblocks); }
    xfs_trans_log_inode(tp, (*sc).ip, XFS_ILOG_CORE);
    xrep_roll_trans(sc)
}

/* Repair the realtime bitmap file metadata. */
unsafe fn xrep_rtbitmap(sc: *mut xfs_scrub) -> i32 {
    let rtb = (*sc).buf as *mut xchk_rtbitmap;
    let mp = (*sc).mp;
    let xg = rtg_group((*sc).sr.rtg);
    let mut blocks: u64;
    let mut busy_gen = 0;
    let mut error: i32;
    if !xfs_has_rtrmapbt((*sc).mp) || !xfs_has_exchange_range((*sc).mp) { return -EOPNOTSUPP; }
    if (*rtb).rbmblocks > U32_MAX as u64 { return 0; }
    blocks = xrep_rtbitmap_calc_blocks(mp, (*rtb).rbmblocks);
    if blocks > UINT_MAX as u64 { return -EOPNOTSUPP; }
    if blocks > (*rtb).resblks {
        let delta = blocks - (*rtb).resblks;
        if delta > UINT_MAX as u64 { return -EOPNOTSUPP; }
        error = xfs_trans_reserve_more((*sc).tp, delta, 0);
        if error != 0 { return error; }
        (*rtb).resblks += delta;
    }
    error = xrep_metadata_inode_forks(sc); if error != 0 { return error; }
    xfs_trans_ijoin((*sc).tp, (*sc).ip, 0);
    error = xrep_rtbitmap_data_mappings(sc, (*rtb).rbmblocks); if error != 0 { return error; }
    error = xrep_rtbitmap_geometry(sc, rtb); if error != 0 { return error; }
    if !xfs_extent_busy_list_empty(xg, &mut busy_gen) {
        error = xfs_extent_busy_flush((*sc).tp, xg, busy_gen, 0); if error != 0 { return error; }
    }
    error = xrep_rtbitmap_find_freespace(rtb); if error != 0 { return error; }
    while !xrep_tempfile_ilock_nowait(sc) {
        if xchk_should_terminate(sc, &mut error) { return error; }
        delay(1);
    }
    xfs_trans_ijoin((*sc).tp, (*sc).tempip, 0);
    error = xrep_tempfile_prealloc(sc, 0, (*rtb).rbmblocks); if error != 0 { return error; }
    if xchk_should_terminate(sc, &mut error) { return error; }
    error = xrep_tempfile_copyin(sc, 0, (*rtb).rbmblocks, xrep_rtbitmap_prep_buf, rtb as *mut _); if error != 0 { return error; }
    error = xrep_tempfile_set_isize(sc, XFS_FSB_TO_B((*sc).mp, (*sc).mp.m_sb.sb_rbmblocks)); if error != 0 { return error; }
    error = xrep_tempexch_trans_reserve(sc, XFS_DATA_FORK, 0, (*rtb).rbmblocks, &mut (*rtb).tempexch); if error != 0 { return error; }
    error = xrep_tempexch_contents(sc, &mut (*rtb).tempexch); if error != 0 { return error; }
    xrep_reap_ifork(sc, (*sc).tempip, XFS_DATA_FORK)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
