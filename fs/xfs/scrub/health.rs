// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2019-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C dependencies: xfs_platform.h, xfs_fs.h, xfs_shared.h, xfs_format.h,
// xfs_trans_resv.h, xfs_mount.h, xfs_btree.h, xfs_ag.h, xfs_health.h,
// xfs_rtgroup.h, scrub/scrub.h, scrub/health.h, scrub/common.h

/*
 * Scrub and In-Core Filesystem Health Assessments
 * ===============================================
 *
 * Online scrub and repair have the time and the ability to perform stronger
 * checks than we can do from the metadata verifiers, because they can
 * cross-reference records between data structures.  Therefore, scrub is in a
 * good position to update the online filesystem health assessments to reflect
 * the good/bad state of the data structure.
 *
 * We therefore extend scrub in the following ways to achieve this:
 *
 * 1. Create a "sick_mask" field in the scrub context.  When we're setting up a
 * scrub call, set this to the default XFS_SICK_* flag(s) for the selected
 * scrub type (call it A).  Scrub and repair functions can override the default
 * sick_mask value if they choose.
 *
 * 2. If the scrubber returns a runtime error code, we exit making no changes
 * to the incore sick state.
 *
 * 3. If the scrubber finds that A is clean, use sick_mask to clear the incore
 * sick flags before exiting.
 *
 * 4. If the scrubber finds that A is corrupt, use sick_mask to set the incore
 * sick flags.  If the user didn't want to repair then we exit, leaving the
 * metadata structure unfixed and the sick flag set.
 *
 * 5. Now we know that A is corrupt and the user wants to repair, so run the
 * repairer.  If the repairer returns an error code, we exit with that error
 * code, having made no further changes to the incore sick state.
 *
 * 6. If repair rebuilds A correctly and the subsequent re-scrub of A is clean,
 * use sick_mask to clear the incore sick flags.  This should have the effect
 * that A is no longer marked sick.
 *
 * 7. If repair rebuilds A incorrectly, the re-scrub will find it corrupt and
 * use sick_mask to set the incore sick flags.  This should have no externally
 * visible effect since we already set them in step (4).
 *
 * There are some complications to this story, however.  For certain types of
 * complementary metadata indices (e.g. inobt/finobt), it is easier to rebuild
 * both structures at the same time.  The following principles apply to this
 * type of repair strategy:
 *
 * 8. Any repair function that rebuilds multiple structures should update
 * sick_mask_visible to reflect whatever other structures are rebuilt, and
 * verify that all the rebuilt structures can pass a scrub check.  The outcomes
 * of 5-7 still apply, but with a sick_mask that covers everything being
 * rebuilt.
 */

/* Map our scrub type to a sick mask and a set of health update functions. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum XchkHealthGroup {
    XHG_NONE = 1,
    XHG_FS,
    XHG_AG,
    XHG_INO,
    XHG_RTGROUP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XchkHealthMap {
    pub group: XchkHealthGroup,
    pub sick_mask: u32,
}

static TYPE_TO_HEALTH_FLAG: [XchkHealthMap; XFS_SCRUB_TYPE_NR as usize] = [
    XchkHealthMap { group: XHG_NONE, sick_mask: 0 },
    XchkHealthMap { group: XHG_AG, sick_mask: XFS_SICK_AG_SB },
    XchkHealthMap { group: XHG_AG, sick_mask: XFS_SICK_AG_AGF },
    XchkHealthMap { group: XHG_AG, sick_mask: XFS_SICK_AG_AGFL },
    XchkHealthMap { group: XHG_AG, sick_mask: XFS_SICK_AG_AGI },
    XchkHealthMap { group: XHG_AG, sick_mask: XFS_SICK_AG_BNOBT },
    XchkHealthMap { group: XHG_AG, sick_mask: XFS_SICK_AG_CNTBT },
    XchkHealthMap { group: XHG_AG, sick_mask: XFS_SICK_AG_INOBT },
    XchkHealthMap { group: XHG_AG, sick_mask: XFS_SICK_AG_FINOBT },
    XchkHealthMap { group: XHG_AG, sick_mask: XFS_SICK_AG_RMAPBT },
    XchkHealthMap { group: XHG_AG, sick_mask: XFS_SICK_AG_REFCNTBT },
    XchkHealthMap { group: XHG_INO, sick_mask: XFS_SICK_INO_CORE },
    XchkHealthMap { group: XHG_INO, sick_mask: XFS_SICK_INO_BMBTD },
    XchkHealthMap { group: XHG_INO, sick_mask: XFS_SICK_INO_BMBTA },
    XchkHealthMap { group: XHG_INO, sick_mask: XFS_SICK_INO_BMBTC },
    XchkHealthMap { group: XHG_INO, sick_mask: XFS_SICK_INO_DIR },
    XchkHealthMap { group: XHG_INO, sick_mask: XFS_SICK_INO_XATTR },
    XchkHealthMap { group: XHG_INO, sick_mask: XFS_SICK_INO_SYMLINK },
    XchkHealthMap { group: XHG_INO, sick_mask: XFS_SICK_INO_PARENT },
    XchkHealthMap { group: XHG_RTGROUP, sick_mask: XFS_SICK_RG_BITMAP },
    XchkHealthMap { group: XHG_RTGROUP, sick_mask: XFS_SICK_RG_SUMMARY },
    XchkHealthMap { group: XHG_FS, sick_mask: XFS_SICK_FS_UQUOTA },
    XchkHealthMap { group: XHG_FS, sick_mask: XFS_SICK_FS_GQUOTA },
    XchkHealthMap { group: XHG_FS, sick_mask: XFS_SICK_FS_PQUOTA },
    XchkHealthMap { group: XHG_FS, sick_mask: XFS_SICK_FS_COUNTERS },
    XchkHealthMap { group: XHG_FS, sick_mask: XFS_SICK_FS_QUOTACHECK },
    XchkHealthMap { group: XHG_FS, sick_mask: XFS_SICK_FS_NLINKS },
    XchkHealthMap { group: XHG_INO, sick_mask: XFS_SICK_INO_DIRTREE },
    XchkHealthMap { group: XHG_FS, sick_mask: XFS_SICK_FS_METAPATH },
    XchkHealthMap { group: XHG_RTGROUP, sick_mask: XFS_SICK_RG_SUPER },
    XchkHealthMap { group: XHG_RTGROUP, sick_mask: XFS_SICK_RG_RMAPBT },
    XchkHealthMap { group: XHG_RTGROUP, sick_mask: XFS_SICK_RG_REFCNTBT },
];

/* Return the health status mask for this scrub type. */
pub unsafe fn xchk_health_mask_for_scrub_type(scrub_type: u32) -> u32 {
    TYPE_TO_HEALTH_FLAG[scrub_type as usize].sick_mask
}

/* If the scrub state is clean, add @mask to the scrub sick mask to clear
 * additional sick flags from the metadata object's sick state. */
pub unsafe fn xchk_mark_healthy_if_clean(sc: *mut XfsScrub, mask: u32) {
    if (*(*sc).sm).sm_flags & (XFS_SCRUB_OFLAG_CORRUPT | XFS_SCRUB_OFLAG_XCORRUPT) == 0 {
        (*sc).healthy_mask |= mask;
    }
}

/* If we're scrubbing a piece of file metadata for the first time, does it look
 * like it has been zapped?  Skip the check if we just repaired the metadata
 * and are revalidating it. */
pub unsafe fn xchk_file_looks_zapped(sc: *mut XfsScrub, mask: u32) -> bool {
    ASSERT((mask & !XFS_SICK_INO_ZAPPED) == 0);
    if (*sc).flags & XREP_ALREADY_FIXED != 0 { return false; }
    xfs_inode_has_sickness((*sc).ip, mask)
}

/* Scrub gave the filesystem a clean bill of health, so clear all the indirect
 * markers of past problems (at least for the fs and ags) so that we can be
 * healthy again. */
unsafe fn xchk_mark_all_healthy(mp: *mut XfsMount) {
    let mut pag: *mut XfsPerag = core::ptr::null_mut();
    let mut rtg: *mut XfsRtgroup = core::ptr::null_mut();
    xfs_fs_mark_healthy(mp, XFS_SICK_FS_INDIRECT);
    while { pag = xfs_perag_next(mp, pag); !pag.is_null() } {
        xfs_group_mark_healthy(pag_group(pag), XFS_SICK_AG_INDIRECT);
    }
    while { rtg = xfs_rtgroup_next(mp, rtg); !rtg.is_null() } {
        xfs_group_mark_healthy(rtg_group(rtg), XFS_SICK_RG_INDIRECT);
    }
}

/* Update filesystem health assessments based on what we found and did. */
pub unsafe fn xchk_update_health(sc: *mut XfsScrub) {
    let mut pag: *mut XfsPerag;
    let mut rtg: *mut XfsRtgroup;
    let mut mask = (*sc).sick_mask;
    let bad = (*(*sc).sm).sm_flags & (XFS_SCRUB_OFLAG_CORRUPT | XFS_SCRUB_OFLAG_XCORRUPT);
    if (*(*sc).sm).sm_type == XFS_SCRUB_TYPE_HEALTHY && bad == 0 {
        xchk_mark_all_healthy((*sc).mp);
        return;
    }
    let bad = bad != 0;
    if !bad { mask |= (*sc).healthy_mask; }
    match TYPE_TO_HEALTH_FLAG[(*(*sc).sm).sm_type as usize].group {
        XchkHealthGroup::XHG_NONE => {}
        XchkHealthGroup::XHG_AG => {
            if mask == 0 { return; }
            pag = xfs_perag_get((*sc).mp, (*(*sc).sm).sm_agno);
            if bad { xfs_group_mark_corrupt(pag_group(pag), mask); }
            else { xfs_group_mark_healthy(pag_group(pag), mask); }
            xfs_perag_put(pag);
        }
        XchkHealthGroup::XHG_INO => {
            if (*sc).ip.is_null() { return; }
            if (*(*sc).sm).sm_flags & XFS_SCRUB_IFLAG_REPAIR != 0 { mask |= XFS_SICK_INO_FORGET; }
            if mask == 0 { return; }
            if bad { xfs_inode_mark_corrupt((*sc).ip, mask); }
            else { xfs_inode_mark_healthy((*sc).ip, mask); }
        }
        XchkHealthGroup::XHG_FS => {
            if mask == 0 { return; }
            if bad { xfs_fs_mark_corrupt((*sc).mp, mask); }
            else { xfs_fs_mark_healthy((*sc).mp, mask); }
        }
        XchkHealthGroup::XHG_RTGROUP => {
            if mask == 0 { return; }
            rtg = xfs_rtgroup_get((*sc).mp, (*(*sc).sm).sm_agno);
            if bad { xfs_group_mark_corrupt(rtg_group(rtg), mask); }
            else { xfs_group_mark_healthy(rtg_group(rtg), mask); }
            xfs_rtgroup_put(rtg);
        }
    }
}

/* Is the given per-AG btree healthy enough for scanning? */
pub unsafe fn xchk_ag_btree_del_cursor_if_sick(sc: *mut XfsScrub, curp: *mut *mut XfsBtreeCur, sm_type: u32) {
    let mut mask = (**curp).bc_ops.sick_mask;
    if (*(*sc).sm).sm_type == sm_type { return; }
    if (*sc).flags & XREP_ALREADY_FIXED != 0 && TYPE_TO_HEALTH_FLAG[(*(*sc).sm).sm_type as usize].group == XchkHealthGroup::XHG_AG {
        mask &= !(*sc).sick_mask;
    }
    if xfs_group_has_sickness((**curp).bc_group, mask) {
        (*(*sc).sm).sm_flags |= XFS_SCRUB_OFLAG_XFAIL;
        xfs_btree_del_cursor(*curp, XFS_BTREE_NOERROR);
        *curp = core::ptr::null_mut();
    }
}

/* Quick scan to double-check that there isn't any evidence of lingering
 * primary health problems. */
pub unsafe fn xchk_health_record(sc: *mut XfsScrub) -> i32 {
    let mp = (*sc).mp;
    let mut pag: *mut XfsPerag = core::ptr::null_mut();
    let mut rtg: *mut XfsRtgroup = core::ptr::null_mut();
    let mut sick: u32 = 0;
    let mut checked: u32 = 0;
    xfs_fs_measure_sickness(mp, &mut sick, &mut checked);
    if sick & XFS_SICK_FS_PRIMARY != 0 { xchk_set_corrupt(sc); }
    while { pag = xfs_perag_next(mp, pag); !pag.is_null() } {
        xfs_group_measure_sickness(pag_group(pag), &mut sick, &mut checked);
        if sick & XFS_SICK_AG_PRIMARY != 0 { xchk_set_corrupt(sc); }
    }
    while { rtg = xfs_rtgroup_next(mp, rtg); !rtg.is_null() } {
        xfs_group_measure_sickness(rtg_group(rtg), &mut sick, &mut checked);
        if sick & XFS_SICK_RG_PRIMARY != 0 { xchk_set_corrupt(sc); }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
