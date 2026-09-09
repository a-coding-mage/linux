/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Dependency declarations supplied by the surrounding translation unit.

#[repr(C)]
pub enum xfs_rtg_inodes {
    XFS_RTGI_BITMAP,   /* allocation bitmap */
    XFS_RTGI_SUMMARY,  /* allocation summary */
    XFS_RTGI_RMAP,     /* rmap btree inode */
    XFS_RTGI_REFCOUNT, /* refcount btree inode */
    XFS_RTGI_MAX,
}

#[repr(C)]
pub union xfs_rtgroup_rsum_or_zone {
    pub rtg_rsum_cache: *mut u8,
    pub rtg_open_zone: *mut xfs_open_zone,
}

#[repr(C)]
pub struct xfs_rtgroup {
    pub rtg_group: xfs_group,
    /* per-rtgroup metadata inodes */
    pub rtg_inodes: [*mut xfs_inode; XFS_RTGI_MAX as usize],
    /* Number of blocks in this group */
    pub rtg_extents: xfs_rtxnum_t,
    /*
     * For bitmap based RT devices this points to a cache of rt summary
     * level per bitmap block with the invariant that rtg_rsum_cache[bbno]
     * > the maximum i for which rsum[i][bbno] != 0, or 0 if
     * rsum[i][bbno] == 0 for all i.
     * Reads and writes are serialized by the rsumip inode lock.
     *
     * For zoned RT devices this points to the open zone structure for
     * a group that is open for writers, or is NULL.
     */
    pub rtg_rsum_or_zone: xfs_rtgroup_rsum_or_zone,
    /*
     * Count of outstanding GC operations for zoned XFS.  Any RTG with a
     * non-zero rtg_gccount will not be picked as new GC victim.
     */
    pub rtg_gccount: atomic_t,
}

/* For zoned RT devices this is set on groups that have no written blocks. */
pub const XFS_RTG_FREE: u32 = XA_MARK_0;

#[inline]
pub unsafe fn to_rtg(xg: *mut xfs_group) -> *mut xfs_rtgroup {
    container_of!(xg, xfs_rtgroup, rtg_group)
}

#[inline]
pub unsafe fn rtg_group(rtg: *mut xfs_rtgroup) -> *mut xfs_group { &mut (*rtg).rtg_group }

#[inline]
pub unsafe fn rtg_mount(rtg: *const xfs_rtgroup) -> *mut xfs_mount { (*rtg).rtg_group.xg_mount }

#[inline]
pub unsafe fn rtg_rgno(rtg: *const xfs_rtgroup) -> xfs_rgnumber_t { (*rtg).rtg_group.xg_gno }

#[inline]
pub unsafe fn rtg_blocks(rtg: *const xfs_rtgroup) -> xfs_rgblock_t { (*rtg).rtg_group.xg_block_count }

#[inline]
pub unsafe fn rtg_bitmap(rtg: *const xfs_rtgroup) -> *mut xfs_inode { (*rtg).rtg_inodes[XFS_RTGI_BITMAP as usize] }

#[inline]
pub unsafe fn rtg_summary(rtg: *const xfs_rtgroup) -> *mut xfs_inode { (*rtg).rtg_inodes[XFS_RTGI_SUMMARY as usize] }

#[inline]
pub unsafe fn rtg_rmap(rtg: *const xfs_rtgroup) -> *mut xfs_inode { (*rtg).rtg_inodes[XFS_RTGI_RMAP as usize] }

#[inline]
pub unsafe fn rtg_refcount(rtg: *const xfs_rtgroup) -> *mut xfs_inode { (*rtg).rtg_inodes[XFS_RTGI_REFCOUNT as usize] }

/* Passive rtgroup references */
#[inline]
pub unsafe fn xfs_rtgroup_get(mp: *mut xfs_mount, rgno: xfs_rgnumber_t) -> *mut xfs_rtgroup {
    to_rtg(xfs_group_get(mp, rgno, XG_TYPE_RTG))
}
#[inline]
pub unsafe fn xfs_rtgroup_hold(rtg: *mut xfs_rtgroup) -> *mut xfs_rtgroup { to_rtg(xfs_group_hold(rtg_group(rtg))) }
#[inline]
pub unsafe fn xfs_rtgroup_put(rtg: *mut xfs_rtgroup) { xfs_group_put(rtg_group(rtg)); }

/* Active rtgroup references */
#[inline]
pub unsafe fn xfs_rtgroup_grab(mp: *mut xfs_mount, rgno: xfs_rgnumber_t) -> *mut xfs_rtgroup { to_rtg(xfs_group_grab(mp, rgno, XG_TYPE_RTG)) }
#[inline]
pub unsafe fn xfs_rtgroup_rele(rtg: *mut xfs_rtgroup) { xfs_group_rele(rtg_group(rtg)); }

#[inline]
pub unsafe fn xfs_rtgroup_next_range(mp: *mut xfs_mount, rtg: *mut xfs_rtgroup, start_rgno: xfs_rgnumber_t, end_rgno: xfs_rgnumber_t) -> *mut xfs_rtgroup {
    to_rtg(xfs_group_next_range(mp, if !rtg.is_null() { rtg_group(rtg) } else { core::ptr::null_mut() }, start_rgno, end_rgno, XG_TYPE_RTG))
}
#[inline]
pub unsafe fn xfs_rtgroup_next(mp: *mut xfs_mount, rtg: *mut xfs_rtgroup) -> *mut xfs_rtgroup {
    xfs_rtgroup_next_range(mp, rtg, 0, (*mp).m_sb.sb_rgcount - 1)
}

#[inline]
pub unsafe fn xfs_verify_rgbno(rtg: *mut xfs_rtgroup, rgbno: xfs_rgblock_t) -> bool {
    ASSERT!(xfs_has_rtgroups(rtg_mount(rtg))); xfs_verify_gbno(rtg_group(rtg), rgbno)
}
#[inline]
pub unsafe fn xfs_verify_rgbext(rtg: *mut xfs_rtgroup, rgbno: xfs_rgblock_t, len: xfs_extlen_t) -> bool {
    ASSERT!(xfs_has_rtgroups(rtg_mount(rtg))); xfs_verify_gbext(rtg_group(rtg), rgbno, len)
}
#[inline]
pub unsafe fn xfs_rgbno_to_rtb(rtg: *mut xfs_rtgroup, rgbno: xfs_rgblock_t) -> xfs_rtblock_t { xfs_gbno_to_fsb(rtg_group(rtg), rgbno) }
#[inline]
pub unsafe fn xfs_rtb_to_rgno(mp: *mut xfs_mount, rtbno: xfs_rtblock_t) -> xfs_rgnumber_t { xfs_fsb_to_gno(mp, rtbno, XG_TYPE_RTG) }
#[inline]
pub unsafe fn xfs_rtb_to_rgbno(mp: *mut xfs_mount, rtbno: xfs_rtblock_t) -> xfs_rgblock_t { xfs_fsb_to_gbno(mp, rtbno, XG_TYPE_RTG) }

#[inline]
pub unsafe fn xfs_rtbno_is_group_start(mp: *mut xfs_mount, rtbno: xfs_rtblock_t) -> bool {
    (rtbno & (*mp).m_groups[XG_TYPE_RTG as usize].blkmask) == 0
}

#[inline]
pub unsafe fn xfs_rtx_to_rgbno(rtg: *mut xfs_rtgroup, rtx: xfs_rtxnum_t) -> xfs_rgblock_t {
    let mp = rtg_mount(rtg);
    if (*mp).m_rtxblklog >= 0 { rtx << (*mp).m_rtxblklog } else { rtx * (*mp).m_sb.sb_rextsize }
}

#[inline]
pub unsafe fn xfs_rtb_to_daddr(mp: *mut xfs_mount, mut rtbno: xfs_rtblock_t) -> xfs_daddr_t {
    let g = &mut (*mp).m_groups[XG_TYPE_RTG as usize];
    if xfs_has_rtgroups(mp) && !g.has_daddr_gaps {
        let rgno = xfs_rtb_to_rgno(mp, rtbno);
        rtbno = rgno as xfs_rtblock_t * g.blocks + (rtbno & g.blkmask);
    }
    XFS_FSB_TO_BB!(mp, g.start_fsb + rtbno)
}

#[inline]
pub unsafe fn xfs_daddr_to_rtb(mp: *mut xfs_mount, daddr: xfs_daddr_t) -> xfs_rtblock_t {
    let g = &mut (*mp).m_groups[XG_TYPE_RTG as usize];
    let bno = XFS_BB_TO_FSBT!(mp, daddr) - g.start_fsb;
    if xfs_has_rtgroups(mp) && !g.has_daddr_gaps {
        let mut rgbno = 0u32;
        let rgno = div_u64_rem(bno, g.blocks, &mut rgbno);
        return (rgno << g.blklog) + rgbno as u64;
    }
    bno
}

/* CONFIG_XFS_RT declarations are retained as external interfaces. */
#[inline]
pub unsafe fn xfs_rtgs_to_rfsbs(mp: *mut xfs_mount, nr_groups: u32) -> xfs_rfsblock_t {
    xfs_groups_to_rfsbs(mp, nr_groups, XG_TYPE_RTG)
}

#[inline]
pub unsafe fn xfs_rtgroup_raw_size(mp: *mut xfs_mount) -> xfs_rgblock_t {
    let g = &(*mp).m_groups[XG_TYPE_RTG as usize];
    if g.has_daddr_gaps { 1u32 << g.blklog } else { g.blocks }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
