// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding XFS translation.

pub unsafe fn xfs_qm_scall_quotaoff(mp: *mut xfs_mount_t, mut flags: uint) -> c_int {
    /*
     * No file system can have quotas enabled on disk but not in core.
     * Note that quota utilities (like quotaoff) _expect_
     * errno == -EEXIST here.
     */
    if ((*mp).m_qflags & flags) == 0 {
        return -EEXIST;
    }

    /*
     * We do not support actually turning off quota accounting any more.
     * Just log a warning and ignore the accounting related flags.
     */
    if (flags & XFS_ALL_QUOTA_ACCT) != 0 {
        xfs_info(mp, "disabling of quota accounting not supported.");
    }

    mutex_lock(&mut (*(*mp).m_quotainfo).qi_quotaofflock);
    (*mp).m_qflags &= !(flags & XFS_ALL_QUOTA_ENFD);
    spin_lock(&mut (*mp).m_sb_lock);
    (*mp).m_sb.sb_qflags = (*mp).m_qflags;
    spin_unlock(&mut (*mp).m_sb_lock);
    mutex_unlock(&mut (*(*mp).m_quotainfo).qi_quotaofflock);

    /* XXX what to do if error ? Revert back to old vals incore ? */
    xfs_sync_sb(mp, false)
}

unsafe fn xfs_qm_scall_trunc_qfile(mp: *mut xfs_mount, type_: xfs_dqtype_t) -> c_int {
    let mut ip: *mut xfs_inode = core::ptr::null_mut();
    let mut tp: *mut xfs_trans = core::ptr::null_mut();
    let mut error: c_int;

    error = xfs_qm_qino_load(mp, type_, &mut ip);
    if error == -ENOENT { return 0; }
    if error != 0 { return error; }

    xfs_ilock(ip, XFS_IOLOCK_EXCL);
    error = xfs_trans_alloc(mp, &M_RES(mp).tr_itruncate, 0, 0, 0, &mut tp);
    if error != 0 {
        xfs_iunlock(ip, XFS_IOLOCK_EXCL);
        xfs_irele(ip);
        return error;
    }

    xfs_ilock(ip, XFS_ILOCK_EXCL);
    xfs_trans_ijoin(tp, ip, 0);
    (*ip).i_disk_size = 0;
    xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE);

    error = xfs_itruncate_extents(&mut tp, ip, XFS_DATA_FORK, 0);
    if error != 0 {
        xfs_trans_cancel(tp);
        xfs_iunlock(ip, XFS_ILOCK_EXCL | XFS_IOLOCK_EXCL);
        xfs_irele(ip);
        return error;
    }

    ASSERT((*ip).i_df.if_nextents == 0);
    xfs_trans_ichgtime(tp, ip, XFS_ICHGTIME_MOD | XFS_ICHGTIME_CHG);
    error = xfs_trans_commit(tp);
    xfs_iunlock(ip, XFS_ILOCK_EXCL | XFS_IOLOCK_EXCL);
    xfs_irele(ip);
    error
}

pub unsafe fn xfs_qm_scall_trunc_qfiles(mp: *mut xfs_mount, flags: uint) -> c_int {
    let mut error: c_int = -EINVAL;
    if !xfs_has_quota(mp) || flags == 0 || (flags & !XFS_QMOPT_QUOTALL) != 0 {
        xfs_debug(mp, "%s: flags=%x m_qflags=%x", __func__, flags, (*mp).m_qflags);
        return -EINVAL;
    }
    if (flags & XFS_QMOPT_UQUOTA) != 0 {
        error = xfs_qm_scall_trunc_qfile(mp, XFS_DQTYPE_USER);
        if error != 0 { return error; }
    }
    if (flags & XFS_QMOPT_GQUOTA) != 0 {
        error = xfs_qm_scall_trunc_qfile(mp, XFS_DQTYPE_GROUP);
        if error != 0 { return error; }
    }
    if (flags & XFS_QMOPT_PQUOTA) != 0 {
        error = xfs_qm_scall_trunc_qfile(mp, XFS_DQTYPE_PROJ);
    }
    error
}

/*
 * Switch on (a given) quota enforcement for a filesystem.  This takes
 * effect immediately.
 * (Switching on quota accounting must be done at mount time.)
 */
pub unsafe fn xfs_qm_scall_quotaon(mp: *mut xfs_mount, mut flags: uint) -> c_int {
    let mut error: c_int;
    let mut qf: uint;
    flags &= XFS_ALL_QUOTA_ENFD;
    if flags == 0 {
        xfs_debug(mp, "%s: zero flags, m_qflags=%x", __func__, (*mp).m_qflags);
        return -EINVAL;
    }
    if (((*mp).m_sb.sb_qflags & XFS_UQUOTA_ACCT) == 0 && (flags & XFS_UQUOTA_ENFD) != 0) ||
       (((*mp).m_sb.sb_qflags & XFS_GQUOTA_ACCT) == 0 && (flags & XFS_GQUOTA_ENFD) != 0) ||
       (((*mp).m_sb.sb_qflags & XFS_PQUOTA_ACCT) == 0 && (flags & XFS_PQUOTA_ENFD) != 0) {
        xfs_debug(mp, "%s: Can't enforce without acct, flags=%x sbflags=%x", __func__, flags, (*mp).m_sb.sb_qflags);
        return -EINVAL;
    }
    if ((*mp).m_qflags & flags) == flags { return -EEXIST; }
    spin_lock(&mut (*mp).m_sb_lock);
    qf = (*mp).m_sb.sb_qflags;
    (*mp).m_sb.sb_qflags = qf | flags;
    spin_unlock(&mut (*mp).m_sb_lock);
    if (qf & flags) == flags { return -EEXIST; }
    error = xfs_sync_sb(mp, false);
    if error != 0 { return error; }
    if (((*mp).m_sb.sb_qflags & XFS_UQUOTA_ACCT) != ((*mp).m_qflags & XFS_UQUOTA_ACCT)) ||
       (((*mp).m_sb.sb_qflags & XFS_PQUOTA_ACCT) != ((*mp).m_qflags & XFS_PQUOTA_ACCT)) ||
       (((*mp).m_sb.sb_qflags & XFS_GQUOTA_ACCT) != ((*mp).m_qflags & XFS_GQUOTA_ACCT)) { return 0; }
    if !XFS_IS_QUOTA_ON(mp) { return -ESRCH; }
    mutex_lock(&mut (*(*mp).m_quotainfo).qi_quotaofflock);
    (*mp).m_qflags |= flags & XFS_ALL_QUOTA_ENFD;
    mutex_unlock(&mut (*(*mp).m_quotainfo).qi_quotaofflock);
    0
}

pub const XFS_QC_MASK: u32 = QC_LIMIT_MASK | QC_TIMER_MASK;

/* Adjust limits of this quota, and the defaults if passed in. */
unsafe fn xfs_setqlim_limits(mp: *mut xfs_mount, res: *mut xfs_dquot_res, qlim: *mut xfs_quota_limits, hard: xfs_qcnt_t, soft: xfs_qcnt_t, tag: *const c_char) -> bool {
    if hard != 0 && hard < soft {
        xfs_debug(mp, "%shard %lld < %ssoft %lld", tag, hard, tag, soft);
        return false;
    }
    (*res).hardlimit = hard;
    (*res).softlimit = soft;
    if !qlim.is_null() { (*qlim).hard = hard; (*qlim).soft = soft; }
    true
}

unsafe fn xfs_setqlim_timer(mp: *mut xfs_mount, res: *mut xfs_dquot_res, qlim: *mut xfs_quota_limits, timer: s64) {
    if !qlim.is_null() {
        (*res).timer = xfs_dquot_set_grace_period(timer);
        (*qlim).time = (*res).timer;
    } else {
        (*res).timer = xfs_dquot_set_timeout(mp, timer);
    }
}

pub unsafe fn xfs_qm_scall_setqlim(mp: *mut xfs_mount, id: xfs_dqid_t, type_: xfs_dqtype_t, newlim: *mut qc_dqblk) -> c_int {
    let q = (*mp).m_quotainfo;
    let mut dqp: *mut xfs_dquot = core::ptr::null_mut();
    let mut tp: *mut xfs_trans = core::ptr::null_mut();
    let mut error: c_int;
    if ((*newlim).d_fieldmask & !XFS_QC_MASK) != 0 { return -EINVAL; }
    if ((*newlim).d_fieldmask & XFS_QC_MASK) == 0 { return 0; }
    error = xfs_qm_dqget(mp, id, type_, true, &mut dqp);
    if error != 0 { ASSERT(error != -ENOENT); return error; }
    let defq = xfs_get_defquota(q, xfs_dquot_type(dqp));
    error = xfs_trans_alloc(mp, &M_RES(mp).tr_qm_setqlim, 0, 0, 0, &mut tp);
    if error != 0 { xfs_qm_dqrele(dqp); return error; }
    mutex_lock(&mut (*dqp).q_qlock);
    xfs_trans_dqjoin(tp, dqp);
    let hard = if ((*newlim).d_fieldmask & QC_SPC_HARD) != 0 { XFS_B_TO_FSB(mp, (*newlim).d_spc_hardlimit) as xfs_qcnt_t } else { (*dqp).q_blk.hardlimit };
    let soft = if ((*newlim).d_fieldmask & QC_SPC_SOFT) != 0 { XFS_B_TO_FSB(mp, (*newlim).d_spc_softlimit) as xfs_qcnt_t } else { (*dqp).q_blk.softlimit };
    if xfs_setqlim_limits(mp, &mut (*dqp).q_blk, if id == 0 { &mut (*defq).blk } else { core::ptr::null_mut() }, hard, soft, b"blk\0".as_ptr() as *const c_char) { xfs_dquot_set_prealloc_limits(dqp); }
    if ((*newlim).d_fieldmask & QC_SPC_TIMER) != 0 { xfs_setqlim_timer(mp, &mut (*dqp).q_blk, if id == 0 { &mut (*defq).blk } else { core::ptr::null_mut() }, (*newlim).d_spc_timer); }
    let hard = if ((*newlim).d_fieldmask & QC_RT_SPC_HARD) != 0 { XFS_B_TO_FSB(mp, (*newlim).d_rt_spc_hardlimit) as xfs_qcnt_t } else { (*dqp).q_rtb.hardlimit };
    let soft = if ((*newlim).d_fieldmask & QC_RT_SPC_SOFT) != 0 { XFS_B_TO_FSB(mp, (*newlim).d_rt_spc_softlimit) as xfs_qcnt_t } else { (*dqp).q_rtb.softlimit };
    let qlim = if id == 0 { &mut (*defq).rtb } else { core::ptr::null_mut() };
    xfs_setqlim_limits(mp, &mut (*dqp).q_rtb, qlim, hard, soft, b"rtb\0".as_ptr() as *const c_char);
    if ((*newlim).d_fieldmask & QC_RT_SPC_TIMER) != 0 { xfs_setqlim_timer(mp, &mut (*dqp).q_rtb, qlim, (*newlim).d_rt_spc_timer); }
    let hard = if ((*newlim).d_fieldmask & QC_INO_HARD) != 0 { (*newlim).d_ino_hardlimit as xfs_qcnt_t } else { (*dqp).q_ino.hardlimit };
    let soft = if ((*newlim).d_fieldmask & QC_INO_SOFT) != 0 { (*newlim).d_ino_softlimit as xfs_qcnt_t } else { (*dqp).q_ino.softlimit };
    let qlim = if id == 0 { &mut (*defq).ino } else { core::ptr::null_mut() };
    xfs_setqlim_limits(mp, &mut (*dqp).q_ino, qlim, hard, soft, b"ino\0".as_ptr() as *const c_char);
    if ((*newlim).d_fieldmask & QC_INO_TIMER) != 0 { xfs_setqlim_timer(mp, &mut (*dqp).q_ino, qlim, (*newlim).d_ino_timer); }
    if id != 0 { xfs_qm_adjust_dqtimers(dqp); }
    (*dqp).q_flags |= XFS_DQFLAG_DIRTY;
    xfs_trans_log_dquot(tp, dqp);
    error = xfs_trans_commit(tp);
    xfs_qm_dqrele(dqp);
    error
}

unsafe fn xfs_qm_scall_getquota_fill_defaults(mp: *mut xfs_mount, type_: xfs_dqtype_t, dst: *mut qc_dqblk) -> c_int {
    let defq = xfs_get_defquota((*mp).m_quotainfo, type_);
    if (*defq).blk.soft == 0 && (*defq).blk.hard == 0 && (*defq).ino.soft == 0 && (*defq).ino.hard == 0 && (*defq).rtb.soft == 0 && (*defq).rtb.hard == 0 { return -ENOENT; }
    core::ptr::write_bytes(dst, 0, 1);
    (*dst).d_spc_softlimit = XFS_FSB_TO_B(mp, (*defq).blk.soft);
    (*dst).d_spc_hardlimit = XFS_FSB_TO_B(mp, (*defq).blk.hard);
    (*dst).d_ino_softlimit = (*defq).ino.soft;
    (*dst).d_ino_hardlimit = (*defq).ino.hard;
    (*dst).d_rt_spc_softlimit = XFS_FSB_TO_B(mp, (*defq).rtb.soft);
    (*dst).d_rt_spc_hardlimit = XFS_FSB_TO_B(mp, (*defq).rtb.hard);
    0
}

unsafe fn xfs_qm_scall_getquota_fill_qc(mp: *mut xfs_mount, _type: xfs_dqtype_t, dqp: *const xfs_dquot, dst: *mut qc_dqblk) {
    core::ptr::write_bytes(dst, 0, 1);
    (*dst).d_spc_hardlimit = XFS_FSB_TO_B(mp, (*dqp).q_blk.hardlimit);
    (*dst).d_spc_softlimit = XFS_FSB_TO_B(mp, (*dqp).q_blk.softlimit);
    (*dst).d_ino_hardlimit = (*dqp).q_ino.hardlimit;
    (*dst).d_ino_softlimit = (*dqp).q_ino.softlimit;
    (*dst).d_space = XFS_FSB_TO_B(mp, (*dqp).q_blk.reserved);
    (*dst).d_ino_count = (*dqp).q_ino.reserved;
    (*dst).d_spc_timer = (*dqp).q_blk.timer;
    (*dst).d_ino_timer = (*dqp).q_ino.timer;
    (*dst).d_ino_warns = 0; (*dst).d_spc_warns = 0;
    (*dst).d_rt_spc_hardlimit = XFS_FSB_TO_B(mp, (*dqp).q_rtb.hardlimit);
    (*dst).d_rt_spc_softlimit = XFS_FSB_TO_B(mp, (*dqp).q_rtb.softlimit);
    (*dst).d_rt_space = XFS_FSB_TO_B(mp, (*dqp).q_rtb.reserved);
    (*dst).d_rt_spc_timer = (*dqp).q_rtb.timer; (*dst).d_rt_spc_warns = 0;
    if !xfs_dquot_is_enforced(dqp) { (*dst).d_spc_timer = 0; (*dst).d_ino_timer = 0; (*dst).d_rt_spc_timer = 0; }
}

pub unsafe fn xfs_qm_scall_getquota(mp: *mut xfs_mount, id: xfs_dqid_t, type_: xfs_dqtype_t, dst: *mut qc_dqblk) -> c_int {
    let mut dqp: *mut xfs_dquot = core::ptr::null_mut();
    if id == 0 { xfs_inodegc_push(mp); }
    let mut error = xfs_qm_dqget(mp, id, type_, false, &mut dqp);
    if error != 0 {
        if error == -ENOENT && id != 0 && xfs_qm_scall_getquota_fill_defaults(mp, type_, dst) == 0 { return 0; }
        return error;
    }
    mutex_lock(&mut (*dqp).q_qlock);
    if XFS_IS_DQUOT_UNINITIALIZED(dqp) { error = -ENOENT; } else { xfs_qm_scall_getquota_fill_qc(mp, type_, dqp, dst); }
    mutex_unlock(&mut (*dqp).q_qlock);
    xfs_qm_dqrele(dqp);
    error
}

pub unsafe fn xfs_qm_scall_getquota_next(mp: *mut xfs_mount, id: *mut xfs_dqid_t, type_: xfs_dqtype_t, dst: *mut qc_dqblk) -> c_int {
    let mut dqp: *mut xfs_dquot = core::ptr::null_mut();
    if *id == 0 { xfs_inodegc_push(mp); }
    let error = xfs_qm_dqget_next(mp, *id, type_, &mut dqp);
    if error != 0 { return error; }
    *id = (*dqp).q_id;
    xfs_qm_scall_getquota_fill_qc(mp, type_, dqp, dst);
    mutex_unlock(&mut (*dqp).q_qlock);
    xfs_qm_dqrele(dqp);
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
