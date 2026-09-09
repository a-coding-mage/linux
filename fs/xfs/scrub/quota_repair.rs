// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Dependencies supplied by the surrounding XFS implementation are intentionally external.

#[repr(C)]
pub struct xrep_quota_info {
    pub sc: *mut xfs_scrub,
    pub need_quotacheck: bool,
}

unsafe fn xrep_quota_item_fill_bmap_hole(
    sc: *mut xfs_scrub,
    dq: *mut xfs_dquot,
    irec: *mut xfs_bmbt_irec,
) -> i32 {
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let mp = (*sc).mp;
    let mut nmaps: i32 = 1;
    let mut error: i32;

    xfs_trans_ijoin((*sc).tp, (*sc).ip, 0);
    error = xfs_trans_reserve_more((*sc).tp, XFS_QM_DQALLOC_SPACE_RES(mp), 0);
    if error != 0 { return error; }
    error = xfs_bmapi_write((*sc).tp, (*sc).ip, (*dq).q_fileoffset,
        XFS_DQUOT_CLUSTER_SIZE_FSB, XFS_BMAPI_METADATA, 0, irec, &mut nmaps);
    if error != 0 { return error; }
    (*dq).q_blkno = XFS_FSB_TO_DADDR(mp, (*irec).br_startblock);
    trace_xrep_dquot_item_fill_bmap_hole((*sc).mp, (*dq).q_type, (*dq).q_id);
    error = xfs_trans_get_buf((*sc).tp, (*mp).m_ddev_targp, (*dq).q_blkno,
        (*mp).m_quotainfo.qi_dqchunklen, 0, &mut bp);
    if error != 0 { return error; }
    (*bp).b_ops = &xfs_dquot_buf_ops;
    xfs_qm_init_dquot_blk((*sc).tp, (*dq).q_id, (*dq).q_type, bp);
    xfs_buf_set_ref(bp, XFS_DQUOT_REF);
    error = xrep_defer_finish(sc);
    if error != 0 { return error; }
    xfs_trans_roll(&mut (*sc).tp)
}

unsafe fn xrep_quota_item_bmap(sc: *mut xfs_scrub, dq: *mut xfs_dquot, dirty: *mut bool) -> i32 {
    let mut irec = core::mem::zeroed::<xfs_bmbt_irec>();
    let mp = (*sc).mp;
    let qi = (*mp).m_quotainfo;
    let offset: xfs_fileoff_t = (*dq).q_id / (*qi).qi_dqperchunk;
    let mut nmaps: i32 = 1;
    let mut error: i32;
    if !xfs_verify_fileoff(mp, offset) { ASSERT(xfs_verify_fileoff(mp, offset)); return -EFSCORRUPTED; }
    (*dq).q_fileoffset = offset;
    error = xfs_bmapi_read((*sc).ip, offset, 1, &mut irec, &mut nmaps, 0);
    if error != 0 { return error; }
    if nmaps < 1 || !xfs_bmap_is_real_extent(&irec) {
        error = xrep_quota_item_fill_bmap_hole(sc, dq, &mut irec);
        if error != 0 { return error; }
    } else if irec.br_state != XFS_EXT_NORM {
        ASSERT(irec.br_state == XFS_EXT_NORM); return -EFSCORRUPTED;
    } else if (*dq).q_blkno != XFS_FSB_TO_DADDR(mp, irec.br_startblock) {
        (*dq).q_blkno = XFS_FSB_TO_DADDR(mp, irec.br_startblock);
    }
    *dirty = true;
    0
}

unsafe fn xrep_quota_item_timer(_sc: *mut xfs_scrub, res: *const xfs_dquot_res, dirty: *mut bool) {
    if (((*res).softlimit != 0 && (*res).count > (*res).softlimit) ||
        ((*res).hardlimit != 0 && (*res).count > (*res).hardlimit)) {
        if (*res).timer == 0 { *dirty = true; }
    } else if (*res).timer != 0 { *dirty = true; }
}

unsafe fn xrep_quota_item(rqi: *mut xrep_quota_info, dq: *mut xfs_dquot) -> i32 {
    let sc = (*rqi).sc;
    let mp = (*sc).mp;
    let mut fs_icount: xfs_ino_t;
    let mut dirty = false;
    let mut error = 0;
    if xchk_should_terminate(sc, &mut error) { return error; }
    xchk_ilock(sc, XFS_ILOCK_EXCL); mutex_lock(&mut (*dq).q_qlock);
    error = xrep_quota_item_bmap(sc, dq, &mut dirty); xchk_iunlock(sc, XFS_ILOCK_EXCL);
    if error != 0 { mutex_unlock(&mut (*dq).q_qlock); return error; }
    if (*dq).q_blk.softlimit > (*dq).q_blk.hardlimit { (*dq).q_blk.softlimit = (*dq).q_blk.hardlimit; dirty = true; }
    if (*dq).q_ino.softlimit > (*dq).q_ino.hardlimit { (*dq).q_ino.softlimit = (*dq).q_ino.hardlimit; dirty = true; }
    if (*dq).q_rtb.softlimit > (*dq).q_rtb.hardlimit { (*dq).q_rtb.softlimit = (*dq).q_rtb.hardlimit; dirty = true; }
    if !xfs_has_reflink(mp) && (*dq).q_blk.count > (*mp).m_sb.sb_dblocks {
        (*dq).q_blk.reserved -= (*dq).q_blk.count; (*dq).q_blk.reserved += (*mp).m_sb.sb_dblocks;
        (*dq).q_blk.count = (*mp).m_sb.sb_dblocks; (*rqi).need_quotacheck = true; dirty = true;
    }
    fs_icount = percpu_counter_sum(&mut (*mp).m_icount);
    if (*dq).q_ino.count > fs_icount { (*dq).q_ino.reserved -= (*dq).q_ino.count; (*dq).q_ino.reserved += fs_icount; (*dq).q_ino.count = fs_icount; (*rqi).need_quotacheck = true; dirty = true; }
    if !xfs_has_reflink(mp) && (*dq).q_rtb.count > (*mp).m_sb.sb_rblocks { (*dq).q_rtb.reserved -= (*dq).q_rtb.count; (*dq).q_rtb.reserved += (*mp).m_sb.sb_rblocks; (*dq).q_rtb.count = (*mp).m_sb.sb_rblocks; (*rqi).need_quotacheck = true; dirty = true; }
    xrep_quota_item_timer(sc, &(*dq).q_blk, &mut dirty); xrep_quota_item_timer(sc, &(*dq).q_ino, &mut dirty); xrep_quota_item_timer(sc, &(*dq).q_rtb, &mut dirty);
    if !dirty { mutex_unlock(&mut (*dq).q_qlock); return error; }
    trace_xrep_dquot_item((*sc).mp, (*dq).q_type, (*dq).q_id); (*dq).q_flags |= XFS_DQFLAG_DIRTY;
    xfs_trans_dqjoin((*sc).tp, dq); if (*dq).q_id != 0 { xfs_qm_adjust_dqlimits(dq); xfs_qm_adjust_dqtimers(dq); }
    xfs_trans_log_dquot((*sc).tp, dq); xfs_trans_roll(&mut (*sc).tp)
}

unsafe fn xrep_quota_fix_timer(mp: *mut xfs_mount, ddq: *const xfs_disk_dquot, softlimit: __be64, countnow: __be64, timer: *mut __be32, timelimit: time64_t) {
    let soft = be64_to_cpu(softlimit); let count = be64_to_cpu(countnow);
    if soft == 0 || count <= soft || *timer != 0 { return; }
    let new_timer = xfs_dquot_set_timeout(mp, ktime_get_real_seconds() + timelimit);
    let t = if (*ddq).d_type & XFS_DQTYPE_BIGTIME != 0 { xfs_dq_unix_to_bigtime(new_timer) } else { new_timer };
    *timer = cpu_to_be32(t);
}

unsafe fn xrep_quota_block(sc: *mut xfs_scrub, daddr: xfs_daddr_t, dqtype: xfs_dqtype_t, id: xfs_dqid_t) -> i32 {
    let qi = (*(*sc).mp).m_quotainfo; let defq = xfs_get_defquota(qi, dqtype);
    let mut bp: *mut xfs_buf = core::ptr::null_mut(); let mut buftype = 0; let mut error;
    error = xfs_trans_read_buf((*sc).mp, (*sc).tp, (*(*sc).mp).m_ddev_targp, daddr, (*qi).qi_dqchunklen, 0, &mut bp, &xfs_dquot_buf_ops);
    match error { -EFSBADCRC | -EFSCORRUPTED => { error = xfs_trans_read_buf((*sc).mp, (*sc).tp, (*(*sc).mp).m_ddev_targp, daddr, (*qi).qi_dqchunklen, 0, &mut bp, core::ptr::null()); if error != 0 { return error; } }, 0 => { let ddq = &(*((*bp).b_addr as *mut xfs_dqblk)).dd_diskdq; if ((*ddq).d_type & XFS_DQTYPE_REC_MASK) != dqtype || id == be32_to_cpu((*ddq).d_id) { xfs_trans_brelse((*sc).tp, bp); return 0; } }, _ => return error }
    let mut dqblk = (*bp).b_addr as *mut xfs_dqblk; (*bp).b_ops = &xfs_dquot_buf_ops;
    for i in 0..(*qi).qi_dqperchunk { let ddq = &mut (*dqblk).dd_diskdq; trace_xrep_disk_dquot((*sc).mp, dqtype, id + i); (*ddq).d_magic = cpu_to_be16(XFS_DQUOT_MAGIC); (*ddq).d_version = XFS_DQUOT_VERSION; (*ddq).d_type = dqtype; (*ddq).d_id = cpu_to_be32(id + i); if xfs_has_bigtime((*sc).mp) && (*ddq).d_id != 0 { (*ddq).d_type |= XFS_DQTYPE_BIGTIME; } xrep_quota_fix_timer((*sc).mp, ddq, (*ddq).d_blk_softlimit, (*ddq).d_bcount, &mut (*ddq).d_btimer, (*defq).blk.time); xrep_quota_fix_timer((*sc).mp, ddq, (*ddq).d_ino_softlimit, (*ddq).d_icount, &mut (*ddq).d_itimer, (*defq).ino.time); xrep_quota_fix_timer((*sc).mp, ddq, (*ddq).d_rtb_softlimit, (*ddq).d_rtbcount, &mut (*ddq).d_rtbtimer, (*defq).rtb.time); uuid_copy(&mut (*dqblk).dd_uuid, &(*(*sc).mp).m_sb.sb_meta_uuid); xfs_update_cksum(dqblk as *mut i8, core::mem::size_of::<xfs_dqblk>(), XFS_DQUOT_CRC_OFF); (*dqblk).dd_lsn = 0; dqblk = dqblk.add(1); }
    buftype = match dqtype { XFS_DQTYPE_USER => XFS_BLFT_UDQUOT_BUF, XFS_DQTYPE_GROUP => XFS_BLFT_GDQUOT_BUF, XFS_DQTYPE_PROJ => XFS_BLFT_PDQUOT_BUF, _ => buftype };
    xfs_trans_buf_set_type((*sc).tp, bp, buftype); xfs_trans_log_buf((*sc).tp, bp, 0, BBTOB((*bp).b_length) - 1); xrep_roll_trans(sc)
}

unsafe fn xrep_quota_data_fork(sc: *mut xfs_scrub, dqtype: xfs_dqtype_t) -> i32 {
    let mut irec = core::mem::zeroed::<xfs_bmbt_irec>(); let mut icur = core::mem::zeroed::<xfs_iext_cursor>();
    let qi = (*(*sc).mp).m_quotainfo; let mut ifp: *mut xfs_ifork; let mut max_dqid_off;
    let mut off: xfs_fileoff_t; let mut fsbno: xfs_fsblock_t; let mut truncate = false; let mut joined = false; let mut error = xrep_metadata_inode_forks(sc); if error != 0 { return error; }
    max_dqid_off = XFS_DQ_ID_MAX / (*qi).qi_dqperchunk; ifp = xfs_ifork_ptr((*sc).ip, XFS_DATA_FORK);
    for_each_xfs_iext!(ifp, &mut icur, &mut irec, {
        if isnullstartblock(irec.br_startblock) { error = -EFSCORRUPTED; break; }
        if irec.br_startoff > max_dqid_off || irec.br_startoff + irec.br_blockcount - 1 > max_dqid_off { truncate = true; break; }
        if irec.br_state == XFS_EXT_UNWRITTEN { let mut nrec = core::mem::zeroed(); let mut nmap = 1; if !joined { xfs_trans_ijoin((*sc).tp, (*sc).ip, 0); joined = true; } error = xfs_bmapi_write((*sc).tp, (*sc).ip, irec.br_startoff, irec.br_blockcount, XFS_BMAPI_CONVERT, 0, &mut nrec, &mut nmap); if error != 0 { break; } ASSERT(nrec.br_startoff == irec.br_startoff); ASSERT(nrec.br_blockcount == irec.br_blockcount); error = xfs_defer_finish(&mut (*sc).tp); if error != 0 { break; } }
    }); if error != 0 { return error; }
    if !joined { xfs_trans_ijoin((*sc).tp, (*sc).ip, 0); joined = true; }
    if truncate { error = xfs_bunmapi_range(&mut (*sc).tp, (*sc).ip, 0, max_dqid_off * (*(*sc).mp).m_sb.sb_blocksize, XFS_MAX_FILEOFF); if error != 0 { return error; } error = xfs_reflink_cancel_cow_blocks((*sc).ip, &mut (*sc).tp, 0, XFS_MAX_FILEOFF, true); if error != 0 { return error; } (*(*sc).ip).i_diflags2 &= !XFS_DIFLAG2_REFLINK; xfs_trans_log_inode((*sc).tp, (*sc).ip, XFS_ILOG_CORE); }
    for_each_xfs_iext!(ifp, &mut icur, &mut irec, { for fsbno = irec.br_startblock, off = irec.br_startoff; fsbno < irec.br_startblock + irec.br_blockcount; fsbno += XFS_DQUOT_CLUSTER_SIZE_FSB, off += XFS_DQUOT_CLUSTER_SIZE_FSB { error = xrep_quota_block(sc, XFS_FSB_TO_DADDR((*sc).mp, fsbno), dqtype, off * (*qi).qi_dqperchunk); if error != 0 { break; } } }); error
}

unsafe fn xrep_quota_problems(sc: *mut xfs_scrub, dqtype: xfs_dqtype_t) -> i32 {
    let mut cursor = core::mem::zeroed::<xchk_dqiter>(); let mut rqi = xrep_quota_info { sc, need_quotacheck: false }; let mut dq = core::ptr::null_mut(); let mut error;
    xchk_dqiter_init(&mut cursor, sc, dqtype); loop { error = xchk_dquot_iter(&mut cursor, &mut dq); if error != 1 { break; } error = xrep_quota_item(&mut rqi, dq); xfs_qm_dqrele(dq); if error != 0 { break; } }
    if error != 0 { return error; } if rqi.need_quotacheck { xrep_force_quotacheck(sc, dqtype); } 0
}

pub unsafe fn xrep_quota(sc: *mut xfs_scrub) -> i32 {
    let dqtype = xchk_quota_to_dqtype(sc);
    if (*sc).ilock_flags & XFS_ILOCK_EXCL == 0 { xchk_ilock(sc, XFS_ILOCK_EXCL); }
    let mut error = xrep_quota_data_fork(sc, dqtype); if error != 0 { return error; }
    error = xrep_defer_finish(sc); if error != 0 { return error; }
    error = xfs_trans_roll(&mut (*sc).tp); if error != 0 { return error; }
    xchk_iunlock(sc, (*sc).ilock_flags);
    error = xrep_quota_problems(sc, dqtype); if error != 0 { return error; }
    xrep_trans_commit(sc)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
