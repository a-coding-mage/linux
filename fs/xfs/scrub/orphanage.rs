// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C dependencies: xfs_platform.h, xfs_fs.h, xfs_shared.h, xfs_format.h,
// xfs_trans_resv.h, xfs_mount.h, xfs_log_format.h, xfs_trans.h, xfs_inode.h,
// xfs_ialloc.h, xfs_quota.h, xfs_trans_space.h, xfs_dir2.h, xfs_icache.h,
// xfs_bmap.h, xfs_bmap_btree.h, xfs_parent.h, xfs_attr_sf.h, scrub/scrub.h,
// scrub/common.h, scrub/repair.h, scrub/trace.h, scrub/orphanage.h,
// scrub/readdir.h, and linux/namei.h.

/*
 * The Orphanage
 * =============
 *
 * If the directory tree is damaged, children of that directory become
 * inaccessible via that file path.  If a child has no other parents, the file
 * is said to be orphaned.  xfs_repair fixes this situation by creating a
 * orphanage directory (specifically, /lost+found) and creating a directory
 * entry pointing to the orphaned file.
 *
 * Online repair follows this tactic by creating a root-owned /lost+found
 * directory if one does not exist.  If an orphan is found, it will move that
 * files into orphanage.
 */

/* Make the orphanage owned by root. */
unsafe fn xrep_chown_orphanage(sc: *mut xfs_scrub, dp: *mut xfs_inode) -> i32 {
    let mut tp: *mut xfs_trans = core::ptr::null_mut();
    let mp = (*sc).mp;
    let (mut udqp, mut gdqp, mut pdqp): (*mut xfs_dquot, *mut xfs_dquot, *mut xfs_dquot) = (core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    let (mut oldu, mut oldg, mut oldp): (*mut xfs_dquot, *mut xfs_dquot, *mut xfs_dquot) = (core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    let inode = VFS_I(dp);
    let mut error = xfs_qm_vop_dqalloc(dp, GLOBAL_ROOT_UID, GLOBAL_ROOT_GID, 0, XFS_QMOPT_QUOTALL, &mut udqp, &mut gdqp, &mut pdqp);
    if error != 0 { return error; }
    error = xfs_trans_alloc_ichange(dp, udqp, gdqp, pdqp, true, &mut tp);
    if error != 0 { xfs_qm_dqrele(udqp); xfs_qm_dqrele(gdqp); xfs_qm_dqrele(pdqp); return error; }
    (*inode).i_mode &= !(S_ISUID | S_ISGID | S_ISVTX);
    if !uid_eq((*inode).i_uid, GLOBAL_ROOT_UID) {
        if XFS_IS_UQUOTA_ON(mp) { oldu = xfs_qm_vop_chown(tp, dp, &mut (*dp).i_udquot, udqp); }
        (*inode).i_uid = GLOBAL_ROOT_UID;
    }
    if !gid_eq((*inode).i_gid, GLOBAL_ROOT_GID) {
        if XFS_IS_GQUOTA_ON(mp) { oldg = xfs_qm_vop_chown(tp, dp, &mut (*dp).i_gdquot, gdqp); }
        (*inode).i_gid = GLOBAL_ROOT_GID;
    }
    if (*dp).i_projid != 0 {
        if XFS_IS_PQUOTA_ON(mp) { oldp = xfs_qm_vop_chown(tp, dp, &mut (*dp).i_pdquot, pdqp); }
        (*dp).i_projid = 0;
    }
    (*dp).i_diflags &= !(XFS_DIFLAG_REALTIME | XFS_DIFLAG_RTINHERIT);
    xfs_trans_log_inode(tp, dp, XFS_ILOG_CORE);
    XFS_STATS_INC(mp, xs_ig_attrchg);
    if xfs_has_wsync(mp) { xfs_trans_set_sync(tp); }
    error = xfs_trans_commit(tp);
    xfs_qm_dqrele(oldu); xfs_qm_dqrele(oldg); xfs_qm_dqrele(oldp);
    xfs_qm_dqrele(udqp); xfs_qm_dqrele(gdqp); xfs_qm_dqrele(pdqp);
    error
}

pub const ORPHANAGE: &[u8] = b"lost+found\0";

pub unsafe fn xrep_orphanage_create(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp;
    let root_inode = VFS_I((*mp).m_rootip);
    let mut error: i32;
    if xfs_is_shutdown(mp) { return -EIO; }
    if xfs_is_readonly(mp) { (*sc).orphanage = core::ptr::null_mut(); return 0; }
    ASSERT((*sc).tp == core::ptr::null_mut()); ASSERT((*sc).orphanage == core::ptr::null_mut());
    let root_dentry = d_find_alias(root_inode);
    if root_dentry.is_null() { return -EFSCORRUPTED; }
    if !d_is_dir(root_dentry) { dput(root_dentry); return -EFSCORRUPTED; }
    let mut orphanage_dentry = start_creating_noperm(root_dentry, &QSTR(ORPHANAGE));
    if IS_ERR(orphanage_dentry) { error = PTR_ERR(orphanage_dentry); dput(root_dentry); return error; }
    if d_really_is_negative(orphanage_dentry) {
        orphanage_dentry = vfs_mkdir(&nop_mnt_idmap, root_inode, orphanage_dentry, 0o750, core::ptr::null_mut());
        if IS_ERR(orphanage_dentry) { error = PTR_ERR(orphanage_dentry); end_creating(orphanage_dentry); dput(root_dentry); return error; }
    }
    if !d_is_dir(orphanage_dentry) { end_creating(orphanage_dentry); dput(root_dentry); return -ENOTDIR; }
    let orphanage_inode = igrab(d_inode(orphanage_dentry));
    if orphanage_inode.is_null() { end_creating(orphanage_dentry); dput(root_dentry); return -ENOENT; }
    error = xrep_chown_orphanage(sc, XFS_I(orphanage_inode));
    if error != 0 { end_creating(orphanage_dentry); dput(root_dentry); return error; }
    (*sc).orphanage = XFS_I(orphanage_inode); (*sc).orphanage_ilock_flags = 0;
    end_creating(orphanage_dentry); dput(root_dentry); error
}

pub unsafe fn xrep_orphanage_ilock(sc: *mut xfs_scrub, ilock_flags: u32) { (*sc).orphanage_ilock_flags |= ilock_flags; xfs_ilock((*sc).orphanage, ilock_flags); }
pub unsafe fn xrep_orphanage_ilock_nowait(sc: *mut xfs_scrub, ilock_flags: u32) -> bool { if xfs_ilock_nowait((*sc).orphanage, ilock_flags) { (*sc).orphanage_ilock_flags |= ilock_flags; true } else { false } }
pub unsafe fn xrep_orphanage_iunlock(sc: *mut xfs_scrub, ilock_flags: u32) { xfs_iunlock((*sc).orphanage, ilock_flags); (*sc).orphanage_ilock_flags &= !ilock_flags; }

pub unsafe fn xrep_orphanage_iolock_two(sc: *mut xfs_scrub) -> i32 {
    let mut error = 0;
    loop {
        if xchk_should_terminate(sc, &mut error) { return error; }
        if xrep_orphanage_ilock_nowait(sc, XFS_IOLOCK_EXCL) {
            if xchk_ilock_nowait(sc, XFS_IOLOCK_EXCL) { break; }
            xrep_orphanage_iunlock(sc, XFS_IOLOCK_EXCL);
        }
        delay(1);
    }
    0
}

pub unsafe fn xrep_orphanage_rele(sc: *mut xfs_scrub) {
    if (*sc).orphanage.is_null() { return; }
    if (*sc).orphanage_ilock_flags != 0 { xfs_iunlock((*sc).orphanage, (*sc).orphanage_ilock_flags); }
    xchk_irele(sc, (*sc).orphanage); (*sc).orphanage = core::ptr::null_mut();
}

pub unsafe fn xrep_orphanage_can_adopt(sc: *mut xfs_scrub) -> bool {
    ASSERT(!(*sc).ip.is_null());
    !(*sc).orphanage.is_null() && (*sc).ip != (*sc).orphanage && !xchk_inode_is_sb_rooted((*sc).ip) && !xfs_is_internal_inode((*sc).ip)
}

// The remaining adoption routines retain the C ABI and operation ordering;
// their external XFS helper types and functions are supplied by dependencies.
pub unsafe fn xrep_adoption_trans_alloc(sc: *mut xfs_scrub, adopt: *mut xrep_adoption) -> i32 {
    let mp = (*sc).mp; let mut child_blkres = 0; ASSERT((*sc).tp.is_null()); ASSERT(!(*sc).ip.is_null()); ASSERT(!(*sc).orphanage.is_null());
    (*adopt).sc = sc; (*adopt).orphanage_blkres = xfs_link_space_res(mp, MAXNAMELEN);
    if S_ISDIR((*VFS_I((*sc).ip)).i_mode) { child_blkres = xfs_rename_space_res(mp, 0, false, xfs_name_dotdot.len, false); }
    if xfs_has_parent(mp) { child_blkres += XFS_ADDAFORK_SPACE_RES(mp); } (*adopt).child_blkres = child_blkres;
    let mut error = xfs_trans_alloc(mp, &(*M_RES(mp)).tr_link, (*adopt).orphanage_blkres + (*adopt).child_blkres, 0, 0, &mut (*sc).tp);
    if error != 0 { return error; }
    xfs_lock_two_inodes((*sc).orphanage, XFS_ILOCK_EXCL, (*sc).ip, XFS_ILOCK_EXCL); (*sc).ilock_flags |= XFS_ILOCK_EXCL; (*sc).orphanage_ilock_flags |= XFS_ILOCK_EXCL;
    xfs_trans_ijoin((*sc).tp, (*sc).orphanage, 0); xfs_trans_ijoin((*sc).tp, (*sc).ip, 0);
    error = xfs_trans_reserve_quota_nblks((*sc).tp, (*sc).orphanage, (*adopt).orphanage_blkres, 0, true);
    if error == 0 && (*adopt).child_blkres != 0 { error = xfs_trans_reserve_quota_nblks((*sc).tp, (*sc).ip, (*adopt).child_blkres, 0, true); }
    if error != 0 { xchk_trans_cancel(sc); xrep_orphanage_iunlock(sc, XFS_ILOCK_EXCL); xchk_iunlock(sc, XFS_ILOCK_EXCL); }
    error
}

pub unsafe fn xrep_adoption_compute_name(adopt: *mut xrep_adoption, xname: *mut xfs_name) -> i32 {
    let sc = (*adopt).sc; let namebuf = (*xname).name; let mut ino = 0; let mut incr = 0; (*adopt).xname = xname;
    (*xname).len = snprintf(namebuf, MAXNAMELEN, b"%llu\0", I_INO((*sc).ip)); (*xname).type_ = xfs_mode_to_ftype((*VFS_I((*sc).ip)).i_mode);
    let mut error = xchk_dir_lookup(sc, (*sc).orphanage, xname, &mut ino);
    while error == 0 && incr < 10000 { incr += 1; (*xname).len = snprintf(namebuf, MAXNAMELEN, b"%llu.%u\0", I_INO((*sc).ip), incr); error = xchk_dir_lookup(sc, (*sc).orphanage, xname, &mut ino); }
    if error == 0 { return -EFSCORRUPTED; } if error != -ENOENT { return error; } 0
}

unsafe fn xrep_adoption_check_dcache(adopt: *mut xrep_adoption) -> i32 { let sc = (*adopt).sc; let mut d = d_find_alias(VFS_I((*sc).orphanage)); if d.is_null() { return 0; } let child = try_lookup_noperm(&QSTR_INIT((*(*adopt).xname).name, (*(*adopt).xname).len), d); if IS_ERR(child) { dput(d); return PTR_ERR(child); } let mut e = 0; if !child.is_null() { trace_xrep_adoption_check_child((*sc).mp, child); if d_is_positive(child) { ASSERT(d_is_negative(child)); e = -EFSCORRUPTED; } dput(child); } dput(d); e }
unsafe fn xrep_adoption_zap_dcache(adopt: *mut xrep_adoption) { let sc = (*adopt).sc; let d = d_find_alias(VFS_I((*sc).orphanage)); if d.is_null() { return; } let q = QSTR_INIT((*(*adopt).xname).name, (*(*adopt).xname).len); let mut child = try_lookup_noperm(&q, d); while !child.is_null() && !IS_ERR(child) { trace_xrep_adoption_invalidate_child((*sc).mp, child); ASSERT(d_is_negative(child)); d_invalidate(child); dput(child); child = d_lookup(d, &q); } dput(d); let mut child = d_find_alias(VFS_I((*sc).ip)); while !child.is_null() { trace_xrep_adoption_invalidate_child((*sc).mp, child); d_invalidate(child); dput(child); child = d_find_alias(VFS_I((*sc).ip)); } }
unsafe fn xrep_adoption_attr_sizeof(adopt: *const xrep_adoption) -> i32 { (core::mem::size_of::<xfs_attr_sf_hdr>() as i32) + xfs_attr_sf_entsize_byname(core::mem::size_of::<xfs_parent_rec>(), (*(*adopt).xname).len) }

pub unsafe fn xrep_adoption_move(adopt: *mut xrep_adoption) -> i32 {
    let sc = (*adopt).sc; let isdir = S_ISDIR((*VFS_I((*sc).ip)).i_mode); trace_xrep_adoption_reparent((*sc).orphanage, (*adopt).xname, I_INO((*sc).ip));
    let mut error = xrep_adoption_check_dcache(adopt); if error != 0 { return error; }
    if !xfs_inode_has_attr_fork((*sc).ip) && xfs_has_parent((*sc).mp) { error = xfs_bmap_add_attrfork((*sc).tp, (*sc).ip, xrep_adoption_attr_sizeof(adopt), true); if error != 0 { return error; } }
    error = xfs_dir_createname((*sc).tp, (*sc).orphanage, (*adopt).xname, I_INO((*sc).ip), (*adopt).orphanage_blkres); if error != 0 { return error; }
    xfs_trans_ichgtime((*sc).tp, (*sc).orphanage, XFS_ICHGTIME_MOD | XFS_ICHGTIME_CHG); if isdir { xfs_bumplink((*sc).tp, (*sc).orphanage); } xfs_trans_log_inode((*sc).tp, (*sc).orphanage, XFS_ILOG_CORE);
    if (*adopt).bump_child_nlink { xfs_bumplink((*sc).tp, (*sc).ip); xfs_trans_log_inode((*sc).tp, (*sc).ip, XFS_ILOG_CORE); }
    if isdir { error = xfs_dir_replace((*sc).tp, (*sc).ip, &xfs_name_dotdot, I_INO((*sc).orphanage), (*adopt).child_blkres); if error != 0 { return error; } }
    if xfs_has_parent((*sc).mp) { error = xfs_parent_addname((*sc).tp, &mut (*adopt).ppargs, (*sc).orphanage, (*adopt).xname, (*sc).ip); if error != 0 { return error; } }
    xfs_dir_update_hook((*sc).orphanage, (*sc).ip, 1, (*adopt).xname); xrep_adoption_zap_dcache(adopt); 0
}

pub unsafe fn xrep_adoption_trans_roll(adopt: *mut xrep_adoption) -> i32 { let sc = (*adopt).sc; trace_xrep_adoption_trans_roll((*sc).orphanage, (*sc).ip, !!((*(*sc).tp).t_flags & XFS_TRANS_DIRTY)); let error = xrep_defer_finish(sc); if error != 0 { return error; } xfs_trans_roll(&mut (*sc).tp) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
