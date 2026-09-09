// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level Rust translation of xfs/scrub/repair.c.
 * External XFS declarations are intentionally left to the surrounding crate.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* The source relies on the XFS kernel ABI and declarations supplied by other
 * translation units.  Keep the same names and pointer-oriented interfaces. */

pub unsafe fn xrep_attempt(sc: *mut xfs_scrub, run: *mut xchk_stats_run) -> i32 {
    let mut error: i32 = 0;
    trace_xrep_attempt(XFS_I(file_inode((*sc).file)), (*sc).sm, error);
    xchk_ag_btcur_free(&mut (*sc).sa);
    xchk_rtgroup_btcur_free(&mut (*sc).sr);
    debug_assert!((*(*sc).ops).repair.is_some());
    (*run).repair_attempted = true;
    let repair_start = xchk_stats_now();
    error = ((*(*sc).ops).repair.unwrap())(sc);
    trace_xrep_done(XFS_I(file_inode((*sc).file)), (*sc).sm, error);
    (*run).repair_ns += xchk_stats_elapsed_ns(repair_start);
    match error {
        0 => { (*(*sc).sm).sm_flags &= !XFS_SCRUB_FLAGS_OUT; (*sc).flags |= XREP_ALREADY_FIXED; (*run).repair_succeeded = true; -EAGAIN }
        -ECHRNG => { (*sc).flags |= XCHK_NEED_DRAIN; (*run).retries += 1; -EAGAIN }
        -EDEADLOCK => {
            if (*sc).flags & XCHK_TRY_HARDER == 0 { (*sc).flags |= XCHK_TRY_HARDER; (*run).retries += 1; return -EAGAIN; }
            0
        }
        _ => { debug_assert!(error != -EAGAIN); error }
    }
}

pub unsafe fn xrep_failure(mp: *mut xfs_mount) {
    xfs_alert_ratelimited(mp, "Corruption not fixed during online repair.  Unmount and run xfs_repair.");
}

pub unsafe fn xrep_probe(sc: *mut xfs_scrub) -> i32 {
    let mut error = 0;
    if xchk_should_terminate(sc, &mut error) { return error; }
    0
}

pub unsafe fn xrep_roll_ag_trans(sc: *mut xfs_scrub) -> i32 {
    if !(*sc).sa.agi_bp.is_null() { xfs_ialloc_log_agi((*sc).tp, (*sc).sa.agi_bp, XFS_AGI_MAGICNUM); xfs_trans_bhold((*sc).tp, (*sc).sa.agi_bp); }
    if !(*sc).sa.agf_bp.is_null() { xfs_alloc_log_agf((*sc).tp, (*sc).sa.agf_bp, XFS_AGF_MAGICNUM); xfs_trans_bhold((*sc).tp, (*sc).sa.agf_bp); }
    let error = xfs_trans_roll(&mut (*sc).tp);
    if error != 0 { return error; }
    if !(*sc).sa.agi_bp.is_null() { xfs_trans_bjoin((*sc).tp, (*sc).sa.agi_bp); }
    if !(*sc).sa.agf_bp.is_null() { xfs_trans_bjoin((*sc).tp, (*sc).sa.agf_bp); }
    0
}

pub unsafe fn xrep_roll_trans(sc: *mut xfs_scrub) -> i32 {
    if (*sc).ip.is_null() { xrep_roll_ag_trans(sc) } else { xfs_trans_roll_inode(&mut (*sc).tp, (*sc).ip) }
}

pub unsafe fn xrep_defer_finish(sc: *mut xfs_scrub) -> i32 {
    if !(*sc).sa.agi_bp.is_null() { xfs_ialloc_log_agi((*sc).tp, (*sc).sa.agi_bp, XFS_AGI_MAGICNUM); xfs_trans_bhold((*sc).tp, (*sc).sa.agi_bp); }
    if !(*sc).sa.agf_bp.is_null() { xfs_alloc_log_agf((*sc).tp, (*sc).sa.agf_bp, XFS_AGF_MAGICNUM); xfs_trans_bhold((*sc).tp, (*sc).sa.agf_bp); }
    let error = xfs_defer_finish(&mut (*sc).tp);
    if error != 0 { return error; }
    if !(*sc).sa.agi_bp.is_null() { xfs_trans_bhold_release((*sc).tp, (*sc).sa.agi_bp); }
    if !(*sc).sa.agf_bp.is_null() { xfs_trans_bhold_release((*sc).tp, (*sc).sa.agf_bp); }
    0
}

pub unsafe fn xrep_ag_has_space(pag: *mut xfs_perag, nr_blocks: xfs_extlen_t, typ: xfs_ag_resv_type) -> bool {
    !xfs_ag_resv_critical(pag, XFS_AG_RESV_RMAPBT) && !xfs_ag_resv_critical(pag, XFS_AG_RESV_METADATA) && (*pag).pagf_freeblks > xfs_ag_resv_needed(pag, typ) + nr_blocks
}

pub unsafe fn xrep_fix_freelist(sc: *mut xfs_scrub, alloc_flags: i32) -> i32 {
    let mut args: xfs_alloc_arg = core::mem::zeroed();
    args.mp = (*sc).mp; args.tp = (*sc).tp; args.agno = pag_agno((*sc).sa.pag); args.alignment = 1; args.pag = (*sc).sa.pag;
    xfs_alloc_fix_freelist(&mut args, alloc_flags)
}

pub unsafe fn xrep_will_attempt(sc: *mut xfs_scrub) -> bool {
    if (*(*sc).sm).sm_flags & XFS_SCRUB_IFLAG_FORCE_REBUILD != 0 { return true; }
    if XFS_TEST_ERROR((*sc).mp, XFS_ERRTAG_FORCE_SCRUB_REPAIR) { return true; }
    xchk_needs_repair((*sc).sm)
}

pub unsafe fn xrep_ino_ensure_extent_count(sc: *mut xfs_scrub, whichfork: i32, nextents: xfs_extnum_t) -> i32 {
    let large = xfs_inode_has_large_extent_counts((*sc).ip);
    let mut max = xfs_iext_max_nextents(large, whichfork);
    if nextents <= max { return 0; }
    if large || !xfs_has_large_extent_counts((*sc).mp) { return -EFSCORRUPTED; }
    max = xfs_iext_max_nextents(true, whichfork);
    if nextents > max { return -EFSCORRUPTED; }
    (*(*sc).ip).i_diflags2 |= XFS_DIFLAG2_NREXT64;
    xfs_trans_log_inode((*sc).tp, (*sc).ip, XFS_ILOG_CORE);
    0
}

pub unsafe fn xrep_inode_set_nblocks(sc: *mut xfs_scrub, new_blocks: i64) {
    let delta = new_blocks - (*(*sc).ip).i_nblocks;
    (*(*sc).ip).i_nblocks = new_blocks;
    xfs_trans_log_inode((*sc).tp, (*sc).ip, XFS_ILOG_CORE);
    if delta != 0 { xfs_trans_mod_dquot_byino((*sc).tp, (*sc).ip, XFS_TRANS_DQ_BCOUNT, delta); }
}

pub unsafe fn xrep_setup_xfbtree(sc: *mut xfs_scrub, descr: *const i8) -> i32 {
    debug_assert!((*sc).tp.is_null());
    xmbuf_alloc((*sc).mp, descr, &mut (*sc).xmbtp)
}

pub unsafe fn xrep_reset_metafile_resv(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp;
    let mut delta = (*mp).m_metafile_resv_used + (*mp).m_metafile_resv_avail - (*mp).m_metafile_resv_target;
    if delta == 0 { return 0; }
    if delta > 0 {
        let give_back = core::cmp::min(delta as u64, (*mp).m_metafile_resv_avail);
        if give_back > 0 { xfs_mod_sb_delalloc(mp, -(give_back as i64)); xfs_add_fdblocks(mp, give_back as i64); (*mp).m_metafile_resv_avail -= give_back; }
        return 0;
    }
    delta = -delta;
    let mut error = xfs_dec_fdblocks(mp, delta, true);
    while error == -ENOSPC { delta -= 1; if delta == 0 { xfs_warn((*sc).mp, "Insufficient free space to reset metabtree reservation after repair."); return 0; } error = xfs_dec_fdblocks(mp, delta, true); }
    if error != 0 { return error; }
    xfs_mod_sb_delalloc(mp, delta); (*mp).m_metafile_resv_avail += delta as u64; 0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
