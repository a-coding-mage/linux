// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* External XFS declarations are supplied by the surrounding translation unit. */

/* Convert a scrub type code to a DQ flag, or return 0 if error. */
unsafe fn xchk_quota_to_dqtype(sc: *mut xfs_scrub) -> xfs_dqtype_t {
    match (*(*sc).sm).sm_type {
        XFS_SCRUB_TYPE_UQUOTA => XFS_DQTYPE_USER,
        XFS_SCRUB_TYPE_GQUOTA => XFS_DQTYPE_GROUP,
        XFS_SCRUB_TYPE_PQUOTA => XFS_DQTYPE_PROJ,
        _ => 0,
    }
}

/* Set us up to scrub a quota. */
unsafe fn xchk_setup_quota(sc: *mut xfs_scrub) -> i32 {
    let dqtype: xfs_dqtype_t;
    let mut error: i32;

    if !XFS_IS_QUOTA_ON((*sc).mp) {
        return -ENOENT;
    }

    dqtype = xchk_quota_to_dqtype(sc);
    if dqtype == 0 {
        return -EINVAL;
    }

    if !xfs_this_quota_on((*sc).mp, dqtype) {
        return -ENOENT;
    }

    if xchk_need_intent_drain(sc) {
        xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN);
    }

    error = xchk_setup_fs(sc);
    if error != 0 {
        return error;
    }

    error = xchk_install_live_inode(sc, xfs_quota_inode((*sc).mp, dqtype));
    if error != 0 {
        return error;
    }

    xchk_ilock(sc, XFS_ILOCK_EXCL);
    0
}

/* Quotas. */
#[repr(C)]
struct xchk_quota_info {
    sc: *mut xfs_scrub,
    last_id: xfs_dqid_t,
}

/* There's a written block backing this dquot, right? */
unsafe fn xchk_quota_item_bmap(
    sc: *mut xfs_scrub,
    dq: *mut xfs_dquot,
    offset: xfs_fileoff_t,
) -> i32 {
    let mut irec: xfs_bmbt_irec = core::mem::zeroed();
    let mp = (*sc).mp;
    let mut nmaps: i32 = 1;
    let error: i32;

    if !xfs_verify_fileoff(mp, offset) {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
        return 0;
    }
    if (*dq).q_fileoffset != offset {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
        return 0;
    }
    error = xfs_bmapi_read((*sc).ip, offset, 1, &mut irec, &mut nmaps, 0);
    if error != 0 {
        return error;
    }
    if nmaps != 1 {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
        return 0;
    }
    if !xfs_verify_fsbno(mp, irec.br_startblock) {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
    }
    if XFS_FSB_TO_DADDR(mp, irec.br_startblock) != (*dq).q_blkno {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
    }
    if !xfs_bmap_is_written_extent(&irec) {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
    }
    0
}

/* Complain if a quota timer is incorrectly set. */
#[inline]
unsafe fn xchk_quota_item_timer(sc: *mut xfs_scrub, offset: xfs_fileoff_t, res: *const xfs_dquot_res) {
    if (((*res).softlimit != 0 && (*res).count > (*res).softlimit) ||
        ((*res).hardlimit != 0 && (*res).count > (*res).hardlimit)) {
        if (*res).timer == 0 {
            xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
        }
    } else if (*res).timer != 0 {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
    }
}

/* Scrub the fields in an individual quota item. */
unsafe fn xchk_quota_item(sqi: *mut xchk_quota_info, dq: *mut xfs_dquot) -> i32 {
    let sc = (*sqi).sc;
    let mp = (*sc).mp;
    let qi = (*mp).m_quotainfo;
    let offset: xfs_fileoff_t;
    let fs_icount: xfs_ino_t;
    let mut error: i32 = 0;

    if xchk_should_terminate(sc, &mut error) { return error; }
    xchk_ilock(sc, XFS_ILOCK_SHARED);
    mutex_lock(&mut (*dq).q_qlock);
    offset = (*dq).q_id / (*qi).qi_dqperchunk;
    if (*dq).q_id != 0 && (*dq).q_id <= (*sqi).last_id {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
    }
    (*sqi).last_id = (*dq).q_id;
    error = xchk_quota_item_bmap(sc, dq, offset);
    xchk_iunlock(sc, XFS_ILOCK_SHARED);
    if !xchk_fblock_process_error(sc, XFS_DATA_FORK, offset, &mut error) {
        mutex_unlock(&mut (*dq).q_qlock);
        return error;
    }
    if (*dq).q_blk.hardlimit > (*mp).m_sb.sb_dblocks { xchk_fblock_set_warning(sc, XFS_DATA_FORK, offset); }
    if (*dq).q_blk.softlimit > (*dq).q_blk.hardlimit { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset); }
    if (*dq).q_ino.hardlimit > M_IGEO(mp).maxicount { xchk_fblock_set_warning(sc, XFS_DATA_FORK, offset); }
    if (*dq).q_ino.softlimit > (*dq).q_ino.hardlimit { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset); }
    if (*dq).q_rtb.hardlimit > (*mp).m_sb.sb_rblocks { xchk_fblock_set_warning(sc, XFS_DATA_FORK, offset); }
    if (*dq).q_rtb.softlimit > (*dq).q_rtb.hardlimit { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset); }
    fs_icount = percpu_counter_sum(&mut (*mp).m_icount);
    if xfs_has_reflink(mp) {
        if (*mp).m_sb.sb_dblocks < (*dq).q_blk.count { xchk_fblock_set_warning(sc, XFS_DATA_FORK, offset); }
        if (*mp).m_sb.sb_rblocks < (*dq).q_rtb.count { xchk_fblock_set_warning(sc, XFS_DATA_FORK, offset); }
    } else {
        if (*mp).m_sb.sb_dblocks < (*dq).q_blk.count { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset); }
        if (*mp).m_sb.sb_rblocks < (*dq).q_rtb.count { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset); }
    }
    if (*dq).q_ino.count > fs_icount { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset); }
    if (*dq).q_id != 0 {
        if (*dq).q_blk.hardlimit != 0 && (*dq).q_blk.count > (*dq).q_blk.hardlimit { xchk_fblock_set_warning(sc, XFS_DATA_FORK, offset); }
        if (*dq).q_ino.hardlimit != 0 && (*dq).q_ino.count > (*dq).q_ino.hardlimit { xchk_fblock_set_warning(sc, XFS_DATA_FORK, offset); }
        if (*dq).q_rtb.hardlimit != 0 && (*dq).q_rtb.count > (*dq).q_rtb.hardlimit { xchk_fblock_set_warning(sc, XFS_DATA_FORK, offset); }
        xchk_quota_item_timer(sc, offset, &(*dq).q_blk);
        xchk_quota_item_timer(sc, offset, &(*dq).q_ino);
        xchk_quota_item_timer(sc, offset, &(*dq).q_rtb);
    }
    mutex_unlock(&mut (*dq).q_qlock);
    if (*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return -ECANCELED; }
    0
}

/* Check the quota's data fork. */
unsafe fn xchk_quota_data_fork(sc: *mut xfs_scrub) -> i32 {
    let mut irec: xfs_bmbt_irec = core::mem::zeroed();
    let mut icur: xfs_iext_cursor = core::mem::zeroed();
    let qi = (*(*sc).mp).m_quotainfo;
    let ifp = xfs_ifork_ptr((*sc).ip, XFS_DATA_FORK);
    let max_dqid_off = XFS_DQ_ID_MAX / (*qi).qi_dqperchunk;
    let mut error = xchk_metadata_inode_forks(sc);
    if error != 0 || (*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return error; }
    /* for_each_xfs_iext(ifp, &icur, &irec) */
    while xfs_iext_next(ifp, &mut icur, &mut irec) {
        if xchk_should_terminate(sc, &mut error) { break; }
        if !xfs_bmap_is_written_extent(&irec) || irec.br_startoff > max_dqid_off ||
            irec.br_startoff + irec.br_blockcount - 1 > max_dqid_off {
            xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, irec.br_startoff);
            break;
        }
    }
    error
}

/* Scrub all of a quota type's items. */
unsafe fn xchk_quota(sc: *mut xfs_scrub) -> i32 {
    let mut cursor: xchk_dqiter = core::mem::zeroed();
    let mut sqi = xchk_quota_info { sc, last_id: 0 };
    let mp = (*sc).mp;
    let qi = (*mp).m_quotainfo;
    let mut dq: *mut xfs_dquot = core::ptr::null_mut();
    let dqtype = xchk_quota_to_dqtype(sc);
    let mut error = xchk_quota_data_fork(sc);
    if error != 0 || (*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return error; }
    xchk_iunlock(sc, (*sc).ilock_flags);
    xchk_dqiter_init(&mut cursor, sc, dqtype);
    while { error = xchk_dquot_iter(&mut cursor, &mut dq); error == 1 } {
        error = xchk_quota_item(&mut sqi, dq);
        xfs_qm_dqrele(dq);
        if error != 0 { break; }
    }
    if error == -ECANCELED { error = 0; }
    if !xchk_fblock_process_error(sc, XFS_DATA_FORK, sqi.last_id * (*qi).qi_dqperchunk, &mut error) { return error; }
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
