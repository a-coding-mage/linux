// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */
// Translated from xfs_fsops.c.  Declarations supplied by included headers are
// intentionally referenced here as external dependencies.

unsafe fn xfs_resizefs_init_new_ags(
    tp: *mut xfs_trans,
    id: *mut aghdr_init_data,
    oagcount: xfs_agnumber_t,
    nagcount: xfs_agnumber_t,
    mut delta: xfs_rfsblock_t,
    last_pag: *mut xfs_perag,
    lastag_extended: *mut bool,
) -> i32 {
    let mp = (*tp).t_mountp;
    let nb = (*mp).m_sb.sb_dblocks + delta;
    let mut error: i32;
    *lastag_extended = false;
    INIT_LIST_HEAD(&mut (*id).buffer_list);
    (*id).agno = nagcount - 1;
    while (*id).agno >= oagcount {
        if (*id).agno == nagcount - 1 {
            (*id).agsize = nb - (*id).agno * (*mp).m_sb.sb_agblocks;
        } else {
            (*id).agsize = (*mp).m_sb.sb_agblocks;
        }
        error = xfs_ag_init_headers(mp, id);
        if error != 0 {
            xfs_buf_delwri_cancel(&mut (*id).buffer_list);
            return error;
        }
        (*id).agno -= 1;
        delta -= (*id).agsize;
    }
    error = xfs_buf_delwri_submit(&mut (*id).buffer_list);
    if error != 0 { return error; }
    if delta != 0 {
        *lastag_extended = true;
        error = xfs_ag_extend_space(last_pag, tp, delta);
    }
    error
}

unsafe fn xfs_growfs_data_private(mp: *mut xfs_mount, input: *mut xfs_growfs_data) -> i32 {
    let oagcount = (*mp).m_sb.sb_agcount;
    let nb = (*input).newblocks;
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let mut error: i32;
    let nagcount: xfs_agnumber_t;
    let mut nagimax: xfs_agnumber_t = 0;
    let delta: i64;
    let mut lastag_extended = false;
    let mut tp: *mut xfs_trans = core::ptr::null_mut();
    let mut id: aghdr_init_data = core::mem::zeroed();
    let last_pag: *mut xfs_perag;

    error = xfs_sb_validate_fsb_count(&(*mp).m_sb, nb); if error != 0 { return error; }
    if nb > (*mp).m_sb.sb_dblocks {
        error = xfs_buf_read_uncached((*mp).m_ddev_targp, XFS_FSB_TO_BB(mp, nb) - XFS_FSS_TO_BB(mp, 1), XFS_FSS_TO_BB(mp, 1), &mut bp, core::ptr::null_mut());
        if error != 0 { return error; }
        xfs_buf_relse(bp);
    }
    error = xfs_growfs_check_rtgeom(mp, nb, (*mp).m_sb.sb_rblocks, (*mp).m_sb.sb_rextsize); if error != 0 { return error; }
    nagcount = xfs_growfs_compute_agcount(mp, &nb);
    delta = nb as i64 - (*mp).m_sb.sb_dblocks as i64;
    if delta < 0 && nagcount < 2 { return -EINVAL; }
    if delta == 0 { return 0; }
    if nagcount < oagcount { return -EINVAL; }
    error = xfs_initialize_perag(mp, oagcount, nagcount, nb, &mut nagimax); if error != 0 { return error; }
    if delta > 0 {
        error = xfs_trans_alloc(mp, &mut (*M_RES(mp)).tr_growdata, XFS_GROWFS_SPACE_RES(mp), 0, XFS_TRANS_RESERVE, &mut tp);
    } else {
        error = xfs_trans_alloc(mp, &mut (*M_RES(mp)).tr_growdata, -delta as u64, 0, 0, &mut tp);
    }
    if error != 0 { if nagcount > oagcount { xfs_free_perag_range(mp, oagcount, nagcount); } return error; }
    last_pag = xfs_perag_get(mp, oagcount - 1);
    if delta > 0 { error = xfs_resizefs_init_new_ags(tp, &mut id, oagcount, nagcount, delta as u64, last_pag, &mut lastag_extended); }
    else { xfs_warn_experimental(mp, XFS_EXPERIMENTAL_SHRINK); error = xfs_ag_shrink_space(last_pag, &mut tp, (-delta) as u64); }
    xfs_perag_put(last_pag); if error != 0 { xfs_trans_cancel(tp); if nagcount > oagcount { xfs_free_perag_range(mp, oagcount, nagcount); } return error; }
    if nagcount > oagcount { xfs_trans_mod_sb(tp, XFS_TRANS_SB_AGCOUNT, nagcount - oagcount); }
    if delta != 0 { xfs_trans_mod_sb(tp, XFS_TRANS_SB_DBLOCKS, delta); }
    if id.nfree != 0 { xfs_trans_mod_sb(tp, XFS_TRANS_SB_FDBLOCKS, id.nfree); }
    if xfs_has_lazysbcount(mp) { xfs_log_sb(tp); }
    xfs_trans_set_sync(tp); error = xfs_trans_commit(tp); if error != 0 { return error; }
    if nagimax != 0 { (*mp).m_maxagi = nagimax; }
    xfs_set_low_space_thresholds(mp); (*mp).m_alloc_set_aside = xfs_alloc_set_aside(mp);
    if delta > 0 { if lastag_extended { let pag = xfs_perag_get(mp, id.agno); xfs_ag_resv_free(pag); xfs_perag_put(pag); } error = xfs_fs_reserve_ag_blocks(mp); if error == -ENOSPC { error = 0; } xfs_rtrmapbt_compute_maxlevels(mp); xfs_rtrefcountbt_compute_maxlevels(mp); }
    return error;
}

unsafe fn xfs_growfs_log_private(mp: *mut xfs_mount, input: *mut xfs_growfs_log) -> i32 {
    let nb = (*input).newblocks;
    if nb < XFS_MIN_LOG_BLOCKS || nb < XFS_B_TO_FSB(mp, XFS_MIN_LOG_BYTES) { return -EINVAL; }
    if nb == (*mp).m_sb.sb_logblocks && (*input).isint == ((*mp).m_sb.sb_logstart != 0) { return -EINVAL; }
    -ENOSYS
}

unsafe fn xfs_growfs_imaxpct(mp: *mut xfs_mount, imaxpct: u32) -> i32 {
    if imaxpct > 100 { return -EINVAL; }
    let mut tp: *mut xfs_trans = core::ptr::null_mut();
    let mut error = xfs_trans_alloc(mp, &mut (*M_RES(mp)).tr_growdata, XFS_GROWFS_SPACE_RES(mp), 0, XFS_TRANS_RESERVE, &mut tp);
    if error != 0 { return error; }
    let dpct = imaxpct as i32 - (*mp).m_sb.sb_imax_pct as i32;
    xfs_trans_mod_sb(tp, XFS_TRANS_SB_IMAXPCT, dpct); xfs_trans_set_sync(tp); error = xfs_trans_commit(tp); error
}

pub unsafe fn xfs_growfs_data(mp: *mut xfs_mount, input: *mut xfs_growfs_data) -> i32 {
    if !capable(CAP_SYS_ADMIN) { return -EPERM; } if !mutex_trylock(&mut (*mp).m_growlock) { return -EWOULDBLOCK; }
    let mut error: i32;
    if (*input).newblocks != (*mp).m_sb.sb_dblocks && (*mp).m_sb.sb_rtstart != 0 { error = -EINVAL; mutex_unlock(&mut (*mp).m_growlock); return error; }
    if (*input).imaxpct != (*mp).m_sb.sb_imax_pct { error = xfs_growfs_imaxpct(mp, (*input).imaxpct); if error != 0 { mutex_unlock(&mut (*mp).m_growlock); return error; } }
    if (*input).newblocks != (*mp).m_sb.sb_dblocks { error = xfs_growfs_data_private(mp, input); if error != 0 { mutex_unlock(&mut (*mp).m_growlock); return error; } }
    if (*mp).m_sb.sb_imax_pct != 0 { let mut icount = (*mp).m_sb.sb_dblocks * (*mp).m_sb.sb_imax_pct as u64; icount /= 100; (*M_IGEO(mp)).maxicount = XFS_FSB_TO_INO(mp, icount); } else { (*M_IGEO(mp)).maxicount = 0; }
    error = xfs_update_secondary_sbs(mp); (*mp).m_generation += 1; mutex_unlock(&mut (*mp).m_growlock); error
}

pub unsafe fn xfs_growfs_log(mp: *mut xfs_mount, input: *mut xfs_growfs_log) -> i32 { if !capable(CAP_SYS_ADMIN) { return -EPERM; } if !mutex_trylock(&mut (*mp).m_growlock) { return -EWOULDBLOCK; } let e = xfs_growfs_log_private(mp, input); mutex_unlock(&mut (*mp).m_growlock); e }

pub unsafe fn xfs_reserve_blocks(mp: *mut xfs_mount, ctr: xfs_free_counter, request: u64) -> i32 {
    ASSERT(ctr < XC_FREE_NR); spin_lock(&mut (*mp).m_sb_lock);
    if (*mp).m_free[ctr].res_total > request { let lcounter = (*mp).m_free[ctr].res_avail as i64 - request as i64; let mut fd = 0; if lcounter > 0 { fd = lcounter; (*mp).m_free[ctr].res_avail -= lcounter as u64; } (*mp).m_free[ctr].res_total = request; if fd != 0 { spin_unlock(&mut (*mp).m_sb_lock); xfs_add_freecounter(mp, ctr, fd); spin_lock(&mut (*mp).m_sb_lock); } spin_unlock(&mut (*mp).m_sb_lock); return 0; }
    let free = xfs_sum_freecounter_raw(mp, ctr) - xfs_freecounter_unavailable(mp, ctr); let delta = request as i64 - (*mp).m_free[ctr].res_total as i64; (*mp).m_free[ctr].res_total = request; let mut error = 0; if delta > 0 && free > 0 { let fd = core::cmp::min(free, delta); spin_unlock(&mut (*mp).m_sb_lock); error = xfs_dec_freecounter(mp, ctr, fd, 0); if error == 0 { xfs_add_freecounter(mp, ctr, fd); } spin_lock(&mut (*mp).m_sb_lock); } spin_unlock(&mut (*mp).m_sb_lock); error
}

pub unsafe fn xfs_fs_goingdown(mp: *mut xfs_mount, inflags: u32) -> i32 { match inflags { XFS_FSOP_GOING_FLAGS_DEFAULT => { if bdev_freeze((*mp).m_super.s_bdev) == 0 { xfs_force_shutdown(mp, SHUTDOWN_FORCE_UMOUNT); bdev_thaw((*mp).m_super.s_bdev); } }, XFS_FSOP_GOING_FLAGS_LOGFLUSH => xfs_force_shutdown(mp, SHUTDOWN_FORCE_UMOUNT), XFS_FSOP_GOING_FLAGS_NOLOGFLUSH => xfs_force_shutdown(mp, SHUTDOWN_FORCE_UMOUNT | SHUTDOWN_LOG_IO_ERROR), _ => return -EINVAL } 0 }

pub unsafe fn xfs_do_force_shutdown(mp: *mut xfs_mount, flags: u32, fname: *mut i8, lnnum: i32) { if xfs_set_shutdown(mp) { xlog_shutdown_wait((*mp).m_log); return; } if !(*mp).m_sb_bp.is_null() { xfs_buf_set_uptodate((*mp).m_sb_bp); } if flags & SHUTDOWN_FORCE_UMOUNT != 0 { xfs_alert(mp, "User initiated shutdown received."); } let (tag, why) = if xlog_force_shutdown((*mp).m_log, flags) != 0 { (XFS_PTAG_SHUTDOWN_LOGERROR, "Log I/O Error") } else if flags & SHUTDOWN_CORRUPT_INCORE != 0 { (XFS_PTAG_SHUTDOWN_CORRUPT, "Corruption of in-memory data") } else if flags & SHUTDOWN_CORRUPT_ONDISK != 0 { (XFS_PTAG_SHUTDOWN_CORRUPT, "Corruption of on-disk metadata") } else if flags & SHUTDOWN_DEVICE_REMOVED != 0 { (XFS_PTAG_SHUTDOWN_IOERROR, "Block device removal") } else { (XFS_PTAG_SHUTDOWN_IOERROR, "Metadata I/O Error") }; trace_xfs_force_shutdown(mp, tag, flags, fname, lnnum); xfs_alert_tag(mp, tag, "%s (0x%x) detected at %pS (%s:%d).  Shutting down filesystem.", why, flags, __return_address, fname, lnnum); xfs_alert(mp, "Please unmount the filesystem and rectify the problem(s)"); if xfs_error_level >= XFS_ERRLEVEL_HIGH { xfs_stack_trace(); } fserror_report_shutdown((*mp).m_super, GFP_KERNEL); xfs_healthmon_report_shutdown(mp, flags); }

pub unsafe fn xfs_fs_reserve_ag_blocks(mp: *mut xfs_mount) -> i32 { let mut pag = core::ptr::null_mut(); let mut error = 0; (*mp).m_finobt_nores = false; while { pag = xfs_perag_next(mp, pag); !pag.is_null() } { let e = xfs_ag_resv_init(pag, core::ptr::null_mut()); if e != 0 && error == 0 { error = e; } } if error != 0 && error != -ENOSPC { xfs_warn(mp, "Error %d reserving per-AG metadata reserve pool.", error); xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_INCORE); return error; } let e = xfs_metafile_resv_init(mp); if e != 0 && e != -ENOSPC { xfs_warn(mp, "Error %d reserving realtime metadata reserve pool.", e); xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_INCORE); if error == 0 { error = e; } } error }

pub unsafe fn xfs_fs_unreserve_ag_blocks(mp: *mut xfs_mount) { let mut pag = core::ptr::null_mut(); xfs_metafile_resv_free(mp); while { pag = xfs_perag_next(mp, pag); !pag.is_null() } { xfs_ag_resv_free(pag); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
