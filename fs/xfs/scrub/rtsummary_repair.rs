// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Dependencies supplied by the surrounding XFS implementation.

/* Set us up to repair the rtsummary file. */
pub unsafe fn xrep_setup_rtsummary(
    sc: *mut xfs_scrub,
    rts: *mut xchk_rtsummary,
) -> i32 {
    let mp: *mut xfs_mount = (*sc).mp;
    let mut blocks: u64;
    let error: i32;

    error = xrep_tempfile_create(sc, S_IFREG);
    if error != 0 {
        return error;
    }

    /*
     * If we're doing a repair, we reserve enough blocks to write out a
     * completely new summary file, plus twice as many blocks as we would
     * need if we can only allocate one block per data fork mapping.  This
     * should cover the preallocation of the temporary file and exchanging
     * the extent mappings.
     *
     * We cannot use xfs_exchmaps_estimate because we have not yet
     * constructed the replacement rtsummary and therefore do not know how
     * many extents it will use.  By the time we do, we will have a dirty
     * transaction (which we cannot drop because we cannot drop the
     * rtsummary ILOCK) and cannot ask for more reservation.
     */
    blocks = (*mp).m_rsumblocks;
    blocks = blocks.wrapping_add(xfs_bmbt_calc_size(mp, blocks).wrapping_mul(2));
    if blocks > UINT_MAX as u64 {
        return -EOPNOTSUPP;
    }

    (*rts).resblks = (*rts).resblks.wrapping_add(blocks);
    0
}

unsafe fn xrep_rtsummary_prep_buf(
    sc: *mut xfs_scrub,
    bp: *mut xfs_buf,
    data: *mut core::ffi::c_void,
) -> i32 {
    let rts: *mut xchk_rtsummary = data as *mut xchk_rtsummary;
    let mp: *mut xfs_mount = (*sc).mp;
    let ondisk: *mut xfs_suminfo_raw;
    let error: i32;

    (*rts).args.mp = mp;
    (*rts).args.tp = (*sc).tp;
    (*rts).args.rtg = (*sc).sr.rtg;
    (*rts).args.sumbp = bp;
    ondisk = xfs_rsumblock_infoptr(&mut (*rts).args, 0);
    (*rts).args.sumbp = core::ptr::null_mut();

    error = xfsum_copyout(sc, (*rts).prep_wordoff, ondisk, (*mp).m_blockwsize);
    if error != 0 {
        return error;
    }

    if xfs_has_rtgroups((*sc).mp) {
        let hdr: *mut xfs_rtbuf_blkinfo = (*bp).b_addr as *mut xfs_rtbuf_blkinfo;

        (*hdr).rt_magic = cpu_to_be32(XFS_RTSUMMARY_MAGIC);
        (*hdr).rt_owner = cpu_to_be64(I_INO((*sc).ip));
        (*hdr).rt_blkno = cpu_to_be64(xfs_buf_daddr(bp));
        (*hdr).rt_lsn = 0;
        uuid_copy(&mut (*hdr).rt_uuid, &(*(*sc).mp).m_sb.sb_meta_uuid);
        (*bp).b_ops = &xfs_rtsummary_buf_ops;
    } else {
        (*bp).b_ops = &xfs_rtbuf_ops;
    }

    (*rts).prep_wordoff = (*rts).prep_wordoff.wrapping_add((*mp).m_blockwsize);
    xfs_trans_buf_set_type((*sc).tp, bp, XFS_BLFT_RTSUMMARY_BUF);
    0
}

/* Repair the realtime summary. */
pub unsafe fn xrep_rtsummary(sc: *mut xfs_scrub) -> i32 {
    let rts: *mut xchk_rtsummary = (*sc).buf as *mut xchk_rtsummary;
    let mp: *mut xfs_mount = (*sc).mp;
    let error: i32;

    /* We require the rmapbt to rebuild anything. */
    if !xfs_has_rmapbt(mp) {
        return -EOPNOTSUPP;
    }
    /* We require atomic file exchange range to rebuild anything. */
    if !xfs_has_exchange_range(mp) {
        return -EOPNOTSUPP;
    }

    /* Walk away if we disagree on the size of the rt bitmap. */
    if (*rts).rbmblocks != (*mp).m_sb.sb_rbmblocks {
        return 0;
    }

    /* Make sure any problems with the fork are fixed. */
    error = xrep_metadata_inode_forks(sc);
    if error != 0 {
        return error;
    }

    /*
     * Try to take ILOCK_EXCL of the temporary file.  We had better be the
     * only ones holding onto this inode, but we can't block while holding
     * the rtsummary file's ILOCK_EXCL.
     */
    while !xrep_tempfile_ilock_nowait(sc) {
        let mut terminate_error: i32 = 0;
        if xchk_should_terminate(sc, &mut terminate_error) {
            return terminate_error;
        }
        delay(1);
    }

    /* Make sure we have space allocated for the entire summary file. */
    xfs_trans_ijoin((*sc).tp, (*sc).ip, 0);
    xfs_trans_ijoin((*sc).tp, (*sc).tempip, 0);
    error = xrep_tempfile_prealloc(sc, 0, (*rts).rsumblocks);
    if error != 0 {
        return error;
    }

    /* Last chance to abort before we start committing fixes. */
    let mut terminate_error: i32 = 0;
    if xchk_should_terminate(sc, &mut terminate_error) {
        return terminate_error;
    }

    /* Copy the rtsummary file that we generated. */
    error = xrep_tempfile_copyin(sc, 0, (*rts).rsumblocks, xrep_rtsummary_prep_buf, rts);
    if error != 0 {
        return error;
    }
    error = xrep_tempfile_set_isize(sc, XFS_FSB_TO_B(mp, (*rts).rsumblocks));
    if error != 0 {
        return error;
    }

    /*
     * Now exchange the contents.  Nothing in repair uses the temporary
     * buffer, so we can reuse it for the tempfile exchrange information.
     */
    error = xrep_tempexch_trans_reserve(sc, XFS_DATA_FORK, 0,
            (*rts).rsumblocks, &mut (*rts).tempexch);
    if error != 0 {
        return error;
    }

    error = xrep_tempexch_contents(sc, &mut (*rts).tempexch);
    if error != 0 {
        return error;
    }

    /* Reset incore state and blow out the summary cache. */
    if !(*(*sc).sr.rtg).rtg_rsum_cache.is_null() {
        core::ptr::write_bytes((*(*sc).sr.rtg).rtg_rsum_cache, 0xFF, (*mp).m_sb.sb_rbmblocks as usize);
    }

    (*mp).m_rsumlevels = (*rts).rsumlevels;
    (*mp).m_rsumblocks = (*rts).rsumblocks;

    /* Free the old rtsummary blocks if they're not in use. */
    xrep_reap_ifork(sc, (*sc).tempip, XFS_DATA_FORK)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
