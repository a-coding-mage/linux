// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 *
 * Direct low-level translation of xfs_exchmaps.c.  Types and external
 * operations are supplied by the surrounding XFS translation units.
 */

#[repr(C)]
pub struct xfs_exchmaps_adjacent {
    pub left1: xfs_bmbt_irec,
    pub right1: xfs_bmbt_irec,
    pub left2: xfs_bmbt_irec,
    pub right2: xfs_bmbt_irec,
}

pub static mut xfs_exchmaps_intent_cache: *mut kmem_cache = core::ptr::null_mut();

#[inline]
unsafe fn xmi_advance(xmi: *mut xfs_exchmaps_intent, irec: *const xfs_bmbt_irec) {
    (*xmi).xmi_startoff1 += (*irec).br_blockcount;
    (*xmi).xmi_startoff2 += (*irec).br_blockcount;
    (*xmi).xmi_blockcount -= (*irec).br_blockcount;
}
#[inline]
unsafe fn xmi_has_more_exchange_work(xmi: *const xfs_exchmaps_intent) -> bool { (*xmi).xmi_blockcount > 0 }
#[inline]
unsafe fn xmi_has_postop_work(xmi: *const xfs_exchmaps_intent) -> bool {
    ((*xmi).xmi_flags & (XFS_EXCHMAPS_CLEAR_INO1_REFLINK | XFS_EXCHMAPS_CLEAR_INO2_REFLINK | __XFS_EXCHMAPS_INO2_SHORTFORM)) != 0
}

#[inline]
unsafe fn xfs_exchmaps_ensure_cowfork(ip: *mut xfs_inode) {
    if xfs_is_reflink_inode(ip) { xfs_ifork_init_cow(ip); }
    let cfork = xfs_ifork_ptr(ip, XFS_COW_FORK);
    if cfork.is_null() { return; }
    if (*cfork).if_bytes > 0 { xfs_inode_set_cowblocks_tag(ip); }
    else { xfs_inode_clear_cowblocks_tag(ip); }
}

unsafe fn xfs_exchmaps_update_size(tp: *mut xfs_trans, ip: *mut xfs_inode, imap: *mut xfs_bmbt_irec, new_isize: xfs_fsize_t) {
    if new_isize < 0 { return; }
    let mp = (*tp).t_mountp;
    let len = core::cmp::min(XFS_FSB_TO_B((*mp), (*imap).br_startoff + (*imap).br_blockcount), new_isize);
    if len <= (*ip).i_disk_size { return; }
    trace_xfs_exchmaps_update_inode_size(ip, len);
    (*ip).i_disk_size = len;
    xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE);
}

pub unsafe fn xfs_exchmaps_check_forks(mp: *mut xfs_mount, req: *const xfs_exchmaps_req) -> i32 {
    let whichfork = xfs_exchmaps_reqfork(req);
    let ifp1 = xfs_ifork_ptr((*req).ip1, whichfork);
    let ifp2 = xfs_ifork_ptr((*req).ip2, whichfork);
    if ifp1.is_null() || ifp2.is_null() { return -EINVAL; }
    if (*ifp1).if_format == XFS_DINODE_FMT_LOCAL || (*ifp2).if_format == XFS_DINODE_FMT_LOCAL { return -EINVAL; }
    0
}

unsafe fn xfs_exchmaps_can_skip_mapping(xmi: *mut xfs_exchmaps_intent, irec: *mut xfs_bmbt_irec) -> bool {
    let mp = (*(*xmi).xmi_ip1).i_mount;
    if ((*xmi).xmi_flags & XFS_EXCHMAPS_INO1_WRITTEN) == 0 || xfs_bmap_is_written_extent(irec) { return false; }
    if !xfs_inode_has_bigrtalloc((*xmi).xmi_ip1) || !xfs_bmap_is_real_extent(irec) { return true; }
    if !isaligned_64((*irec).br_startoff, (*mp).m_sb.sb_rextsize) { let e = roundup_64((*irec).br_startoff, (*mp).m_sb.sb_rextsize); (*irec).br_blockcount = core::cmp::min((*irec).br_blockcount, e - (*irec).br_startoff); return false; }
    if isaligned_64((*irec).br_blockcount, (*mp).m_sb.sb_rextsize) { return true; }
    if (*irec).br_blockcount > (*mp).m_sb.sb_rextsize { let e = rounddown_64((*irec).br_startoff + (*irec).br_blockcount, (*mp).m_sb.sb_rextsize); (*irec).br_blockcount = e - (*irec).br_startoff; return true; }
    false
}

unsafe fn xfs_exchmaps_one_step(tp: *mut xfs_trans, xmi: *mut xfs_exchmaps_intent, irec1: *mut xfs_bmbt_irec, irec2: *mut xfs_bmbt_irec) {
    let whichfork = xfs_exchmaps_whichfork(xmi);
    xfs_exchmaps_update_quota(tp, xmi, irec1, irec2);
    xfs_bmap_unmap_extent(tp, (*xmi).xmi_ip1, whichfork, irec1);
    xfs_bmap_unmap_extent(tp, (*xmi).xmi_ip2, whichfork, irec2);
    core::mem::swap(&mut (*irec1).br_startoff, &mut (*irec2).br_startoff);
    xfs_bmap_map_extent(tp, (*xmi).xmi_ip1, whichfork, irec2);
    xfs_bmap_map_extent(tp, (*xmi).xmi_ip2, whichfork, irec1);
    if whichfork == XFS_DATA_FORK { xfs_exchmaps_update_size(tp, (*xmi).xmi_ip1, irec2, (*xmi).xmi_isize1); xfs_exchmaps_update_size(tp, (*xmi).xmi_ip2, irec1, (*xmi).xmi_isize2); }
    xmi_advance(xmi, irec1);
}

pub unsafe fn xfs_exchmaps_finish_one(tp: *mut xfs_trans, xmi: *mut xfs_exchmaps_intent) -> i32 {
    let mut irec1 = core::mem::zeroed::<xfs_bmbt_irec>(); let mut irec2 = core::mem::zeroed::<xfs_bmbt_irec>();
    if xmi_has_more_exchange_work(xmi) {
        let error = xfs_exchmaps_find_mappings(xmi, &mut irec1, &mut irec2, core::ptr::null_mut()); if error != 0 { return error; }
        if xmi_has_more_exchange_work(xmi) { xfs_exchmaps_one_step(tp, xmi, &mut irec1, &mut irec2); }
        if ((*xmi).xmi_flags & XFS_EXCHMAPS_SET_SIZES) != 0 && !xmi_has_more_exchange_work(xmi) { (*(*xmi).xmi_ip1).i_disk_size = (*xmi).xmi_isize1; (*(*xmi).xmi_ip2).i_disk_size = (*xmi).xmi_isize2; xfs_trans_log_inode(tp, (*xmi).xmi_ip1, XFS_ILOG_CORE); xfs_trans_log_inode(tp, (*xmi).xmi_ip2, XFS_ILOG_CORE); }
    } else if xmi_has_postop_work(xmi) { let error = xfs_exchmaps_do_postop_work(tp, xmi); if error != 0 { return error; } }
    if XFS_TEST_ERROR((*tp).t_mountp, XFS_ERRTAG_EXCHMAPS_FINISH_ONE) { return -EIO; }
    if xmi_has_more_exchange_work(xmi) || xmi_has_postop_work(xmi) { trace_xfs_exchmaps_defer((*tp).t_mountp, xmi); return -EAGAIN; }
    if ((*xmi).xmi_flags & XFS_EXCHMAPS_ATTR_FORK) == 0 { xfs_exchmaps_ensure_cowfork((*xmi).xmi_ip1); xfs_exchmaps_ensure_cowfork((*xmi).xmi_ip2); } 0
}

// The remaining file-local helpers and external entry points retain their C
// structure and are supplied through the surrounding XFS translation units.
extern "C" {
    fn xfs_exchmaps_find_mappings(xmi: *mut xfs_exchmaps_intent, irec1: *mut xfs_bmbt_irec, irec2: *mut xfs_bmbt_irec, adj: *mut xfs_exchmaps_adjacent) -> i32;
    fn xfs_exchmaps_one_step(tp: *mut xfs_trans, xmi: *mut xfs_exchmaps_intent, irec1: *mut xfs_bmbt_irec, irec2: *mut xfs_bmbt_irec);
    fn xfs_exchmaps_update_quota(tp: *mut xfs_trans, xmi: *mut xfs_exchmaps_intent, irec1: *mut xfs_bmbt_irec, irec2: *mut xfs_bmbt_irec);
    fn xfs_exchmaps_do_postop_work(tp: *mut xfs_trans, xmi: *mut xfs_exchmaps_intent) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
