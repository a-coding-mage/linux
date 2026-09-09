// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Live Quotacheck Repair
// ======================
//
// Use the live quota counter information that we collected to replace the
// counter values in the incore dquots.  A scrub->repair cycle should have left
// the live data and hooks active, so this is safe so long as we make sure the
// dquot is locked.

/* Commit new counters to a dquot. */
unsafe fn xqcheck_commit_dquot(
    xqc: *mut xqcheck,
    dqtype: xfs_dqtype_t,
    dq: *mut xfs_dquot,
) -> i32 {
    let mut xcdq: xqcheck_dquot = core::mem::zeroed();
    let counts: *mut xfarray = xqcheck_counters_for(xqc, dqtype);
    let mut delta: i64;
    let mut dirty = false;
    let mut error = 0;

    error = xchk_trans_alloc((*xqc).sc, 0);
    if error != 0 { return error; }

    mutex_lock(&mut (*dq).q_qlock);
    xfs_trans_dqjoin((*(*xqc).sc).tp, dq);

    if xchk_iscan_aborted(&(*xqc).iscan) {
        error = -ECANCELED;
        goto_out_cancel!();
    }

    mutex_lock(&mut (*xqc).lock);
    error = xfarray_load_sparse(counts, (*dq).q_id, &mut xcdq);
    if error != 0 { goto_out_unlock!(); }

    delta = xcdq.icount as i64 - (*dq).q_ino.count;
    if delta != 0 {
        (*dq).q_ino.reserved += delta;
        (*dq).q_ino.count += delta;
        dirty = true;
    }
    delta = xcdq.bcount as i64 - (*dq).q_blk.count;
    if delta != 0 {
        (*dq).q_blk.reserved += delta;
        (*dq).q_blk.count += delta;
        dirty = true;
    }
    delta = xcdq.rtbcount as i64 - (*dq).q_rtb.count;
    if delta != 0 {
        (*dq).q_rtb.reserved += delta;
        (*dq).q_rtb.count += delta;
        dirty = true;
    }

    xcdq.flags |= XQCHECK_DQUOT_REPAIR_SCANNED | XQCHECK_DQUOT_WRITTEN;
    error = xfarray_store(counts, (*dq).q_id, &xcdq);
    if error == -EFBIG { error = -ECANCELED; }
    mutex_unlock(&mut (*xqc).lock);
    if error != 0 || !dirty { goto_out_cancel!(); }

    trace_xrep_quotacheck_dquot((*(*xqc).sc).mp, (*dq).q_type, (*dq).q_id);
    (*dq).q_flags |= XFS_DQFLAG_DIRTY;
    if (*dq).q_id != 0 { xfs_qm_adjust_dqtimers(dq); }
    xfs_trans_log_dquot((*(*xqc).sc).tp, dq);
    return xrep_trans_commit((*xqc).sc);

    macro_rules! goto_out_unlock { () => {{ mutex_unlock(&mut (*xqc).lock); goto_out_cancel!(); }}; }
    macro_rules! goto_out_cancel { () => {{ xchk_trans_cancel((*xqc).sc); return error; }}; }
}

/* Commit new quota counters for a particular quota type. */
unsafe fn xqcheck_commit_dqtype(xqc: *mut xqcheck, dqtype: u32) -> i32 {
    let mut cursor: xchk_dqiter = core::mem::zeroed();
    let mut xcdq: xqcheck_dquot = core::mem::zeroed();
    let sc = (*xqc).sc;
    let mp = (*sc).mp;
    let counts = xqcheck_counters_for(xqc, dqtype);
    let mut dq: *mut xfs_dquot = core::ptr::null_mut();
    let mut cur: xfarray_idx_t = XFARRAY_CURSOR_INIT;
    let mut error: i32;

    xchk_dqiter_init(&mut cursor, sc, dqtype);
    while { error = xchk_dquot_iter(&mut cursor, &mut dq); error == 1 } {
        error = xqcheck_commit_dquot(xqc, dqtype as xfs_dqtype_t, dq);
        xfs_qm_dqrele(dq);
        if error != 0 { break; }
    }
    if error != 0 { return error; }

    mutex_lock(&mut (*xqc).lock);
    while { error = xfarray_iter(counts, &mut cur, &mut xcdq); error == 1 } {
        let id: xfs_dqid_t = cur - 1;
        if xcdq.flags & XQCHECK_DQUOT_REPAIR_SCANNED != 0 { continue; }
        mutex_unlock(&mut (*xqc).lock);
        error = xfs_qm_dqget(mp, id, dqtype as xfs_dqtype_t, true, &mut dq);
        if error != 0 { return error; }
        error = xqcheck_commit_dquot(xqc, dqtype as xfs_dqtype_t, dq);
        xfs_qm_dqrele(dq);
        if error != 0 { return error; }
        mutex_lock(&mut (*xqc).lock);
    }
    mutex_unlock(&mut (*xqc).lock);
    error
}

/* Figure out quota CHKD flags for the running quota types. */
#[inline]
unsafe fn xqcheck_chkd_flags(mp: *mut xfs_mount) -> u32 {
    let mut ret = 0;
    if XFS_IS_UQUOTA_ON(mp) { ret |= XFS_UQUOTA_CHKD; }
    if XFS_IS_GQUOTA_ON(mp) { ret |= XFS_GQUOTA_CHKD; }
    if XFS_IS_PQUOTA_ON(mp) { ret |= XFS_PQUOTA_CHKD; }
    ret
}

/* Commit the new dquot counters. */
unsafe fn xrep_quotacheck(sc: *mut xfs_scrub) -> i32 {
    let xqc: *mut xqcheck = (*sc).buf as *mut xqcheck;
    let qflags = xqcheck_chkd_flags((*sc).mp);
    let mut error: i32;

    xrep_update_qflags(sc, qflags, 0);
    error = xrep_trans_commit(sc);
    if error != 0 { return error; }

    if !(*xqc).ucounts.is_null() {
        error = xqcheck_commit_dqtype(xqc, XFS_DQTYPE_USER);
        if error != 0 { return error; }
    }
    if !(*xqc).gcounts.is_null() {
        error = xqcheck_commit_dqtype(xqc, XFS_DQTYPE_GROUP);
        if error != 0 { return error; }
    }
    if !(*xqc).pcounts.is_null() {
        error = xqcheck_commit_dqtype(xqc, XFS_DQTYPE_PROJ);
        if error != 0 { return error; }
    }

    error = xchk_trans_alloc(sc, 0);
    if error != 0 { return error; }
    xrep_update_qflags(sc, 0, qflags);
    xrep_trans_commit(sc)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
