// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Realtime Summary
 *
 * We check the realtime summary by scanning the realtime bitmap file to create
 * a new summary file incore, and then compare the computed version against the
 * ondisk version.  The xfile functionality stores this data in pageable
 * memory.
 */

pub unsafe fn xchk_setup_rtsummary(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp;
    let mut rts: *mut xchk_rtsummary;
    let mut error: i32;

    if xchk_need_intent_drain(sc) {
        xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN);
    }

    rts = kvzalloc_flex::<xchk_rtsummary>(mp.m_blockwsize, XCHK_GFP_FLAGS);
    if rts.is_null() {
        return -ENOMEM;
    }
    (*sc).buf = rts as *mut core::ffi::c_void;

    error = xchk_rtgroup_init(sc, (*sc).sm.sm_agno, &mut (*sc).sr);
    if error != 0 { return error; }

    if xchk_could_repair(sc) {
        error = xrep_setup_rtsummary(sc, rts);
        if error != 0 { return error; }
    }

    error = xfile_create("realtime summary file", XFS_FSB_TO_B(mp, mp.m_rsumblocks), &mut (*sc).xfile);
    if error != 0 { return error; }

    error = xchk_trans_alloc(sc, (*rts).resblks);
    if error != 0 { return error; }

    error = xchk_install_live_inode(sc, rtg_summary((*sc).sr.rtg));
    if error != 0 { return error; }
    error = xchk_ino_dqattach(sc);
    if error != 0 { return error; }
    error = xchk_rtgroup_lock(sc, &mut (*sc).sr, XFS_RTGLOCK_BITMAP);
    if error != 0 { return error; }

    if mp.m_sb.sb_rblocks != 0 {
        (*rts).rextents = xfs_blen_to_rtbxlen(mp, mp.m_sb.sb_rblocks);
        (*rts).rbmblocks = xfs_rtbitmap_blockcount(mp);
        (*rts).rsumblocks = xfs_rtsummary_blockcount(mp, &mut (*rts).rsumlevels);
    }
    0
}

#[inline]
unsafe fn xfsum_load(sc: *mut xfs_scrub, sumoff: xfs_rtsumoff_t, rawinfo: *mut xfs_suminfo_raw) -> i32 {
    xfile_load((*sc).xfile, rawinfo as *mut core::ffi::c_void,
        core::mem::size_of::<xfs_suminfo_raw>(), sumoff << XFS_WORDLOG)
}

#[inline]
unsafe fn xfsum_store(sc: *mut xfs_scrub, sumoff: xfs_rtsumoff_t, rawinfo: xfs_suminfo_raw) -> i32 {
    xfile_store((*sc).xfile, &rawinfo as *const _ as *const core::ffi::c_void,
        core::mem::size_of::<xfs_suminfo_raw>(), sumoff << XFS_WORDLOG)
}

#[inline]
pub unsafe fn xfsum_copyout(sc: *mut xfs_scrub, sumoff: xfs_rtsumoff_t,
        rawinfo: *mut xfs_suminfo_raw, nr_words: u32) -> i32 {
    xfile_load((*sc).xfile, rawinfo as *mut core::ffi::c_void,
        (nr_words << XFS_WORDLOG) as usize, sumoff << XFS_WORDLOG)
}

#[inline]
unsafe fn xchk_rtsum_inc(mp: *mut xfs_mount, v: *mut xfs_suminfo_raw) -> xfs_suminfo_t {
    if xfs_has_rtgroups(mp) {
        be32_add_cpu(&mut (*v).rtg, 1);
        return be32_to_cpu((*v).rtg);
    }
    (*v).old = (*v).old.wrapping_add(1);
    (*v).old
}

unsafe fn xchk_rtsum_record_free(rtg: *mut xfs_rtgroup, tp: *mut xfs_trans,
        rec: *const xfs_rtalloc_rec, priv_: *mut core::ffi::c_void) -> i32 {
    let mp = rtg_mount(rtg);
    let sc = priv_ as *mut xfs_scrub;
    let mut error = 0;
    if xchk_should_terminate(sc, &mut error) { return error; }

    let rbmoff = xfs_rtx_to_rbmblock(mp, (*rec).ar_startext);
    let lenlog = xfs_highbit64((*rec).ar_extcount);
    let offs = xfs_rtsumoffs(mp, lenlog, rbmoff);
    let rtbno = xfs_rtx_to_rtb(rtg, (*rec).ar_startext);
    let rtlen = xfs_rtxlen_to_extlen(mp, (*rec).ar_extcount);
    if !xfs_verify_rtbext(mp, rtbno, rtlen) {
        xchk_ip_xref_set_corrupt(sc, rtg_bitmap(rtg));
        return -EFSCORRUPTED;
    }

    let mut v: xfs_suminfo_raw = core::mem::zeroed();
    error = xfsum_load(sc, offs, &mut v);
    if error != 0 { return error; }
    let value = xchk_rtsum_inc((*sc).mp, &mut v);
    trace_xchk_rtsum_record_free(mp, (*rec).ar_startext, (*rec).ar_extcount, lenlog, offs, value);
    xfsum_store(sc, offs, v)
}

unsafe fn xchk_rtsum_compute(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp;
    let rtg = (*sc).sr.rtg;
    if XFS_FSB_TO_B(mp, xfs_rtbitmap_blockcount(mp)) != rtg_bitmap(rtg).i_disk_size {
        return -EFSCORRUPTED;
    }
    xfs_rtalloc_query_all(rtg, (*sc).tp, xchk_rtsum_record_free, sc as *mut core::ffi::c_void)
}

unsafe fn xchk_rtsum_compare(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp;
    let ip = (*sc).ip;
    let rts = (*sc).buf as *mut xchk_rtsummary;
    let mut off: xfs_fileoff_t = 0;
    let endoff = XFS_B_TO_FSB(mp, (*ip).i_disk_size);
    let mut icur: xfs_iext_cursor = core::mem::zeroed();
    let mut map: xfs_bmbt_irec = core::mem::zeroed();
    if xfs_iext_lookup_extent(ip, &mut (*ip).i_df, endoff, &mut icur, &mut map) {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, endoff); return 0;
    }
    while off < endoff {
        let mut nmap = 1;
        let mut error = 0;
        if xchk_should_terminate(sc, &mut error) { return error; }
        if ((*sc).sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 { return 0; }
        error = xfs_bmapi_read(ip, off, endoff - off, &mut map, &mut nmap, XFS_DATA_FORK);
        if !xchk_fblock_process_error(sc, XFS_DATA_FORK, off, &mut error) { return error; }
        if nmap != 1 || !xfs_bmap_is_written_extent(&map) { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, off); return 0; }
        off += map.br_blockcount;
    }

    let mut sumoff: xfs_rtsumoff_t = 0;
    off = 0;
    while off < endoff {
        let mut error = xfs_rtsummary_read_buf(&mut (*rts).args, off);
        if !xchk_fblock_process_error(sc, XFS_DATA_FORK, off, &mut error) { return error; }
        error = xfsum_copyout(sc, sumoff, (*rts).words, mp.m_blockwsize);
        if error != 0 { xfs_rtbuf_cache_relse(&mut (*rts).args); return error; }
        let ondisk_info = xfs_rsumblock_infoptr(&mut (*rts).args, 0);
        if memcmp(ondisk_info, (*rts).words, (mp.m_blockwsize << XFS_WORDLOG) as usize) != 0 {
            xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, off);
            xfs_rtbuf_cache_relse(&mut (*rts).args); return error;
        }
        xfs_rtbuf_cache_relse(&mut (*rts).args);
        off += 1; sumoff += mp.m_blockwsize;
    }
    0
}

pub unsafe fn xchk_rtsummary(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp;
    let rtg = (*sc).sr.rtg;
    let rbmip = rtg_bitmap(rtg);
    let rsumip = rtg_summary(rtg);
    let rts = (*sc).buf as *mut xchk_rtsummary;
    if mp.m_sb.sb_rextents != (*rts).rextents { xchk_ip_set_corrupt(sc, rbmip); return 0; }
    if mp.m_rsumlevels != (*rts).rsumlevels { xchk_ip_set_corrupt(sc, rsumip); return 0; }
    if mp.m_rsumblocks != (*rts).rsumblocks { xchk_ip_set_corrupt(sc, rsumip); return 0; }
    if ((*rsumip).i_disk_size & mp.m_blockmask) != 0 { xchk_ip_set_corrupt(sc, rsumip); return 0; }
    if (*rsumip).i_disk_size < XFS_FSB_TO_B(mp, (*rts).rsumblocks) { xchk_ip_set_corrupt(sc, rsumip); return 0; }
    let mut error = xchk_metadata_inode_forks(sc);
    if error != 0 || ((*sc).sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 { return error; }
    error = xchk_rtsum_compute(sc);
    if error == -EFSCORRUPTED { xchk_ip_xref_set_corrupt(sc, rbmip); return 0; }
    if error != 0 { return error; }
    xchk_rtsum_compare(sc)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
