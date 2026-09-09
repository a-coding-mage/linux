// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2019-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// C dependencies supplied by the surrounding XFS implementation are intentionally external.

const XCHK_FSCOUNT_MIN_VARIANCE: u64 = 512;

unsafe fn xchk_fscount_warmup(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp;
    let mut agi_bp: *mut xfs_buf = core::ptr::null_mut();
    let mut agf_bp: *mut xfs_buf = core::ptr::null_mut();
    let mut pag: *mut xfs_perag = core::ptr::null_mut();
    let mut error: i32 = 0;

    while { pag = xfs_perag_next(mp, pag); !pag.is_null() } {
        if xchk_should_terminate(sc, &mut error) { break; }
        if xfs_perag_initialised_agi(pag) && xfs_perag_initialised_agf(pag) { continue; }
        error = xfs_ialloc_read_agi(pag, (*sc).tp, 0, &mut agi_bp);
        if error != 0 { break; }
        error = xfs_alloc_read_agf(pag, (*sc).tp, 0, &mut agf_bp);
        if error != 0 { break; }
        if !xfs_perag_initialised_agi(pag) || !xfs_perag_initialised_agf(pag) {
            error = -EFSCORRUPTED;
            break;
        }
        xfs_buf_relse(agf_bp); agf_bp = core::ptr::null_mut();
        xfs_buf_relse(agi_bp); agi_bp = core::ptr::null_mut();
    }
    if !agf_bp.is_null() { xfs_buf_relse(agf_bp); }
    if !agi_bp.is_null() { xfs_buf_relse(agi_bp); }
    if !pag.is_null() { xfs_perag_rele(pag); }
    error
}

unsafe fn xchk_fsfreeze(sc: *mut xfs_scrub) -> i32 {
    let error = freeze_super((*(*sc).mp).m_super, FREEZE_HOLDER_KERNEL, core::ptr::null_mut());
    trace_xchk_fsfreeze(sc, error); error
}

unsafe fn xchk_fsthaw(sc: *mut xfs_scrub) -> i32 {
    let error = thaw_super((*(*sc).mp).m_super, FREEZE_HOLDER_KERNEL, core::ptr::null_mut());
    trace_xchk_fsthaw(sc, error); error
}

unsafe fn xchk_fscounters_freeze(sc: *mut xfs_scrub) -> i32 {
    let fsc = (*sc).buf as *mut xchk_fscounters;
    let mut error: i32;
    if (*sc).flags & XCHK_HAVE_FREEZE_PROT != 0 {
        (*sc).flags &= !XCHK_HAVE_FREEZE_PROT;
        mnt_drop_write_file((*sc).file);
    }
    loop {
        error = xchk_fsfreeze(sc);
        if error != -EBUSY { break; }
        if xchk_should_terminate(sc, &mut error) { return error; }
        delay(HZ / 10);
    }
    if error != 0 { return error; }
    (*fsc).frozen = true; 0
}

unsafe extern "C" fn xchk_fscounters_cleanup(buf: *mut core::ffi::c_void) {
    let fsc = buf as *mut xchk_fscounters;
    let sc = (*fsc).sc;
    if !(*fsc).frozen { return; }
    let error = xchk_fsthaw(sc);
    if error != 0 { xfs_emerg((*sc).mp, "still frozen after scrub, err=%d", error); }
    else { (*fsc).frozen = false; }
}

pub unsafe fn xchk_setup_fscounters(sc: *mut xfs_scrub) -> i32 {
    if !xfs_has_lazysbcount((*sc).mp) { xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN); }
    (*sc).buf = kzalloc_obj::<xchk_fscounters>(XCHK_GFP_FLAGS);
    if (*sc).buf.is_null() { return -ENOMEM; }
    (*sc).buf_cleanup = Some(xchk_fscounters_cleanup);
    let fsc = (*sc).buf as *mut xchk_fscounters;
    (*fsc).sc = sc;
    xfs_icount_range((*sc).mp, &mut (*fsc).icount_min, &mut (*fsc).icount_max);
    let mut error = xchk_fscount_warmup(sc);
    if error != 0 { return error; }
    if (*sc).flags & XCHK_TRY_HARDER != 0 || xchk_could_repair(sc) {
        error = xchk_fscounters_freeze(sc); if error != 0 { return error; }
    }
    xchk_trans_alloc_empty(sc); 0
}

unsafe fn xchk_fscount_btreeblks(sc: *mut xfs_scrub, fsc: *mut xchk_fscounters, agno: xfs_agnumber_t) -> i32 {
    let mut blocks: xfs_filblks_t = 0;
    let mut error = xchk_ag_init_existing(sc, agno, &mut (*sc).sa);
    if error == 0 { error = xfs_btree_count_blocks((*sc).sa.bno_cur, &mut blocks); }
    if error == 0 { (*fsc).fdblocks += blocks - 1; error = xfs_btree_count_blocks((*sc).sa.cnt_cur, &mut blocks); }
    if error == 0 { (*fsc).fdblocks += blocks - 1; }
    xchk_ag_free(sc, &mut (*sc).sa); error
}

unsafe fn xchk_fscount_aggregate_agcounts(sc: *mut xfs_scrub, fsc: *mut xchk_fscounters) -> i32 {
    let mp = (*sc).mp; let mut pag: *mut xfs_perag = core::ptr::null_mut(); let mut delayed: u64; let mut tries = 8; let mut error = 0;
    'retry: loop {
        (*fsc).icount = 0; (*fsc).ifree = 0; (*fsc).fdblocks = 0;
        while { pag = xfs_perag_next(mp, pag); !pag.is_null() } {
            if xchk_should_terminate(sc, &mut error) { break; }
            if !xfs_perag_initialised_agi(pag) || !xfs_perag_initialised_agf(pag) { error = -EFSCORRUPTED; break; }
            (*fsc).icount += (*pag).pagi_count; (*fsc).ifree += (*pag).pagi_freecount;
            (*fsc).fdblocks += (*pag).pagf_freeblks + (*pag).pagf_flcount;
            if xfs_has_lazysbcount(mp) { (*fsc).fdblocks += (*pag).pagf_btreeblks; }
            else { error = xchk_fscount_btreeblks(sc, fsc, pag_agno(pag)); if error != 0 { break; } }
            (*fsc).fdblocks -= (*pag).pag_meta_resv.ar_reserved + (*pag).pag_rmapbt_resv.ar_orig_reserved;
        }
        if !pag.is_null() { xfs_perag_rele(pag); }
        if error != 0 { xchk_set_incomplete(sc); return error; }
        (*fsc).fdblocks -= (*mp).m_free[XC_FREE_BLOCKS].res_avail;
        delayed = percpu_counter_sum(&(*mp).m_delalloc_blks); (*fsc).fdblocks -= delayed;
        trace_xchk_fscounters_calc(mp, (*fsc).icount, (*fsc).ifree, (*fsc).fdblocks, delayed);
        if (*fsc).icount < (*fsc).icount_min || (*fsc).icount > (*fsc).icount_max || (*fsc).fdblocks > (*mp).m_sb.sb_dblocks || (*fsc).ifree > (*fsc).icount_max { return -EFSCORRUPTED; }
        if (*fsc).ifree > (*fsc).icount { if tries > 0 { tries -= 1; continue 'retry; } return -EDEADLOCK; }
        return 0;
    }
}

#[cfg(CONFIG_XFS_RT)]
unsafe extern "C" fn xchk_fscount_add_frextent(_rtg: *mut xfs_rtgroup, _tp: *mut xfs_trans, rec: *const xfs_rtalloc_rec, priv_: *mut core::ffi::c_void) -> i32 {
    let fsc = priv_ as *mut xchk_fscounters; (*fsc).frextents += (*rec).ar_extcount; let mut error = 0; xchk_should_terminate((*fsc).sc, &mut error); error
}

unsafe fn xchk_fscount_count_frextents(sc: *mut xfs_scrub, fsc: *mut xchk_fscounters) -> i32 {
    (*fsc).frextents = 0; (*fsc).frextents_delayed = 0; let mp = (*sc).mp;
    if !xfs_has_realtime(mp) || xfs_has_zoned(mp) { return 0; }
    let mut rtg: *mut xfs_rtgroup = core::ptr::null_mut();
    while { rtg = xfs_rtgroup_next(mp, rtg); !rtg.is_null() } {
        xfs_rtgroup_lock(rtg, XFS_RTGLOCK_BITMAP_SHARED);
        let error = xfs_rtalloc_query_all(rtg, (*sc).tp, Some(xchk_fscount_add_frextent), fsc as *mut _);
        xfs_rtgroup_unlock(rtg, XFS_RTGLOCK_BITMAP_SHARED);
        if error != 0 { xchk_set_incomplete(sc); xfs_rtgroup_rele(rtg); return error; }
    }
    (*fsc).frextents_delayed = percpu_counter_sum(&(*mp).m_delalloc_rtextents); 0
}

#[cfg(not(CONFIG_XFS_RT))]
unsafe fn xchk_fscount_count_frextents(_sc: *mut xfs_scrub, fsc: *mut xchk_fscounters) -> i32 { (*fsc).frextents = 0; (*fsc).frextents_delayed = 0; 0 }

unsafe fn xchk_fscount_within_range(sc: *mut xfs_scrub, old_value: i64, counter: *mut percpu_counter, expected: u64) -> bool {
    let curr_value = percpu_counter_sum(counter); trace_xchk_fscounters_within_range((*sc).mp, expected, curr_value, old_value);
    if curr_value < 0 || curr_value as u64 == expected { return curr_value >= 0; }
    if (*sc).sm.sm_flags & XFS_SCRUB_IFLAG_REPAIR != 0 { return false; }
    let min_value = core::cmp::min(old_value, curr_value); let max_value = core::cmp::max(old_value, curr_value);
    expected as i64 >= min_value && expected as i64 <= max_value
}

pub unsafe fn xchk_fscounters(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp; let fsc = (*sc).buf as *mut xchk_fscounters; let icount = percpu_counter_sum(&(*mp).m_icount); let ifree = percpu_counter_sum(&(*mp).m_ifree); let fdblocks = xfs_sum_freecounter_raw(mp, XC_FREE_BLOCKS); let frextents = xfs_sum_freecounter_raw(mp, XC_FREE_RTEXTENTS); let mut try_again = false;
    if icount < 0 || ifree < 0 { xchk_set_corrupt(sc); }
    if fdblocks < 0 || frextents < 0 { if !(*fsc).frozen { return -EDEADLOCK; } xchk_set_corrupt(sc); return 0; }
    if icount < (*fsc).icount_min || icount > (*fsc).icount_max || fdblocks as u64 > (*mp).m_sb.sb_dblocks || frextents as u64 > (*mp).m_sb.sb_rextents || (ifree > icount && (ifree - icount) as u64 > XCHK_FSCOUNT_MIN_VARIANCE) { xchk_set_corrupt(sc); }
    let mut error = xchk_fscount_aggregate_agcounts(sc, fsc); if !xchk_process_error(sc, 0, XFS_SB_BLOCK(mp), &mut error) { return error; }
    error = xchk_fscount_count_frextents(sc, fsc); if !xchk_process_error(sc, 0, XFS_SB_BLOCK(mp), &mut error) { return error; }
    if (*sc).sm.sm_flags & XFS_SCRUB_OFLAG_INCOMPLETE != 0 { return 0; }
    if !xchk_fscount_within_range(sc, icount, &mut (*mp).m_icount, (*fsc).icount) || !xchk_fscount_within_range(sc, ifree, &mut (*mp).m_ifree, (*fsc).ifree) || !xchk_fscount_within_range(sc, fdblocks, &mut (*mp).m_free[XC_FREE_BLOCKS].count, (*fsc).fdblocks) { if (*fsc).frozen { xchk_set_corrupt(sc); } else { try_again = true; } }
    if !xfs_has_zoned(mp) && !xchk_fscount_within_range(sc, frextents, &mut (*mp).m_free[XC_FREE_RTEXTENTS].count, (*fsc).frextents - (*fsc).frextents_delayed) { if (*fsc).frozen { xchk_set_corrupt(sc); } else { try_again = true; } }
    if try_again { -EDEADLOCK } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
