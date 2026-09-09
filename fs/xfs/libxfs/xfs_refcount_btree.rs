// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */
// Dependencies supplied by the surrounding XFS translation.

static mut XFS_REFCOUNTBT_CUR_CACHE: *mut kmem_cache = core::ptr::null_mut();

unsafe fn xfs_refcountbt_dup_cursor(cur: *mut xfs_btree_cur) -> *mut xfs_btree_cur {
    xfs_refcountbt_init_cursor((*cur).bc_mp, (*cur).bc_tp, (*cur).bc_ag.agbp,
        to_perag((*cur).bc_group))
}

unsafe fn xfs_refcountbt_set_root(cur: *mut xfs_btree_cur, ptr: *const xfs_btree_ptr, inc: i32) {
    let agbp = (*cur).bc_ag.agbp;
    let agf = (*agbp).b_addr as *mut xfs_agf;
    let pag = (*agbp).b_pag;
    assert!((*ptr).s != 0);
    (*agf).agf_refcount_root = (*ptr).s;
    be32_add_cpu(&mut (*agf).agf_refcount_level, inc);
    (*pag).pagf_refcount_level += inc as u32;
    xfs_alloc_log_agf((*cur).bc_tp, agbp, XFS_AGF_REFCOUNT_ROOT | XFS_AGF_REFCOUNT_LEVEL);
}

unsafe fn xfs_refcountbt_alloc_block(cur: *mut xfs_btree_cur, _start: *const xfs_btree_ptr,
    new: *mut xfs_btree_ptr, stat: *mut i32) -> i32 {
    let agbp = (*cur).bc_ag.agbp;
    let agf = (*agbp).b_addr as *mut xfs_agf;
    let mut args: xfs_alloc_arg = core::mem::zeroed();
    args.tp = (*cur).bc_tp;
    args.mp = (*cur).bc_mp;
    args.pag = to_perag((*cur).bc_group);
    args.oinfo = XFS_RMAP_OINFO_REFC;
    args.minlen = 1; args.maxlen = 1; args.prod = 1;
    args.resv = XFS_AG_RESV_METADATA;
    let error = xfs_alloc_vextent_near_bno(&mut args,
        xfs_agbno_to_fsb(args.pag, xfs_refc_block(args.mp)));
    if error != 0 { return error; }
    if args.fsbno == NULLFSBLOCK { *stat = 0; return 0; }
    assert!(args.agno == (*cur).bc_group.xg_gno);
    assert!(args.len == 1);
    (*new).s = cpu_to_be32(args.agbno);
    be32_add_cpu(&mut (*agf).agf_refcount_blocks, 1);
    xfs_alloc_log_agf((*cur).bc_tp, agbp, XFS_AGF_REFCOUNT_BLOCKS);
    *stat = 1;
    0
}

unsafe fn xfs_refcountbt_free_block(cur: *mut xfs_btree_cur, bp: *mut xfs_buf) -> i32 {
    let mp = (*cur).bc_mp;
    let agbp = (*cur).bc_ag.agbp;
    let agf = (*agbp).b_addr as *mut xfs_agf;
    let fsbno = XFS_DADDR_TO_FSB(mp, xfs_buf_daddr(bp));
    be32_add_cpu(&mut (*agf).agf_refcount_blocks, -1);
    xfs_alloc_log_agf((*cur).bc_tp, agbp, XFS_AGF_REFCOUNT_BLOCKS);
    xfs_free_extent_later((*cur).bc_tp, fsbno, 1, &XFS_RMAP_OINFO_REFC,
        XFS_AG_RESV_METADATA, 0)
}

unsafe fn xfs_refcountbt_get_minrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 {
    (*cur).bc_mp.m_refc_mnr[(level != 0) as usize]
}
unsafe fn xfs_refcountbt_get_maxrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 {
    (*cur).bc_mp.m_refc_mxr[(level != 0) as usize]
}
unsafe fn xfs_refcountbt_init_key_from_rec(key: *mut xfs_btree_key, rec: *const xfs_btree_rec) {
    (*key).refc.rc_startblock = (*rec).refc.rc_startblock;
}
unsafe fn xfs_refcountbt_init_high_key_from_rec(key: *mut xfs_btree_key, rec: *const xfs_btree_rec) {
    let x = be32_to_cpu((*rec).refc.rc_startblock) + be32_to_cpu((*rec).refc.rc_blockcount) - 1;
    (*key).refc.rc_startblock = cpu_to_be32(x);
}
unsafe fn xfs_refcountbt_init_rec_from_cur(cur: *mut xfs_btree_cur, rec: *mut xfs_btree_rec) {
    let start = xfs_refcount_encode_startblock((*cur).bc_rec.rc.rc_startblock, (*cur).bc_rec.rc.rc_domain);
    (*rec).refc.rc_startblock = cpu_to_be32(start);
    (*rec).refc.rc_blockcount = cpu_to_be32((*cur).bc_rec.rc.rc_blockcount);
    (*rec).refc.rc_refcount = cpu_to_be32((*cur).bc_rec.rc.rc_refcount);
}
unsafe fn xfs_refcountbt_init_ptr_from_cur(cur: *mut xfs_btree_cur, ptr: *mut xfs_btree_ptr) {
    let agf = (*cur).bc_ag.agbp.b_addr as *mut xfs_agf;
    assert!((*cur).bc_group.xg_gno == be32_to_cpu((*agf).agf_seqno));
    (*ptr).s = (*agf).agf_refcount_root;
}
unsafe fn xfs_refcountbt_cmp_key_with_cur(cur: *mut xfs_btree_cur, key: *const xfs_btree_key) -> i32 {
    let start = xfs_refcount_encode_startblock((*cur).bc_rec.rc.rc_startblock, (*cur).bc_rec.rc.rc_domain);
    cmp_int(be32_to_cpu((*key).refc.rc_startblock), start)
}
unsafe fn xfs_refcountbt_cmp_two_keys(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key,
    k2: *const xfs_btree_key, mask: *const xfs_btree_key) -> i32 {
    assert!(mask.is_null() || (*mask).refc.rc_startblock != 0);
    cmp_int(be32_to_cpu((*k1).refc.rc_startblock), be32_to_cpu((*k2).refc.rc_startblock))
}

unsafe fn xfs_refcountbt_verify(bp: *mut xfs_buf) -> xfs_failaddr_t {
    let mp = (*bp).b_mount;
    let block = XFS_BUF_TO_BLOCK(bp);
    let pag = (*bp).b_pag;
    if !xfs_verify_magic(bp, (*block).bb_magic) || !xfs_has_reflink(mp) { return __this_address!(); }
    let fa = xfs_btree_agblock_v5hdr_verify(bp); if !fa.is_null() { return fa; }
    let level = be16_to_cpu((*block).bb_level);
    if !pag.is_null() && xfs_perag_initialised_agf(pag) {
        let maxlevel = (*pag).pagf_refcount_level;
        if level >= maxlevel { return __this_address!(); }
    } else if level >= (*mp).m_refc_maxlevels { return __this_address!(); }
    xfs_btree_agblock_verify(bp, (*mp).m_refc_mxr[(level != 0) as usize])
}

unsafe fn xfs_refcountbt_read_verify(bp: *mut xfs_buf) {
    if !xfs_btree_agblock_verify_crc(bp) { xfs_verifier_error(bp, -EFSBADCRC, __this_address!()); }
    else { let fa = xfs_refcountbt_verify(bp); if !fa.is_null() { xfs_verifier_error(bp, -EFSCORRUPTED, fa); } }
    if (*bp).b_error != 0 { trace_xfs_btree_corrupt(bp, _RET_IP!()); }
}
unsafe fn xfs_refcountbt_write_verify(bp: *mut xfs_buf) {
    let fa = xfs_refcountbt_verify(bp);
    if !fa.is_null() { trace_xfs_btree_corrupt(bp, _RET_IP!()); xfs_verifier_error(bp, -EFSCORRUPTED, fa); return; }
    xfs_btree_agblock_calc_crc(bp);
}

pub static XFS_REFCOUNTBT_BUF_OPS: xfs_buf_ops = xfs_buf_ops {
    name: "xfs_refcountbt", magic: [0, cpu_to_be32(XFS_REFC_CRC_MAGIC)],
    verify_read: xfs_refcountbt_read_verify, verify_write: xfs_refcountbt_write_verify,
    verify_struct: xfs_refcountbt_verify,
};

unsafe fn xfs_refcountbt_keys_inorder(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key, k2: *const xfs_btree_key) -> bool {
    be32_to_cpu((*k1).refc.rc_startblock) < be32_to_cpu((*k2).refc.rc_startblock)
}
unsafe fn xfs_refcountbt_recs_inorder(_cur: *mut xfs_btree_cur, r1: *const xfs_btree_rec, r2: *const xfs_btree_rec) -> bool {
    be32_to_cpu((*r1).refc.rc_startblock) + be32_to_cpu((*r1).refc.rc_blockcount) <= be32_to_cpu((*r2).refc.rc_startblock)
}
unsafe fn xfs_refcountbt_keys_contiguous(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key, k2: *const xfs_btree_key, mask: *const xfs_btree_key) -> enum_xbtree_key_contig {
    assert!(mask.is_null() || (*mask).refc.rc_startblock != 0);
    xbtree_key_contig(be32_to_cpu((*k1).refc.rc_startblock), be32_to_cpu((*k2).refc.rc_startblock))
}

pub static XFS_REFCOUNTBT_OPS: xfs_btree_ops = xfs_btree_ops {
    name: "refcount", type_: XFS_BTREE_TYPE_AG,
    rec_len: core::mem::size_of::<xfs_refcount_rec>() as u32,
    key_len: core::mem::size_of::<xfs_refcount_key>() as u32,
    ptr_len: XFS_BTREE_SHORT_PTR_LEN,
    lru_refs: XFS_REFC_BTREE_REF, statoff: XFS_STATS_CALC_INDEX(xs_refcbt_2),
    sick_mask: XFS_SICK_AG_REFCNTBT,
    dup_cursor: xfs_refcountbt_dup_cursor, set_root: xfs_refcountbt_set_root,
    alloc_block: xfs_refcountbt_alloc_block, free_block: xfs_refcountbt_free_block,
    get_minrecs: xfs_refcountbt_get_minrecs, get_maxrecs: xfs_refcountbt_get_maxrecs,
    init_key_from_rec: xfs_refcountbt_init_key_from_rec,
    init_high_key_from_rec: xfs_refcountbt_init_high_key_from_rec,
    init_rec_from_cur: xfs_refcountbt_init_rec_from_cur,
    init_ptr_from_cur: xfs_refcountbt_init_ptr_from_cur,
    cmp_key_with_cur: xfs_refcountbt_cmp_key_with_cur,
    buf_ops: &XFS_REFCOUNTBT_BUF_OPS, cmp_two_keys: xfs_refcountbt_cmp_two_keys,
    keys_inorder: xfs_refcountbt_keys_inorder, recs_inorder: xfs_refcountbt_recs_inorder,
    keys_contiguous: xfs_refcountbt_keys_contiguous,
};

pub unsafe fn xfs_refcountbt_init_cursor(mp: *mut xfs_mount, tp: *mut xfs_trans, agbp: *mut xfs_buf, pag: *mut xfs_perag) -> *mut xfs_btree_cur {
    assert!(pag_agno(pag) < (*mp).m_sb.sb_agcount);
    let cur = xfs_btree_alloc_cursor(mp, tp, &XFS_REFCOUNTBT_OPS, (*mp).m_refc_maxlevels, XFS_REFCOUNTBT_CUR_CACHE);
    (*cur).bc_group = xfs_group_hold(pag_group(pag));
    (*cur).bc_refc.nr_ops = 0; (*cur).bc_refc.shape_changes = 0; (*cur).bc_ag.agbp = agbp;
    if !agbp.is_null() { (*cur).bc_nlevels = be32_to_cpu((*((*agbp).b_addr as *mut xfs_agf)).agf_refcount_level); }
    cur
}

pub unsafe fn xfs_refcountbt_commit_staged_btree(cur: *mut xfs_btree_cur, tp: *mut xfs_trans, agbp: *mut xfs_buf) {
    let agf = (*agbp).b_addr as *mut xfs_agf; let afake = (*cur).bc_ag.afake;
    assert!((*cur).bc_flags & XFS_BTREE_STAGING != 0);
    (*agf).agf_refcount_root = cpu_to_be32((*afake).af_root); (*agf).agf_refcount_level = cpu_to_be32((*afake).af_levels); (*agf).agf_refcount_blocks = cpu_to_be32((*afake).af_blocks);
    xfs_alloc_log_agf(tp, agbp, XFS_AGF_REFCOUNT_BLOCKS | XFS_AGF_REFCOUNT_ROOT | XFS_AGF_REFCOUNT_LEVEL);
    xfs_btree_commit_afakeroot(cur, tp, agbp);
}

unsafe fn xfs_refcountbt_block_maxrecs(blocklen: u32, leaf: bool) -> u32 {
    if leaf { blocklen / core::mem::size_of::<xfs_refcount_rec>() as u32 } else { blocklen / (core::mem::size_of::<xfs_refcount_key>() as u32 + core::mem::size_of::<xfs_refcount_ptr_t>() as u32) }
}
pub unsafe fn xfs_refcountbt_maxrecs(_mp: *mut xfs_mount, mut blocklen: u32, leaf: bool) -> u32 { blocklen -= XFS_REFCOUNT_BLOCK_LEN; xfs_refcountbt_block_maxrecs(blocklen, leaf) }
pub unsafe fn xfs_refcountbt_maxlevels_ondisk() -> u32 {
    let blocklen = XFS_MIN_CRC_BLOCKSIZE - XFS_BTREE_SBLOCK_CRC_LEN;
    let minrecs = [xfs_refcountbt_block_maxrecs(blocklen, true) / 2, xfs_refcountbt_block_maxrecs(blocklen, false) / 2];
    xfs_btree_compute_maxlevels(minrecs.as_ptr(), XFS_MAX_CRC_AG_BLOCKS)
}
pub unsafe fn xfs_refcountbt_compute_maxlevels(mp: *mut xfs_mount) { if !xfs_has_reflink(mp) { (*mp).m_refc_maxlevels = 0; return; } (*mp).m_refc_maxlevels = xfs_btree_compute_maxlevels((*mp).m_refc_mnr.as_ptr(), (*mp).m_sb.sb_agblocks); assert!((*mp).m_refc_maxlevels <= xfs_refcountbt_maxlevels_ondisk()); }
pub unsafe fn xfs_refcountbt_calc_size(mp: *mut xfs_mount, len: u64) -> xfs_extlen_t { xfs_btree_calc_size((*mp).m_refc_mnr.as_ptr(), len) }
pub unsafe fn xfs_refcountbt_max_size(mp: *mut xfs_mount, agblocks: xfs_agblock_t) -> xfs_extlen_t { if (*mp).m_refc_mxr[0] == 0 { 0 } else { xfs_refcountbt_calc_size(mp, agblocks as u64) } }

pub unsafe fn xfs_refcountbt_calc_reserves(mp: *mut xfs_mount, tp: *mut xfs_trans, pag: *mut xfs_perag, ask: *mut xfs_extlen_t, used: *mut xfs_extlen_t) -> i32 {
    if !xfs_has_reflink(mp) { return 0; }
    let mut agbp = core::ptr::null_mut(); let error = xfs_alloc_read_agf(pag, tp, 0, &mut agbp); if error != 0 { return error; }
    let agf = (*agbp).b_addr as *mut xfs_agf; let mut agblocks = be32_to_cpu((*agf).agf_length); let tree_len = be32_to_cpu((*agf).agf_refcount_blocks); xfs_trans_brelse(tp, agbp);
    if xfs_ag_contains_log(mp, pag_agno(pag)) { agblocks -= (*mp).m_sb.sb_logblocks; }
    *ask += xfs_refcountbt_max_size(mp, agblocks); *used += tree_len; error
}
pub unsafe fn xfs_refcountbt_init_cur_cache() -> i32 { XFS_REFCOUNTBT_CUR_CACHE = kmem_cache_create("xfs_refcbt_cur", xfs_btree_cur_sizeof(xfs_refcountbt_maxlevels_ondisk()), 0, 0, core::ptr::null_mut()); if XFS_REFCOUNTBT_CUR_CACHE.is_null() { -ENOMEM } else { 0 } }
pub unsafe fn xfs_refcountbt_destroy_cur_cache() { kmem_cache_destroy(XFS_REFCOUNTBT_CUR_CACHE); XFS_REFCOUNTBT_CUR_CACHE = core::ptr::null_mut(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
