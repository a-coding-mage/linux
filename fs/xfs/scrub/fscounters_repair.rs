// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Dependencies supplied by the surrounding XFS translation unit.

/*
 * FS Summary Counters
 * ===================
 *
 * We correct errors in the filesystem summary counters by setting them to the
 * values computed during the obligatory scrub phase.  However, we must be
 * careful not to allow any other thread to change the counters while we're
 * computing and setting new values.  To achieve this, we freeze the
 * filesystem for the whole operation if the REPAIR flag is set.  The checking
 * function is stricter when we've frozen the fs.
 */

/*
 * Reset the superblock counters.  Caller is responsible for freezing the
 * filesystem during the calculation and reset phases.
 */
pub unsafe fn xrep_fscounters(sc: *mut xfs_scrub) -> i32 {
    let mp: *mut xfs_mount = (*sc).mp;
    let fsc: *mut xchk_fscounters = (*sc).buf as *mut xchk_fscounters;

    /*
     * Reinitialize the in-core counters from what we computed.  We froze
     * the filesystem, so there shouldn't be anyone else trying to modify
     * these counters.
     */
    if !(*fsc).frozen {
        ASSERT((*fsc).frozen);
        return -EFSCORRUPTED;
    }

    trace_xrep_reset_counters(mp, fsc);

    percpu_counter_set(&mut (*mp).m_icount, (*fsc).icount);
    percpu_counter_set(&mut (*mp).m_ifree, (*fsc).ifree);
    xfs_set_freecounter(mp, XC_FREE_BLOCKS, (*fsc).fdblocks);

    /*
     * Online repair is only supported on v5 file systems, which require
     * lazy sb counters and thus no update of sb_fdblocks here.  But
     * sb_frextents only uses a lazy counter with rtgroups, and thus needs
     * to be updated directly here otherwise.  And for that we need to keep
     * track of the delalloc reservations separately, as they are are
     * subtracted from m_frextents, but not included in sb_frextents.
     */
    if !xfs_has_zoned(mp) {
        xfs_set_freecounter(
            mp,
            XC_FREE_RTEXTENTS,
            (*fsc).frextents - (*fsc).frextents_delayed,
        );
        if !xfs_has_rtgroups(mp) {
            (*mp).m_sb.sb_frextents = (*fsc).frextents;
        }
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
