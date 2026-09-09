// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// C dependencies supplied by the surrounding XFS translation.

static XFS_METAFILE_TYPE_STRS: &[XfsMetafileTypeStr] = &[
    XFS_METAFILE_TYPE_STR,
];

#[repr(C)]
struct XfsMetafileTypeStr {
    mtype: XfsMetafileType,
    name: *const core::ffi::c_char,
}

extern "C" {
    static XFS_METAFILE_TYPE_STR: XfsMetafileTypeStr;
}

pub unsafe fn xfs_metafile_type_str(metatype: XfsMetafileType) -> *const core::ffi::c_char {
    let mut i: usize = 0;

    while i < XFS_METAFILE_TYPE_STRS.len() {
        if XFS_METAFILE_TYPE_STRS[i].mtype == metatype {
            return XFS_METAFILE_TYPE_STRS[i].name;
        }
        i += 1;
    }

    core::ptr::null()
}

/* Set up an inode to be recognized as a metadata directory inode. */
pub unsafe fn xfs_metafile_set_iflag(
    tp: *mut XfsTrans,
    ip: *mut XfsInode,
    metafile_type: XfsMetafileType,
) {
    (*vfs_i(ip)).i_mode &= !0o777;
    (*vfs_i(ip)).i_uid = GLOBAL_ROOT_UID;
    (*vfs_i(ip)).i_gid = GLOBAL_ROOT_GID;
    if s_isdir((*vfs_i(ip)).i_mode) {
        (*ip).i_diflags |= XFS_METADIR_DIFLAGS;
    } else {
        (*ip).i_diflags |= XFS_METAFILE_DIFLAGS;
    }
    (*ip).i_diflags2 &= !XFS_DIFLAG2_DAX;
    (*ip).i_diflags2 |= XFS_DIFLAG2_METADATA;
    (*ip).i_metatype = metafile_type;
    xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE);

    XFS_STATS_DEC((*ip).i_mount, xs_inodes_active);
    XFS_STATS_INC((*ip).i_mount, xs_inodes_meta);
}

/* Clear the metadata directory inode flag. */
pub unsafe fn xfs_metafile_clear_iflag(tp: *mut XfsTrans, ip: *mut XfsInode) {
    ASSERT(xfs_is_metadir_inode(ip));
    ASSERT((*vfs_i(ip)).i_nlink == 0);

    (*ip).i_diflags2 &= !XFS_DIFLAG2_METADATA;
    xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE);
    XFS_STATS_INC((*ip).i_mount, xs_inodes_active);
    XFS_STATS_DEC((*ip).i_mount, xs_inodes_meta);
}

/* Is the metafile reservations at or beneath a certain threshold? */
unsafe fn xfs_metafile_resv_can_cover(mp: *mut XfsMount, rhs: i64) -> bool {
    /* The amount of space available is the reservation plus global free space. */
    if (*mp).m_metafile_resv_avail >= rhs {
        return true;
    }

    xfs_compare_freecounter(
        mp,
        XC_FREE_BLOCKS,
        rhs - (*mp).m_metafile_resv_avail,
        2048,
    ) >= 0
}

/* Is the metafile reservation critically low on blocks? */
pub unsafe fn xfs_metafile_resv_critical(mp: *mut XfsMount) -> bool {
    ASSERT(xfs_has_metadir(mp));
    trace_xfs_metafile_resv_critical(mp, 0);

    if !xfs_metafile_resv_can_cover(mp, (*mp).m_rtbtree_maxlevels) {
        return true;
    }
    if !xfs_metafile_resv_can_cover(mp, div_u64((*mp).m_metafile_resv_target, 10)) {
        return true;
    }

    XFS_TEST_ERROR(mp, XFS_ERRTAG_METAFILE_RESV_CRITICAL)
}

/* Allocate a block from the metadata file's reservation. */
pub unsafe fn xfs_metafile_resv_alloc_space(ip: *mut XfsInode, args: *mut XfsAllocArg) {
    let mp = (*ip).i_mount;
    let mut len: i64 = (*args).len;

    ASSERT(xfs_is_metadir_inode(ip));
    ASSERT((*args).resv == XFS_AG_RESV_METAFILE);
    trace_xfs_metafile_resv_alloc_space(mp, (*args).len);

    mutex_lock(&mut (*mp).m_metafile_resv_lock);
    if (*mp).m_metafile_resv_avail > 0 {
        let from_resv = core::cmp::min(len, (*mp).m_metafile_resv_avail);
        (*mp).m_metafile_resv_avail -= from_resv;
        xfs_mod_delalloc(ip, 0, -from_resv);
        xfs_trans_mod_sb((*args).tp, XFS_TRANS_SB_RES_FDBLOCKS, -from_resv);
        len -= from_resv;
    }
    if len != 0 {
        let field: u32 = if xfs_dec_fdblocks((*ip).i_mount, len, true) != 0 {
            XFS_TRANS_SB_FDBLOCKS
        } else {
            XFS_TRANS_SB_RES_FDBLOCKS
        };
        xfs_trans_mod_sb((*args).tp, field, -len);
    }
    (*mp).m_metafile_resv_used += (*args).len;
    mutex_unlock(&mut (*mp).m_metafile_resv_lock);

    (*ip).i_nblocks += (*args).len;
    xfs_trans_log_inode((*args).tp, ip, XFS_ILOG_CORE);
}

/* Free a block to the metadata file's reservation. */
pub unsafe fn xfs_metafile_resv_free_space(
    ip: *mut XfsInode,
    tp: *mut XfsTrans,
    mut len: XfsFilblks,
) {
    let mp = (*ip).i_mount;
    ASSERT(xfs_is_metadir_inode(ip));
    trace_xfs_metafile_resv_free_space(mp, len);

    (*ip).i_nblocks -= len;
    xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE);
    mutex_lock(&mut (*mp).m_metafile_resv_lock);
    (*mp).m_metafile_resv_used -= len;
    let mut to_resv = (*mp).m_metafile_resv_target
        - ((*mp).m_metafile_resv_used + (*mp).m_metafile_resv_avail);
    if to_resv > 0 {
        to_resv = core::cmp::min(to_resv, len);
        (*mp).m_metafile_resv_avail += to_resv;
        xfs_mod_delalloc(ip, 0, to_resv);
        xfs_trans_mod_sb(tp, XFS_TRANS_SB_RES_FDBLOCKS, to_resv);
        len -= to_resv;
    }
    mutex_unlock(&mut (*mp).m_metafile_resv_lock);
    if len != 0 {
        xfs_trans_mod_sb(tp, XFS_TRANS_SB_FDBLOCKS, len);
    }
}

unsafe fn __xfs_metafile_resv_free(mp: *mut XfsMount) {
    if (*mp).m_metafile_resv_avail != 0 {
        xfs_mod_sb_delalloc(mp, -((*mp).m_metafile_resv_avail as i64));
        xfs_add_fdblocks(mp, (*mp).m_metafile_resv_avail);
    }
    (*mp).m_metafile_resv_avail = 0;
    (*mp).m_metafile_resv_used = 0;
    (*mp).m_metafile_resv_target = 0;
}

/* Release unused metafile space reservation. */
pub unsafe fn xfs_metafile_resv_free(mp: *mut XfsMount) {
    if !xfs_has_metadir(mp) { return; }
    trace_xfs_metafile_resv_free(mp, 0);
    mutex_lock(&mut (*mp).m_metafile_resv_lock);
    __xfs_metafile_resv_free(mp);
    mutex_unlock(&mut (*mp).m_metafile_resv_lock);
}

/* Set up a metafile space reservation. */
pub unsafe fn xfs_metafile_resv_init(mp: *mut XfsMount) -> i32 {
    let mut rtg: *mut XfsRtgroup = core::ptr::null_mut();
    let mut used: XfsFilblks = 0;
    let mut target: XfsFilblks = 0;
    let mut error: i32 = 0;
    let dblocks_avail: XfsRfsblock = (*mp).m_sb.sb_dblocks / 4;

    if !xfs_has_metadir(mp) { return 0; }
    mutex_lock(&mut (*mp).m_metafile_resv_lock);
    __xfs_metafile_resv_free(mp);

    while {
        rtg = xfs_rtgroup_next(mp, rtg);
        !rtg.is_null()
    } {
        if xfs_has_rtrmapbt(mp) {
            used += (*rtg_rmap(rtg)).i_nblocks;
            target += xfs_rtrmapbt_calc_reserves(mp);
        }
        if xfs_has_rtreflink(mp) {
            used += (*rtg_refcount(rtg)).i_nblocks;
            target += xfs_rtrefcountbt_calc_reserves(mp);
        }
    }
    if target == 0 { mutex_unlock(&mut (*mp).m_metafile_resv_lock); return 0; }
    if used > target { target = used; }
    else if target > dblocks_avail { target = dblocks_avail; }
    let hidden_space = target - used;
    error = xfs_dec_fdblocks(mp, hidden_space, true);
    if error != 0 {
        trace_xfs_metafile_resv_init_error(mp, 0);
        mutex_unlock(&mut (*mp).m_metafile_resv_lock);
        return error;
    }
    xfs_mod_sb_delalloc(mp, hidden_space);
    (*mp).m_metafile_resv_target = target;
    (*mp).m_metafile_resv_used = used;
    (*mp).m_metafile_resv_avail = hidden_space;
    trace_xfs_metafile_resv_init(mp, target);
    mutex_unlock(&mut (*mp).m_metafile_resv_lock);
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
