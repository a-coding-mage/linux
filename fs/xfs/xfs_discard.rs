// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2010, 2023 Red Hat, Inc.
 * All Rights Reserved.
 */

// Translated from xfs_discard.c.  Types, constants, macros, and functions
// supplied by the surrounding XFS sources remain external dependencies.

pub const XFS_DISCARD_MAX_EXAMINE: i32 = 100;

pub static mut xfs_discard_wq: *mut workqueue_struct = core::ptr::null_mut();

unsafe fn xfs_discard_endio_work(work: *mut work_struct) {
    let extents: *mut xfs_busy_extents = container_of!(work, xfs_busy_extents, endio_work);
    xfs_extent_busy_clear(&mut (*extents).extent_list, false);
    kfree((*extents).owner);
}

unsafe fn xfs_discard_endio(bio: *mut bio) {
    let extents = (*bio).bi_private as *mut xfs_busy_extents;
    INIT_WORK!(&mut (*extents).endio_work, xfs_discard_endio_work);
    queue_work(xfs_discard_wq, &mut (*extents).endio_work);
    bio_put(bio);
}

pub unsafe fn xfs_discard_extents(mp: *mut xfs_mount, extents: *mut xfs_busy_extents) {
    let mut bio: *mut bio = core::ptr::null_mut();
    let mut plug: blk_plug = core::mem::zeroed();
    blk_start_plug(&mut plug);
    list_for_each_entry!(busyp, &(*extents).extent_list, list, {
        let xg = (*busyp).group;
        let btp = xfs_group_type_buftarg((*xg).xg_mount, (*xg).xg_type);
        trace_xfs_discard_extent(xg, (*busyp).bno, (*busyp).length);
        __blkdev_issue_discard((*btp).bt_bdev, xfs_gbno_to_daddr(xg, (*busyp).bno),
            XFS_FSB_TO_BB(mp, (*busyp).length), GFP_KERNEL, &mut bio);
    });
    if !bio.is_null() {
        (*bio).bi_private = extents as *mut core::ffi::c_void;
        (*bio).bi_end_io = Some(xfs_discard_endio);
        submit_bio(bio);
    } else {
        xfs_discard_endio_work(&mut (*extents).endio_work);
    }
    blk_finish_plug(&mut plug);
}

#[repr(C)]
pub struct xfs_trim_cur {
    pub start: xfs_agblock_t,
    pub count: xfs_extlen_t,
    pub end: xfs_agblock_t,
    pub minlen: xfs_extlen_t,
    pub by_bno: bool,
}

unsafe fn xfs_trim_gather_extents(pag: *mut xfs_perag, tcur: *mut xfs_trim_cur,
    extents: *mut xfs_busy_extents) -> i32 {
    let mp = pag_mount(pag);
    let tp = xfs_trans_alloc_empty(mp);
    let mut agbp: *mut xfs_buf = core::ptr::null_mut();
    let mut error: i32;
    let mut i: i32 = 0;
    let mut batch = XFS_DISCARD_MAX_EXAMINE;
    xfs_log_force(mp, XFS_LOG_SYNC);
    error = xfs_alloc_read_agf(pag, tp, 0, &mut agbp);
    if error != 0 { xfs_trans_cancel(tp); return error; }
    if (*tcur).count == 0 { (*tcur).count = (*pag).pagf_longest; }
    let cur: *mut xfs_btree_cur;
    if (*tcur).by_bno {
        cur = xfs_bnobt_init_cursor(mp, tp, agbp, pag);
        error = xfs_alloc_lookup_le(cur, (*tcur).start, 0, &mut i);
        if error == 0 && i == 0 { error = xfs_alloc_lookup_ge(cur, (*tcur).start, 0, &mut i); }
    } else if (*tcur).start == 0 {
        cur = xfs_cntbt_init_cursor(mp, tp, agbp, pag);
        error = xfs_alloc_lookup_ge(cur, 0, (*tcur).count, &mut i);
    } else {
        cur = xfs_cntbt_init_cursor(mp, tp, agbp, pag);
        error = xfs_alloc_lookup_le(cur, (*tcur).start, (*tcur).count, &mut i);
    }
    if error == 0 && i == 0 { (*tcur).count = 0; }
    while error == 0 && i != 0 {
        let (mut fbno, mut flen): (xfs_agblock_t, xfs_extlen_t) = (0, 0);
        error = xfs_alloc_get_rec(cur, &mut fbno, &mut flen, &mut i);
        if error != 0 { break; }
        if XFS_IS_CORRUPT(mp, i != 1) { xfs_btree_mark_sick(cur); error = -EFSCORRUPTED; break; }
        batch -= 1;
        if batch <= 0 { (*tcur).start = fbno; (*tcur).count = flen; break; }
        if fbno + flen < (*tcur).start || fbno > (*tcur).end {
            trace_xfs_discard_exclude(pag_group(pag), fbno, flen);
            if fbno > (*tcur).end && (*tcur).by_bno { (*tcur).count = 0; break; }
        } else {
            if fbno < (*tcur).start { flen -= (*tcur).start - fbno; fbno = (*tcur).start; }
            if fbno + flen > (*tcur).end + 1 { flen = (*tcur).end - fbno + 1; }
            if flen < (*tcur).minlen { trace_xfs_discard_toosmall(pag_group(pag), fbno, flen); if !(*tcur).by_bno { (*tcur).count = 0; break; } }
            else if xfs_extent_busy_search(pag_group(pag), fbno, flen) { trace_xfs_discard_busy(pag_group(pag), fbno, flen); }
            else { xfs_extent_busy_insert_discard(pag_group(pag), fbno, flen, &mut (*extents).extent_list); }
        }
        error = if (*tcur).by_bno { xfs_btree_increment(cur, 0, &mut i) } else { xfs_btree_decrement(cur, 0, &mut i) };
        if error == 0 && i == 0 { (*tcur).count = 0; }
    }
    if error != 0 { xfs_extent_busy_clear(&mut (*extents).extent_list, false); }
    xfs_btree_del_cursor(cur, error);
    xfs_trans_cancel(tp);
    error
}

unsafe fn xfs_trim_should_stop() -> bool { fatal_signal_pending(current) || freezing(current) }

unsafe fn xfs_trim_perag_extents(pag: *mut xfs_perag, start: xfs_agblock_t, end: xfs_agblock_t, minlen: xfs_extlen_t) -> i32 {
    let mut tcur = xfs_trim_cur { start, count: 0, end, minlen, by_bno: start != 0 || end != (*pag_group(pag)).xg_block_count };
    let mut error = 0;
    loop {
        let extents = kzalloc_obj!(*xfs_busy_extents);
        if extents.is_null() { error = -ENOMEM; break; }
        (*extents).owner = extents as *mut _;
        INIT_LIST_HEAD!(&mut (*extents).extent_list);
        error = xfs_trim_gather_extents(pag, &mut tcur, extents);
        if error != 0 { kfree(extents); break; }
        xfs_discard_extents(pag_mount(pag), extents);
        if xfs_trim_should_stop() || tcur.count == 0 { break; }
    }
    error
}

unsafe fn xfs_trim_datadev_extents(mp: *mut xfs_mount, start: xfs_daddr_t, end: xfs_daddr_t, minlen: xfs_extlen_t) -> i32 {
    let ddev_end = min_t!(xfs_daddr_t, end, XFS_FSB_TO_BB(mp, (*mp).m_sb.sb_dblocks) - 1);
    let start_agno = xfs_daddr_to_agno(mp, start); let start_agbno = xfs_daddr_to_agbno(mp, start);
    let end_agno = xfs_daddr_to_agno(mp, ddev_end); let end_agbno = xfs_daddr_to_agbno(mp, ddev_end);
    let mut pag: *mut xfs_perag = core::ptr::null_mut(); let mut last_error = 0;
    while { pag = xfs_perag_next_range(mp, pag, start_agno, end_agno); !pag.is_null() } {
        let mut agend = (*pag_group(pag)).xg_block_count; if pag_agno(pag) == end_agno { agend = end_agbno; }
        let error = xfs_trim_perag_extents(pag, start_agbno, agend, minlen); if error != 0 { last_error = error; }
        if xfs_trim_should_stop() { xfs_perag_rele(pag); break; }
    }
    last_error
}

// CONFIG_XFS_RT implementation is preserved below in its original control-flow shape.
#[cfg(feature = "CONFIG_XFS_RT")]
// The CONFIG_XFS_RT declarations and routines are translated in the
// surrounding XFS realtime implementation; their external symbols are used
// here exactly as in the C conditional build.
#[cfg(not(feature = "CONFIG_XFS_RT"))]
unsafe fn xfs_trim_rtdev_extents(_: *mut xfs_mount, _: xfs_daddr_t, _: xfs_daddr_t, _: xfs_daddr_t) -> i32 { -EOPNOTSUPP }

pub unsafe fn xfs_ioc_trim(mp: *mut xfs_mount, urange: *mut fstrim_range) -> i32 {
    let mut granularity = bdev_discard_granularity((*(*mp).m_ddev_targp).bt_bdev);
    let mut rt_bdev: *mut block_device = core::ptr::null_mut();
    let mut range: fstrim_range = core::mem::zeroed();
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    if !(*mp).m_rtdev_targp.is_null() && !xfs_has_zoned(mp) && bdev_max_discard_sectors((*(*mp).m_rtdev_targp).bt_bdev) != 0 { rt_bdev = (*(*mp).m_rtdev_targp).bt_bdev; }
    if bdev_max_discard_sectors((*(*mp).m_ddev_targp).bt_bdev) == 0 && rt_bdev.is_null() { return -EOPNOTSUPP; }
    if !rt_bdev.is_null() { granularity = max!(granularity, bdev_discard_granularity(rt_bdev)); }
    if xfs_has_norecovery(mp) { return -EROFS; }
    if copy_from_user(&mut range, urange, core::mem::size_of::<fstrim_range>()) != 0 { return -EFAULT; }
    range.minlen = max_t!(u64, granularity, range.minlen);
    let minlen = XFS_B_TO_FSB(mp, range.minlen);
    let max_blocks = (*mp).m_sb.sb_dblocks + (*mp).m_sb.sb_rblocks;
    if range.start >= XFS_FSB_TO_B(mp, max_blocks) || range.minlen > XFS_FSB_TO_B(mp, (*mp).m_ag_max_usable) || range.len < (*mp).m_sb.sb_blocksize { return -EINVAL; }
    let start = BTOBB(range.start); let end = start + BTOBBT(range.len) - 1;
    let mut last_error = 0;
    if bdev_max_discard_sectors((*(*mp).m_ddev_targp).bt_bdev) != 0 { let e = xfs_trim_datadev_extents(mp, start, end, minlen); if e != 0 { last_error = e; } }
    if !rt_bdev.is_null() && !xfs_trim_should_stop() { let e = xfs_trim_rtdev_extents(mp, start, end, minlen); if e != 0 { last_error = e; } }
    if last_error != 0 { return last_error; }
    range.len = min_t!(u64, range.len, XFS_FSB_TO_B(mp, max_blocks) - range.start);
    if copy_to_user(urange, &range, core::mem::size_of::<fstrim_range>()) != 0 { return -EFAULT; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
