// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/*
 * The C translation unit includes the XFS and scrub headers listed below.
 * Their Rust declarations are supplied by the surrounding translation.
 */

/* Figure out which block the btree cursor was pointing to. */
#[inline]
unsafe fn xchk_btree_cur_fsbno(
    cur: *mut xfs_btree_cur,
    level: ::std::os::raw::c_int,
) -> xfs_fsblock_t {
    if level < (*cur).bc_nlevels
        && !(*cur).bc_levels[level as usize].bp.is_null()
    {
        return XFS_DADDR_TO_FSB(
            (*cur).bc_mp,
            xfs_buf_daddr((*cur).bc_levels[level as usize].bp),
        );
    }

    if level == (*cur).bc_nlevels - 1
        && (*cur).bc_ops.r#type == XFS_BTREE_TYPE_INODE
    {
        return XFS_INODE_TO_FSB((*cur).bc_ino.ip);
    }

    NULLFSBLOCK
}

/*
 * The C source defines CREATE_TRACE_POINTS before including scrub/trace.h;
 * trace event implementations are provided by the surrounding translation.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
