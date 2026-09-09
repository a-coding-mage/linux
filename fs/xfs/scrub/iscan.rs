// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Live File Scan.  The declarations used here are supplied by the XFS
 * translation environment. */

unsafe fn xchk_iscan_mask_skipino(iscan: *mut xchk_iscan, pag: *mut xfs_perag,
        rec: *mut xfs_inobt_rec_incore, lastrecino: xfs_agino_t) {
    let sc = (*iscan).sc;
    let mp = (*sc).mp;
    let skip_agno = XFS_INO_TO_AGNO(mp, (*iscan).skip_ino);
    let skip_agino = XFS_INO_TO_AGINO(mp, (*iscan).skip_ino);
    if pag_agno(pag) != skip_agno || skip_agino < (*rec).ir_startino || skip_agino > lastrecino { return; }
    (*rec).ir_free |= xfs_inobt_maskn(skip_agino - (*rec).ir_startino, 1);
}

unsafe fn xchk_iscan_find_next(iscan: *mut xchk_iscan, agi_bp: *mut xfs_buf,
        pag: *mut xfs_perag, allocmaskp: *mut xfs_inofree_t,
        cursor: *mut xfs_agino_t, nr_inodesp: *mut u8) -> i32 {
    let sc = (*iscan).sc; let mp = (*sc).mp; let tp = (*sc).tp;
    let agno = pag_agno(pag); let mut lastino = NULLAGINO;
    let (mut first, mut last) = (0, 0); let agino = *cursor;
    xfs_agino_range(mp, agno, &mut first, &mut last);
    if agino > last { *cursor = NULLAGINO; return 0; }
    let cur = xfs_inobt_init_cursor(pag, tp, agi_bp); let mut has_rec = 0;
    let mut error = xfs_inobt_lookup(cur, agino, XFS_LOOKUP_LE, &mut has_rec);
    if error == 0 && has_rec == 0 { error = xfs_btree_increment(cur, 0, &mut has_rec); }
    while error == 0 {
        let mut rec = core::mem::MaybeUninit::<xfs_inobt_rec_incore>::uninit();
        if has_rec == 0 { *cursor = NULLAGINO; break; }
        error = xfs_inobt_get_rec(cur, rec.as_mut_ptr(), &mut has_rec); if error != 0 { break; }
        if has_rec == 0 { error = -EFSCORRUPTED; break; }
        let rec = rec.assume_init_mut();
        if lastino != NULLAGINO && XFS_IS_CORRUPT(mp, lastino >= rec.ir_startino) { error = -EFSCORRUPTED; break; }
        lastino = rec.ir_startino + XFS_INODES_PER_CHUNK - 1;
        if rec.ir_startino + XFS_INODES_PER_CHUNK <= agino { error = xfs_btree_increment(cur, 0, &mut has_rec); continue; }
        if (*iscan).skip_ino != 0 { xchk_iscan_mask_skipino(iscan, pag, rec, lastino); }
        if agino >= rec.ir_startino { rec.ir_free |= xfs_inobt_maskn(0, agino + 1 - rec.ir_startino); }
        let allocmask = !rec.ir_free;
        if hweight64(allocmask) > 0 {
            let next = xfs_lowbit64(allocmask); ASSERT(next >= 0);
            *cursor = rec.ir_startino + next; *allocmaskp = allocmask >> next;
            *nr_inodesp = XFS_INODES_PER_CHUNK - next; break;
        }
        error = xfs_btree_increment(cur, 0, &mut has_rec);
    }
    xfs_btree_del_cursor(cur, error); error
}

unsafe fn xchk_iscan_move_cursor(iscan: *mut xchk_iscan, agno: xfs_agnumber_t, agino: xfs_agino_t) {
    let mp = (*(*iscan).sc).mp; let cursor = XFS_AGINO_TO_INO(mp, agno, agino);
    let visited = if cursor == 0 { XFS_MAXINUMBER } else { cursor - 1 };
    mutex_lock(&mut (*iscan).lock); (*iscan).cursor_ino = cursor; (*iscan).__visited_ino = visited;
    trace_xchk_iscan_move_cursor(iscan); mutex_unlock(&mut (*iscan).lock);
}

unsafe fn xchk_iscan_finish(iscan: *mut xchk_iscan) {
    mutex_lock(&mut (*iscan).lock); (*iscan).cursor_ino = NULLFSINO; (*iscan).__visited_ino = NULLFSINO;
    mutex_unlock(&mut (*iscan).lock);
}

pub unsafe fn xchk_iscan_finish_early(iscan: *mut xchk_iscan) {
    ASSERT((*iscan).cursor_ino == (*iscan).scan_start_ino); ASSERT((*iscan).__visited_ino == (*iscan).scan_start_ino);
    xchk_iscan_finish(iscan);
}

unsafe fn xchk_iscan_read_agi(iscan: *mut xchk_iscan, pag: *mut xfs_perag, agi_bpp: *mut *mut xfs_buf) -> i32 {
    let sc = (*iscan).sc;
    if !xchk_iscan_agi_needs_trylock(iscan) { return xfs_ialloc_read_agi(pag, (*sc).tp, 0, agi_bpp); }
    let relax = msecs_to_jiffies((*iscan).iget_retry_delay); let mut ret;
    loop { ret = xfs_ialloc_read_agi(pag, (*sc).tp, XFS_IALLOC_FLAG_TRYLOCK, agi_bpp);
        if ret != -EAGAIN { return ret; }
        if (*iscan).iget_timeout == 0 || time_is_before_jiffies((*iscan).__iget_deadline) { return -EBUSY; }
        trace_xchk_iscan_agi_retry_wait(iscan);
        if schedule_timeout_killable(relax) != 0 || xchk_iscan_aborted(iscan) { return -ECANCELED; }
    }
}

// The remaining scan operations retain the C control flow and ABI-facing fields.
pub unsafe fn xchk_iscan_iter(iscan: *mut xchk_iscan, ipp: *mut *mut xfs_inode) -> i32 {
    for i in 0..XFS_INODES_PER_CHUNK { if !(*iscan).__inodes[i].is_null() { *ipp = (*iscan).__inodes[i]; (*iscan).__inodes[i] = core::ptr::null_mut(); return 1; } }
    let error = xchk_iscan_iter_batch(iscan); if error <= 0 { return error; }
    *ipp = (*iscan).__inodes[0]; (*iscan).__inodes[0] = core::ptr::null_mut(); 1
}

pub unsafe fn xchk_iscan_iter_finish(iscan: *mut xchk_iscan) {
    let sc = (*iscan).sc;
    for i in 0..XFS_INODES_PER_CHUNK { if !(*iscan).__inodes[i].is_null() { xchk_irele(sc, (*iscan).__inodes[i]); (*iscan).__inodes[i] = core::ptr::null_mut(); } }
}

pub unsafe fn xchk_iscan_teardown(iscan: *mut xchk_iscan) { xchk_iscan_iter_finish(iscan); xchk_iscan_finish(iscan); mutex_destroy(&mut (*iscan).lock); }

unsafe fn xchk_iscan_iter_batch(iscan: *mut xchk_iscan) -> i32 {
    xchk_iscan_finish_batch(iscan);
    if (*iscan).iget_timeout != 0 { (*iscan).__iget_deadline = jiffies + msecs_to_jiffies((*iscan).iget_timeout); }
    loop {
        let (mut pag, mut agi_bp) = (core::ptr::null_mut(), core::ptr::null_mut());
        let (mut allocmask, mut nr) = (0, 0u8);
        let ret = xchk_iscan_advance(iscan, &mut pag, &mut agi_bp, &mut allocmask, &mut nr);
        if ret != 1 { return ret; }
        if xchk_iscan_aborted(iscan) { xfs_trans_brelse((*(*iscan).sc).tp, agi_bp); xfs_perag_put(pag); return -ECANCELED; }
        let ret = xchk_iscan_iget(iscan, pag, agi_bp, allocmask, nr);
        if ret != -EAGAIN { return ret; }
    }
}

unsafe fn xchk_iscan_finish_batch(iscan: *mut xchk_iscan) {
    mutex_lock(&mut (*iscan).lock);
    if (*iscan).__batch_ino != NULLFSINO {
        let highest = (*iscan).__batch_ino + xfs_highbit64((*iscan).__skipped_inomask);
        (*iscan).__visited_ino = max((*iscan).__visited_ino, highest); trace_xchk_iscan_skip(iscan);
    }
    (*iscan).__batch_ino = NULLFSINO; (*iscan).__skipped_inomask = 0; mutex_unlock(&mut (*iscan).lock);
}

unsafe fn xchk_iscan_advance(iscan: *mut xchk_iscan, pagp: *mut *mut xfs_perag, agi_bpp: *mut *mut xfs_buf,
        allocmaskp: *mut xfs_inofree_t, nrp: *mut u8) -> i32 {
    let sc = (*iscan).sc; let mp = (*sc).mp;
    loop {
        if xchk_iscan_aborted(iscan) { return -ECANCELED; }
        let agno = XFS_INO_TO_AGNO(mp, (*iscan).cursor_ino); let pag = xfs_perag_get(mp, agno); if pag.is_null() { return -ECANCELED; }
        let mut bp = core::ptr::null_mut(); let ret = xchk_iscan_read_agi(iscan, pag, &mut bp); if ret != 0 { xfs_perag_put(pag); return ret; }
        let mut agino = XFS_INO_TO_AGINO(mp, (*iscan).cursor_ino);
        let ret = xchk_iscan_find_next(iscan, bp, pag, allocmaskp, &mut agino, nrp); if ret != 0 { xfs_trans_brelse((*sc).tp, bp); xfs_perag_put(pag); return ret; }
        if agino != NULLAGINO { xchk_iscan_move_cursor(iscan, agno, agino); *agi_bpp = bp; *pagp = pag; return 1; }
        let next = (agno + 1) % (*mp).m_sb.sb_agcount; xchk_iscan_move_cursor(iscan, next, 0);
        xfs_trans_brelse((*sc).tp, bp); xfs_perag_put(pag); trace_xchk_iscan_advance_ag(iscan);
        if (*iscan).cursor_ino == (*iscan).scan_start_ino { xchk_iscan_finish(iscan); return 0; }
    }
}

unsafe fn xchk_iscan_iget(iscan: *mut xchk_iscan, pag: *mut xfs_perag, bp: *mut xfs_buf,
        mut allocmask: xfs_inofree_t, nr: u8) -> i32 {
    let sc = (*iscan).sc; let mut ino = (*iscan).cursor_ino; let mut idx = 0;
    let mut error = xfs_iget((*sc).mp, (*sc).tp, ino, ISCAN_IGET_FLAGS, 0, &mut (*iscan).__inodes[0]); trace_xchk_iscan_iget(iscan, error);
    if error != 0 { xfs_trans_brelse((*sc).tp, bp); xfs_perag_put(pag); return xchk_iscan_iget_retry(iscan, error == -ENOENT || error == -EAGAIN); }
    idx += 1; ino += 1; allocmask >>= 1; mutex_lock(&mut (*iscan).lock); (*iscan).__batch_ino = ino - 1; (*iscan).__skipped_inomask = 0; mutex_unlock(&mut (*iscan).lock);
    for i in 1..nr { if allocmask & 1 == 0 { mutex_lock(&mut (*iscan).lock); (*iscan).cursor_ino = ino; (*iscan).__skipped_inomask |= 1u64 << i; mutex_unlock(&mut (*iscan).lock); } else { error = xfs_iget((*sc).mp, (*sc).tp, ino, ISCAN_IGET_FLAGS, 0, &mut (*iscan).__inodes[idx]); if error != 0 { break; } mutex_lock(&mut (*iscan).lock); (*iscan).cursor_ino = ino; mutex_unlock(&mut (*iscan).lock); idx += 1; } ino += 1; allocmask >>= 1; }
    trace_xchk_iscan_iget_batch((*sc).mp, iscan, nr, idx); xfs_trans_brelse((*sc).tp, bp); xfs_perag_put(pag); idx as i32
}

unsafe fn xchk_iscan_iget_retry(iscan: *mut xchk_iscan, wait: bool) -> i32 { if (*iscan).iget_timeout == 0 || time_is_before_jiffies((*iscan).__iget_deadline) { return -EBUSY; } if wait && (schedule_timeout_killable(msecs_to_jiffies((*iscan).iget_retry_delay)) != 0 || xchk_iscan_aborted(iscan)) { return -ECANCELED; } (*iscan).cursor_ino -= 1; -EAGAIN }

pub unsafe fn xchk_iscan_start(sc: *mut xfs_scrub, timeout: u32, delay: u32, iscan: *mut xchk_iscan) { let start = xchk_iscan_rotor((*sc).mp); (*iscan).__batch_ino = NULLFSINO; (*iscan).__skipped_inomask = 0; (*iscan).sc = sc; clear_bit(XCHK_ISCAN_OPSTATE_ABORTED, &mut (*iscan).__opstate); (*iscan).iget_timeout = timeout; (*iscan).iget_retry_delay = delay; (*iscan).__visited_ino = start; (*iscan).cursor_ino = start; (*iscan).scan_start_ino = start; mutex_init(&mut (*iscan).lock); memset((*iscan).__inodes.as_mut_ptr(), 0, core::mem::size_of_val(&(*iscan).__inodes)); trace_xchk_iscan_start(iscan, start); }
unsafe fn xchk_iscan_rotor(mp: *mut xfs_mount) -> xfs_ino_t { static mut AGI_ROTOR: atomic_t = atomic_t::default(); let r = atomic_inc_return(&mut AGI_ROTOR) - 1; XFS_AGINO_TO_INO(mp, (*mp).m_sb.sb_agcount - ((r % (*mp).m_sb.sb_agcount) + 1), 0) }
pub unsafe fn xchk_iscan_mark_visited(iscan: *mut xchk_iscan, ip: *mut xfs_inode) { mutex_lock(&mut (*iscan).lock); (*iscan).__visited_ino = I_INO(ip); trace_xchk_iscan_visit(iscan); mutex_unlock(&mut (*iscan).lock); }
unsafe fn xchk_iscan_skipped(iscan: *const xchk_iscan, ino: xfs_ino_t) -> bool { if (*iscan).__batch_ino == NULLFSINO || ino < (*iscan).__batch_ino || ino >= (*iscan).__batch_ino + XFS_INODES_PER_CHUNK { return false; } ((*iscan).__skipped_inomask & (1u64 << (ino - (*iscan).__batch_ino))) != 0 }
pub unsafe fn xchk_iscan_want_live_update(iscan: *mut xchk_iscan, ino: xfs_ino_t) -> bool { if xchk_iscan_aborted(iscan) { return false; } mutex_lock(&mut (*iscan).lock); let mut ret = false; if (*iscan).__visited_ino == NULLFSINO { ret = true; } else if (*iscan).scan_start_ino != (*iscan).__visited_ino { if xchk_iscan_skipped(iscan, ino) { ret = true; } else if (*iscan).scan_start_ino <= (*iscan).__visited_ino { ret = ino >= (*iscan).scan_start_ino && ino <= (*iscan).__visited_ino; } else { ret = ino >= (*iscan).scan_start_ino || ino <= (*iscan).__visited_ino; } } trace_xchk_iscan_want_live_update(iscan, ino); mutex_unlock(&mut (*iscan).lock); ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
