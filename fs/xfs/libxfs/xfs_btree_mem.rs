/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* C dependencies are supplied by the surrounding XFS translation unit. */

pub unsafe fn xfbtree_set_root(
    cur: *mut xfs_btree_cur,
    ptr: *const xfs_btree_ptr,
    inc: i32,
) {
    debug_assert!((*(*cur).bc_ops).type_ == XFS_BTREE_TYPE_MEM);
    (*(*(*cur).bc_mem.xfbtree).root_mut()) = *ptr;
    (*cur).bc_mem.xfbtree.as_mut().unwrap().nlevels += inc;
}

pub unsafe fn xfbtree_init_ptr_from_cur(
    cur: *mut xfs_btree_cur,
    ptr: *mut xfs_btree_ptr,
) {
    debug_assert!((*(*cur).bc_ops).type_ == XFS_BTREE_TYPE_MEM);
    *ptr = (*cur).bc_mem.xfbtree.as_ref().unwrap().root;
}

pub unsafe fn xfbtree_dup_cursor(cur: *mut xfs_btree_cur) -> *mut xfs_btree_cur {
    debug_assert!((*(*cur).bc_ops).type_ == XFS_BTREE_TYPE_MEM);
    let ncur = xfs_btree_alloc_cursor(
        (*cur).bc_mp,
        (*cur).bc_tp,
        (*cur).bc_ops,
        (*cur).bc_maxlevels,
        (*cur).bc_cache,
    );
    (*ncur).bc_flags = (*cur).bc_flags;
    (*ncur).bc_nlevels = (*cur).bc_nlevels;
    (*ncur).bc_mem.xfbtree = (*cur).bc_mem.xfbtree;
    if !(*cur).bc_group.is_null() {
        (*ncur).bc_group = xfs_group_hold((*cur).bc_group);
    }
    ncur
}

pub unsafe fn xfbtree_destroy(xfbt: *mut xfbtree) {
    xfs_buftarg_drain((*xfbt).target);
}

#[inline]
unsafe fn xfbtree_rec_bytes(_mp: *mut xfs_mount, _ops: *const xfs_btree_ops) -> u32 {
    XMBUF_BLOCKSIZE - XFS_BTREE_LBLOCK_CRC_LEN
}

unsafe fn xfbtree_init_leaf_block(
    mp: *mut xfs_mount,
    xfbt: *mut xfbtree,
    ops: *const xfs_btree_ops,
) -> i32 {
    let bno = (*xfbt).highest_bno;
    (*xfbt).highest_bno += 1;
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let error = xfs_buf_get((*xfbt).target, xfbno_to_daddr(bno), XFBNO_BBSIZE, &mut bp);
    if error != 0 { return error; }
    trace_xfbtree_create_root_buf(xfbt, bp);
    (*bp).b_ops = (*ops).buf_ops;
    xfs_btree_init_buf(mp, bp, ops, 0, 0, (*xfbt).owner);
    xfs_buf_relse(bp);
    (*xfbt).root.l = cpu_to_be64(bno);
    0
}

pub unsafe fn xfbtree_init(
    mp: *mut xfs_mount,
    xfbt: *mut xfbtree,
    btp: *mut xfs_buftarg,
    ops: *const xfs_btree_ops,
) -> i32 {
    let blocklen = xfbtree_rec_bytes(mp, ops);
    let keyptr_len = (*ops).key_len + core::mem::size_of::<u64>() as u32;
    if !xfs_has_crc(mp) { debug_assert!(xfs_has_crc(mp)); return -EINVAL; }
    if (*ops).ptr_len != XFS_BTREE_LONG_PTR_LEN { debug_assert!((*ops).ptr_len == XFS_BTREE_LONG_PTR_LEN); return -EINVAL; }
    core::ptr::write_bytes(xfbt as *mut u8, 0, core::mem::size_of::<xfbtree>());
    (*xfbt).target = btp;
    (*xfbt).maxrecs[0] = blocklen / (*ops).rec_len;
    (*xfbt).maxrecs[1] = blocklen / keyptr_len;
    (*xfbt).minrecs[0] = (*xfbt).maxrecs[0] / 2;
    (*xfbt).minrecs[1] = (*xfbt).maxrecs[1] / 2;
    (*xfbt).highest_bno = 0;
    (*xfbt).nlevels = 1;
    let error = xfbtree_init_leaf_block(mp, xfbt, ops);
    if error != 0 { xfs_buftarg_drain((*xfbt).target); return error; }
    trace_xfbtree_init(mp, xfbt, ops);
    0
}

pub unsafe fn xfbtree_alloc_block(cur: *mut xfs_btree_cur, _start: *const xfs_btree_ptr, new: *mut xfs_btree_ptr, stat: *mut i32) -> i32 {
    let xfbt = (*cur).bc_mem.xfbtree;
    let bno = (*xfbt).highest_bno; (*xfbt).highest_bno += 1;
    debug_assert!((*(*cur).bc_ops).type_ == XFS_BTREE_TYPE_MEM);
    trace_xfbtree_alloc_block(xfbt, cur, bno);
    if !xfbtree_verify_bno(xfbt, bno) { debug_assert!(xfbtree_verify_bno(xfbt, bno)); *stat = 0; return 0; }
    (*new).l = cpu_to_be64(bno); *stat = 1; 0
}

pub unsafe fn xfbtree_free_block(cur: *mut xfs_btree_cur, bp: *mut xfs_buf) -> i32 {
    let xfbt = (*cur).bc_mem.xfbtree;
    let bno = xfs_daddr_to_xfbno(xfs_buf_daddr(bp));
    debug_assert!((*(*cur).bc_ops).type_ == XFS_BTREE_TYPE_MEM);
    trace_xfbtree_free_block(xfbt, cur, bno);
    if bno + 1 == (*xfbt).highest_bno { (*xfbt).highest_bno -= 1; }
    0
}

pub unsafe fn xfbtree_get_minrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 { (*(*cur).bc_mem.xfbtree).minrecs[(level != 0) as usize] }
pub unsafe fn xfbtree_get_maxrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 { (*(*cur).bc_mem.xfbtree).maxrecs[(level != 0) as usize] }

unsafe fn xfbtree_buf_match(xfbt: *mut xfbtree, lip: *const xfs_log_item) -> *mut xfs_buf {
    if (*lip).li_type != XFS_LI_BUF { return core::ptr::null_mut(); }
    /* container_of(lip, xfs_buf_log_item, bli_item), supplied by the XFS layout. */
    let bli = xfs_buf_log_item_from_item(lip);
    let bp = (*bli).bli_buf;
    if (*bp).b_target != (*xfbt).target { return core::ptr::null_mut(); }
    bp
}

/* Transaction walking and list mutation use the surrounding XFS list APIs. */
pub unsafe fn xfbtree_trans_commit(_xfbt: *mut xfbtree, _tp: *mut xfs_trans) -> i32 { todo!("translate list_for_each_entry_safe transaction walk") }
pub unsafe fn xfbtree_trans_cancel(_xfbt: *mut xfbtree, _tp: *mut xfs_trans) { todo!("translate list_for_each_entry_safe transaction walk") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
