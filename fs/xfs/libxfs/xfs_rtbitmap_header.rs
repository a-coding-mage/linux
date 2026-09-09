// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependency supplied by xfs_rtgroup.h in the original header.

#[repr(C)]
pub struct xfs_rtalloc_args {
    pub rtg: *mut xfs_rtgroup,
    pub mp: *mut xfs_mount,
    pub tp: *mut xfs_trans,
    pub rbmbp: *mut xfs_buf, // bitmap block buffer
    pub sumbp: *mut xfs_buf, // summary block buffer
    pub rbmoff: xfs_fileoff_t, // bitmap block number
    pub sumoff: xfs_fileoff_t, // summary block number
}

#[inline]
pub unsafe fn xfs_rtx_to_rtb(rtg: *mut xfs_rtgroup, rtx: xfs_rtxnum_t) -> xfs_rtblock_t {
    let mp = rtg_mount(rtg);
    let start = xfs_group_start_fsb(rtg_group(rtg));
    if (*mp).m_rtxblklog >= 0 {
        start + (rtx << (*mp).m_rtxblklog)
    } else {
        start + (rtx * (*mp).m_sb.sb_rextsize)
    }
}

#[inline]
pub unsafe fn xfs_rgbno_to_rtx(mp: *mut xfs_mount, rgbno: xfs_rgblock_t) -> xfs_rtxnum_t {
    if (*mp).m_rtxblklog >= 0 { rgbno >> (*mp).m_rtxblklog } else { rgbno / (*mp).m_sb.sb_rextsize }
}

#[inline]
pub unsafe fn xfs_rtbxlen_to_blen(mp: *mut xfs_mount, rtbxlen: xfs_rtbxlen_t) -> u64 {
    if (*mp).m_rtxblklog >= 0 { rtbxlen << (*mp).m_rtxblklog } else { rtbxlen * (*mp).m_sb.sb_rextsize }
}

#[inline]
pub unsafe fn xfs_rtxlen_to_extlen(mp: *mut xfs_mount, rtxlen: xfs_rtxlen_t) -> xfs_extlen_t {
    if (*mp).m_rtxblklog >= 0 { rtxlen << (*mp).m_rtxblklog } else { rtxlen * (*mp).m_sb.sb_rextsize }
}

/* Compute the misalignment between an extent length and a realtime extent. */
#[inline]
pub unsafe fn xfs_extlen_to_rtxmod(mp: *mut xfs_mount, len: xfs_extlen_t) -> u32 {
    if (*mp).m_rtxblklog >= 0 { len & (*mp).m_rtxblkmask } else { len % (*mp).m_sb.sb_rextsize }
}

#[inline]
pub unsafe fn xfs_extlen_to_rtxlen(mp: *mut xfs_mount, len: xfs_extlen_t) -> xfs_rtxlen_t {
    if (*mp).m_rtxblklog >= 0 { len >> (*mp).m_rtxblklog } else { len / (*mp).m_sb.sb_rextsize }
}

#[inline]
pub unsafe fn xfs_blen_to_rtbxlen(mp: *mut xfs_mount, blen: u64) -> xfs_rtbxlen_t {
    if (*mp).m_rtxblklog >= 0 { blen >> (*mp).m_rtxblklog } else { div_u64(blen, (*mp).m_sb.sb_rextsize) }
}

#[inline]
pub unsafe fn xfs_blen_to_rtxoff(mp: *mut xfs_mount, mut blen: xfs_filblks_t) -> xfs_extlen_t {
    if (*mp).m_rtxblklog >= 0 { blen & (*mp).m_rtxblkmask } else { do_div(&mut blen, (*mp).m_sb.sb_rextsize) }
}

#[inline]
pub unsafe fn xfs_blen_roundup_rtx(mp: *mut xfs_mount, blen: xfs_filblks_t) -> xfs_filblks_t {
    roundup_64(blen, (*mp).m_sb.sb_rextsize)
}

#[inline]
pub unsafe fn xfs_rtb_to_rtx(mp: *mut xfs_mount, mut rtbno: xfs_rtblock_t) -> xfs_rtxnum_t {
    rtbno &= (*mp).m_groups[XG_TYPE_RTG].blkmask;
    if (*mp).m_rtxblklog >= 0 { rtbno >> (*mp).m_rtxblklog } else { div_u64(rtbno, (*mp).m_sb.sb_rextsize) }
}

#[inline]
pub unsafe fn xfs_rgbno_to_rtxoff(mp: *mut xfs_mount, rgbno: xfs_rgblock_t) -> xfs_extlen_t { rgbno % (*mp).m_sb.sb_rextsize }

#[inline]
pub unsafe fn xfs_rtb_to_rtxoff(mp: *mut xfs_mount, mut rtbno: xfs_rtblock_t) -> xfs_extlen_t {
    rtbno &= (*mp).m_groups[XG_TYPE_RTG].blkmask;
    if (*mp).m_rtxblklog >= 0 { rtbno & (*mp).m_rtxblkmask } else { do_div(&mut rtbno, (*mp).m_sb.sb_rextsize) }
}

#[inline]
pub unsafe fn xfs_fileoff_roundup_rtx(mp: *mut xfs_mount, off: xfs_fileoff_t) -> xfs_rtblock_t { roundup_64(off, (*mp).m_sb.sb_rextsize) }
#[inline]
pub unsafe fn xfs_fileoff_rounddown_rtx(mp: *mut xfs_mount, off: xfs_fileoff_t) -> xfs_rtblock_t { rounddown_64(off, (*mp).m_sb.sb_rextsize) }

#[inline]
pub unsafe fn xfs_rtx_to_rbmblock(mp: *mut xfs_mount, rtx: xfs_rtxnum_t) -> xfs_fileoff_t {
    if xfs_has_rtgroups(mp) { div_u64(rtx, (*mp).m_rtx_per_rbmblock) } else { rtx >> (*mp).m_blkbit_log }
}

#[inline]
pub unsafe fn xfs_rtx_to_rbmword(mp: *mut xfs_mount, rtx: xfs_rtxnum_t) -> u32 {
    if xfs_has_rtgroups(mp) {
        let mut modulo = 0;
        div_u64_rem(rtx >> XFS_NBWORDLOG, (*mp).m_blockwsize, &mut modulo);
        modulo
    } else { (rtx >> XFS_NBWORDLOG) & ((*mp).m_blockwsize - 1) }
}

#[inline]
pub unsafe fn xfs_rbmblock_to_rtx(mp: *mut xfs_mount, rbmoff: xfs_fileoff_t) -> xfs_rtxnum_t {
    if xfs_has_rtgroups(mp) { rbmoff * (*mp).m_rtx_per_rbmblock } else { rbmoff << (*mp).m_blkbit_log }
}

#[inline]
pub unsafe fn xfs_rbmblock_wordptr(args: *mut xfs_rtalloc_args, index: u32) -> *mut xfs_rtword_raw {
    let mp = (*args).mp;
    let hdr = (*(*args).rbmbp).b_addr as *mut xfs_rtbuf_blkinfo;
    let words = if xfs_has_rtgroups(mp) { hdr.add(1) as *mut xfs_rtword_raw } else { (*(*args).rbmbp).b_addr as *mut xfs_rtword_raw };
    words.add(index as usize)
}

#[inline]
pub unsafe fn xfs_rtbitmap_getword(args: *mut xfs_rtalloc_args, index: u32) -> xfs_rtword_t {
    let word = xfs_rbmblock_wordptr(args, index);
    if xfs_has_rtgroups((*args).mp) { be32_to_cpu((*word).rtg) } else { (*word).old }
}

#[inline]
pub unsafe fn xfs_rtbitmap_setword(args: *mut xfs_rtalloc_args, index: u32, value: xfs_rtword_t) {
    let word = xfs_rbmblock_wordptr(args, index);
    if xfs_has_rtgroups((*args).mp) { (*word).rtg = cpu_to_be32(value); } else { (*word).old = value; }
}

#[inline]
pub unsafe fn xfs_rtsumoffs(mp: *mut xfs_mount, log2_len: i32, rbmoff: xfs_fileoff_t) -> xfs_rtsumoff_t { log2_len * (*mp).m_sb.sb_rbmblocks + rbmoff }
#[inline]
pub unsafe fn xfs_rtsumoffs_to_block(mp: *mut xfs_mount, rsumoff: xfs_rtsumoff_t) -> xfs_fileoff_t {
    if xfs_has_rtgroups(mp) { rsumoff / (*mp).m_blockwsize } else { XFS_B_TO_FSBT(mp, rsumoff * core::mem::size_of::<xfs_suminfo_t>()) }
}
#[inline]
pub unsafe fn xfs_rtsumoffs_to_infoword(mp: *mut xfs_mount, rsumoff: xfs_rtsumoff_t) -> u32 {
    let mask = (*mp).m_blockmask >> XFS_SUMINFOLOG;
    if xfs_has_rtgroups(mp) { rsumoff % (*mp).m_blockwsize } else { rsumoff & mask }
}

#[inline]
pub unsafe fn xfs_rsumblock_infoptr(args: *mut xfs_rtalloc_args, index: u32) -> *mut xfs_suminfo_raw {
    let hdr = (*(*args).sumbp).b_addr as *mut xfs_rtbuf_blkinfo;
    let info = if xfs_has_rtgroups((*args).mp) { hdr.add(1) as *mut xfs_suminfo_raw } else { (*(*args).sumbp).b_addr as *mut xfs_suminfo_raw };
    info.add(index as usize)
}

#[inline]
pub unsafe fn xfs_suminfo_get(args: *mut xfs_rtalloc_args, index: u32) -> xfs_suminfo_t {
    let info = xfs_rsumblock_infoptr(args, index);
    if xfs_has_rtgroups((*args).mp) { be32_to_cpu((*info).rtg) } else { (*info).old }
}

#[inline]
pub unsafe fn xfs_suminfo_add(args: *mut xfs_rtalloc_args, index: u32, delta: i32) -> xfs_suminfo_t {
    let info = xfs_rsumblock_infoptr(args, index);
    if xfs_has_rtgroups((*args).mp) { be32_add_cpu(&mut (*info).rtg, delta); be32_to_cpu((*info).rtg) } else { (*info).old += delta; (*info).old }
}

#[inline]
pub unsafe fn xfs_rtblock_ops(mp: *mut xfs_mount, type_: xfs_rtg_inodes) -> *const xfs_buf_ops {
    if xfs_has_rtgroups(mp) { if type_ == XFS_RTGI_SUMMARY { &xfs_rtsummary_buf_ops } else { &xfs_rtbitmap_buf_ops } } else { &xfs_rtbuf_ops }
}

#[repr(C)]
pub struct xfs_rtalloc_rec { pub ar_startext: xfs_rtxnum_t, pub ar_extcount: xfs_rtbxlen_t }
pub type xfs_rtalloc_query_range_fn = unsafe extern "C" fn(*mut xfs_rtgroup, *mut xfs_trans, *const xfs_rtalloc_rec, *mut core::ffi::c_void) -> i32;

#[cfg(feature = "CONFIG_XFS_RT")]
extern "C" {
    pub fn xfs_rtbuf_cache_relse(args: *mut xfs_rtalloc_args);
    pub fn xfs_rtbitmap_read_buf(args: *mut xfs_rtalloc_args, block: xfs_fileoff_t) -> i32;
    pub fn xfs_rtsummary_read_buf(args: *mut xfs_rtalloc_args, block: xfs_fileoff_t) -> i32;
    pub fn xfs_rtcheck_range(args: *mut xfs_rtalloc_args, start: xfs_rtxnum_t, len: xfs_rtxlen_t, val: i32, new_: *mut xfs_rtxnum_t, stat: *mut i32) -> i32;
    pub fn xfs_rtfind_back(args: *mut xfs_rtalloc_args, start: xfs_rtxnum_t, rtblock: *mut xfs_rtxnum_t) -> i32;
    pub fn xfs_rtfind_forw(args: *mut xfs_rtalloc_args, start: xfs_rtxnum_t, limit: xfs_rtxnum_t, rtblock: *mut xfs_rtxnum_t) -> i32;
    pub fn xfs_rtmodify_range(args: *mut xfs_rtalloc_args, start: xfs_rtxnum_t, len: xfs_rtxlen_t, val: i32) -> i32;
    pub fn xfs_rtget_summary(args: *mut xfs_rtalloc_args, log: i32, bbno: xfs_fileoff_t, sum: *mut xfs_suminfo_t) -> i32;
    pub fn xfs_rtmodify_summary(args: *mut xfs_rtalloc_args, log: i32, bbno: xfs_fileoff_t, delta: i32) -> i32;
    pub fn xfs_rtfree_range(args: *mut xfs_rtalloc_args, start: xfs_rtxnum_t, len: xfs_rtxlen_t) -> i32;
    pub fn xfs_rtalloc_query_range(rtg: *mut xfs_rtgroup, tp: *mut xfs_trans, start: xfs_rtxnum_t, end: xfs_rtxnum_t, fn_: xfs_rtalloc_query_range_fn, priv_: *mut core::ffi::c_void) -> i32;
    pub fn xfs_rtalloc_query_all(rtg: *mut xfs_rtgroup, tp: *mut xfs_trans, fn_: xfs_rtalloc_query_range_fn, priv_: *mut core::ffi::c_void) -> i32;
    pub fn xfs_rtalloc_extent_is_free(rtg: *mut xfs_rtgroup, tp: *mut xfs_trans, start: xfs_rtxnum_t, len: xfs_rtxlen_t, is_free: *mut bool) -> i32;
    pub fn xfs_rtfree_extent(tp: *mut xfs_trans, rtg: *mut xfs_rtgroup, start: xfs_rtxnum_t, len: xfs_rtxlen_t) -> i32;
    pub fn xfs_rtfree_blocks(tp: *mut xfs_trans, rtg: *mut xfs_rtgroup, rtbno: xfs_fsblock_t, rtlen: xfs_filblks_t) -> i32;
    pub fn xfs_rtbitmap_rtx_per_rbmblock(mp: *mut xfs_mount) -> xfs_rtxnum_t;
    pub fn xfs_rtbitmap_blockcount(mp: *mut xfs_mount) -> xfs_filblks_t;
    pub fn xfs_rtbitmap_blockcount_len(mp: *mut xfs_mount, rtextents: xfs_rtbxlen_t) -> xfs_filblks_t;
    pub fn xfs_rtsummary_blockcount(mp: *mut xfs_mount, rsumlevels: *mut u32) -> xfs_filblks_t;
    pub fn xfs_rtfile_initialize_blocks(rtg: *mut xfs_rtgroup, type_: xfs_rtg_inodes, offset_fsb: xfs_fileoff_t, end_fsb: xfs_fileoff_t, data: *mut core::ffi::c_void) -> i32;
    pub fn xfs_rtbitmap_create(rtg: *mut xfs_rtgroup, ip: *mut xfs_inode, tp: *mut xfs_trans, init: bool) -> i32;
    pub fn xfs_rtsummary_create(rtg: *mut xfs_rtgroup, ip: *mut xfs_inode, tp: *mut xfs_trans, init: bool) -> i32;
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
#[inline]
pub unsafe fn xfs_rtfree_blocks(_: *mut xfs_trans, _: *mut xfs_rtgroup, _: xfs_fsblock_t, _: xfs_filblks_t) -> i32 { -ENOSYS }

#[cfg(not(feature = "CONFIG_XFS_RT"))]
#[inline]
pub unsafe fn xfs_rtbitmap_blockcount_len(_: *mut xfs_mount, _: xfs_rtbxlen_t) -> xfs_filblks_t { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
