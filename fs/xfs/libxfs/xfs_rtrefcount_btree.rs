// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C dependencies are supplied by the surrounding translation unit.

static mut XFS_RTREFCOUNTBT_CUR_CACHE: *mut kmem_cache = core::ptr::null_mut();
extern "C" { static XFS_RTREFCOUNTBT_OPS: xfs_btree_ops; }

unsafe fn xfs_rtrefcountbt_dup_cursor(cur: *mut xfs_btree_cur) -> *mut xfs_btree_cur {
    xfs_rtrefcountbt_init_cursor((*cur).bc_tp, to_rtg((*cur).bc_group))
}

unsafe fn xfs_rtrefcountbt_get_minrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 {
    if level == (*cur).bc_nlevels - 1 {
        let ifp = xfs_btree_ifork_ptr(cur);
        return xfs_rtrefcountbt_maxrecs((*cur).bc_mp, (*ifp).if_broot_bytes, level == 0) / 2;
    }
    (*(*cur).bc_mp).m_rtrefc_mnr[(level != 0) as usize]
}

unsafe fn xfs_rtrefcountbt_get_maxrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 {
    if level == (*cur).bc_nlevels - 1 {
        let ifp = xfs_btree_ifork_ptr(cur);
        return xfs_rtrefcountbt_maxrecs((*cur).bc_mp, (*ifp).if_broot_bytes, level == 0);
    }
    (*(*cur).bc_mp).m_rtrefc_mxr[(level != 0) as usize]
}

unsafe fn xfs_rtrefcountbt_droot_maxrecs(mut blocklen: u32, leaf: bool) -> u32 {
    blocklen -= core::mem::size_of::<xfs_rtrefcount_root>() as u32;
    if leaf { blocklen / core::mem::size_of::<xfs_refcount_rec>() as u32 }
    else { blocklen / (2 * core::mem::size_of::<xfs_refcount_key>() as u32 + core::mem::size_of::<xfs_rtrefcount_ptr_t>() as u32) }
}

unsafe fn xfs_rtrefcountbt_get_dmaxrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 {
    if level != (*cur).bc_nlevels - 1 { return (*(*cur).bc_mp).m_rtrefc_mxr[(level != 0) as usize]; }
    xfs_rtrefcountbt_droot_maxrecs((*cur).bc_ino.forksize, level == 0) as i32
}

unsafe fn xfs_rtrefcountbt_init_key_from_rec(key: *mut xfs_btree_key, rec: *const xfs_btree_rec) {
    (*key).refc.rc_startblock = (*rec).refc.rc_startblock;
}

unsafe fn xfs_rtrefcountbt_init_high_key_from_rec(key: *mut xfs_btree_key, rec: *const xfs_btree_rec) {
    let x = be32_to_cpu((*rec).refc.rc_startblock).wrapping_add(be32_to_cpu((*rec).refc.rc_blockcount)).wrapping_sub(1);
    (*key).refc.rc_startblock = cpu_to_be32(x);
}

unsafe fn xfs_rtrefcountbt_init_rec_from_cur(cur: *mut xfs_btree_cur, rec: *mut xfs_btree_rec) {
    let irec = &(*cur).bc_rec.rc;
    let start = xfs_refcount_encode_startblock(irec.rc_startblock, irec.rc_domain);
    (*rec).refc.rc_startblock = cpu_to_be32(start);
    (*rec).refc.rc_blockcount = cpu_to_be32((*cur).bc_rec.rc.rc_blockcount);
    (*rec).refc.rc_refcount = cpu_to_be32((*cur).bc_rec.rc.rc_refcount);
}

unsafe fn xfs_rtrefcountbt_init_ptr_from_cur(_cur: *mut xfs_btree_cur, ptr: *mut xfs_btree_ptr) { (*ptr).l = 0; }

unsafe fn xfs_rtrefcountbt_cmp_key_with_cur(cur: *mut xfs_btree_cur, key: *const xfs_btree_key) -> i32 {
    let start = xfs_refcount_encode_startblock((*cur).bc_rec.rc.rc_startblock, (*cur).bc_rec.rc.rc_domain);
    cmp_int(be32_to_cpu((*key).refc.rc_startblock), start)
}

unsafe fn xfs_rtrefcountbt_cmp_two_keys(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key, k2: *const xfs_btree_key, mask: *const xfs_btree_key) -> i32 {
    ASSERT(mask.is_null() || (*mask).refc.rc_startblock != 0);
    cmp_int(be32_to_cpu((*k1).refc.rc_startblock), be32_to_cpu((*k2).refc.rc_startblock))
}

unsafe fn xfs_rtrefcountbt_verify(bp: *mut xfs_buf) -> xfs_failaddr_t {
    let mp = (*(*bp).b_target).bt_mount;
    let block = XFS_BUF_TO_BLOCK(bp);
    if !xfs_verify_magic(bp, (*block).bb_magic) || !xfs_has_reflink(mp) { return __this_address; }
    let fa = xfs_btree_fsblock_v5hdr_verify(bp, XFS_RMAP_OWN_UNKNOWN);
    if !fa.is_null() { return fa; }
    let level = be16_to_cpu((*block).bb_level);
    if level >= (*mp).m_rtrefc_maxlevels { return __this_address; }
    xfs_btree_fsblock_verify(bp, (*mp).m_rtrefc_mxr[(level != 0) as usize])
}

unsafe fn xfs_rtrefcountbt_read_verify(bp: *mut xfs_buf) {
    if !xfs_btree_fsblock_verify_crc(bp) { xfs_verifier_error(bp, -EFSBADCRC, __this_address); }
    else { let fa = xfs_rtrefcountbt_verify(bp); if !fa.is_null() { xfs_verifier_error(bp, -EFSCORRUPTED, fa); } }
    if (*bp).b_error != 0 { trace_xfs_btree_corrupt(bp, _RET_IP_); }
}

unsafe fn xfs_rtrefcountbt_write_verify(bp: *mut xfs_buf) {
    let fa = xfs_rtrefcountbt_verify(bp);
    if !fa.is_null() { trace_xfs_btree_corrupt(bp, _RET_IP_); xfs_verifier_error(bp, -EFSCORRUPTED, fa); return; }
    xfs_btree_fsblock_calc_crc(bp);
}

unsafe fn xfs_rtrefcountbt_keys_inorder(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key, k2: *const xfs_btree_key) -> bool { be32_to_cpu((*k1).refc.rc_startblock) < be32_to_cpu((*k2).refc.rc_startblock) }
unsafe fn xfs_rtrefcountbt_recs_inorder(_cur: *mut xfs_btree_cur, r1: *const xfs_btree_rec, r2: *const xfs_btree_rec) -> bool { be32_to_cpu((*r1).refc.rc_startblock).wrapping_add(be32_to_cpu((*r1).refc.rc_blockcount)) <= be32_to_cpu((*r2).refc.rc_startblock) }
unsafe fn xfs_rtrefcountbt_keys_contiguous(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key, k2: *const xfs_btree_key, mask: *const xfs_btree_key) -> enum_xbtree_key_contig { ASSERT(mask.is_null() || (*mask).refc.rc_startblock != 0); xbtree_key_contig(be32_to_cpu((*k1).refc.rc_startblock), be32_to_cpu((*k2).refc.rc_startblock)) }

unsafe fn xfs_rtrefcountbt_move_ptrs(mp: *mut xfs_mount, broot: *mut xfs_btree_block, old_size: i16, new_size: usize, numrecs: u32) {
    let sptr = xfs_rtrefcount_broot_ptr_addr(mp, broot, 1, old_size as usize);
    let dptr = xfs_rtrefcount_broot_ptr_addr(mp, broot, 1, new_size);
    core::ptr::copy(sptr as *const u8, dptr as *mut u8, numrecs as usize * core::mem::size_of::<xfs_rtrefcount_ptr_t>());
}

unsafe fn xfs_rtrefcountbt_broot_realloc(cur: *mut xfs_btree_cur, new_numrecs: u32) -> *mut xfs_btree_block {
    let mp = (*cur).bc_mp;
    let ifp = xfs_btree_ifork_ptr(cur);
    let level = (*cur).bc_nlevels - 1;
    let old_size = (*ifp).if_broot_bytes;
    let new_size = xfs_rtrefcount_broot_space_calc(mp, level, new_numrecs);
    if new_size == old_size { return (*ifp).if_broot; }
    if new_size > old_size {
        if old_size == 0 { return xfs_broot_realloc(ifp, new_size); }
        let old_numrecs = xfs_rtrefcountbt_maxrecs(mp, old_size, level == 0);
        let broot = xfs_broot_realloc(ifp, new_size);
        if level > 0 { xfs_rtrefcountbt_move_ptrs(mp, broot, old_size as i16, new_size as usize, old_numrecs); }
        return broot;
    }
    ASSERT(!(*ifp).if_broot.is_null() && old_size > 0);
    if new_size == 0 { return xfs_broot_realloc(ifp, 0); }
    if level > 0 { xfs_rtrefcountbt_move_ptrs(mp, (*ifp).if_broot, old_size as i16, new_size as usize, new_numrecs); }
    xfs_broot_realloc(ifp, new_size)
}

// The remaining btree operation table and root conversion routines retain the C ABI and
// are declared with the same external types supplied by the filesystem translation.
pub const XFS_RTREFCOUNTBT_BUF_OPS: xfs_buf_ops = xfs_buf_ops { name: "xfs_rtrefcountbt", magic: [0, cpu_to_be32(XFS_RTREFC_CRC_MAGIC)], verify_read: xfs_rtrefcountbt_read_verify, verify_write: xfs_rtrefcountbt_write_verify, verify_struct: xfs_rtrefcountbt_verify };

pub unsafe fn xfs_rtrefcountbt_maxrecs(mp: *mut xfs_mount, mut blocklen: u32, leaf: bool) -> u32 { blocklen -= XFS_RTREFCOUNT_BLOCK_LEN; if leaf { blocklen / core::mem::size_of::<xfs_refcount_rec>() as u32 } else { blocklen / (core::mem::size_of::<xfs_refcount_key>() as u32 + core::mem::size_of::<xfs_rtrefcount_ptr_t>() as u32) } }
pub unsafe fn xfs_rtrefcountbt_maxlevels_ondisk() -> u32 { let blocklen = XFS_MIN_CRC_BLOCKSIZE - XFS_BTREE_LBLOCK_CRC_LEN; let minrecs = [xfs_rtrefcountbt_block_maxrecs(blocklen, true) / 2, xfs_rtrefcountbt_block_maxrecs(blocklen, false) / 2]; xfs_btree_compute_maxlevels(minrecs.as_ptr(), XFS_MAX_RGBLOCKS) }
unsafe fn xfs_rtrefcountbt_block_maxrecs(blocklen: u32, leaf: bool) -> u32 { if leaf { blocklen / core::mem::size_of::<xfs_refcount_rec>() as u32 } else { blocklen / (core::mem::size_of::<xfs_refcount_key>() as u32 + core::mem::size_of::<xfs_rtrefcount_ptr_t>() as u32) } }

pub unsafe fn xfs_rtrefcountbt_init_cur_cache() -> i32 { XFS_RTREFCOUNTBT_CUR_CACHE = kmem_cache_create("xfs_rtrefcountbt", xfs_btree_cur_sizeof(xfs_rtrefcountbt_maxlevels_ondisk()), 0, 0, None); if XFS_RTREFCOUNTBT_CUR_CACHE.is_null() { -ENOMEM } else { 0 } }
pub unsafe fn xfs_rtrefcountbt_destroy_cur_cache() { kmem_cache_destroy(XFS_RTREFCOUNTBT_CUR_CACHE); XFS_RTREFCOUNTBT_CUR_CACHE = core::ptr::null_mut(); }

pub unsafe fn xfs_rtrefcountbt_compute_maxlevels(mp: *mut xfs_mount) { if !xfs_has_rtreflink(mp) { (*mp).m_rtrefc_maxlevels = 0; return; } let d = xfs_btree_space_to_height((*mp).m_rtrefc_mnr, (*mp).m_sb.sb_dblocks); let r = xfs_btree_compute_maxlevels((*mp).m_rtrefc_mnr, (*mp).m_sb.sb_rgextents); (*mp).m_rtrefc_maxlevels = core::cmp::min(d, r) + 1; }
pub unsafe fn xfs_rtrefcountbt_calc_size(mp: *mut xfs_mount, len: u64) -> u64 { xfs_btree_calc_size((*mp).m_rtrefc_mnr, len) }
pub unsafe fn xfs_rtrefcountbt_calc_reserves(mp: *mut xfs_mount) -> xfs_filblks_t { if !xfs_has_rtreflink(mp) { 0 } else if (*mp).m_rtrefc_mxr[0] == 0 { 0 } else { xfs_rtrefcountbt_calc_size(mp, (*mp).m_sb.sb_rgextents) } }

/* Allocate a new rt refcount btree cursor. */
pub unsafe fn xfs_rtrefcountbt_init_cursor(tp: *mut xfs_trans, rtg: *mut xfs_rtgroup) -> *mut xfs_btree_cur {
    let ip = rtg_refcount(rtg);
    let mp = rtg_mount(rtg);
    xfs_assert_ilocked(ip, XFS_ILOCK_SHARED | XFS_ILOCK_EXCL);
    let cur = xfs_btree_alloc_cursor(mp, tp, &XFS_RTREFCOUNTBT_OPS, (*mp).m_rtrefc_maxlevels, XFS_RTREFCOUNTBT_CUR_CACHE);
    (*cur).bc_ino.ip = ip;
    (*cur).bc_refc.nr_ops = 0;
    (*cur).bc_refc.shape_changes = 0;
    (*cur).bc_group = xfs_group_hold(rtg_group(rtg));
    (*cur).bc_nlevels = be16_to_cpu((*(*ip).i_df.if_broot).bb_level) + 1;
    (*cur).bc_ino.forksize = xfs_inode_fork_size(ip, XFS_DATA_FORK);
    (*cur).bc_ino.whichfork = XFS_DATA_FORK;
    cur
}

/* Install a new rt reverse mapping btree root. */
pub unsafe fn xfs_rtrefcountbt_commit_staged_btree(cur: *mut xfs_btree_cur, tp: *mut xfs_trans) {
    let ifake = (*cur).bc_ino.ifake;
    let ifp = xfs_ifork_ptr((*cur).bc_ino.ip, XFS_DATA_FORK);
    let flags = XFS_ILOG_CORE | XFS_ILOG_DBROOT;
    ASSERT((*cur).bc_flags & XFS_BTREE_STAGING != 0);
    ASSERT((*(*ifake).if_fork).if_format == XFS_DINODE_FMT_META_BTREE);
    xfs_idestroy_fork(ifp);
    core::ptr::copy_nonoverlapping((*ifake).if_fork, ifp, 1);
    (*(*cur).bc_ino.ip).i_projid = (*(*cur).bc_group).xg_gno;
    xfs_trans_log_inode(tp, (*cur).bc_ino.ip, flags);
    xfs_btree_commit_ifakeroot(cur, tp, XFS_DATA_FORK);
}

pub unsafe fn xfs_rtrefcountbt_from_disk(ip: *mut xfs_inode, dblock: *mut xfs_rtrefcount_root, dblocklen: i32, rblock: *mut xfs_btree_block) {
    let mp = (*ip).i_mount;
    let rblocklen = xfs_rtrefcount_broot_space(mp, dblock);
    xfs_btree_init_block(mp, rblock, &XFS_RTREFCOUNTBT_OPS, 0, 0, I_INO(ip));
    (*rblock).bb_level = (*dblock).bb_level; (*rblock).bb_numrecs = (*dblock).bb_numrecs;
    let n = be16_to_cpu((*dblock).bb_numrecs) as usize;
    if be16_to_cpu((*rblock).bb_level) > 0 {
        let max = xfs_rtrefcountbt_droot_maxrecs(dblocklen as u32, false);
        core::ptr::copy_nonoverlapping(xfs_rtrefcount_droot_key_addr(dblock, 1), xfs_rtrefcount_key_addr(rblock, 1), 2 * core::mem::size_of::<xfs_refcount_key>() * n);
        core::ptr::copy_nonoverlapping(xfs_rtrefcount_droot_ptr_addr(dblock, 1, max), xfs_rtrefcount_broot_ptr_addr(mp, rblock, 1, rblocklen), core::mem::size_of::<u64>() * n);
    } else { core::ptr::copy_nonoverlapping(xfs_rtrefcount_droot_rec_addr(dblock, 1), xfs_rtrefcount_rec_addr(rblock, 1), core::mem::size_of::<xfs_refcount_rec>() * n); }
}

pub unsafe fn xfs_rtrefcountbt_to_disk(mp: *mut xfs_mount, rblock: *mut xfs_btree_block, rblocklen: i32, dblock: *mut xfs_rtrefcount_root, dblocklen: i32) {
    (*dblock).bb_level = (*rblock).bb_level; (*dblock).bb_numrecs = (*rblock).bb_numrecs;
    let n = be16_to_cpu((*rblock).bb_numrecs) as usize;
    if be16_to_cpu((*rblock).bb_level) > 0 {
        let max = xfs_rtrefcountbt_droot_maxrecs(dblocklen as u32, false);
        core::ptr::copy_nonoverlapping(xfs_rtrefcount_key_addr(rblock, 1), xfs_rtrefcount_droot_key_addr(dblock, 1), 2 * core::mem::size_of::<xfs_refcount_key>() * n);
        core::ptr::copy_nonoverlapping(xfs_rtrefcount_broot_ptr_addr(mp, rblock, 1, rblocklen), xfs_rtrefcount_droot_ptr_addr(dblock, 1, max), core::mem::size_of::<u64>() * n);
    } else { core::ptr::copy_nonoverlapping(xfs_rtrefcount_rec_addr(rblock, 1), xfs_rtrefcount_droot_rec_addr(dblock, 1), core::mem::size_of::<xfs_refcount_rec>() * n); }
}

pub unsafe fn xfs_iformat_rtrefcount(ip: *mut xfs_inode, dip: *mut xfs_dinode) -> i32 { let mp = (*ip).i_mount; let d = XFS_DFORK_PTR(dip, XFS_DATA_FORK); let n = be16_to_cpu((*d).bb_numrecs); let level = be16_to_cpu((*d).bb_level); let size = XFS_DFORK_SIZE(dip, mp, XFS_DATA_FORK); if !xfs_has_reflink(mp) || level >= (*mp).m_rtrefc_maxlevels || xfs_rtrefcount_droot_space_calc(level, n) > size { xfs_inode_mark_sick(ip, XFS_SICK_INO_CORE); return -EFSCORRUPTED; } let b = xfs_broot_alloc(xfs_ifork_ptr(ip, XFS_DATA_FORK), xfs_rtrefcount_broot_space_calc(mp, level, n)); if !b.is_null() { xfs_rtrefcountbt_from_disk(ip, d, size, b); } 0 }

pub unsafe fn xfs_iflush_rtrefcount(ip: *mut xfs_inode, dip: *mut xfs_dinode) { let ifp = xfs_ifork_ptr(ip, XFS_DATA_FORK); let d = XFS_DFORK_PTR(dip, XFS_DATA_FORK); ASSERT(!(*ifp).if_broot.is_null()); xfs_rtrefcountbt_to_disk((*ip).i_mount, (*ifp).if_broot, (*ifp).if_broot_bytes, d, XFS_DFORK_SIZE(dip, (*ip).i_mount, XFS_DATA_FORK)); }

pub unsafe fn xfs_rtrefcountbt_create(_rtg: *mut xfs_rtgroup, ip: *mut xfs_inode, tp: *mut xfs_trans, _init: bool) -> i32 { let ifp = xfs_ifork_ptr(ip, XFS_DATA_FORK); let mp = (*ip).i_mount; (*ifp).if_format = XFS_DINODE_FMT_META_BTREE; let b = xfs_broot_realloc(ifp, xfs_rtrefcount_broot_space_calc(mp, 0, 0)); if !b.is_null() { xfs_btree_init_block(mp, b, &XFS_RTREFCOUNTBT_OPS, 0, 0, I_INO(ip)); } xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE | XFS_ILOG_DBROOT); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
