// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * Copyright (c) 2013 Red Hat, Inc.
 * All Rights Reserved.
 */

// The declarations referenced here are supplied by the surrounding XFS
// translation units.

unsafe fn xfs_attr3_rmt_stale(
    dp: *mut xfs_inode,
    mut blkno: xfs_dablk_t,
    mut blkcnt: i32,
) -> i32 {
    let mut map: xfs_bmbt_irec = core::mem::zeroed();
    let mut nmap: i32;
    let mut error: i32;

    while blkcnt > 0 {
        nmap = 1;
        error = xfs_bmapi_read(dp, blkno as xfs_fileoff_t, blkcnt, &mut map, &mut nmap, XFS_BMAPI_ATTRFORK);
        if error != 0 { return error; }
        if XFS_IS_CORRUPT((*dp).i_mount, nmap != 1) { return -EFSCORRUPTED; }
        error = xfs_attr_rmtval_stale(dp, &mut map, 0);
        if error != 0 { return error; }
        blkno += map.br_blockcount;
        blkcnt -= map.br_blockcount as i32;
    }
    0
}

unsafe fn xfs_attr3_leaf_inactive(
    trans: *mut *mut xfs_trans,
    dp: *mut xfs_inode,
    bp: *mut xfs_buf,
) -> i32 {
    let mut ichdr: xfs_attr3_icleaf_hdr = core::mem::zeroed();
    let mp = (*bp).b_mount;
    let leaf = (*bp).b_addr as *mut xfs_attr_leafblock;
    let mut entry = xfs_attr3_leaf_entryp(leaf);
    let mut error = 0;
    xfs_attr3_leaf_hdr_from_disk((*mp).m_attr_geo, &mut ichdr, leaf);
    for i in 0..ichdr.count {
        if (*entry).nameidx == 0 || ((*entry).flags & XFS_ATTR_LOCAL) != 0 { entry = entry.add(1); continue; }
        let name_rmt = xfs_attr3_leaf_name_remote(leaf, i);
        if (*name_rmt).valueblk != 0 {
            let blkcnt = xfs_attr3_rmt_blocks((*dp).i_mount, be32_to_cpu((*name_rmt).valuelen));
            error = xfs_attr3_rmt_stale(dp, be32_to_cpu((*name_rmt).valueblk), blkcnt);
            if error != 0 { break; }
        }
        entry = entry.add(1);
    }
    if error == 0 { xfs_trans_brelse(*trans, bp); }
    error
}

unsafe fn xfs_attr3_node_inactive(
    trans: *mut *mut xfs_trans, dp: *mut xfs_inode, mut bp: *mut xfs_buf, level: i32,
) -> i32 {
    let mp = (*dp).i_mount;
    let mut ichdr: xfs_da3_icnode_hdr = core::mem::zeroed();
    if level > XFS_DA_NODE_MAXDEPTH {
        xfs_buf_mark_corrupt(bp); xfs_trans_brelse(*trans, bp);
        xfs_dirattr_mark_sick(dp, XFS_ATTR_FORK); return -EFSCORRUPTED;
    }
    xfs_da3_node_hdr_from_disk(mp, &mut ichdr, (*bp).b_addr);
    let parent_blkno = xfs_buf_daddr(bp);
    if ichdr.count == 0 { xfs_trans_brelse(*trans, bp); return 0; }
    let mut child_fsb = be32_to_cpu(ichdr.btree[0].before);
    xfs_trans_brelse(*trans, bp); bp = core::ptr::null_mut();
    while ichdr.count > 0 {
        let mut child_bp: *mut xfs_buf = core::ptr::null_mut();
        let mut error = xfs_da3_node_read(*trans, dp, child_fsb, &mut child_bp, XFS_ATTR_FORK);
        if error != 0 { return error; }
        let child_blkno = xfs_buf_daddr(child_bp);
        let info = (*child_bp).b_addr as *mut xfs_da_blkinfo;
        match (*info).magic {
            x if x == cpu_to_be16(XFS_DA_NODE_MAGIC) || x == cpu_to_be16(XFS_DA3_NODE_MAGIC) => error = xfs_attr3_node_inactive(trans, dp, child_bp, level + 1),
            x if x == cpu_to_be16(XFS_ATTR_LEAF_MAGIC) || x == cpu_to_be16(XFS_ATTR3_LEAF_MAGIC) => error = xfs_attr3_leaf_inactive(trans, dp, child_bp),
            _ => { xfs_buf_mark_corrupt(child_bp); xfs_trans_brelse(*trans, child_bp); xfs_dirattr_mark_sick(dp, XFS_ATTR_FORK); error = -EFSCORRUPTED; }
        }
        if error != 0 { return error; }
        error = xfs_trans_get_buf(*trans, (*mp).m_ddev_targp, child_blkno, XFS_FSB_TO_BB(mp, (*mp).m_attr_geo.fsbcount), 0, &mut child_bp);
        if error != 0 { return error; }
        xfs_trans_binval(*trans, child_bp);
        error = xfs_da3_node_read_mapped(*trans, dp, parent_blkno, &mut bp, XFS_ATTR_FORK);
        if error != 0 { return error; }
        xfs_attr3_node_entry_remove(*trans, dp, bp, 0);
        xfs_da3_node_hdr_from_disk(mp, &mut ichdr, (*bp).b_addr); bp = core::ptr::null_mut();
        if ichdr.count > 0 { child_fsb = be32_to_cpu(ichdr.btree[0].before); error = xfs_trans_roll_inode(trans, dp); if error != 0 { return error; } }
    }
    0
}

unsafe fn xfs_attr3_root_inactive(trans: *mut *mut xfs_trans, dp: *mut xfs_inode) -> i32 {
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let error = xfs_da3_node_read(*trans, dp, 0, &mut bp, XFS_ATTR_FORK);
    if error != 0 { return error; }
    let info = (*bp).b_addr as *mut xfs_da_blkinfo;
    let mut error = match (*info).magic {
        x if x == cpu_to_be16(XFS_DA_NODE_MAGIC) || x == cpu_to_be16(XFS_DA3_NODE_MAGIC) => xfs_attr3_node_inactive(trans, dp, bp, 1),
        x if x == cpu_to_be16(XFS_ATTR_LEAF_MAGIC) || x == cpu_to_be16(XFS_ATTR3_LEAF_MAGIC) => xfs_attr3_leaf_inactive(trans, dp, bp),
        _ => { xfs_dirattr_mark_sick(dp, XFS_ATTR_FORK); xfs_buf_mark_corrupt(bp); xfs_trans_brelse(*trans, bp); -EFSCORRUPTED }
    };
    if error == 0 { error = xfs_attr3_leaf_init(*trans, dp, 0); }
    if error == 0 { error = xfs_trans_roll_inode(trans, dp); }
    error
}

pub unsafe fn xfs_attr_inactive(dp: *mut xfs_inode) -> i32 {
    let mp = (*dp).i_mount;
    let mut lock_mode = XFS_ILOCK_SHARED;
    xfs_ilock(dp, lock_mode);
    if !xfs_inode_has_attr_fork(dp) { xfs_ifork_zap_attr(dp); xfs_iunlock(dp, lock_mode); return 0; }
    xfs_iunlock(dp, lock_mode);
    let mut trans: *mut xfs_trans = core::ptr::null_mut();
    let mut error = xfs_trans_alloc(mp, &mut M_RES(mp).tr_attrinval, 0, 0, 0, &mut trans);
    if error != 0 { xfs_ifork_zap_attr(dp); return error; }
    lock_mode = XFS_ILOCK_EXCL; xfs_ilock(dp, lock_mode);
    if !xfs_inode_has_attr_fork(dp) { xfs_trans_cancel(trans); xfs_ifork_zap_attr(dp); xfs_iunlock(dp, lock_mode); return 0; }
    xfs_trans_ijoin(trans, dp, 0);
    if (*dp).i_af.if_nextents > 0 {
        error = xfs_attr3_root_inactive(&mut trans, dp);
        if error == 0 { error = xfs_itruncate_extents(&mut trans, dp, XFS_ATTR_FORK, XFS_FSB_TO_B(mp, (*mp).m_attr_geo.fsbcount)); }
        if error == 0 { let mut bp: *mut xfs_buf = core::ptr::null_mut(); error = xfs_da_get_buf(trans, dp, 0, &mut bp, XFS_ATTR_FORK); if error == 0 { xfs_trans_binval(trans, bp); error = xfs_itruncate_extents(&mut trans, dp, XFS_ATTR_FORK, 0); } }
    }
    if error != 0 { xfs_trans_cancel(trans); xfs_ifork_zap_attr(dp); xfs_iunlock(dp, lock_mode); return error; }
    xfs_attr_fork_remove(dp, trans); error = xfs_trans_commit(trans); xfs_iunlock(dp, lock_mode); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
