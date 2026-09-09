// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* External XFS and scrub dependencies are supplied by the surrounding crate. */

/*
 * Live Inode Link Count Repair
 * ============================
 *
 * Use the live inode link count information that we collected to replace the
 * nlink values of the incore inodes.  A scrub->repair cycle should have left
 * the live data and hooks active, so this is safe so long as we make sure the
 * inode is locked.
 */

/* Set up to repair inode link counts. */
pub unsafe fn xrep_setup_nlinks(sc: *mut xfs_scrub) -> i32 {
    xrep_orphanage_try_create(sc)
}

/*
 * Inodes that aren't the root directory or the orphanage, have a nonzero link
 * count, and no observed parents should be moved to the orphanage.
 */
#[inline]
unsafe fn xrep_nlinks_is_orphaned(
    sc: *mut xfs_scrub,
    ip: *mut xfs_inode,
    actual_nlink: u32,
    obs: *const xchk_nlink,
) -> bool {
    if (*obs).parents != 0 {
        return false;
    }
    if xchk_inode_is_dirtree_root(ip) || ip == (*sc).orphanage {
        return false;
    }
    actual_nlink != 0
}

/* Remove an inode from the unlinked list. */
unsafe fn xrep_nlinks_iunlink_remove(sc: *mut xfs_scrub) -> i32 {
    let pag: *mut xfs_perag = xfs_perag_get((*sc).mp, XFS_INODE_TO_AGNO((*sc).ip));
    let error = xfs_iunlink_remove((*sc).tp, pag, (*sc).ip);
    xfs_perag_put(pag);
    error
}

/*
 * Correct the link count of the given inode.  Because we have to grab locks
 * and resources in a certain order, it's possible that this will be a no-op.
 */
unsafe fn xrep_nlinks_repair_inode(xnc: *mut xchk_nlink_ctrs) -> i32 {
    let mut obs: xchk_nlink = core::mem::zeroed();
    let sc: *mut xfs_scrub = (*xnc).sc;
    let mp: *mut xfs_mount = (*sc).mp;
    let ip: *mut xfs_inode = (*sc).ip;
    let mut total_links: u64;
    let mut actual_nlink: u64;
    let mut orphanage_available = false;
    let mut dirty = false;
    let mut error: i32;

    /* Ignore temporary files being used to stage repairs. */
    if xrep_is_tempfile(ip) {
        return 0;
    }

    if xrep_orphanage_can_adopt(sc) {
        error = xrep_orphanage_iolock_two(sc);
        if error != 0 { return error; }
        error = xrep_adoption_trans_alloc(sc, &mut (*xnc).adoption);
        if error != 0 {
            xchk_iunlock(sc, XFS_IOLOCK_EXCL);
            xrep_orphanage_iunlock(sc, XFS_IOLOCK_EXCL);
        } else {
            orphanage_available = true;
        }
    }

    if !orphanage_available {
        xchk_ilock(sc, XFS_IOLOCK_EXCL);
        error = xfs_trans_alloc(mp, &mut *M_RES(mp).tr_link, 0, 0, 0, &mut (*sc).tp);
        if error != 0 {
            xchk_iunlock(sc, XFS_IOLOCK_EXCL);
            return error;
        }
        xchk_ilock(sc, XFS_ILOCK_EXCL);
        xfs_trans_ijoin((*sc).tp, ip, 0);
    }

    mutex_lock(&mut (*xnc).lock);
    if xchk_iscan_aborted(&(*xnc).collect_iscan) {
        error = -ECANCELED;
        mutex_unlock(&mut (*xnc).lock);
        return xrep_nlinks_repair_inode_unlock(sc, orphanage_available, error);
    }
    error = xfarray_load_sparse((*xnc).nlinks, I_INO(ip), &mut obs);
    if error != 0 {
        mutex_unlock(&mut (*xnc).lock);
        return xrep_nlinks_repair_inode_unlock(sc, orphanage_available, error);
    }
    mutex_unlock(&mut (*xnc).lock);

    total_links = xchk_nlink_total(ip, &obs);
    actual_nlink = VFS_I(ip).i_nlink;

    if !S_ISDIR(VFS_I(ip).i_mode) && obs.children != 0 {
        trace_xrep_nlinks_unfixable_inode(mp, ip, &obs);
        error = 0;
        return xrep_nlinks_repair_inode_trans(sc, orphanage_available, error);
    }

    if orphanage_available && xrep_nlinks_is_orphaned(sc, ip, actual_nlink as u32, &obs) {
        error = xrep_adoption_compute_name(&mut (*xnc).adoption, &mut (*xnc).xname);
        if error != 0 { return xrep_nlinks_repair_inode_trans(sc, orphanage_available, error); }
        error = xrep_adoption_move(&mut (*xnc).adoption);
        if error != 0 { return xrep_nlinks_repair_inode_trans(sc, orphanage_available, error); }
        mutex_lock(&mut (*xnc).lock);
        error = xfarray_load_sparse((*xnc).nlinks, I_INO(ip), &mut obs);
        mutex_unlock(&mut (*xnc).lock);
        if error != 0 { return xrep_nlinks_repair_inode_trans(sc, orphanage_available, error); }
        total_links = xchk_nlink_total(ip, &obs);
        actual_nlink = VFS_I(ip).i_nlink;
        dirty = true;
    }

    if total_links > 0 && xfs_inode_on_unlinked_list(ip) {
        error = xrep_nlinks_iunlink_remove(sc);
        if error != 0 { return xrep_nlinks_repair_inode_trans(sc, orphanage_available, error); }
        dirty = true;
    }
    if total_links == 0 && !xfs_inode_on_unlinked_list(ip) {
        if actual_nlink != 0 { clear_nlink(VFS_I(ip)); }
        error = xfs_iunlink((*sc).tp, ip);
        if error != 0 {
            if actual_nlink != 0 { set_nlink(VFS_I(ip), actual_nlink); }
            return xrep_nlinks_repair_inode_trans(sc, orphanage_available, error);
        }
        dirty = true;
    }
    if total_links != actual_nlink {
        trace_xrep_nlinks_update_inode(mp, ip, &obs);
        set_nlink(VFS_I(ip), core::cmp::min(total_links, XFS_NLINK_PINNED));
        dirty = true;
    }
    if !dirty { return xrep_nlinks_repair_inode_trans(sc, orphanage_available, 0); }
    xfs_trans_log_inode((*sc).tp, ip, XFS_ILOG_CORE);
    error = xrep_trans_commit(sc);
    xrep_nlinks_repair_inode_unlock(sc, orphanage_available, error)
}

unsafe fn xrep_nlinks_repair_inode_trans(sc: *mut xfs_scrub, orphanage_available: bool, error: i32) -> i32 {
    xchk_trans_cancel(sc);
    xrep_nlinks_repair_inode_unlock(sc, orphanage_available, error)
}

unsafe fn xrep_nlinks_repair_inode_unlock(sc: *mut xfs_scrub, orphanage_available: bool, error: i32) -> i32 {
    xchk_iunlock(sc, XFS_ILOCK_EXCL);
    if orphanage_available {
        xrep_orphanage_iunlock(sc, XFS_ILOCK_EXCL);
        xrep_orphanage_iunlock(sc, XFS_IOLOCK_EXCL);
    }
    xchk_iunlock(sc, XFS_IOLOCK_EXCL);
    error
}

/* Try to visit every inode in the filesystem for repairs. */
unsafe fn xrep_nlinks_iter(xnc: *mut xchk_nlink_ctrs, ipp: *mut *mut xfs_inode) -> i32 {
    let mut error;
    loop {
        error = xchk_iscan_iter(&mut (*xnc).compare_iscan, ipp);
        if error != -EBUSY { break; }
    }
    error
}

/* Commit the new inode link counters. */
pub unsafe fn xrep_nlinks(sc: *mut xfs_scrub) -> i32 {
    let xnc: *mut xchk_nlink_ctrs = (*sc).buf as *mut xchk_nlink_ctrs;
    let mut error: i32;
    if !xfs_has_ftype((*sc).mp) { return -EOPNOTSUPP; }
    xchk_iscan_start(sc, 30000, 100, &mut (*xnc).compare_iscan);
    ASSERT((*sc).ip.is_null());
    while {
        error = xrep_nlinks_iter(xnc, &mut (*sc).ip);
        error == 1
    } {
        xchk_trans_cancel(sc);
        error = xrep_nlinks_repair_inode(xnc);
        xchk_iscan_mark_visited(&mut (*xnc).compare_iscan, (*sc).ip);
        xchk_irele(sc, (*sc).ip);
        (*sc).ip = core::ptr::null_mut();
        if error != 0 { break; }
        if xchk_should_terminate(sc, &mut error) { break; }
        xchk_trans_alloc_empty(sc);
    }
    xchk_iscan_iter_finish(&mut (*xnc).compare_iscan);
    xchk_iscan_teardown(&mut (*xnc).compare_iscan);
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
