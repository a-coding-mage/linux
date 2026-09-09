// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2006 Silicon Graphics, Inc.
 * All Rights Reserved.
 */
// Dependencies supplied by the corresponding XFS headers.

unsafe fn xlog_recover_dquot_ra_pass2(
    log: *mut xlog,
    item: *mut xlog_recover_item,
) {
    let mp = (*log).l_mp;
    let mut recddq: *mut xfs_disk_dquot;
    let dq_f: *mut xfs_dq_logformat;
    let mut type_: u32;

    if (*mp).m_qflags == 0 {
        return;
    }

    recddq = (*item).ri_buf[1].iov_base as *mut xfs_disk_dquot;
    if recddq.is_null() {
        return;
    }
    if (*item).ri_buf[1].iov_len < core::mem::size_of::<xfs_disk_dquot>() {
        return;
    }

    type_ = (*recddq).d_type & XFS_DQTYPE_REC_MASK;
    ASSERT(type_ != 0);
    if (*log).l_quotaoffs_flag & type_ != 0 {
        return;
    }

    dq_f = (*item).ri_buf[0].iov_base as *mut xfs_dq_logformat;
    ASSERT(!dq_f.is_null());
    ASSERT((*dq_f).qlf_len == 1);

    xlog_buf_readahead(
        log,
        (*dq_f).qlf_blkno,
        XFS_FSB_TO_BB(mp, (*dq_f).qlf_len),
        &xfs_dquot_buf_ra_ops,
    );
}

/*
 * Recover a dquot record
 */
unsafe fn xlog_recover_dquot_commit_pass2(
    log: *mut xlog,
    buffer_list: *mut list_head,
    item: *mut xlog_recover_item,
    current_lsn: xfs_lsn_t,
) -> i32 {
    let mp = (*log).l_mp;
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let dqb: *mut xfs_dqblk;
    let ddq: *mut xfs_disk_dquot;
    let recddq: *mut xfs_disk_dquot;
    let dq_f: *mut xfs_dq_logformat;
    let mut fa: xfs_failaddr_t;
    let mut error: i32;
    let mut type_: u32;

    /*
     * Filesystems are required to send in quota flags at mount time.
     */
    if (*mp).m_qflags == 0 {
        return 0;
    }

    recddq = (*item).ri_buf[1].iov_base as *mut xfs_disk_dquot;
    if recddq.is_null() {
        xfs_alert((*log).l_mp, "NULL dquot in %s.");
        return -EFSCORRUPTED;
    }
    if (*item).ri_buf[1].iov_len < core::mem::size_of::<xfs_disk_dquot>() {
        xfs_alert((*log).l_mp, "dquot too small (%zd) in %s.");
        return -EFSCORRUPTED;
    }

    /* This type of quotas was turned off, so ignore this record. */
    type_ = (*recddq).d_type & XFS_DQTYPE_REC_MASK;
    ASSERT(type_ != 0);
    if (*log).l_quotaoffs_flag & type_ != 0 {
        return 0;
    }

    /* Quota is on and the dquot needs to be replayed. */
    dq_f = (*item).ri_buf[0].iov_base as *mut xfs_dq_logformat;
    ASSERT(!dq_f.is_null());
    fa = xfs_dquot_verify(mp, recddq, (*dq_f).qlf_id);
    if !fa.is_null() {
        xfs_alert(mp, "corrupt dquot ID 0x%x in log at %pS");
        return -EFSCORRUPTED;
    }
    ASSERT((*dq_f).qlf_len == 1);

    error = xfs_trans_read_buf(
        mp,
        core::ptr::null_mut(),
        (*mp).m_ddev_targp,
        (*dq_f).qlf_blkno,
        XFS_FSB_TO_BB(mp, (*dq_f).qlf_len),
        0,
        &mut bp,
        &xfs_dquot_buf_ops,
    );
    if error != 0 {
        return error;
    }

    ASSERT(!bp.is_null());
    dqb = xfs_buf_offset(bp, (*dq_f).qlf_boffset);
    ddq = &mut (*dqb).dd_diskdq;

    if xfs_has_crc(mp) {
        let lsn = be64_to_cpu((*dqb).dd_lsn);
        if lsn != 0 && lsn != u64::MAX && XFS_LSN_CMP(lsn, current_lsn) >= 0 {
            xfs_buf_relse(bp);
            return error;
        }
    }

    core::ptr::copy_nonoverlapping(
        recddq as *const u8,
        ddq as *mut u8,
        (*item).ri_buf[1].iov_len,
    );
    if xfs_has_crc(mp) {
        xfs_update_cksum(
            dqb as *mut i8,
            core::mem::size_of::<xfs_dqblk>(),
            XFS_DQUOT_CRC_OFF,
        );
    }

    /* Validate the recovered dquot. */
    fa = xfs_dqblk_verify((*log).l_mp, dqb, (*dq_f).qlf_id);
    if !fa.is_null() {
        XFS_CORRUPTION_ERROR(
            "Bad dquot after recovery",
            XFS_ERRLEVEL_LOW,
            mp,
            dqb,
            core::mem::size_of::<xfs_dqblk>(),
        );
        xfs_alert(mp, "Metadata corruption detected at %pS, dquot 0x%x");
        error = -EFSCORRUPTED;
        xfs_buf_relse(bp);
        return error;
    }

    ASSERT((*dq_f).qlf_size == 2);
    ASSERT((*bp).b_mount == mp);
    xfs_buf_delwri_queue(bp, buffer_list);

    xfs_buf_relse(bp);
    error
}

#[no_mangle]
pub static xlog_dquot_item_ops: xlog_recover_item_ops = xlog_recover_item_ops {
    item_type: XFS_LI_DQUOT,
    ra_pass2: Some(xlog_recover_dquot_ra_pass2),
    commit_pass2: Some(xlog_recover_dquot_commit_pass2),
};

/*
 * Recover QUOTAOFF records. We simply make a note of it in the xlog
 * structure, so that we know not to do any dquot item or dquot buffer recovery,
 * of that type.
 */
unsafe fn xlog_recover_quotaoff_commit_pass1(
    log: *mut xlog,
    item: *mut xlog_recover_item,
) -> i32 {
    let qoff_f = (*item).ri_buf[0].iov_base as *mut xfs_qoff_logformat;
    ASSERT(!qoff_f.is_null());

    /*
     * The logitem format's flag tells us if this was user quotaoff,
     * group/project quotaoff or both.
     */
    if (*qoff_f).qf_flags & XFS_UQUOTA_ACCT != 0 {
        (*log).l_quotaoffs_flag |= XFS_DQTYPE_USER;
    }
    if (*qoff_f).qf_flags & XFS_PQUOTA_ACCT != 0 {
        (*log).l_quotaoffs_flag |= XFS_DQTYPE_PROJ;
    }
    if (*qoff_f).qf_flags & XFS_GQUOTA_ACCT != 0 {
        (*log).l_quotaoffs_flag |= XFS_DQTYPE_GROUP;
    }

    0
}

#[no_mangle]
pub static xlog_quotaoff_item_ops: xlog_recover_item_ops = xlog_recover_item_ops {
    item_type: XFS_LI_QUOTAOFF,
    commit_pass1: Some(xlog_recover_quotaoff_commit_pass1),
    /* nothing to commit in pass2 */
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
