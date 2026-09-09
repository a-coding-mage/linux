// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Dependencies supplied by the surrounding XFS translation unit.

/* Set us up with a transaction and an empty context. */
pub unsafe fn xchk_setup_rgsuperblock(
    sc: *mut xfs_scrub,
) -> c_int {
    if xchk_need_intent_drain(sc) {
        xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN);
    }
    xchk_trans_alloc(sc, 0)
}

/* Cross-reference with the other rt metadata. */
unsafe fn xchk_rgsuperblock_xref(
    sc: *mut xfs_scrub,
) {
    if ((*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 {
        return;
    }

    xchk_xref_is_used_rt_space(sc, xfs_rgbno_to_rtb((*sc).sr.rtg, 0), 1);
    xchk_xref_is_only_rt_owned_by(sc, 0, 1, &XFS_RMAP_OINFO_FS);
}

pub unsafe fn xchk_rgsuperblock(
    sc: *mut xfs_scrub,
) -> c_int {
    let rgno: xfs_rgnumber_t = (*(*sc).sm).sm_agno;
    let flags: c_uint;
    let mut error: c_int;

    /*
     * Only rtgroup 0 has a superblock.  We may someday want to use higher
     * rgno for other functions, similar to what we do with the primary
     * super scrub function.
     */
    if rgno != 0 {
        return -ENOENT;
    }

    /*
     * Grab an active reference to the rtgroup structure.  If we can't get
     * it, we're racing with something that's tearing down the group, so
     * signal that the group no longer exists.  Take the rtbitmap in shared
     * mode so that the group can't change while we're doing things.
     */
    error = xchk_rtgroup_init_existing(sc, rgno, &mut (*sc).sr);
    if !xchk_xref_process_error(sc, 0, 0, &mut error) {
        return error;
    }

    if xfs_has_rtrmapbt((*sc).mp) {
        flags = XFS_RTGLOCK_BITMAP | XFS_RTGLOCK_RMAP;
    } else {
        flags = XFS_RTGLOCK_BITMAP_SHARED;
    }

    error = xchk_rtgroup_lock(sc, &mut (*sc).sr, flags);
    if error != 0 {
        return error;
    }

    /*
     * Since we already validated the rt superblock at mount time, we don't
     * need to check its contents again.  All we need is to cross-reference.
     */
    xchk_rgsuperblock_xref(sc);
    0
}

#[cfg(CONFIG_XFS_ONLINE_REPAIR)]
pub unsafe fn xrep_rgsuperblock(
    sc: *mut xfs_scrub,
) -> c_int {
    let sb_bp: *mut xfs_buf;

    ASSERT(rtg_rgno((*sc).sr.rtg) == 0);

    sb_bp = xfs_trans_getsb((*sc).tp);
    xfs_log_sb((*sc).tp);
    xfs_log_rtsb((*sc).tp, sb_bp);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
