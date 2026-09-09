// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
 * Copyright (C) 2010 Red Hat, Inc.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding XFS translation are intentionally
// left external; this file is a source-level translation of xfs_trans.c.

pub static mut xfs_trans_cache: *mut kmem_cache = core::ptr::null_mut();

// CONFIG_TRACEPOINTS selects whether the following helper is emitted.
unsafe fn xfs_trans_trace_reservations(mp: *mut xfs_mount) {
    let mut res = M_RES(mp) as *mut xfs_trans_res;
    let end_res = (M_RES(mp).wrapping_add(1)) as *mut xfs_trans_res;
    let mut i = 0;
    while res < end_res {
        trace_xfs_trans_resv_calc(mp, i, res);
        i += 1;
        res = res.add(1);
    }
}

pub unsafe fn xfs_trans_init(mp: *mut xfs_mount) {
    xfs_trans_resv_calc(mp, M_RES(mp));
    xfs_trans_trace_reservations(mp);
}

unsafe fn xfs_trans_free(tp: *mut xfs_trans) {
    xfs_extent_busy_sort(&mut (*tp).t_busy);
    xfs_extent_busy_clear(&mut (*tp).t_busy, false);
    trace_xfs_trans_free(tp, _RET_IP_());
    xfs_trans_clear_context(tp);
    if ((*tp).t_flags & XFS_TRANS_NO_WRITECOUNT) == 0 {
        sb_end_intwrite((*(*tp).t_mountp).m_super);
    }
    xfs_trans_free_dqinfo(tp);
    kmem_cache_free(xfs_trans_cache, tp);
}

unsafe fn xfs_trans_dup(tp: *mut xfs_trans) -> *mut xfs_trans {
    trace_xfs_trans_dup(tp, _RET_IP_());
    let ntp = kmem_cache_zalloc(xfs_trans_cache, GFP_KERNEL | __GFP_NOFAIL);
    (*ntp).t_mountp = (*tp).t_mountp;
    INIT_LIST_HEAD(&mut (*ntp).t_items);
    INIT_LIST_HEAD(&mut (*ntp).t_busy);
    INIT_LIST_HEAD(&mut (*ntp).t_dfops);
    (*ntp).t_highest_agno = NULLAGNUMBER;
    ASSERT(((*tp).t_flags & XFS_TRANS_PERM_LOG_RES) != 0);
    ASSERT(!(*tp).t_ticket.is_null());
    (*ntp).t_flags = XFS_TRANS_PERM_LOG_RES |
        ((*tp).t_flags & XFS_TRANS_RESERVE) |
        ((*tp).t_flags & XFS_TRANS_NO_WRITECOUNT) |
        ((*tp).t_flags & XFS_TRANS_RES_FDBLKS);
    (*tp).t_flags |= XFS_TRANS_NO_WRITECOUNT;
    (*ntp).t_ticket = xfs_log_ticket_get((*tp).t_ticket);
    ASSERT((*tp).t_blk_res >= (*tp).t_blk_res_used);
    (*ntp).t_blk_res = (*tp).t_blk_res - (*tp).t_blk_res_used;
    (*tp).t_blk_res = (*tp).t_blk_res_used;
    (*ntp).t_rtx_res = (*tp).t_rtx_res - (*tp).t_rtx_res_used;
    (*tp).t_rtx_res = (*tp).t_rtx_res_used;
    xfs_defer_move(ntp, tp);
    xfs_trans_dup_dqinfo(tp, ntp);
    ntp
}

unsafe fn xfs_trans_reserve(tp: *mut xfs_trans, resp: *mut xfs_trans_res,
        blocks: u32, rtextents: u32) -> i32 {
    let mp = (*tp).t_mountp;
    let mut error = 0;
    let rsvd = ((*tp).t_flags & XFS_TRANS_RESERVE) != 0;
    ASSERT((*resp).tr_logres > 0);
    if blocks > 0 {
        error = xfs_dec_fdblocks(mp, blocks, rsvd);
        if error != 0 { return -ENOSPC; }
        (*tp).t_blk_res += blocks;
    }
    if ((*resp).tr_logflags & XFS_TRANS_PERM_LOG_RES) != 0 {
        (*tp).t_flags |= XFS_TRANS_PERM_LOG_RES;
    }
    error = xfs_log_reserve(mp, (*resp).tr_logres, (*resp).tr_logcount,
        &mut (*tp).t_ticket, ((*tp).t_flags & XFS_TRANS_PERM_LOG_RES) != 0);
    if error != 0 { goto_undo_blocks(mp, tp, blocks, error); }
    (*tp).t_log_res = (*resp).tr_logres;
    (*tp).t_log_count = (*resp).tr_logcount;
    if rtextents > 0 {
        error = xfs_dec_frextents(mp, rtextents);
        if error != 0 {
            error = -ENOSPC;
            xfs_log_ticket_ungrant((*mp).m_log, (*tp).t_ticket);
            (*tp).t_ticket = core::ptr::null_mut();
            (*tp).t_log_res = 0;
            (*tp).t_flags &= !XFS_TRANS_PERM_LOG_RES;
            goto_undo_blocks(mp, tp, blocks, error);
        }
        (*tp).t_rtx_res += rtextents;
    }
    0
}

unsafe fn goto_undo_blocks(mp: *mut xfs_mount, tp: *mut xfs_trans,
        blocks: u32, error: i32) -> i32 {
    if blocks > 0 { xfs_add_fdblocks(mp, blocks); (*tp).t_blk_res = 0; }
    error
}

unsafe fn __xfs_trans_alloc(mp: *mut xfs_mount, flags: u32) -> *mut xfs_trans {
    ASSERT((flags & XFS_TRANS_RES_FDBLKS) == 0 || xfs_has_lazysbcount(mp));
    let tp = kmem_cache_zalloc(xfs_trans_cache, GFP_KERNEL | __GFP_NOFAIL);
    if (flags & XFS_TRANS_NO_WRITECOUNT) == 0 { sb_start_intwrite((*mp).m_super); }
    xfs_trans_set_context(tp);
    (*tp).t_flags = flags;
    (*tp).t_mountp = mp;
    INIT_LIST_HEAD(&mut (*tp).t_items);
    INIT_LIST_HEAD(&mut (*tp).t_busy);
    INIT_LIST_HEAD(&mut (*tp).t_dfops);
    (*tp).t_highest_agno = NULLAGNUMBER;
    tp
}

pub unsafe fn xfs_trans_alloc(mp: *mut xfs_mount, resp: *mut xfs_trans_res,
        blocks: u32, rtextents: u32, flags: u32, tpp: *mut *mut xfs_trans) -> i32 {
    ASSERT((*resp).tr_logres > 0);
    let mut retry = true;
    loop {
        let tp = __xfs_trans_alloc(mp, flags);
        WARN_ON((*(*mp).m_super).s_writers.frozen == SB_FREEZE_COMPLETE);
        let error = xfs_trans_reserve(tp, resp, blocks, rtextents);
        if error == -ENOSPC && retry {
            xfs_trans_cancel(tp);
            let e = xfs_blockgc_flush_all(mp);
            if e != 0 { return e; }
            retry = false;
            continue;
        }
        if error != 0 { xfs_trans_cancel(tp); return error; }
        trace_xfs_trans_alloc(tp, _RET_IP_());
        *tpp = tp;
        return 0;
    }
}

pub unsafe fn xfs_trans_alloc_empty(mp: *mut xfs_mount) -> *mut xfs_trans {
    __xfs_trans_alloc(mp, XFS_TRANS_NO_WRITECOUNT)
}

pub unsafe fn xfs_trans_mod_sb(tp: *mut xfs_trans, field: u32, mut delta: i64) {
    let mut flags = XFS_TRANS_DIRTY | XFS_TRANS_SB_DIRTY;
    let mp = (*tp).t_mountp;
    match field {
        XFS_TRANS_SB_ICOUNT => { (*tp).t_icount_delta += delta; if xfs_has_lazysbcount(mp) { flags &= !XFS_TRANS_SB_DIRTY; } }
        XFS_TRANS_SB_IFREE => { (*tp).t_ifree_delta += delta; if xfs_has_lazysbcount(mp) { flags &= !XFS_TRANS_SB_DIRTY; } }
        XFS_TRANS_SB_FDBLOCKS => {
            if delta < 0 { (*tp).t_blk_res_used += (-delta) as u32; if (*tp).t_blk_res_used > (*tp).t_blk_res { xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_INCORE); } }
            else if delta > 0 && ((*tp).t_flags & XFS_TRANS_RES_FDBLKS) != 0 { let n = core::cmp::min(delta, UINT_MAX as i64 - (*tp).t_blk_res as i64); (*tp).t_blk_res += n as u32; delta -= n; }
            (*tp).t_fdblocks_delta += delta; if xfs_has_lazysbcount(mp) { flags &= !XFS_TRANS_SB_DIRTY; }
        }
        XFS_TRANS_SB_RES_FDBLOCKS => { (*tp).t_res_fdblocks_delta += delta; if xfs_has_lazysbcount(mp) { flags &= !XFS_TRANS_SB_DIRTY; } }
        XFS_TRANS_SB_FREXTENTS => { if delta < 0 { (*tp).t_rtx_res_used += (-delta) as u32; ASSERT((*tp).t_rtx_res_used <= (*tp).t_rtx_res); } (*tp).t_frextents_delta += delta; if xfs_has_rtgroups(mp) { flags &= !XFS_TRANS_SB_DIRTY; } }
        XFS_TRANS_SB_RES_FREXTENTS => { ASSERT(delta < 0); (*tp).t_res_frextents_delta += delta; if xfs_has_rtgroups(mp) { flags &= !XFS_TRANS_SB_DIRTY; } }
        XFS_TRANS_SB_DBLOCKS => (*tp).t_dblocks_delta += delta,
        XFS_TRANS_SB_AGCOUNT => { ASSERT(delta > 0); (*tp).t_agcount_delta += delta; }
        XFS_TRANS_SB_IMAXPCT => (*tp).t_imaxpct_delta += delta,
        XFS_TRANS_SB_REXTSIZE => (*tp).t_rextsize_delta += delta,
        XFS_TRANS_SB_RBMBLOCKS => (*tp).t_rbmblocks_delta += delta,
        XFS_TRANS_SB_RBLOCKS => (*tp).t_rblocks_delta += delta,
        XFS_TRANS_SB_REXTENTS => (*tp).t_rextents_delta += delta,
        XFS_TRANS_SB_REXTSLOG => (*tp).t_rextslog_delta += delta,
        XFS_TRANS_SB_RGCOUNT => { ASSERT(delta > 0); (*tp).t_rgcount_delta += delta; }
        _ => { ASSERT(false); return; }
    }
    (*tp).t_flags |= flags;
}

unsafe fn xfs_trans_apply_sb_deltas(tp: *mut xfs_trans) {
    let mp = (*tp).t_mountp;
    let bp = xfs_trans_getsb(tp);
    let sbp = (*bp).b_addr as *mut xfs_dsb;
    let mut whole = false;
    if !xfs_has_lazysbcount(mp) {
        if (*tp).t_icount_delta != 0 { be64_add_cpu(&mut (*sbp).sb_icount, (*tp).t_icount_delta); }
        if (*tp).t_ifree_delta != 0 { be64_add_cpu(&mut (*sbp).sb_ifree, (*tp).t_ifree_delta); }
        if (*tp).t_fdblocks_delta != 0 { be64_add_cpu(&mut (*sbp).sb_fdblocks, (*tp).t_fdblocks_delta); }
        if (*tp).t_res_fdblocks_delta != 0 { be64_add_cpu(&mut (*sbp).sb_fdblocks, (*tp).t_res_fdblocks_delta); }
    }
    if ((*tp).t_frextents_delta != 0 || (*tp).t_res_frextents_delta != 0) && !xfs_has_rtgroups(mp) {
        let d = (*tp).t_frextents_delta + (*tp).t_res_frextents_delta;
        spin_lock(&mut (*mp).m_sb_lock); be64_add_cpu(&mut (*sbp).sb_frextents, d); (*mp).m_sb.sb_frextents += d; spin_unlock(&mut (*mp).m_sb_lock);
    }
    if (*tp).t_dblocks_delta != 0 { be64_add_cpu(&mut (*sbp).sb_dblocks, (*tp).t_dblocks_delta); (*(*mp).m_ddev_targp).bt_nr_sectors += XFS_FSB_TO_BB(mp, (*tp).t_dblocks_delta); whole = true; }
    if (*tp).t_agcount_delta != 0 { be32_add_cpu(&mut (*sbp).sb_agcount, (*tp).t_agcount_delta); whole = true; }
    if (*tp).t_imaxpct_delta != 0 { (*sbp).sb_imax_pct += (*tp).t_imaxpct_delta as u8; whole = true; }
    if (*tp).t_rextsize_delta != 0 { be32_add_cpu(&mut (*sbp).sb_rextsize, (*tp).t_rextsize_delta); if xfs_has_rtgroups(mp) { (*sbp).sb_rgblklog = xfs_compute_rgblklog(be32_to_cpu((*sbp).sb_rgextents), be32_to_cpu((*sbp).sb_rextsize)); } whole = true; }
    if (*tp).t_rbmblocks_delta != 0 { be32_add_cpu(&mut (*sbp).sb_rbmblocks, (*tp).t_rbmblocks_delta); whole = true; }
    if (*tp).t_rblocks_delta != 0 { be64_add_cpu(&mut (*sbp).sb_rblocks, (*tp).t_rblocks_delta); (*(*mp).m_rtdev_targp).bt_nr_sectors += XFS_FSB_TO_BB(mp, (*tp).t_rblocks_delta); whole = true; }
    if (*tp).t_rextents_delta != 0 { be64_add_cpu(&mut (*sbp).sb_rextents, (*tp).t_rextents_delta); whole = true; }
    if (*tp).t_rextslog_delta != 0 { (*sbp).sb_rextslog += (*tp).t_rextslog_delta as u8; whole = true; }
    if (*tp).t_rgcount_delta != 0 { be32_add_cpu(&mut (*sbp).sb_rgcount, (*tp).t_rgcount_delta); whole = true; }
    xfs_trans_buf_set_type(tp, bp, XFS_BLFT_SB_BUF);
    if whole { xfs_trans_log_buf(tp, bp, 0, core::mem::size_of::<xfs_dsb>() - 1); }
    else { xfs_trans_log_buf(tp, bp, offset_of!(xfs_dsb, sb_icount), offset_of!(xfs_dsb, sb_frextents) + core::mem::size_of_val(&(*sbp).sb_frextents) - 1); }
}

pub const XFS_ICOUNT_BATCH: i32 = 128;

pub unsafe fn xfs_trans_unreserve_and_mod_sb(tp: *mut xfs_trans) {
    let mp = (*tp).t_mountp;
    let mut blkdelta = (*tp).t_blk_res as i64;
    let mut rtxdelta = (*tp).t_rtx_res as i64;
    let mut idelta = 0i64;
    let mut ifreedelta = 0i64;
    ASSERT((*tp).t_blk_res != 0 || (*tp).t_fdblocks_delta >= 0);
    if xfs_has_lazysbcount(mp) || ((*tp).t_flags & XFS_TRANS_SB_DIRTY) != 0 { blkdelta += (*tp).t_fdblocks_delta; ASSERT(blkdelta >= 0); }
    ASSERT((*tp).t_rtx_res != 0 || (*tp).t_frextents_delta >= 0);
    if xfs_has_rtgroups(mp) || ((*tp).t_flags & XFS_TRANS_SB_DIRTY) != 0 { rtxdelta += (*tp).t_frextents_delta; ASSERT(rtxdelta >= 0); }
    if xfs_has_lazysbcount(mp) || ((*tp).t_flags & XFS_TRANS_SB_DIRTY) != 0 { idelta = (*tp).t_icount_delta; ifreedelta = (*tp).t_ifree_delta; }
    if blkdelta != 0 { xfs_add_fdblocks(mp, blkdelta); }
    if idelta != 0 { percpu_counter_add_batch(&mut (*mp).m_icount, idelta, XFS_ICOUNT_BATCH); }
    if ifreedelta != 0 { percpu_counter_add(&mut (*mp).m_ifree, ifreedelta); }
    if rtxdelta != 0 { xfs_add_frextents(mp, rtxdelta); }
    if ((*tp).t_flags & XFS_TRANS_SB_DIRTY) == 0 { return; }
    spin_lock(&mut (*mp).m_sb_lock);
    (*mp).m_sb.sb_fdblocks += (*tp).t_fdblocks_delta + (*tp).t_res_fdblocks_delta;
    (*mp).m_sb.sb_icount += idelta; (*mp).m_sb.sb_ifree += ifreedelta;
    (*mp).m_sb.sb_dblocks += (*tp).t_dblocks_delta; (*mp).m_sb.sb_agcount += (*tp).t_agcount_delta;
    (*mp).m_sb.sb_imax_pct += (*tp).t_imaxpct_delta;
    if (*tp).t_rextsize_delta != 0 { xfs_mount_sb_set_rextsize(mp, &mut (*mp).m_sb, (*mp).m_sb.sb_rextsize + (*tp).t_rextsize_delta as u32); }
    (*mp).m_sb.sb_rbmblocks += (*tp).t_rbmblocks_delta; (*mp).m_sb.sb_rblocks += (*tp).t_rblocks_delta;
    (*mp).m_sb.sb_rextents += (*tp).t_rextents_delta; (*mp).m_sb.sb_rextslog += (*tp).t_rextslog_delta; (*mp).m_sb.sb_rgcount += (*tp).t_rgcount_delta;
    spin_unlock(&mut (*mp).m_sb_lock);
    ASSERT((*mp).m_sb.sb_imax_pct >= 0); ASSERT((*mp).m_sb.sb_rextslog >= 0);
}

pub unsafe fn xfs_trans_add_item(tp: *mut xfs_trans, lip: *mut xfs_log_item) {
    ASSERT((*lip).li_log == (*(*tp).t_mountp).m_log); ASSERT((*lip).li_ailp == (*(*tp).t_mountp).m_ail); ASSERT(list_empty(&(*lip).li_trans)); ASSERT(!test_bit(XFS_LI_DIRTY, &(*lip).li_flags));
    list_add_tail(&mut (*lip).li_trans, &mut (*tp).t_items); trace_xfs_trans_add_item(tp, _RET_IP_());
}
pub unsafe fn xfs_trans_del_item(lip: *mut xfs_log_item) { clear_bit(XFS_LI_DIRTY, &mut (*lip).li_flags); list_del_init(&mut (*lip).li_trans); }

unsafe fn xfs_trans_free_items(tp: *mut xfs_trans, abort: bool) {
    trace_xfs_trans_free_items(tp, _RET_IP_());
    let mut lip = list_first_entry_or_null(&(*tp).t_items, xfs_log_item, li_trans);
    while !lip.is_null() {
        let next = list_next_entry_or_null(lip, &(*tp).t_items, li_trans);
        xfs_trans_del_item(lip); if abort { trace_xfs_trans_free_abort(lip); set_bit(XFS_LI_ABORTED, &mut (*lip).li_flags); }
        if let Some(release) = (*(*lip).li_ops).iop_release { release(lip); }
        lip = next;
    }
}

unsafe fn xfs_trans_precommit_sort(_unused: *mut core::ffi::c_void, a: *const list_head, b: *const list_head) -> i32 {
    let lia = container_of(a, xfs_log_item, li_trans); let lib = container_of(b, xfs_log_item, li_trans);
    let sa = (*(*lia).li_ops).iop_sort; let sb = (*(*lib).li_ops).iop_sort;
    if sa.is_none() && sb.is_none() { return 0; } if sa.is_none() { return 1; } if sb.is_none() { return -1; }
    let diff = sa.unwrap()(lia) - sb.unwrap()(lib); if diff < 0 { -1 } else if diff > 0 { 1 } else { 0 }
}

unsafe fn xfs_trans_run_precommits(tp: *mut xfs_trans) -> i32 {
    let mp = (*tp).t_mountp; let mut error = 0;
    list_sort(core::ptr::null_mut(), &mut (*tp).t_items, xfs_trans_precommit_sort);
    let mut lip = list_first_entry_or_null(&(*tp).t_items, xfs_log_item, li_trans);
    while !lip.is_null() { let next = list_next_entry_or_null(lip, &(*tp).t_items, li_trans); if test_bit(XFS_LI_DIRTY, &(*lip).li_flags) { if let Some(pre) = (*(*lip).li_ops).iop_precommit { error = pre(tp, lip); if error != 0 { break; } } } lip = next; }
    if error != 0 { xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_INCORE); } error
}

unsafe fn __xfs_trans_commit(tp: *mut xfs_trans, regrant: bool) -> i32 {
    let mp = (*tp).t_mountp; let log = (*mp).m_log; let mut commit_seq: xfs_csn_t = 0; let mut error = 0; let sync = ((*tp).t_flags & XFS_TRANS_SYNC) != 0;
    trace_xfs_trans_commit(tp, _RET_IP_());
    if ((*tp).t_flags & XFS_TRANS_SB_DIRTY) != 0 { xfs_trans_apply_sb_deltas(tp); } xfs_trans_apply_dquot_deltas(tp);
    error = xfs_trans_run_precommits(tp); if error != 0 { goto_out_unreserve(tp, regrant, error); }
    if ((*tp).t_flags & XFS_TRANS_DIRTY) == 0 { return goto_out_unreserve(tp, regrant, 0); }
    if xlog_is_shutdown(log) { return goto_out_unreserve(tp, regrant, -EIO); }
    ASSERT(!(*tp).t_ticket.is_null()); xlog_cil_commit(log, tp, &mut commit_seq, regrant); xfs_trans_free(tp);
    if sync { error = xfs_log_force_seq(mp, commit_seq, XFS_LOG_SYNC, core::ptr::null_mut()); XFS_STATS_INC(mp, xs_trans_sync); } else { XFS_STATS_INC(mp, xs_trans_async); } error
}

unsafe fn goto_out_unreserve(tp: *mut xfs_trans, regrant: bool, error: i32) -> i32 {
    let mp = (*tp).t_mountp; let log = (*mp).m_log; xfs_trans_unreserve_and_mod_sb(tp); xfs_trans_unreserve_and_mod_dquots(tp, true);
    if !(*tp).t_ticket.is_null() { if regrant && !xlog_is_shutdown(log) { xfs_log_ticket_regrant(log, (*tp).t_ticket); } else { xfs_log_ticket_ungrant(log, (*tp).t_ticket); } (*tp).t_ticket = core::ptr::null_mut(); }
    xfs_trans_free_items(tp, error != 0); xfs_trans_free(tp); XFS_STATS_INC(mp, xs_trans_empty); error
}

pub unsafe fn xfs_trans_commit(tp: *mut xfs_trans) -> i32 {
    WARN_ON_ONCE(!list_empty(&(*tp).t_dfops) && ((*tp).t_flags & XFS_TRANS_PERM_LOG_RES) == 0);
    if ((*tp).t_flags & XFS_TRANS_PERM_LOG_RES) != 0 { let error = xfs_defer_finish_noroll(&mut (tp as *mut xfs_trans)); if error != 0 { xfs_trans_cancel(tp); return error; } }
    __xfs_trans_commit(tp, false)
}

pub unsafe fn xfs_trans_cancel(tp: *mut xfs_trans) {
    let mp = (*tp).t_mountp; let log = (*mp).m_log; let mut dirty = ((*tp).t_flags & XFS_TRANS_DIRTY) != 0;
    trace_xfs_trans_cancel(tp, _RET_IP_());
    if !list_empty(&(*tp).t_dfops) { ASSERT(((*tp).t_flags & XFS_TRANS_PERM_LOG_RES) != 0); dirty = true; xfs_defer_cancel(tp); }
    if dirty && !xfs_is_shutdown(mp) { XFS_ERROR_REPORT("xfs_trans_cancel", XFS_ERRLEVEL_LOW, mp); xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_INCORE); }
    xfs_trans_unreserve_and_mod_sb(tp); xfs_trans_unreserve_and_mod_dquots(tp, false);
    if !(*tp).t_ticket.is_null() { xfs_log_ticket_ungrant(log, (*tp).t_ticket); (*tp).t_ticket = core::ptr::null_mut(); }
    xfs_trans_free_items(tp, dirty); xfs_trans_free(tp);
}

pub unsafe fn xfs_trans_roll(tpp: *mut *mut xfs_trans) -> i32 {
    let tp = *tpp; let log_res = (*tp).t_log_res; let log_count = (*tp).t_log_count; ASSERT(log_res > 0); trace_xfs_trans_roll(tp, _RET_IP_());
    *tpp = xfs_trans_dup(tp); let error = __xfs_trans_commit(tp, true); let ntp = *tpp; xfs_trans_set_context(ntp); if error != 0 { return error; }
    let error = xfs_log_regrant((*ntp).t_mountp, (*ntp).t_ticket); if error != 0 { return error; } (*ntp).t_log_res = log_res; (*ntp).t_log_count = log_count; 0
}

pub unsafe fn xfs_trans_alloc_inode(ip: *mut xfs_inode, resv: *mut xfs_trans_res, dblocks: u32, rblocks: u32, force: bool, tpp: *mut *mut xfs_trans) -> i32 {
    let mp = (*ip).i_mount; let mut retried = false;
    loop {
        let mut tp = core::ptr::null_mut(); let error = xfs_trans_alloc(mp, resv, dblocks, xfs_extlen_to_rtxlen(mp, rblocks), if force { XFS_TRANS_RESERVE } else { 0 }, &mut tp); if error != 0 { return error; }
        xfs_ilock(ip, XFS_ILOCK_EXCL); xfs_trans_ijoin(tp, ip, 0); let mut error = xfs_qm_dqattach_locked(ip, false); if error != 0 { ASSERT(error != -ENOENT); xfs_trans_cancel(tp); xfs_iunlock(ip, XFS_ILOCK_EXCL); return error; }
        error = xfs_trans_reserve_quota_nblks(tp, ip, dblocks, rblocks, force);
        if (error == -EDQUOT || error == -ENOSPC) && !retried { xfs_trans_cancel(tp); xfs_iunlock(ip, XFS_ILOCK_EXCL); xfs_blockgc_free_quota(ip, 0); retried = true; continue; }
        if error != 0 { xfs_trans_cancel(tp); xfs_iunlock(ip, XFS_ILOCK_EXCL); return error; } *tpp = tp; return 0;
    }
}

pub unsafe fn xfs_trans_reserve_more(tp: *mut xfs_trans, blocks: u32, rtextents: u32) -> i32 {
    let rsvd = ((*tp).t_flags & XFS_TRANS_RESERVE) != 0; if blocks != 0 && xfs_dec_fdblocks((*tp).t_mountp, blocks, rsvd) != 0 { return -ENOSPC; }
    if rtextents != 0 && xfs_dec_frextents((*tp).t_mountp, rtextents) != 0 { if blocks != 0 { xfs_add_fdblocks((*tp).t_mountp, blocks); } return -ENOSPC; }
    (*tp).t_blk_res += blocks; (*tp).t_rtx_res += rtextents; 0
}

pub unsafe fn xfs_trans_reserve_more_inode(tp: *mut xfs_trans, ip: *mut xfs_inode, dblocks: u32, rblocks: u32, mut force_quota: bool) -> i32 {
    let mp = (*ip).i_mount; let rtx = xfs_extlen_to_rtxlen(mp, rblocks); xfs_assert_ilocked(ip, XFS_ILOCK_EXCL);
    let error = xfs_trans_reserve_more(tp, dblocks, rtx); if error != 0 { return error; }
    if !XFS_IS_QUOTA_ON(mp) || xfs_is_quota_inode(&(*mp).m_sb, I_INO(ip)) { return 0; }
    if ((*tp).t_flags & XFS_TRANS_RESERVE) != 0 { force_quota = true; }
    let error = xfs_trans_reserve_quota_nblks(tp, ip, dblocks, rblocks, force_quota); if error == 0 { return 0; }
    xfs_add_fdblocks(mp, dblocks); (*tp).t_blk_res -= dblocks; xfs_add_frextents(mp, rtx); (*tp).t_rtx_res -= rtx; error
}

pub unsafe fn xfs_trans_alloc_icreate(mp: *mut xfs_mount, resv: *mut xfs_trans_res, udqp: *mut xfs_dquot, gdqp: *mut xfs_dquot, pdqp: *mut xfs_dquot, dblocks: u32, tpp: *mut *mut xfs_trans) -> i32 {
    let mut retried = false; let mut flushed = false;
    loop { let mut tp = core::ptr::null_mut(); let mut error = xfs_trans_alloc(mp, resv, dblocks, 0, 0, &mut tp);
        if error == -ENOSPC && !flushed { xfs_flush_inodes(mp); flushed = true; continue; } if error != 0 { return error; }
        error = xfs_trans_reserve_quota_icreate(tp, udqp, gdqp, pdqp, dblocks);
        if (error == -EDQUOT || error == -ENOSPC) && !retried { xfs_trans_cancel(tp); xfs_blockgc_free_dquots(mp, udqp, gdqp, pdqp, 0); retried = true; continue; }
        if error != 0 { xfs_trans_cancel(tp); return error; } *tpp = tp; return 0;
    }
}

pub unsafe fn xfs_trans_alloc_ichange(ip: *mut xfs_inode, new_udqp: *mut xfs_dquot, new_gdqp: *mut xfs_dquot, new_pdqp: *mut xfs_dquot, force: bool, tpp: *mut *mut xfs_trans) -> i32 {
    let mp = (*ip).i_mount; let mut retried = false;
    loop { let mut tp = core::ptr::null_mut(); let mut error = xfs_trans_alloc(mp, &mut M_RES(mp).tr_ichange, 0, 0, 0, &mut tp); if error != 0 { return error; }
        xfs_ilock(ip, XFS_ILOCK_EXCL); xfs_trans_ijoin(tp, ip, XFS_ILOCK_EXCL); if xfs_is_metadir_inode(ip) { *tpp = tp; return 0; }
        error = xfs_qm_dqattach_locked(ip, false); if error != 0 { ASSERT(error != -ENOENT); xfs_trans_cancel(tp); return error; }
        let udqp = if new_udqp != (*ip).i_udquot { new_udqp } else { core::ptr::null_mut() }; let gdqp = if new_gdqp != (*ip).i_gdquot { new_gdqp } else { core::ptr::null_mut() }; let pdqp = if new_pdqp != (*ip).i_pdquot { new_pdqp } else { core::ptr::null_mut() };
        if !udqp.is_null() || !gdqp.is_null() || !pdqp.is_null() { let mut dblocks = 0; let mut rblocks = 0; let mut qflags = XFS_QMOPT_RES_REGBLKS; let isrt = XFS_IS_REALTIME_INODE(ip); if force { qflags |= XFS_QMOPT_FORCE_RES; } if isrt { error = xfs_iread_extents(tp, ip, XFS_DATA_FORK); if error != 0 { xfs_trans_cancel(tp); return error; } } xfs_inode_count_blocks(tp, ip, &mut dblocks, &mut rblocks); if isrt { rblocks += (*ip).i_delayed_blks; } else { dblocks += (*ip).i_delayed_blks; }
            error = xfs_trans_reserve_quota_bydquots(tp, mp, udqp, gdqp, pdqp, dblocks, 1, qflags); if (error == -EDQUOT || error == -ENOSPC) && !retried { xfs_trans_cancel(tp); xfs_blockgc_free_dquots(mp, udqp, gdqp, pdqp, 0); retried = true; continue; } if error != 0 { xfs_trans_cancel(tp); return error; }
            qflags = XFS_QMOPT_RES_RTBLKS | (qflags & XFS_QMOPT_FORCE_RES); error = xfs_trans_reserve_quota_bydquots(tp, mp, udqp, gdqp, pdqp, rblocks, 0, qflags); if (error == -EDQUOT || error == -ENOSPC) && !retried { xfs_trans_cancel(tp); xfs_blockgc_free_dquots(mp, udqp, gdqp, pdqp, 0); retried = true; continue; } if error != 0 { xfs_trans_cancel(tp); return error; }
        } *tpp = tp; return 0;
    }
}

pub unsafe fn xfs_trans_alloc_dir(dp: *mut xfs_inode, resv: *mut xfs_trans_res, ip: *mut xfs_inode, dblocks: *mut u32, tpp: *mut *mut xfs_trans, nospace_error: *mut i32) -> i32 {
    let mp = (*ip).i_mount; let mut retried = false;
    loop { *nospace_error = 0; let mut resblks = *dblocks; let mut tp = core::ptr::null_mut(); let mut error = xfs_trans_alloc(mp, resv, resblks, 0, 0, &mut tp); if error == -ENOSPC { *nospace_error = error; resblks = 0; error = xfs_trans_alloc(mp, resv, 0, 0, 0, &mut tp); } if error != 0 { return error; }
        xfs_lock_two_inodes(dp, XFS_ILOCK_EXCL, ip, XFS_ILOCK_EXCL); xfs_trans_ijoin(tp, dp, 0); xfs_trans_ijoin(tp, ip, 0); error = xfs_qm_dqattach_locked(dp, false); if error != 0 { ASSERT(error != -ENOENT); xfs_trans_cancel(tp); xfs_iunlock(dp, XFS_ILOCK_EXCL); if dp != ip { xfs_iunlock(ip, XFS_ILOCK_EXCL); } return error; } error = xfs_qm_dqattach_locked(ip, false); if error != 0 { ASSERT(error != -ENOENT); xfs_trans_cancel(tp); xfs_iunlock(dp, XFS_ILOCK_EXCL); if dp != ip { xfs_iunlock(ip, XFS_ILOCK_EXCL); } return error; }
        if resblks == 0 { *tpp = tp; *dblocks = 0; return 0; }
        error = xfs_trans_reserve_quota_nblks(tp, dp, resblks, 0, false); if error == -EDQUOT || error == -ENOSPC { if !retried { xfs_trans_cancel(tp); xfs_iunlock(dp, XFS_ILOCK_EXCL); if dp != ip { xfs_iunlock(ip, XFS_ILOCK_EXCL); } xfs_blockgc_free_quota(dp, 0); retried = true; continue; } *nospace_error = error; resblks = 0; error = 0; } if error != 0 { xfs_trans_cancel(tp); xfs_iunlock(dp, XFS_ILOCK_EXCL); if dp != ip { xfs_iunlock(ip, XFS_ILOCK_EXCL); } return error; }
        *tpp = tp; *dblocks = resblks; return 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
