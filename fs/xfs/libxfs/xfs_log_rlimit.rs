// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2013 Jie Liu.
 * All Rights Reserved.
 */

/* Dependencies are supplied by the surrounding XFS translation unit. */

#[inline]
unsafe fn xfs_want_minlogsize_fixes(sb: *mut xfs_sb) -> bool {
    xfs_sb_is_v5(sb) &&
        xfs_sb_has_incompat_feature(sb, XFS_SB_FEAT_INCOMPAT_PARENT)
}

/* Calculate the maximum length in bytes required for a local attribute value. */
unsafe fn xfs_log_calc_max_attrsetm_res(mp: *mut xfs_mount) -> i32 {
    let mut size: i32 = xfs_attr_leaf_entsize_local_max((*(*mp).m_attr_geo).blksize)
        - MAXNAMELEN - 1;
    let mut nblks: i32 = XFS_DAENTER_SPACE_RES(mp, XFS_ATTR_FORK);
    nblks += XFS_B_TO_FSB(mp, size);

    /* Correct the old unit conversion error for newer feature sets. */
    if xfs_want_minlogsize_fixes(&mut (*mp).m_sb) {
        size = XFS_B_TO_FSB(mp, size);
    }

    nblks += XFS_NEXTENTADD_SPACE_RES(mp, size, XFS_ATTR_FORK);

    M_RES(mp).tr_attrsetm.tr_logres + M_RES(mp).tr_attrsetrt.tr_logres * nblks
}

unsafe fn xfs_log_calc_trans_resv_for_minlogblocks(
    mp: *mut xfs_mount,
    resv: *mut xfs_trans_resv,
) {
    let rmap_maxlevels: u32 = (*mp).m_rmap_maxlevels;

    if xfs_want_minlogsize_fixes(&mut (*mp).m_sb) {
        xfs_trans_resv_calc(mp, resv);
        (*resv).tr_atomic_ioend = M_RES(mp).tr_atomic_ioend;
        return;
    }

    if xfs_has_rmapbt(mp) && xfs_has_reflink(mp) {
        (*mp).m_rmap_maxlevels = XFS_OLD_REFLINK_RMAP_MAXLEVELS;
    }

    xfs_trans_resv_calc(mp, resv);
    (*resv).tr_atomic_ioend = M_RES(mp).tr_atomic_ioend;

    if xfs_has_reflink(mp) {
        (*resv).tr_write.tr_logcount = XFS_WRITE_LOG_COUNT_REFLINK;
        (*resv).tr_itruncate.tr_logcount = XFS_ITRUNCATE_LOG_COUNT_REFLINK;
        (*resv).tr_qm_dqalloc.tr_logcount = XFS_WRITE_LOG_COUNT_REFLINK;
    } else if xfs_has_rmapbt(mp) {
        (*resv).tr_write.tr_logcount = XFS_WRITE_LOG_COUNT;
        (*resv).tr_itruncate.tr_logcount = XFS_ITRUNCATE_LOG_COUNT;
        (*resv).tr_qm_dqalloc.tr_logcount = XFS_WRITE_LOG_COUNT;
    }

    (*resv).tr_write.tr_logres = xfs_calc_write_reservation_minlogsize(mp);
    (*resv).tr_itruncate.tr_logres = xfs_calc_itruncate_reservation_minlogsize(mp);
    (*resv).tr_qm_dqalloc.tr_logres = xfs_calc_qm_dqalloc_reservation_minlogsize(mp);

    (*mp).m_rmap_maxlevels = rmap_maxlevels;
}

pub unsafe fn xfs_log_get_max_trans_res(
    mp: *mut xfs_mount,
    max_resp: *mut xfs_trans_res,
) {
    let mut resv: xfs_trans_resv = core::mem::zeroed();
    let attr_space = xfs_log_calc_max_attrsetm_res(mp);
    xfs_log_calc_trans_resv_for_minlogblocks(mp, &mut resv);

    let resp = &resv as *const xfs_trans_resv as *const xfs_trans_res;
    let end_resp = (&resv as *const xfs_trans_resv).add(1) as *const xfs_trans_res;
    let mut i: u32 = 0;
    let mut log_space: i32 = 0;
    let mut cur = resp;
    while cur < end_resp {
        let tmp = if (*cur).tr_logcount > 1 {
            (*cur).tr_logres * (*cur).tr_logcount
        } else {
            (*cur).tr_logres
        };
        trace_xfs_trans_resv_calc_minlogsize(mp, i, cur);
        if log_space < tmp {
            log_space = tmp;
            *max_resp = *cur;
        }
        i += 1;
        cur = cur.add(1);
    }

    if attr_space > log_space {
        *max_resp = resv.tr_attrsetm;
        (*max_resp).tr_logres = attr_space;
    }
    trace_xfs_log_get_max_trans_res(mp, max_resp);
}

pub unsafe fn xfs_log_calc_minimum_size(mp: *mut xfs_mount) -> i32 {
    let mut tres: xfs_trans_res = core::mem::zeroed();
    let mut min_logblks: i32 = 0;
    let mut lsunit: i32 = 0;

    xfs_log_get_max_trans_res(mp, &mut tres);
    let mut max_logres = xfs_log_calc_unit_res(mp, tres.tr_logres);
    if tres.tr_logcount > 1 {
        max_logres *= tres.tr_logcount;
    }
    if xfs_has_logv2(mp) && (*mp).m_sb.sb_logsunit > 1 {
        lsunit = BTOBB((*mp).m_sb.sb_logsunit);
    }

    if lsunit != 0 {
        min_logblks = roundup_64(BTOBB(max_logres), lsunit) + 2 * lsunit;
    } else {
        min_logblks = BTOBB(max_logres) + 2 * BBSIZE;
    }
    min_logblks *= XFS_MIN_LOG_FACTOR;
    XFS_BB_TO_FSB(mp, min_logblks)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
