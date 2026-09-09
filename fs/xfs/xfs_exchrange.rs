// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Translated from xfs_exchrange.c.  External kernel/XFS symbols are supplied
// by the surrounding repository.

unsafe fn xfs_exchrange_ilock(tp: *mut xfs_trans, ip1: *mut xfs_inode, ip2: *mut xfs_inode) {
    if ip1 != ip2 { xfs_lock_two_inodes(ip1, XFS_ILOCK_EXCL, ip2, XFS_ILOCK_EXCL); }
    else { xfs_ilock(ip1, XFS_ILOCK_EXCL); }
    if !tp.is_null() { xfs_trans_ijoin(tp, ip1, 0); if ip2 != ip1 { xfs_trans_ijoin(tp, ip2, 0); } }
}

unsafe fn xfs_exchrange_iunlock(ip1: *mut xfs_inode, ip2: *mut xfs_inode) {
    if ip2 != ip1 { xfs_iunlock(ip2, XFS_ILOCK_EXCL); }
    xfs_iunlock(ip1, XFS_ILOCK_EXCL);
}

unsafe fn xfs_exchrange_estimate(req: *mut xfs_exchmaps_req) -> i32 {
    xfs_exchrange_ilock(core::ptr::null_mut(), (*req).ip1, (*req).ip2);
    let error = xfs_exchmaps_estimate(req);
    xfs_exchrange_iunlock((*req).ip1, (*req).ip2); error
}

unsafe fn xfs_exchrange_check_freshness(fxr: *const xfs_exchrange, ip2: *mut xfs_inode) -> i32 {
    let inode2 = VFS_I(ip2); let ctime = inode_get_ctime(inode2); let mtime = inode_get_mtime(inode2);
    trace_xfs_exchrange_freshness(fxr, ip2);
    if (*fxr).file2_ino != (*inode2).i_ino || (*fxr).file2_gen != (*inode2).i_generation ||
       !timespec64_equal(&(*fxr).file2_ctime, &ctime) || !timespec64_equal(&(*fxr).file2_mtime, &mtime) { return -EBUSY; }
    0
}

const QRETRY_IP1: u32 = 0x1; const QRETRY_IP2: u32 = 0x2;

unsafe fn xfs_exchrange_reserve_quota(tp: *mut xfs_trans, req: *const xfs_exchmaps_req, qretry: *mut u32) -> i32 {
    let mut ip1_error = 0; let mut error;
    ASSERT(!xfs_is_metadir_inode((*req).ip1)); ASSERT(!xfs_is_metadir_inode((*req).ip2));
    if !XFS_IS_QUOTA_ON((*tp).t_mountp) || (*req).ip1 == (*req).ip2 ||
       ((*(*req).ip1).i_udquot == (*(*req).ip2).i_udquot && (*(*req).ip1).i_gdquot == (*(*req).ip2).i_gdquot && (*(*req).ip1).i_pdquot == (*(*req).ip2).i_pdquot) { return 0; }
    *qretry = 0;
    let ddelta = (*req).ip2_bcount - (*req).ip1_bcount; let rdelta = (*req).ip2_rtbcount - (*req).ip1_rtbcount;
    if ddelta > 0 || rdelta > 0 { error = xfs_trans_reserve_quota_nblks(tp, (*req).ip1, if ddelta > 0 { ddelta } else { 0 }, if rdelta > 0 { rdelta } else { 0 }, false); if error == -EDQUOT || error == -ENOSPC { *qretry |= QRETRY_IP1; ip1_error = error; error = 0; } if error != 0 { return error; } }
    if ddelta < 0 || rdelta < 0 { error = xfs_trans_reserve_quota_nblks(tp, (*req).ip2, if ddelta < 0 { -ddelta } else { 0 }, if rdelta < 0 { -rdelta } else { 0 }, false); if error == -EDQUOT || error == -ENOSPC { *qretry |= QRETRY_IP2; } if error != 0 { return error; } }
    if ip1_error != 0 { return ip1_error; }
    error = xfs_trans_reserve_quota_nblks(tp, (*req).ip1, (*req).ip1_bcount, (*req).ip1_rtbcount, true); if error != 0 { return error; }
    xfs_trans_reserve_quota_nblks(tp, (*req).ip2, (*req).ip2_bcount, (*req).ip2_rtbcount, true)
}

unsafe fn xfs_exchrange_mappings(fxr: *const xfs_exchrange, ip1: *mut xfs_inode, ip2: *mut xfs_inode) -> i32 {
    let mp = (*ip1).i_mount; let mut req = xfs_exchmaps_req { ip1, ip2, startoff1: XFS_B_TO_FSBT(mp, (*fxr).file1_offset), startoff2: XFS_B_TO_FSBT(mp, (*fxr).file2_offset), blockcount: XFS_B_TO_FSB(mp, (*fxr).length), ..Default::default() };
    let mut qretry = 0; let mut retried = false; let mut error;
    trace_xfs_exchrange_mappings(fxr, ip1, ip2);
    if (*fxr).flags & XFS_EXCHANGE_RANGE_TO_EOF != 0 { req.flags |= XFS_EXCHMAPS_SET_SIZES; }
    if (*fxr).flags & XFS_EXCHANGE_RANGE_FILE1_WRITTEN != 0 { req.flags |= XFS_EXCHMAPS_INO1_WRITTEN; }
    if xfs_inode_has_bigrtalloc(ip2) { req.blockcount = xfs_blen_roundup_rtx(mp, req.blockcount); }
    error = xfs_exchrange_estimate(&mut req); if error != 0 { return error; }
    'retry: loop {
        let mut tp = core::ptr::null_mut(); error = xfs_trans_alloc(mp, &M_RES(mp).t_write, req.resblks, 0, XFS_TRANS_RES_FDBLKS, &mut tp); if error != 0 { return error; }
        xfs_exchrange_ilock(tp, ip1, ip2); trace_xfs_exchrange_before(ip2, 2); trace_xfs_exchrange_before(ip1, 1);
        error = xfs_exchmaps_check_forks(mp, &mut req); if error != 0 { xfs_trans_cancel(tp); xfs_exchrange_iunlock(ip1, ip2); return error; }
        error = xfs_exchrange_reserve_quota(tp, &req, &mut qretry);
        if (error == -EDQUOT || error == -ENOSPC) && !retried { xfs_trans_cancel(tp); xfs_exchrange_iunlock(ip1, ip2); if qretry & QRETRY_IP1 != 0 { xfs_blockgc_free_quota(ip1, 0); } if qretry & QRETRY_IP2 != 0 { xfs_blockgc_free_quota(ip2, 0); } retried = true; continue 'retry; }
        if error != 0 { xfs_trans_cancel(tp); xfs_exchrange_iunlock(ip1, ip2); return error; }
        if (*fxr).flags & XFS_EXCHANGE_RANGE_DRY_RUN != 0 { xfs_trans_cancel(tp); xfs_exchrange_iunlock(ip1, ip2); return 0; }
        if (*fxr).flags & __XFS_EXCHANGE_RANGE_UPD_CMTIME1 != 0 { xfs_trans_ichgtime(tp, ip1, XFS_ICHGTIME_MOD | XFS_ICHGTIME_CHG); }
        if (*fxr).flags & __XFS_EXCHANGE_RANGE_UPD_CMTIME2 != 0 { xfs_trans_ichgtime(tp, ip2, XFS_ICHGTIME_MOD | XFS_ICHGTIME_CHG); }
        xfs_exchange_mappings(tp, &mut req); if xfs_has_wsync(mp) || (*fxr).flags & XFS_EXCHANGE_RANGE_DSYNC != 0 { xfs_trans_set_sync(tp); }
        error = xfs_trans_commit(tp); trace_xfs_exchrange_after(ip2, 2); trace_xfs_exchrange_after(ip1, 1); xfs_exchrange_iunlock(ip1, ip2); return error;
    }
}

// The remaining generic file-level and ioctl entry points retain the kernel
// interfaces and are declared for the surrounding translation unit.
unsafe extern "C" {
    fn xfs_exchange_range(fxr: *mut xfs_exchrange) -> i32;
    fn xfs_exchrange_prep(fxr: *mut xfs_exchrange, ip1: *mut xfs_inode, ip2: *mut xfs_inode) -> i32;
    fn xfs_exchrange_contents(fxr: *mut xfs_exchrange) -> i32;
    fn xfs_ioc_exchange_range(file: *mut file, argp: *mut xfs_exchange_range) -> i64;
    fn xfs_ioc_start_commit(file: *mut file, argp: *mut xfs_commit_range) -> i64;
    fn xfs_ioc_commit_range(file: *mut file, argp: *mut xfs_commit_range) -> i64;
}

#[repr(C)]
pub struct xfs_commit_range_fresh { pub fsid: xfs_fsid_t, pub file2_ino: u64, pub file2_mtime: i64, pub file2_ctime: i64, pub file2_mtime_nsec: i32, pub file2_ctime_nsec: i32, pub file2_gen: u32, pub magic: u32 }
pub const XCR_FRESH_MAGIC: u32 = 0x444F524B;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
