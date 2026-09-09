// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
 * Copyright (C) 2017 Oracle.
 * All Rights Reserved.
 */

/* Dependencies are supplied by the surrounding translation unit. */

/*
 * Verify that an AG block number pointer neither points outside the AG
 * nor points at static metadata.
 */
#[inline]
unsafe fn xfs_verify_agno_agbno(
    mp: *mut xfs_mount,
    agno: xfs_agnumber_t,
    agbno: xfs_agblock_t,
) -> bool {
    let eoag: xfs_agblock_t = xfs_ag_block_count(mp, agno);

    if agbno >= eoag {
        return false;
    }
    if agbno <= XFS_AGFL_BLOCK(mp) {
        return false;
    }
    true
}

/*
 * Verify that an FS block number pointer neither points outside the
 * filesystem nor points at static AG metadata.
 */
#[inline]
pub unsafe fn xfs_verify_fsbno(mp: *mut xfs_mount, fsbno: xfs_fsblock_t) -> bool {
    let agno: xfs_agnumber_t = XFS_FSB_TO_AGNO(mp, fsbno);

    if agno >= (*mp).m_sb.sb_agcount {
        return false;
    }
    xfs_verify_agno_agbno(mp, agno, XFS_FSB_TO_AGBNO(mp, fsbno))
}

/*
 * Verify that a data device extent is fully contained inside the filesystem,
 * does not cross an AG boundary, and does not point at static metadata.
 */
pub unsafe fn xfs_verify_fsbext(
    mp: *mut xfs_mount,
    fsbno: xfs_fsblock_t,
    len: xfs_fsblock_t,
) -> bool {
    if fsbno.wrapping_add(len) <= fsbno {
        return false;
    }

    if !xfs_verify_fsbno(mp, fsbno) {
        return false;
    }

    if !xfs_verify_fsbno(mp, fsbno.wrapping_add(len).wrapping_sub(1)) {
        return false;
    }

    XFS_FSB_TO_AGNO(mp, fsbno)
        == XFS_FSB_TO_AGNO(mp, fsbno.wrapping_add(len).wrapping_sub(1))
}

/*
 * Verify that an AG inode number pointer neither points outside the AG
 * nor points at static metadata.
 */
#[inline]
unsafe fn xfs_verify_agno_agino(
    mp: *mut xfs_mount,
    agno: xfs_agnumber_t,
    agino: xfs_agino_t,
) -> bool {
    let mut first: xfs_agino_t = 0;
    let mut last: xfs_agino_t = 0;

    xfs_agino_range(mp, agno, &mut first, &mut last);
    agino >= first && agino <= last
}

/*
 * Verify that an FS inode number pointer neither points outside the
 * filesystem nor points at static AG metadata.
 */
#[inline]
pub unsafe fn xfs_verify_ino(mp: *mut xfs_mount, ino: xfs_ino_t) -> bool {
    let agno: xfs_agnumber_t = XFS_INO_TO_AGNO(mp, ino);
    let agino: xfs_agino_t = XFS_INO_TO_AGINO(mp, ino);

    if agno >= (*mp).m_sb.sb_agcount {
        return false;
    }
    if XFS_AGINO_TO_INO(mp, agno, agino) != ino {
        return false;
    }
    xfs_verify_agno_agino(mp, agno, agino)
}

/* Is this an internal inode number? */
#[inline]
pub unsafe fn xfs_is_sb_inum(mp: *mut xfs_mount, ino: xfs_ino_t) -> bool {
    ino == (*mp).m_sb.sb_rbmino
        || ino == (*mp).m_sb.sb_rsumino
        || (xfs_has_quota(mp) && xfs_is_quota_inode(&mut (*mp).m_sb, ino))
}

/*
 * Verify that a directory entry's inode number doesn't point at an internal
 * inode, empty space, or static AG metadata.
 */
pub unsafe fn xfs_verify_dir_ino(mp: *mut xfs_mount, ino: xfs_ino_t) -> bool {
    if xfs_is_sb_inum(mp, ino) {
        return false;
    }
    xfs_verify_ino(mp, ino)
}

/*
 * Verify that a realtime block number pointer neither points outside the
 * allocatable areas of the rtgroup nor off the end of the realtime device.
 */
#[inline]
pub unsafe fn xfs_verify_rtbno(mp: *mut xfs_mount, rtbno: xfs_rtblock_t) -> bool {
    if xfs_has_rtgroups(mp) {
        let rgno: xfs_rgnumber_t = xfs_rtb_to_rgno(mp, rtbno);
        let rtx: xfs_rtxnum_t = xfs_rtb_to_rtx(mp, rtbno);

        if rgno >= (*mp).m_sb.sb_rgcount {
            return false;
        }
        if rtx >= xfs_rtgroup_extents(mp, rgno) {
            return false;
        }
        if xfs_has_rtsb(mp) && rgno == 0 && rtx == 0 {
            return false;
        }
        return true;
    }

    rtbno < (*mp).m_sb.sb_rblocks
}

/*
 * Verify that an allocated realtime device extent neither points outside
 * allocatable areas of the rtgroup, across an rtgroup boundary, nor off the
 * end of the realtime device.
 */
pub unsafe fn xfs_verify_rtbext(
    mp: *mut xfs_mount,
    rtbno: xfs_rtblock_t,
    len: xfs_filblks_t,
) -> bool {
    if rtbno.wrapping_add(len) <= rtbno {
        return false;
    }

    if !xfs_verify_rtbno(mp, rtbno) {
        return false;
    }

    if !xfs_verify_rtbno(mp, rtbno.wrapping_add(len).wrapping_sub(1)) {
        return false;
    }

    if xfs_has_rtgroups(mp)
        && xfs_rtb_to_rgno(mp, rtbno)
            != xfs_rtb_to_rgno(mp, rtbno.wrapping_add(len).wrapping_sub(1))
    {
        return false;
    }

    true
}

/* Calculate the range of valid icount values. */
#[inline]
pub unsafe fn xfs_icount_range(
    mp: *mut xfs_mount,
    min: *mut u64,
    max: *mut u64,
) {
    let mut nr_inos: u64 = 0;
    let mut pag: *mut xfs_perag = core::ptr::null_mut();

    /* root, rtbitmap, rtsum all live in the first chunk */
    *min = XFS_INODES_PER_CHUNK;

    while {
        pag = xfs_perag_next(mp, pag);
        !pag.is_null()
    } {
        nr_inos = nr_inos.wrapping_add((*pag).agino_max.wrapping_sub((*pag).agino_min).wrapping_add(1));
    }
    *max = nr_inos;
}

/* Sanity-checking of inode counts. */
pub unsafe fn xfs_verify_icount(mp: *mut xfs_mount, icount: u64) -> bool {
    let mut min: u64 = 0;
    let mut max: u64 = 0;

    xfs_icount_range(mp, &mut min, &mut max);
    icount >= min && icount <= max
}

/* Sanity-checking of dir/attr block offsets. */
pub unsafe fn xfs_verify_dablk(_mp: *mut xfs_mount, dabno: xfs_fileoff_t) -> bool {
    let max_dablk: xfs_dablk_t = (-1i32) as xfs_dablk_t;

    dabno <= max_dablk
}

/* Check that a file block offset does not exceed the maximum. */
pub unsafe fn xfs_verify_fileoff(_mp: *mut xfs_mount, off: xfs_fileoff_t) -> bool {
    off <= XFS_MAX_FILEOFF
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
