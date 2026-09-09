// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Translated from the C implementation.  XFS declarations supplied by the
// surrounding tree are intentionally left as external dependencies.

static mut RCBAGBT_CUR_CACHE: *mut kmem_cache = core::ptr::null_mut();

unsafe fn rcbagbt_init_key_from_rec(
    key: *mut xfs_btree_key,
    rec: *const xfs_btree_rec,
) {
    let bag_key = key as *mut rcbag_key;
    let bag_rec = rec as *const rcbag_rec;

    (*bag_key).rbg_startblock = (*bag_rec).rbg_startblock;
    (*bag_key).rbg_blockcount = (*bag_rec).rbg_blockcount;
}

unsafe fn rcbagbt_init_rec_from_cur(cur: *mut xfs_btree_cur, rec: *mut xfs_btree_rec) {
    let bag_rec = rec as *mut rcbag_rec;
    let bag_irec = &mut (*cur).bc_rec as *mut _ as *mut rcbag_rec;

    (*bag_rec).rbg_startblock = (*bag_irec).rbg_startblock;
    (*bag_rec).rbg_blockcount = (*bag_irec).rbg_blockcount;
    (*bag_rec).rbg_refcount = (*bag_irec).rbg_refcount;
}

unsafe fn rcbagbt_cmp_key_with_cur(
    cur: *mut xfs_btree_cur,
    key: *const xfs_btree_key,
) -> i32 {
    let rec = &mut (*cur).bc_rec as *mut _ as *mut rcbag_rec;
    let kp = key as *const rcbag_key;
    let cmp = cmp_int((*kp).rbg_startblock, (*rec).rbg_startblock);
    if cmp != 0 { cmp } else { cmp_int((*kp).rbg_blockcount, (*rec).rbg_blockcount) }
}

unsafe fn rcbagbt_cmp_two_keys(
    _cur: *mut xfs_btree_cur,
    k1: *const xfs_btree_key,
    k2: *const xfs_btree_key,
    mask: *const xfs_btree_key,
) -> i32 {
    ASSERT(mask.is_null());
    let kp1 = k1 as *const rcbag_key;
    let kp2 = k2 as *const rcbag_key;
    let cmp = cmp_int((*kp1).rbg_startblock, (*kp2).rbg_startblock);
    if cmp != 0 { cmp } else { cmp_int((*kp1).rbg_blockcount, (*kp2).rbg_blockcount) }
}

unsafe fn rcbagbt_keys_inorder(
    _cur: *mut xfs_btree_cur,
    k1: *const xfs_btree_key,
    k2: *const xfs_btree_key,
) -> i32 {
    let kp1 = k1 as *const rcbag_key;
    let kp2 = k2 as *const rcbag_key;
    if (*kp1).rbg_startblock > (*kp2).rbg_startblock { return 0; }
    if (*kp1).rbg_startblock < (*kp2).rbg_startblock { return 1; }
    if (*kp1).rbg_blockcount > (*kp2).rbg_blockcount { return 0; }
    if (*kp1).rbg_blockcount < (*kp2).rbg_blockcount { return 1; }
    0
}

unsafe fn rcbagbt_recs_inorder(
    _cur: *mut xfs_btree_cur,
    r1: *const xfs_btree_rec,
    r2: *const xfs_btree_rec,
) -> i32 {
    let rp1 = r1 as *const rcbag_rec;
    let rp2 = r2 as *const rcbag_rec;
    if (*rp1).rbg_startblock > (*rp2).rbg_startblock { return 0; }
    if (*rp1).rbg_startblock < (*rp2).rbg_startblock { return 1; }
    if (*rp1).rbg_blockcount > (*rp2).rbg_blockcount { return 0; }
    if (*rp1).rbg_blockcount < (*rp2).rbg_blockcount { return 1; }
    0
}

unsafe fn rcbagbt_verify(bp: *mut xfs_buf) -> xfs_failaddr_t {
    let mp = (*bp).b_mount;
    let block = XFS_BUF_TO_BLOCK(bp);
    if !xfs_verify_magic(bp, (*block).bb_magic) { return __this_address!(); }
    let fa = xfs_btree_fsblock_v5hdr_verify(bp, XFS_RMAP_OWN_UNKNOWN);
    if !fa.is_null() { return fa; }
    let level = be16_to_cpu((*block).bb_level);
    if level >= rcbagbt_maxlevels_possible() { return __this_address!(); }
    let maxrecs = rcbagbt_maxrecs(mp, XFBNO_BLOCKSIZE, level == 0);
    xfs_btree_memblock_verify(bp, maxrecs)
}

unsafe fn rcbagbt_rw_verify(bp: *mut xfs_buf) {
    let fa = rcbagbt_verify(bp);
    if !fa.is_null() { xfs_verifier_error(bp, -EFSCORRUPTED, fa); }
}

/* skip crc checks on in-memory btrees to save time */
static RCBAGBT_MEM_BUF_OPS: xfs_buf_ops = xfs_buf_ops {
    name: "rcbagbt_mem",
    magic: [0, cpu_to_be32(RCBAG_MAGIC)],
    verify_read: Some(rcbagbt_rw_verify),
    verify_write: Some(rcbagbt_rw_verify),
    verify_struct: Some(rcbagbt_verify),
};

static RCBAGBT_MEM_OPS: xfs_btree_ops = xfs_btree_ops {
    name: "rcbag",
    r#type: XFS_BTREE_TYPE_MEM,
    rec_len: core::mem::size_of::<rcbag_rec>(),
    key_len: core::mem::size_of::<rcbag_key>(),
    ptr_len: XFS_BTREE_LONG_PTR_LEN,
    lru_refs: 1,
    statoff: XFS_STATS_CALC_INDEX(xs_rcbag_2),
    dup_cursor: Some(xfbtree_dup_cursor),
    set_root: Some(xfbtree_set_root),
    alloc_block: Some(xfbtree_alloc_block),
    free_block: Some(xfbtree_free_block),
    get_minrecs: Some(xfbtree_get_minrecs),
    get_maxrecs: Some(xfbtree_get_maxrecs),
    init_key_from_rec: Some(rcbagbt_init_key_from_rec),
    init_rec_from_cur: Some(rcbagbt_init_rec_from_cur),
    init_ptr_from_cur: Some(xfbtree_init_ptr_from_cur),
    cmp_key_with_cur: Some(rcbagbt_cmp_key_with_cur),
    buf_ops: &RCBAGBT_MEM_BUF_OPS,
    cmp_two_keys: Some(rcbagbt_cmp_two_keys),
    keys_inorder: Some(rcbagbt_keys_inorder),
    recs_inorder: Some(rcbagbt_recs_inorder),
};

pub unsafe fn rcbagbt_mem_cursor(mp: *mut xfs_mount, tp: *mut xfs_trans, xfbtree: *mut xfbtree) -> *mut xfs_btree_cur {
    let cur = xfs_btree_alloc_cursor(mp, tp, &RCBAGBT_MEM_OPS, rcbagbt_maxlevels_possible(), RCBAGBT_CUR_CACHE);
    (*cur).bc_mem.xfbtree = xfbtree;
    (*cur).bc_nlevels = (*xfbtree).nlevels;
    cur
}

pub unsafe fn rcbagbt_mem_init(mp: *mut xfs_mount, xfbt: *mut xfbtree, btp: *mut xfs_buftarg) -> i32 {
    (*xfbt).owner = 0;
    xfbtree_init(mp, xfbt, btp, &RCBAGBT_MEM_OPS)
}

unsafe fn rcbagbt_block_maxrecs(blocklen: u32, leaf: bool) -> u32 {
    if leaf { blocklen / core::mem::size_of::<rcbag_rec>() as u32 }
    else { blocklen / (core::mem::size_of::<rcbag_key>() as u32 + core::mem::size_of::<rcbag_ptr_t>() as u32) }
}

pub unsafe fn rcbagbt_maxrecs(_mp: *mut xfs_mount, mut blocklen: u32, leaf: bool) -> u32 {
    blocklen -= RCBAG_BLOCK_LEN;
    rcbagbt_block_maxrecs(blocklen, leaf)
}

pub unsafe fn rcbagbt_maxlevels_possible() -> u32 {
    let blocklen = XFBNO_BLOCKSIZE - XFS_BTREE_LBLOCK_CRC_LEN;
    let minrecs = [rcbagbt_block_maxrecs(blocklen, true) / 2, rcbagbt_block_maxrecs(blocklen, false) / 2];
    xfs_btree_space_to_height(minrecs.as_ptr(), ULLONG_MAX)
}

pub unsafe fn rcbagbt_calc_size(nr_records: u64) -> u64 {
    let blocklen = XFBNO_BLOCKSIZE - XFS_BTREE_LBLOCK_CRC_LEN;
    let minrecs = [rcbagbt_block_maxrecs(blocklen, true) / 2, rcbagbt_block_maxrecs(blocklen, false) / 2];
    xfs_btree_calc_size(minrecs.as_ptr(), nr_records)
}

pub unsafe fn rcbagbt_init_cur_cache() -> i32 {
    RCBAGBT_CUR_CACHE = kmem_cache_create("xfs_rcbagbt_cur", xfs_btree_cur_sizeof(rcbagbt_maxlevels_possible()), 0, 0, core::ptr::null_mut());
    if RCBAGBT_CUR_CACHE.is_null() { return -ENOMEM; }
    0
}

pub unsafe fn rcbagbt_destroy_cur_cache() {
    kmem_cache_destroy(RCBAGBT_CUR_CACHE);
    RCBAGBT_CUR_CACHE = core::ptr::null_mut();
}

pub unsafe fn rcbagbt_lookup_eq(cur: *mut xfs_btree_cur, rmap: *const xfs_rmap_irec, success: *mut i32) -> i32 {
    let rec = &mut (*cur).bc_rec as *mut _ as *mut rcbag_rec;
    (*rec).rbg_startblock = (*rmap).rm_startblock;
    (*rec).rbg_blockcount = (*rmap).rm_blockcount;
    xfs_btree_lookup(cur, XFS_LOOKUP_EQ, success)
}

pub unsafe fn rcbagbt_get_rec(cur: *mut xfs_btree_cur, rec: *mut rcbag_rec, has: *mut i32) -> i32 {
    let mut btrec: *mut xfs_btree_rec = core::ptr::null_mut();
    let error = xfs_btree_get_rec(cur, &mut btrec, has);
    if error != 0 || *has == 0 { return error; }
    core::ptr::copy_nonoverlapping(btrec as *const rcbag_rec, rec, 1);
    0
}

pub unsafe fn rcbagbt_update(cur: *mut xfs_btree_cur, rec: *const rcbag_rec) -> i32 {
    let mut btrec: xfs_btree_rec = core::mem::zeroed();
    core::ptr::copy_nonoverlapping(rec, &mut btrec as *mut _ as *mut rcbag_rec, 1);
    xfs_btree_update(cur, &mut btrec)
}

pub unsafe fn rcbagbt_insert(cur: *mut xfs_btree_cur, rec: *const rcbag_rec, success: *mut i32) -> i32 {
    let btrec = &mut (*cur).bc_rec as *mut _ as *mut rcbag_rec;
    core::ptr::copy_nonoverlapping(rec, btrec, 1);
    xfs_btree_insert(cur, success)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
