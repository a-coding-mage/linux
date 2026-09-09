// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2006 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding XFS translation.

unsafe fn xfs_fill_statvfs_from_dquot(
    statp: *mut kstatfs,
    ip: *mut xfs_inode,
    dqp: *mut xfs_dquot,
) {
    let mut blkres: *mut xfs_dquot_res = unsafe { &mut (*dqp).q_blk };
    let limit: u64;

    if unsafe { XFS_IS_REALTIME_MOUNT((*ip).i_mount) }
        && unsafe { ((*ip).i_diflags & (XFS_DIFLAG_RTINHERIT | XFS_DIFLAG_REALTIME)) != 0 }
    {
        blkres = unsafe { &mut (*dqp).q_rtb };
    }

    limit = unsafe {
        if (*blkres).softlimit != 0 {
            (*blkres).softlimit
        } else {
            (*blkres).hardlimit
        }
    };
    if limit != 0 {
        let mut remaining: u64 = 0;

        if unsafe { limit > (*blkres).reserved } {
            remaining = unsafe { limit - (*blkres).reserved };
        }

        unsafe {
            (*statp).f_blocks = core::cmp::min((*statp).f_blocks, limit);
            (*statp).f_bfree = core::cmp::min((*statp).f_bfree, remaining);
        }
    }

    limit = unsafe {
        if (*dqp).q_ino.softlimit != 0 {
            (*dqp).q_ino.softlimit
        } else {
            (*dqp).q_ino.hardlimit
        }
    };
    if limit != 0 {
        let mut remaining: u64 = 0;

        if unsafe { limit > (*dqp).q_ino.reserved } {
            remaining = unsafe { limit - (*dqp).q_ino.reserved };
        }

        unsafe {
            (*statp).f_files = core::cmp::min((*statp).f_files, limit);
            (*statp).f_ffree = core::cmp::min((*statp).f_ffree, remaining);
        }
    }
}

/*
 * Directory tree accounting is implemented using project quotas, where
 * the project identifier is inherited from parent directories.
 * A statvfs (df, etc.) of a directory that is using project quota should
 * return a statvfs of the project, not the entire filesystem.
 * This makes such trees appear as if they are filesystems in themselves.
 */
pub unsafe fn xfs_qm_statvfs(ip: *mut xfs_inode, statp: *mut kstatfs) {
    let mp: *mut xfs_mount = unsafe { (*ip).i_mount };
    let mut dqp: *mut xfs_dquot = core::ptr::null_mut();

    if unsafe { xfs_qm_dqget(mp, (*ip).i_projid, XFS_DQTYPE_PROJ, false, &mut dqp) } == 0 {
        unsafe {
            mutex_lock(&mut (*dqp).q_qlock);
            xfs_fill_statvfs_from_dquot(statp, ip, dqp);
            mutex_unlock(&mut (*dqp).q_qlock);
            xfs_qm_dqrele(dqp);
        }
    }
}

unsafe fn xfs_qm_validate_state_change(
    mp: *mut xfs_mount,
    uqd: u32,
    gqd: u32,
    pqd: u32,
) -> i32 {
    let state: bool;

    /* Is quota state changing? */
    state = (uqd != 0 && unsafe { !XFS_IS_UQUOTA_ON(mp) })
        || (uqd == 0 && unsafe { XFS_IS_UQUOTA_ON(mp) })
        || (gqd != 0 && unsafe { !XFS_IS_GQUOTA_ON(mp) })
        || (gqd == 0 && unsafe { XFS_IS_GQUOTA_ON(mp) })
        || (pqd != 0 && unsafe { !XFS_IS_PQUOTA_ON(mp) })
        || (pqd == 0 && unsafe { XFS_IS_PQUOTA_ON(mp) });

    if state
        && unsafe { (xfs_dev_is_read_only(mp, "changing quota state") || xfs_has_norecovery(mp)) }
    {
        1
    } else {
        0
    }
}

pub unsafe fn xfs_qm_newmount(
    mp: *mut xfs_mount,
    needquotamount: *mut bool,
    quotaflags: *mut u32,
) -> i32 {
    let mut quotaondisk: u32;
    let mut uquotaondisk: u32 = 0;
    let mut gquotaondisk: u32 = 0;
    let mut pquotaondisk: u32 = 0;

    quotaondisk = unsafe {
        if xfs_has_quota(mp) && ((*mp).m_sb.sb_qflags & XFS_ALL_QUOTA_ACCT) != 0 {
            1
        } else {
            0
        }
    };

    if quotaondisk != 0 {
        unsafe {
            uquotaondisk = (*mp).m_sb.sb_qflags & XFS_UQUOTA_ACCT;
            pquotaondisk = (*mp).m_sb.sb_qflags & XFS_PQUOTA_ACCT;
            gquotaondisk = (*mp).m_sb.sb_qflags & XFS_GQUOTA_ACCT;
        }
    }

    /*
     * If the device itself is read-only and/or in norecovery
     * mode, we can't allow the user to change the state of
     * quota on the mount - this would generate a transaction
     * on the ro device, which would lead to an I/O error and
     * shutdown.
     */
    if unsafe { xfs_qm_validate_state_change(mp, uquotaondisk, gquotaondisk, pquotaondisk) != 0 } {
        unsafe {
            if xfs_has_metadir(mp) {
                xfs_warn(mp, "metadir enabled, please mount without any quota mount options");
            } else {
                xfs_warn(
                    mp,
                    "please mount with{}{}{}{}.",
                    if quotaondisk == 0 { "out quota" } else { "" },
                    if uquotaondisk != 0 { " usrquota" } else { "" },
                    if gquotaondisk != 0 { " grpquota" } else { "" },
                    if pquotaondisk != 0 { " prjquota" } else { "" },
                );
            }
        }
        return -EPERM;
    }

    if unsafe { XFS_IS_QUOTA_ON(mp) } || quotaondisk != 0 {
        /* Call mount_quotas only if we won't have to do a quotacheck. */
        if quotaondisk != 0 && unsafe { !XFS_QM_NEED_QUOTACHECK(mp) } {
            unsafe { xfs_qm_mount_quotas(mp); }
        } else {
            /* Clear quota flags, but remember them until quota setup is ready. */
            unsafe {
                *needquotamount = true;
                *quotaflags = (*mp).m_qflags;
                (*mp).m_qflags = 0;
            }
        }
    }

    0
}

/*
 * If the sysadmin didn't provide any quota mount options, restore the quota
 * accounting and enforcement state from the ondisk superblock.  Only do this
 * for metadir filesystems because this is a behavior change.
 */
pub unsafe fn xfs_qm_resume_quotaon(mp: *mut xfs_mount) {
    if unsafe { !xfs_has_metadir(mp) } {
        return;
    }
    if unsafe { xfs_has_norecovery(mp) } {
        return;
    }

    unsafe {
        (*mp).m_qflags = (*mp).m_sb.sb_qflags & (XFS_ALL_QUOTA_ACCT | XFS_ALL_QUOTA_ENFD);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
