// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2001,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */
// C dependencies: xfs_platform.h, xfs_fs.h, xfs_shared.h, xfs_format.h,
// xfs_log_format.h, xfs_trans_resv.h, xfs_mount.h, xfs_btree.h,
// xfs_btree_staging.h, xfs_alloc_btree.h, xfs_alloc.h, xfs_extent_busy.h,
// xfs_error.h, xfs_health.h, xfs_trace.h, xfs_trans.h, xfs_ag.h

static mut xfs_allocbt_cur_cache: *mut kmem_cache = core::ptr::null_mut();

unsafe fn xfs_bnobt_dup_cursor(cur: *mut xfs_btree_cur) -> *mut xfs_btree_cur {
    xfs_bnobt_init_cursor((*cur).bc_mp, (*cur).bc_tp, (*cur).bc_ag.agbp,
        to_perag((*cur).bc_group))
}

unsafe fn xfs_cntbt_dup_cursor(cur: *mut xfs_btree_cur) -> *mut xfs_btree_cur {
    xfs_cntbt_init_cursor((*cur).bc_mp, (*cur).bc_tp, (*cur).bc_ag.agbp,
        to_perag((*cur).bc_group))
}

unsafe fn xfs_allocbt_set_root(cur: *mut xfs_btree_cur,
    ptr: *const xfs_btree_ptr, inc: i32) {
    let pag = to_perag((*cur).bc_group);
    let agbp = (*cur).bc_ag.agbp;
    let agf = (*agbp).b_addr as *mut xfs_agf;
    assert!((*ptr).s != 0);
    if xfs_btree_is_bno((*cur).bc_ops) {
        (*agf).agf_bno_root = (*ptr).s;
        be32_add_cpu(&mut (*agf).agf_bno_level, inc);
        (*pag).pagf_bno_level += inc;
    } else {
        (*agf).agf_cnt_root = (*ptr).s;
        be32_add_cpu(&mut (*agf).agf_cnt_level, inc);
        (*pag).pagf_cnt_level += inc;
    }
    xfs_alloc_log_agf((*cur).bc_tp, agbp, XFS_AGF_ROOTS | XFS_AGF_LEVELS);
}

unsafe fn xfs_allocbt_alloc_block(cur: *mut xfs_btree_cur,
    _start: *const xfs_btree_ptr, new: *mut xfs_btree_ptr, stat: *mut i32) -> i32 {
    let mut bno: xfs_agblock_t = 0;
    let error = xfs_alloc_get_freelist(to_perag((*cur).bc_group), (*cur).bc_tp,
        (*cur).bc_ag.agbp, &mut bno, 1);
    if error != 0 { return error; }
    if bno == NULLAGBLOCK {
        *stat = 0;
        return 0;
    }
    atomic64_inc(&mut (*(*cur).bc_mp).m_allocbt_blks);
    xfs_extent_busy_reuse((*cur).bc_group, bno, 1, false);
    (*new).s = cpu_to_be32(bno);
    *stat = 1;
    0
}

unsafe fn xfs_allocbt_free_block(cur: *mut xfs_btree_cur, bp: *mut xfs_buf) -> i32 {
    let agbp = (*cur).bc_ag.agbp;
    let bno = xfs_daddr_to_agbno((*cur).bc_mp, xfs_buf_daddr(bp));
    let error = xfs_alloc_put_freelist(to_perag((*cur).bc_group), (*cur).bc_tp,
        agbp, core::ptr::null_mut(), bno, 1);
    if error != 0 { return error; }
    atomic64_dec(&mut (*(*cur).bc_mp).m_allocbt_blks);
    xfs_extent_busy_insert((*cur).bc_tp, pag_group((*agbp).b_pag), bno, 1,
        XFS_EXTENT_BUSY_SKIP_DISCARD);
    0
}

unsafe fn xfs_allocbt_get_minrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 {
    (*cur).bc_mp.as_ref().unwrap().m_alloc_mnr[(level != 0) as usize]
}
unsafe fn xfs_allocbt_get_maxrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 {
    (*cur).bc_mp.as_ref().unwrap().m_alloc_mxr[(level != 0) as usize]
}
unsafe fn xfs_allocbt_init_key_from_rec(key: *mut xfs_btree_key, rec: *const xfs_btree_rec) {
    (*key).alloc.ar_startblock = (*rec).alloc.ar_startblock;
    (*key).alloc.ar_blockcount = (*rec).alloc.ar_blockcount;
}
unsafe fn xfs_bnobt_init_high_key_from_rec(key: *mut xfs_btree_key, rec: *const xfs_btree_rec) {
    let x = be32_to_cpu((*rec).alloc.ar_startblock) + be32_to_cpu((*rec).alloc.ar_blockcount) - 1;
    (*key).alloc.ar_startblock = cpu_to_be32(x);
    (*key).alloc.ar_blockcount = 0;
}
unsafe fn xfs_cntbt_init_high_key_from_rec(key: *mut xfs_btree_key, rec: *const xfs_btree_rec) {
    (*key).alloc.ar_blockcount = (*rec).alloc.ar_blockcount;
    (*key).alloc.ar_startblock = 0;
}
unsafe fn xfs_allocbt_init_rec_from_cur(cur: *mut xfs_btree_cur, rec: *mut xfs_btree_rec) {
    (*rec).alloc.ar_startblock = cpu_to_be32((*cur).bc_rec.a.ar_startblock);
    (*rec).alloc.ar_blockcount = cpu_to_be32((*cur).bc_rec.a.ar_blockcount);
}
unsafe fn xfs_allocbt_init_ptr_from_cur(cur: *mut xfs_btree_cur, ptr: *mut xfs_btree_ptr) {
    let agf = (*(*cur).bc_ag.agbp).b_addr as *mut xfs_agf;
    assert!((*cur).bc_group.xg_gno == be32_to_cpu((*agf).agf_seqno));
    (*ptr).s = if xfs_btree_is_bno((*cur).bc_ops) { (*agf).agf_bno_root } else { (*agf).agf_cnt_root };
}

unsafe fn xfs_bnobt_cmp_key_with_cur(cur: *mut xfs_btree_cur, key: *const xfs_btree_key) -> i32 {
    cmp_int(be32_to_cpu((*key).alloc.ar_startblock), (*cur).bc_rec.a.ar_startblock)
}
unsafe fn xfs_cntbt_cmp_key_with_cur(cur: *mut xfs_btree_cur, key: *const xfs_btree_key) -> i32 {
    let a = cmp_int(be32_to_cpu((*key).alloc.ar_blockcount), (*cur).bc_rec.a.ar_blockcount);
    if a != 0 { a } else { cmp_int(be32_to_cpu((*key).alloc.ar_startblock), (*cur).bc_rec.a.ar_startblock) }
}
unsafe fn xfs_bnobt_cmp_two_keys(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key, k2: *const xfs_btree_key, mask: *const xfs_btree_key) -> i32 {
    assert!(mask.is_null() || (*mask).alloc.ar_startblock != 0);
    cmp_int(be32_to_cpu((*k1).alloc.ar_startblock), be32_to_cpu((*k2).alloc.ar_startblock))
}
unsafe fn xfs_cntbt_cmp_two_keys(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key, k2: *const xfs_btree_key, mask: *const xfs_btree_key) -> i32 {
    assert!(mask.is_null() || ((*mask).alloc.ar_blockcount != 0 && (*mask).alloc.ar_startblock != 0));
    let a = cmp_int(be32_to_cpu((*k1).alloc.ar_blockcount), be32_to_cpu((*k2).alloc.ar_blockcount));
    if a != 0 { a } else { cmp_int(be32_to_cpu((*k1).alloc.ar_startblock), be32_to_cpu((*k2).alloc.ar_startblock)) }
}

unsafe fn xfs_allocbt_verify(bp: *mut xfs_buf) -> xfs_failaddr_t {
    let mp = (*bp).b_mount;
    let block = XFS_BUF_TO_BLOCK(bp);
    let pag = (*bp).b_pag;
    if !xfs_verify_magic(bp, (*block).bb_magic) { return __this_address!(); }
    if xfs_has_crc(mp) {
        let fa = xfs_btree_agblock_v5hdr_verify(bp);
        if !fa.is_null() { return fa; }
    }
    let level = be16_to_cpu((*block).bb_level) as usize;
    if !pag.is_null() && xfs_perag_initialised_agf(pag) {
        let (maxlevel, repair_maxlevel) = if (*(*bp).b_ops).magic[0] == cpu_to_be32(XFS_ABTC_MAGIC) {
            ((*pag).pagf_cnt_level, 0)
            // CONFIG_XFS_ONLINE_REPAIR may provide pagf_repair_cnt_level.
        } else {
            ((*pag).pagf_bno_level, 0)
            // CONFIG_XFS_ONLINE_REPAIR may provide pagf_repair_bno_level.
        };
        if level >= core::cmp::max(maxlevel, repair_maxlevel) as usize { return __this_address!(); }
    } else if level >= (*mp).m_alloc_maxlevels as usize { return __this_address!(); }
    xfs_btree_agblock_verify(bp, (*mp).m_alloc_mxr[level != 0])
}

unsafe fn xfs_allocbt_read_verify(bp: *mut xfs_buf) {
    if !xfs_btree_agblock_verify_crc(bp) { xfs_verifier_error(bp, -EFSBADCRC, __this_address!()); }
    else { let fa = xfs_allocbt_verify(bp); if !fa.is_null() { xfs_verifier_error(bp, -EFSCORRUPTED, fa); } }
    if (*bp).b_error != 0 { trace_xfs_btree_corrupt(bp, _RET_IP!()); }
}
unsafe fn xfs_allocbt_write_verify(bp: *mut xfs_buf) {
    let fa = xfs_allocbt_verify(bp);
    if !fa.is_null() { trace_xfs_btree_corrupt(bp, _RET_IP!()); xfs_verifier_error(bp, -EFSCORRUPTED, fa); return; }
    xfs_btree_agblock_calc_crc(bp);
}

pub static xfs_bnobt_buf_ops: xfs_buf_ops = xfs_buf_ops {
    name: "xfs_bnobt", magic: [cpu_to_be32(XFS_ABTB_MAGIC), cpu_to_be32(XFS_ABTB_CRC_MAGIC)],
    verify_read: xfs_allocbt_read_verify, verify_write: xfs_allocbt_write_verify, verify_struct: xfs_allocbt_verify,
};
pub static xfs_cntbt_buf_ops: xfs_buf_ops = xfs_buf_ops {
    name: "xfs_cntbt", magic: [cpu_to_be32(XFS_ABTC_MAGIC), cpu_to_be32(XFS_ABTC_CRC_MAGIC)],
    verify_read: xfs_allocbt_read_verify, verify_write: xfs_allocbt_write_verify, verify_struct: xfs_allocbt_verify,
};

unsafe fn xfs_bnobt_keys_inorder(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key, k2: *const xfs_btree_key) -> bool { be32_to_cpu((*k1).alloc.ar_startblock) < be32_to_cpu((*k2).alloc.ar_startblock) }
unsafe fn xfs_bnobt_recs_inorder(_cur: *mut xfs_btree_cur, r1: *const xfs_btree_rec, r2: *const xfs_btree_rec) -> bool { be32_to_cpu((*r1).alloc.ar_startblock) + be32_to_cpu((*r1).alloc.ar_blockcount) <= be32_to_cpu((*r2).alloc.ar_startblock) }
unsafe fn xfs_cntbt_keys_inorder(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key, k2: *const xfs_btree_key) -> bool { be32_to_cpu((*k1).alloc.ar_blockcount) < be32_to_cpu((*k2).alloc.ar_blockcount) || ((*k1).alloc.ar_blockcount == (*k2).alloc.ar_blockcount && be32_to_cpu((*k1).alloc.ar_startblock) < be32_to_cpu((*k2).alloc.ar_startblock)) }
unsafe fn xfs_cntbt_recs_inorder(_cur: *mut xfs_btree_cur, r1: *const xfs_btree_rec, r2: *const xfs_btree_rec) -> bool { be32_to_cpu((*r1).alloc.ar_blockcount) < be32_to_cpu((*r2).alloc.ar_blockcount) || ((*r1).alloc.ar_blockcount == (*r2).alloc.ar_blockcount && be32_to_cpu((*r1).alloc.ar_startblock) < be32_to_cpu((*r2).alloc.ar_startblock)) }
unsafe fn xfs_allocbt_keys_contiguous(_cur: *mut xfs_btree_cur, key1: *const xfs_btree_key, key2: *const xfs_btree_key, mask: *const xfs_btree_key) -> xbtree_key_contig { assert!(mask.is_null() || (*mask).alloc.ar_startblock != 0); xbtree_key_contig(be32_to_cpu((*key1).alloc.ar_startblock), be32_to_cpu((*key2).alloc.ar_startblock)) }

// The following operation tables preserve the C callback wiring and constants.
pub static xfs_bnobt_ops: xfs_btree_ops = xfs_btree_ops {
    name: "bno", type_: XFS_BTREE_TYPE_AG, rec_len: core::mem::size_of::<xfs_alloc_rec_t>(), key_len: core::mem::size_of::<xfs_alloc_key_t>(), ptr_len: XFS_BTREE_SHORT_PTR_LEN,
    lru_refs: XFS_ALLOC_BTREE_REF, statoff: XFS_STATS_CALC_INDEX(xs_abtb_2), sick_mask: XFS_SICK_AG_BNOBT,
    dup_cursor: xfs_bnobt_dup_cursor, set_root: xfs_allocbt_set_root, alloc_block: xfs_allocbt_alloc_block, free_block: xfs_allocbt_free_block,
    get_minrecs: xfs_allocbt_get_minrecs, get_maxrecs: xfs_allocbt_get_maxrecs, init_key_from_rec: xfs_allocbt_init_key_from_rec,
    init_high_key_from_rec: xfs_bnobt_init_high_key_from_rec, init_rec_from_cur: xfs_allocbt_init_rec_from_cur, init_ptr_from_cur: xfs_allocbt_init_ptr_from_cur,
    cmp_key_with_cur: xfs_bnobt_cmp_key_with_cur, buf_ops: &xfs_bnobt_buf_ops, cmp_two_keys: xfs_bnobt_cmp_two_keys, keys_inorder: xfs_bnobt_keys_inorder,
    recs_inorder: xfs_bnobt_recs_inorder, keys_contiguous: xfs_allocbt_keys_contiguous,
};
pub static xfs_cntbt_ops: xfs_btree_ops = xfs_btree_ops {
    name: "cnt", type_: XFS_BTREE_TYPE_AG, rec_len: core::mem::size_of::<xfs_alloc_rec_t>(), key_len: core::mem::size_of::<xfs_alloc_key_t>(), ptr_len: XFS_BTREE_SHORT_PTR_LEN,
    lru_refs: XFS_ALLOC_BTREE_REF, statoff: XFS_STATS_CALC_INDEX(xs_abtc_2), sick_mask: XFS_SICK_AG_CNTBT,
    dup_cursor: xfs_cntbt_dup_cursor, set_root: xfs_allocbt_set_root, alloc_block: xfs_allocbt_alloc_block, free_block: xfs_allocbt_free_block,
    get_minrecs: xfs_allocbt_get_minrecs, get_maxrecs: xfs_allocbt_get_maxrecs, init_key_from_rec: xfs_allocbt_init_key_from_rec,
    init_high_key_from_rec: xfs_cntbt_init_high_key_from_rec, init_rec_from_cur: xfs_allocbt_init_rec_from_cur, init_ptr_from_cur: xfs_allocbt_init_ptr_from_cur,
    cmp_key_with_cur: xfs_cntbt_cmp_key_with_cur, buf_ops: &xfs_cntbt_buf_ops, cmp_two_keys: xfs_cntbt_cmp_two_keys, keys_inorder: xfs_cntbt_keys_inorder,
    recs_inorder: xfs_cntbt_recs_inorder, keys_contiguous: core::option::Option::None,
};

pub unsafe fn xfs_bnobt_init_cursor(mp: *mut xfs_mount, tp: *mut xfs_trans, agbp: *mut xfs_buf, pag: *mut xfs_perag) -> *mut xfs_btree_cur {
    let cur = xfs_btree_alloc_cursor(mp, tp, &xfs_bnobt_ops, (*mp).m_alloc_maxlevels, xfs_allocbt_cur_cache);
    (*cur).bc_group = xfs_group_hold(pag_group(pag)); (*cur).bc_ag.agbp = agbp;
    if !agbp.is_null() { (*cur).bc_nlevels = be32_to_cpu((*((*agbp).b_addr as *mut xfs_agf)).agf_bno_level); } cur
}
pub unsafe fn xfs_cntbt_init_cursor(mp: *mut xfs_mount, tp: *mut xfs_trans, agbp: *mut xfs_buf, pag: *mut xfs_perag) -> *mut xfs_btree_cur {
    let cur = xfs_btree_alloc_cursor(mp, tp, &xfs_cntbt_ops, (*mp).m_alloc_maxlevels, xfs_allocbt_cur_cache);
    (*cur).bc_group = xfs_group_hold(pag_group(pag)); (*cur).bc_ag.agbp = agbp;
    if !agbp.is_null() { (*cur).bc_nlevels = be32_to_cpu((*((*agbp).b_addr as *mut xfs_agf)).agf_cnt_level); } cur
}

pub unsafe fn xfs_allocbt_commit_staged_btree(cur: *mut xfs_btree_cur, tp: *mut xfs_trans, agbp: *mut xfs_buf) {
    let agf = (*agbp).b_addr as *mut xfs_agf; let afake = (*cur).bc_ag.afake;
    assert!((*cur).bc_flags & XFS_BTREE_STAGING != 0);
    if xfs_btree_is_bno((*cur).bc_ops) { (*agf).agf_bno_root = cpu_to_be32((*afake).af_root); (*agf).agf_bno_level = cpu_to_be32((*afake).af_levels); }
    else { (*agf).agf_cnt_root = cpu_to_be32((*afake).af_root); (*agf).agf_cnt_level = cpu_to_be32((*afake).af_levels); }
    xfs_alloc_log_agf(tp, agbp, XFS_AGF_ROOTS | XFS_AGF_LEVELS); xfs_btree_commit_afakeroot(cur, tp, agbp);
}

#[inline] unsafe fn xfs_allocbt_block_maxrecs(blocklen: u32, leaf: bool) -> u32 { if leaf { blocklen / core::mem::size_of::<xfs_alloc_rec_t>() as u32 } else { blocklen / (core::mem::size_of::<xfs_alloc_key_t>() as u32 + core::mem::size_of::<xfs_alloc_ptr_t>() as u32) } }
pub unsafe fn xfs_allocbt_maxrecs(mp: *mut xfs_mount, mut blocklen: u32, leaf: bool) -> u32 { blocklen -= XFS_ALLOC_BLOCK_LEN(mp); xfs_allocbt_block_maxrecs(blocklen, leaf) }
pub const XFS_MAX_FREESP_RECORDS: u32 = (XFS_MAX_AG_BLOCKS + 1) / 2;
pub unsafe fn xfs_allocbt_maxlevels_ondisk() -> u32 { let blocklen = core::cmp::min(XFS_MIN_BLOCKSIZE - XFS_BTREE_SBLOCK_LEN, XFS_MIN_CRC_BLOCKSIZE - XFS_BTREE_SBLOCK_CRC_LEN); let minrecs = [xfs_allocbt_block_maxrecs(blocklen, true) / 2, xfs_allocbt_block_maxrecs(blocklen, false) / 2]; xfs_btree_compute_maxlevels(minrecs.as_ptr(), XFS_MAX_FREESP_RECORDS) }
pub unsafe fn xfs_allocbt_calc_size(mp: *mut xfs_mount, len: u64) -> xfs_extlen_t { xfs_btree_calc_size((*mp).m_alloc_mnr.as_ptr(), len) }
pub unsafe fn xfs_allocbt_init_cur_cache() -> i32 { xfs_allocbt_cur_cache = kmem_cache_create("xfs_bnobt_cur", xfs_btree_cur_sizeof(xfs_allocbt_maxlevels_ondisk()), 0, 0, None); if xfs_allocbt_cur_cache.is_null() { -ENOMEM } else { 0 } }
pub unsafe fn xfs_allocbt_destroy_cur_cache() { kmem_cache_destroy(xfs_allocbt_cur_cache); xfs_allocbt_cur_cache = core::ptr::null_mut(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
