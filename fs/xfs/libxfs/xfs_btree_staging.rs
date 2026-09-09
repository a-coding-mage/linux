// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2020 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/* Staging Cursors and Fake Roots for Btrees */

pub unsafe fn xfs_btree_stage_afakeroot(cur: *mut xfs_btree_cur, afake: *mut xbtree_afakeroot) {
    ASSERT(!((*cur).bc_flags & XFS_BTREE_STAGING) != 0);
    ASSERT((*cur).bc_ops.as_ref().unwrap().type_ != XFS_BTREE_TYPE_INODE);
    ASSERT((*cur).bc_tp.is_null());
    (*cur).bc_ag.afake = afake;
    (*cur).bc_nlevels = (*afake).af_levels;
    (*cur).bc_flags |= XFS_BTREE_STAGING;
}

pub unsafe fn xfs_btree_commit_afakeroot(cur: *mut xfs_btree_cur, tp: *mut xfs_trans, agbp: *mut xfs_buf) {
    ASSERT((*cur).bc_flags & XFS_BTREE_STAGING != 0);
    ASSERT((*cur).bc_tp.is_null());
    trace_xfs_btree_commit_afakeroot(cur);
    (*cur).bc_ag.afake = core::ptr::null_mut();
    (*cur).bc_ag.agbp = agbp;
    (*cur).bc_flags &= !XFS_BTREE_STAGING;
    (*cur).bc_tp = tp;
}

pub unsafe fn xfs_btree_stage_ifakeroot(cur: *mut xfs_btree_cur, ifake: *mut xbtree_ifakeroot) {
    ASSERT(!((*cur).bc_flags & XFS_BTREE_STAGING) != 0);
    ASSERT((*cur).bc_ops.as_ref().unwrap().type_ == XFS_BTREE_TYPE_INODE);
    ASSERT((*cur).bc_tp.is_null());
    (*cur).bc_ino.ifake = ifake;
    (*cur).bc_nlevels = (*ifake).if_levels;
    (*cur).bc_ino.forksize = (*ifake).if_fork_size;
    (*cur).bc_ino.whichfork = XFS_STAGING_FORK;
    (*cur).bc_flags |= XFS_BTREE_STAGING;
}

pub unsafe fn xfs_btree_commit_ifakeroot(cur: *mut xfs_btree_cur, tp: *mut xfs_trans, whichfork: i32) {
    ASSERT((*cur).bc_flags & XFS_BTREE_STAGING != 0);
    ASSERT((*cur).bc_tp.is_null());
    trace_xfs_btree_commit_ifakeroot(cur);
    (*cur).bc_ino.ifake = core::ptr::null_mut();
    (*cur).bc_ino.whichfork = whichfork;
    (*cur).bc_flags &= !XFS_BTREE_STAGING;
    (*cur).bc_tp = tp;
}

unsafe fn xfs_btree_bload_drop_buf(bbl: *mut xfs_btree_bload, buffers_list: *mut list_head, bpp: *mut *mut xfs_buf) -> i32 {
    let bp = *bpp;
    if bp.is_null() { return 0; }
    xfs_buf_set_uptodate(bp);
    xfs_buf_delwri_queue_here(bp, buffers_list);
    xfs_buf_relse(bp);
    *bpp = core::ptr::null_mut();
    (*bbl).nr_dirty += 1;
    if (*bbl).max_dirty == 0 || (*bbl).nr_dirty < (*bbl).max_dirty { return 0; }
    let error = xfs_buf_delwri_submit(buffers_list);
    if error != 0 { return error; }
    (*bbl).nr_dirty = 0;
    0
}

unsafe fn xfs_btree_bload_prep_block(cur: *mut xfs_btree_cur, bbl: *mut xfs_btree_bload, buffers_list: *mut list_head, level: u32, nr_this_block: u32, ptrp: *mut xfs_btree_ptr, bpp: *mut *mut xfs_buf, blockp: *mut *mut xfs_btree_block, priv_: *mut core::ffi::c_void) -> i32 {
    let mut new_ptr = core::mem::zeroed::<xfs_btree_ptr>();
    let mut new_bp = core::ptr::null_mut();
    let mut new_block = core::ptr::null_mut();
    if xfs_btree_at_iroot(cur, level) {
        let ifp = xfs_btree_ifork_ptr(cur);
        ASSERT((*bpp).is_null());
        let new_size = ((*bbl).iroot_size.unwrap())(cur, level, nr_this_block, priv_);
        (*ifp).if_broot = kzalloc(new_size, GFP_KERNEL | __GFP_NOFAIL) as *mut _;
        (*ifp).if_broot_bytes = new_size as i32;
        xfs_btree_init_block((*cur).bc_mp, (*ifp).if_broot, (*cur).bc_ops, level, nr_this_block, I_INO((*cur).bc_ino.ip));
        *bpp = core::ptr::null_mut(); *blockp = (*ifp).if_broot;
        xfs_btree_set_ptr_null(cur, ptrp); return 0;
    }
    xfs_btree_set_ptr_null(cur, &mut new_ptr);
    let mut ret = ((*bbl).claim_block.unwrap())(cur, &mut new_ptr, priv_);
    if ret != 0 { return ret; }
    ASSERT(!xfs_btree_ptr_is_null(cur, &new_ptr));
    ret = xfs_btree_get_buf_block(cur, &new_ptr, &mut new_block, &mut new_bp);
    if ret != 0 { return ret; }
    if !(*blockp).is_null() { xfs_btree_set_sibling(cur, *blockp, &new_ptr, XFS_BB_RIGHTSIB); }
    ret = xfs_btree_bload_drop_buf(bbl, buffers_list, bpp);
    if ret != 0 { return ret; }
    xfs_btree_init_block_cur(cur, new_bp, level, nr_this_block);
    xfs_btree_set_sibling(cur, new_block, ptrp, XFS_BB_LEFTSIB);
    *bpp = new_bp; *blockp = new_block; xfs_btree_copy_ptrs(cur, ptrp, &new_ptr, 1); 0
}

unsafe fn xfs_btree_bload_leaf(cur: *mut xfs_btree_cur, recs_this_block: u32, get_records: Option<xfs_btree_bload_get_records_fn>, block: *mut xfs_btree_block, priv_: *mut core::ffi::c_void) -> i32 {
    let mut j = 1; while j <= recs_this_block {
        let ret = get_records.unwrap()(cur, j, block, recs_this_block - j + 1, priv_);
        if ret < 0 { return ret; } j += ret as u32;
    } 0
}

unsafe fn xfs_btree_bload_node(cur: *mut xfs_btree_cur, recs_this_block: u32, child_ptr: *mut xfs_btree_ptr, block: *mut xfs_btree_block) -> i32 {
    for j in 1..=recs_this_block {
        let mut child_key = core::mem::zeroed::<xfs_btree_key>(); let mut child_block = core::ptr::null_mut(); let mut child_bp = core::ptr::null_mut();
        ASSERT(!xfs_btree_ptr_is_null(cur, &*child_ptr));
        let ret = xfs_btree_read_buf_block(cur, child_ptr, 0, &mut child_block, &mut child_bp); if ret != 0 { return ret; }
        let block_ptr = xfs_btree_ptr_addr(cur, j, block); xfs_btree_copy_ptrs(cur, block_ptr, child_ptr, 1);
        let block_key = xfs_btree_key_addr(cur, j, block); xfs_btree_get_keys(cur, child_block, &mut child_key); xfs_btree_copy_keys(cur, block_key, &child_key, 1);
        xfs_btree_get_sibling(cur, child_block, child_ptr, XFS_BB_RIGHTSIB); xfs_buf_relse(child_bp);
    } 0
}

unsafe fn xfs_btree_bload_max_npb(cur: *mut xfs_btree_cur, bbl: *mut xfs_btree_bload, level: u32) -> u32 {
    if level == (*cur).bc_nlevels - 1 && (*cur).bc_ops.as_ref().unwrap().get_dmaxrecs.is_some() { return ((*cur).bc_ops.as_ref().unwrap().get_dmaxrecs.unwrap())(cur, level); }
    let mut ret = ((*cur).bc_ops.as_ref().unwrap().get_maxrecs)(cur, level); ret -= if level == 0 { (*bbl).leaf_slack as u32 } else { (*bbl).node_slack as u32 }; ret
}

unsafe fn xfs_btree_bload_desired_npb(cur: *mut xfs_btree_cur, bbl: *mut xfs_btree_bload, level: u32) -> u32 {
    let npb = xfs_btree_bload_max_npb(cur, bbl, level); if level == (*cur).bc_nlevels - 1 { return max(1, npb); } max(((*cur).bc_ops.as_ref().unwrap().get_minrecs)(cur, level), npb)
}

unsafe fn xfs_btree_bload_level_geometry(cur: *mut xfs_btree_cur, bbl: *mut xfs_btree_bload, level: u32, nr_this_level: u64, avg_per_block: *mut u32, blocks: *mut u64, blocks_with_extra: *mut u64) {
    let maxnr = if (*cur).bc_ops.as_ref().unwrap().get_dmaxrecs.is_some() { ((*cur).bc_ops.as_ref().unwrap().get_dmaxrecs.unwrap())(cur, level) } else { ((*cur).bc_ops.as_ref().unwrap().get_maxrecs)(cur, level) };
    let desired = xfs_btree_bload_desired_npb(cur, bbl, level); *blocks = (nr_this_level / desired as u64).max(1);
    let mut npb = nr_this_level / *blocks; *blocks_with_extra = nr_this_level % *blocks;
    if npb > maxnr as u64 || (npb == maxnr as u64 && *blocks_with_extra > 0) { *blocks += 1; npb = nr_this_level / *blocks; *blocks_with_extra = nr_this_level % *blocks; }
    *avg_per_block = npb.min(nr_this_level) as u32;
    trace_xfs_btree_bload_level_geometry(cur, level, nr_this_level, *avg_per_block, desired, *blocks, *blocks_with_extra);
}

unsafe fn xfs_btree_bload_ensure_slack(cur: *mut xfs_btree_cur, slack: *mut i32, level: i32) {
    let maxr = ((*cur).bc_ops.as_ref().unwrap().get_maxrecs)(cur, level as u32) as i32; let minr = ((*cur).bc_ops.as_ref().unwrap().get_minrecs)(cur, level as u32) as i32;
    if *slack < 0 { *slack = maxr - ((maxr + minr) >> 1); } *slack = (*slack).min(maxr - minr);
}

pub unsafe fn xfs_btree_bload_compute_geometry(cur: *mut xfs_btree_cur, bbl: *mut xfs_btree_bload, nr_records: u64) -> i32 {
    let ops = (*cur).bc_ops; let mut nr_blocks = 0; let mut nr_this_level = nr_records; ASSERT((*cur).bc_flags & XFS_BTREE_STAGING != 0);
    (*cur).bc_nlevels = (*cur).bc_maxlevels - 1; xfs_btree_bload_ensure_slack(cur, &mut (*bbl).leaf_slack, 0); xfs_btree_bload_ensure_slack(cur, &mut (*bbl).node_slack, 1); (*bbl).nr_records = nr_records;
    (*cur).bc_nlevels = 1; while (*cur).bc_nlevels <= (*cur).bc_maxlevels { let level = (*cur).bc_nlevels - 1; let mut avg = 0; let mut level_blocks = 0; let mut extra = 0; xfs_btree_bload_level_geometry(cur,bbl,level,nr_this_level,&mut avg,&mut level_blocks,&mut extra);
        if (*ops).type_ == XFS_BTREE_TYPE_INODE { if (level != 0 || (*ops).geom_flags & XFS_BTGEO_IROOT_RECORDS != 0) && nr_this_level <= avg as u64 { nr_blocks += 1; break; } (*cur).bc_nlevels += 1; if (*cur).bc_nlevels > (*cur).bc_maxlevels { break; } xfs_btree_bload_level_geometry(cur,bbl,level,nr_this_level,&mut avg,&mut level_blocks,&mut extra); } else if nr_this_level <= avg as u64 { nr_blocks += 1; break; } else { (*cur).bc_nlevels += 1; if (*cur).bc_nlevels > (*cur).bc_maxlevels { break; } }
        nr_blocks += level_blocks; nr_this_level = level_blocks;
    }
    if (*cur).bc_nlevels > (*cur).bc_maxlevels { return -EOVERFLOW; } (*bbl).btree_height = (*cur).bc_nlevels; (*bbl).nr_blocks = if (*ops).type_ == XFS_BTREE_TYPE_INODE { nr_blocks - 1 } else { nr_blocks }; 0
}

pub unsafe fn xfs_btree_bload(cur: *mut xfs_btree_cur, bbl: *mut xfs_btree_bload, priv_: *mut core::ffi::c_void) -> i32 {
    let mut buffers_list = core::mem::zeroed::<list_head>(); let mut child_ptr = core::mem::zeroed::<xfs_btree_ptr>(); let mut ptr = core::mem::zeroed::<xfs_btree_ptr>(); let mut bp = core::ptr::null_mut(); let mut block = core::ptr::null_mut(); let mut nr_this_level = (*bbl).nr_records; let mut blocks; let mut extra; let mut total_blocks = 0; let mut avg = 0; ASSERT((*cur).bc_flags & XFS_BTREE_STAGING != 0); INIT_LIST_HEAD(&mut buffers_list); (*cur).bc_nlevels = (*bbl).btree_height; xfs_btree_set_ptr_null(cur,&mut child_ptr); xfs_btree_set_ptr_null(cur,&mut ptr); (*bbl).nr_dirty=0;
    xfs_btree_bload_level_geometry(cur,bbl,0,nr_this_level,&mut avg,&mut blocks,&mut extra); for i in 0..blocks { let n = avg + if i < extra {1} else {0}; let mut ret=xfs_btree_bload_prep_block(cur,bbl,&mut buffers_list,0,n,&mut ptr,&mut bp,&mut block,priv_); if ret!=0 { xfs_buf_delwri_cancel(&mut buffers_list); if !bp.is_null(){xfs_buf_relse(bp)}; return ret;} trace_xfs_btree_bload_block(cur,0,i,blocks,&ptr,n); ret=xfs_btree_bload_leaf(cur,n,(*bbl).get_records,block,priv_); if ret!=0 {xfs_buf_delwri_cancel(&mut buffers_list);if !bp.is_null(){xfs_buf_relse(bp)};return ret;} if i==0{xfs_btree_copy_ptrs(cur,&mut child_ptr,&ptr,1);} } total_blocks+=blocks; let mut ret=xfs_btree_bload_drop_buf(bbl,&mut buffers_list,&mut bp); if ret!=0{ xfs_buf_delwri_cancel(&mut buffers_list);return ret;}
    for level in 1..(*cur).bc_nlevels { let mut first_ptr=core::mem::zeroed(); nr_this_level=blocks; block=core::ptr::null_mut(); xfs_btree_set_ptr_null(cur,&mut ptr); xfs_btree_bload_level_geometry(cur,bbl,level,nr_this_level,&mut avg,&mut blocks,&mut extra); for i in 0..blocks { let n=avg+if i<extra{1}else{0}; ret=xfs_btree_bload_prep_block(cur,bbl,&mut buffers_list,level,n,&mut ptr,&mut bp,&mut block,priv_); if ret!=0{break;} trace_xfs_btree_bload_block(cur,level,i,blocks,&ptr,n); ret=xfs_btree_bload_node(cur,n,&mut child_ptr,block);if ret!=0{break;}if i==0{xfs_btree_copy_ptrs(cur,&mut first_ptr,&ptr,1);} } if ret!=0{break;} total_blocks+=blocks; ret=xfs_btree_bload_drop_buf(bbl,&mut buffers_list,&mut bp);if ret!=0{break;} xfs_btree_copy_ptrs(cur,&mut child_ptr,&first_ptr,1); }
    if ret==0 { if (*cur).bc_ops.as_ref().unwrap().type_==XFS_BTREE_TYPE_INODE {ASSERT(xfs_btree_ptr_is_null(cur,&ptr));(*cur).bc_ino.ifake.as_mut().unwrap().if_levels=(*cur).bc_nlevels;(*cur).bc_ino.ifake.as_mut().unwrap().if_blocks=total_blocks-1;}else{(*cur).bc_ag.afake.as_mut().unwrap().af_root=be32_to_cpu(ptr.s);(*cur).bc_ag.afake.as_mut().unwrap().af_levels=(*cur).bc_nlevels;(*cur).bc_ag.afake.as_mut().unwrap().af_blocks=total_blocks;} ret=xfs_buf_delwri_submit(&mut buffers_list);if ret==0&&!list_empty(&buffers_list){ret=-EIO;} } xfs_buf_delwri_cancel(&mut buffers_list);if !bp.is_null(){xfs_buf_relse(bp)} ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
