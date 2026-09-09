// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2008, Christoph Hellwig
 * All Rights Reserved.
 */

// Dependencies are supplied by the surrounding XFS translation unit.

static unsafe fn xfs_qm_fill_state(
    tstate: *mut qc_type_state,
    mp: *mut xfs_mount,
    type_: xfs_dqtype_t,
) -> c_int {
    let mut ip: *mut xfs_inode = core::ptr::null_mut();
    let error = xfs_qm_qino_load(mp, type_, &mut ip);
    if error != 0 {
        (*tstate).ino = NULLFSINO;
        return if error != -ENOENT { error } else { 0 };
    }

    let defq = xfs_get_defquota((*mp).m_quotainfo, type_);

    (*tstate).ino = I_INO(ip);
    (*tstate).flags |= QCI_SYSFILE;
    (*tstate).blocks = (*ip).i_nblocks;
    (*tstate).nextents = (*ip).i_df.if_nextents;
    (*tstate).spc_timelimit = (*defq).blk.time as u32;
    (*tstate).ino_timelimit = (*defq).ino.time as u32;
    (*tstate).rt_spc_timelimit = (*defq).rtb.time as u32;
    (*tstate).spc_warnlimit = 0;
    (*tstate).ino_warnlimit = 0;
    (*tstate).rt_spc_warnlimit = 0;
    xfs_irele(ip);
    0
}

/*
 * Return quota status information, such as enforcements, quota file inode
 * numbers etc.
 */
unsafe fn xfs_fs_get_quota_state(
    sb: *mut super_block,
    state: *mut qc_state,
) -> c_int {
    let mp = XFS_M(sb);
    let q = (*mp).m_quotainfo;
    core::ptr::write_bytes(state, 0, 1);
    if !XFS_IS_QUOTA_ON(mp) {
        return 0;
    }
    (*state).s_incoredqs = core::cmp::min((*q).qi_dquots, UINT_MAX as u64);
    if XFS_IS_UQUOTA_ON(mp) { (*state).s_state[USRQUOTA].flags |= QCI_ACCT_ENABLED; }
    if XFS_IS_UQUOTA_ENFORCED(mp) { (*state).s_state[USRQUOTA].flags |= QCI_LIMITS_ENFORCED; }
    if XFS_IS_GQUOTA_ON(mp) { (*state).s_state[GRPQUOTA].flags |= QCI_ACCT_ENABLED; }
    if XFS_IS_GQUOTA_ENFORCED(mp) { (*state).s_state[GRPQUOTA].flags |= QCI_LIMITS_ENFORCED; }
    if XFS_IS_PQUOTA_ON(mp) { (*state).s_state[PRJQUOTA].flags |= QCI_ACCT_ENABLED; }
    if XFS_IS_PQUOTA_ENFORCED(mp) { (*state).s_state[PRJQUOTA].flags |= QCI_LIMITS_ENFORCED; }

    let mut error = xfs_qm_fill_state(&mut (*state).s_state[USRQUOTA], mp, XFS_DQTYPE_USER);
    if error != 0 { return error; }
    error = xfs_qm_fill_state(&mut (*state).s_state[GRPQUOTA], mp, XFS_DQTYPE_GROUP);
    if error != 0 { return error; }
    error = xfs_qm_fill_state(&mut (*state).s_state[PRJQUOTA], mp, XFS_DQTYPE_PROJ);
    if error != 0 { return error; }
    0
}

unsafe fn xfs_quota_type(type_: c_int) -> xfs_dqtype_t {
    match type_ {
        USRQUOTA => XFS_DQTYPE_USER,
        GRPQUOTA => XFS_DQTYPE_GROUP,
        _ => XFS_DQTYPE_PROJ,
    }
}

const XFS_QC_SETINFO_MASK: u32 = QC_TIMER_MASK;

/* Adjust quota timers & warnings */
unsafe fn xfs_fs_set_info(
    sb: *mut super_block,
    type_: c_int,
    info: *mut qc_info,
) -> c_int {
    let mp = XFS_M(sb);
    let mut newlim: qc_dqblk = core::mem::zeroed();
    if sb_rdonly(sb) { return -EROFS; }
    if !XFS_IS_QUOTA_ON(mp) { return -ENOSYS; }
    if (*info).i_fieldmask & !XFS_QC_SETINFO_MASK != 0 { return -EINVAL; }
    if (*info).i_fieldmask & XFS_QC_SETINFO_MASK == 0 { return 0; }
    newlim.d_fieldmask = (*info).i_fieldmask;
    newlim.d_spc_timer = (*info).i_spc_timelimit;
    newlim.d_ino_timer = (*info).i_ino_timelimit;
    newlim.d_rt_spc_timer = (*info).i_rt_spc_timelimit;
    newlim.d_ino_warns = (*info).i_ino_warnlimit;
    newlim.d_spc_warns = (*info).i_spc_warnlimit;
    newlim.d_rt_spc_warns = (*info).i_rt_spc_warnlimit;
    xfs_qm_scall_setqlim(mp, 0, xfs_quota_type(type_), &mut newlim)
}

unsafe fn xfs_quota_flags(uflags: c_uint) -> c_uint {
    let mut flags = 0;
    if uflags & FS_QUOTA_UDQ_ACCT != 0 { flags |= XFS_UQUOTA_ACCT; }
    if uflags & FS_QUOTA_PDQ_ACCT != 0 { flags |= XFS_PQUOTA_ACCT; }
    if uflags & FS_QUOTA_GDQ_ACCT != 0 { flags |= XFS_GQUOTA_ACCT; }
    if uflags & FS_QUOTA_UDQ_ENFD != 0 { flags |= XFS_UQUOTA_ENFD; }
    if uflags & FS_QUOTA_GDQ_ENFD != 0 { flags |= XFS_GQUOTA_ENFD; }
    if uflags & FS_QUOTA_PDQ_ENFD != 0 { flags |= XFS_PQUOTA_ENFD; }
    flags
}

unsafe fn xfs_quota_enable(sb: *mut super_block, uflags: c_uint) -> c_int {
    let mp = XFS_M(sb);
    if sb_rdonly(sb) { return -EROFS; }
    if !XFS_IS_QUOTA_ON(mp) { return -ENOSYS; }
    xfs_qm_scall_quotaon(mp, xfs_quota_flags(uflags))
}

unsafe fn xfs_quota_disable(sb: *mut super_block, uflags: c_uint) -> c_int {
    let mp = XFS_M(sb);
    if sb_rdonly(sb) { return -EROFS; }
    if !XFS_IS_QUOTA_ON(mp) { return -ENOSYS; }
    xfs_qm_scall_quotaoff(mp, xfs_quota_flags(uflags))
}

unsafe fn xfs_fs_rm_xquota(sb: *mut super_block, uflags: c_uint) -> c_int {
    let mp = XFS_M(sb);
    let mut flags = 0;
    if sb_rdonly(sb) { return -EROFS; }
    if XFS_IS_QUOTA_ON(mp) { return -EINVAL; }
    if uflags & !(FS_USER_QUOTA | FS_GROUP_QUOTA | FS_PROJ_QUOTA) != 0 { return -EINVAL; }
    if uflags & FS_USER_QUOTA != 0 { flags |= XFS_QMOPT_UQUOTA; }
    if uflags & FS_GROUP_QUOTA != 0 { flags |= XFS_QMOPT_GQUOTA; }
    if uflags & FS_PROJ_QUOTA != 0 { flags |= XFS_QMOPT_PQUOTA; }
    xfs_qm_scall_trunc_qfiles(mp, flags)
}

unsafe fn xfs_fs_get_dqblk(sb: *mut super_block, qid: kqid, qdq: *mut qc_dqblk) -> c_int {
    let mp = XFS_M(sb);
    if !XFS_IS_QUOTA_ON(mp) { return -ENOSYS; }
    let id = from_kqid(&init_user_ns, qid);
    xfs_qm_scall_getquota(mp, id, xfs_quota_type(qid.type_), qdq)
}

/* Return quota info for active quota >= this qid */
unsafe fn xfs_fs_get_nextdqblk(sb: *mut super_block, qid: *mut kqid, qdq: *mut qc_dqblk) -> c_int {
    let mp = XFS_M(sb);
    if !XFS_IS_QUOTA_ON(mp) { return -ENOSYS; }
    let mut id = from_kqid(&init_user_ns, *qid);
    let ret = xfs_qm_scall_getquota_next(mp, &mut id, xfs_quota_type((*qid).type_), qdq);
    if ret != 0 { return ret; }
    *qid = make_kqid(current_user_ns(), (*qid).type_, id);
    0
}

unsafe fn xfs_fs_set_dqblk(sb: *mut super_block, qid: kqid, qdq: *mut qc_dqblk) -> c_int {
    let mp = XFS_M(sb);
    if sb_rdonly(sb) { return -EROFS; }
    if !XFS_IS_QUOTA_ON(mp) { return -ENOSYS; }
    xfs_qm_scall_setqlim(mp, from_kqid(&init_user_ns, qid), xfs_quota_type(qid.type_), qdq)
}

#[repr(C)]
pub static xfs_quotactl_operations: quotactl_ops = quotactl_ops {
    get_state: Some(xfs_fs_get_quota_state),
    set_info: Some(xfs_fs_set_info),
    quota_enable: Some(xfs_quota_enable),
    quota_disable: Some(xfs_quota_disable),
    rm_xquota: Some(xfs_fs_rm_xquota),
    get_dqblk: Some(xfs_fs_get_dqblk),
    get_nextdqblk: Some(xfs_fs_get_nextdqblk),
    set_dqblk: Some(xfs_fs_set_dqblk),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
