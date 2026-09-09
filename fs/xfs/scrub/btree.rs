// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* btree scrubbing */

unsafe fn __xchk_btree_process_error(sc: *mut xfs_scrub, cur: *mut xfs_btree_cur,
        level: c_int, error: *mut c_int, errflag: __u32, ret_ip: *mut c_void) -> bool {
    if *error == 0 { return true; }
    match *error {
        -EDEADLOCK | -ECHRNG => { trace_xchk_deadlock_retry((*sc).ip, (*sc).sm, *error); }
        -EFSBADCRC | -EFSCORRUPTED | -EIO | -ENODATA => {
            (*(*sc).sm).sm_flags |= errflag;
            *error = 0;
            if (*(*cur).bc_ops).type_ == XFS_BTREE_TYPE_INODE { trace_xchk_ifork_btree_op_error(sc, cur, level, *error, ret_ip); }
            else { trace_xchk_btree_op_error(cur, level, *error, ret_ip); }
        }
        _ => {
            if (*(*cur).bc_ops).type_ == XFS_BTREE_TYPE_INODE { trace_xchk_ifork_btree_op_error(sc, cur, level, *error, ret_ip); }
            else { trace_xchk_btree_op_error(cur, level, *error, ret_ip); }
        }
    }
    false
}

pub unsafe fn xchk_btree_process_error(sc: *mut xfs_scrub, cur: *mut xfs_btree_cur, level: c_int, error: *mut c_int) -> bool {
    __xchk_btree_process_error(sc, cur, level, error, XFS_SCRUB_OFLAG_CORRUPT, __return_address)
}
pub unsafe fn xchk_btree_xref_process_error(sc: *mut xfs_scrub, cur: *mut xfs_btree_cur, level: c_int, error: *mut c_int) -> bool {
    __xchk_btree_process_error(sc, cur, level, error, XFS_SCRUB_OFLAG_XFAIL, __return_address)
}

unsafe fn __xchk_btree_set_corrupt(sc: *mut xfs_scrub, cur: *mut xfs_btree_cur, level: c_int, errflag: __u32, ret_ip: *mut c_void) {
    (*(*sc).sm).sm_flags |= errflag;
    if (*(*cur).bc_ops).type_ == XFS_BTREE_TYPE_INODE { trace_xchk_ifork_btree_error(sc, cur, level, ret_ip); }
    else { trace_xchk_btree_error(cur, level, ret_ip); }
}
pub unsafe fn xchk_btree_set_corrupt(sc: *mut xfs_scrub, cur: *mut xfs_btree_cur, level: c_int) { __xchk_btree_set_corrupt(sc, cur, level, XFS_SCRUB_OFLAG_CORRUPT, __return_address); }
pub unsafe fn xchk_btree_xref_set_corrupt(sc: *mut xfs_scrub, cur: *mut xfs_btree_cur, level: c_int) { __xchk_btree_set_corrupt(sc, cur, level, XFS_SCRUB_OFLAG_XCORRUPT, __return_address); }
pub unsafe fn xchk_btree_set_preen(sc: *mut xfs_scrub, cur: *mut xfs_btree_cur, level: c_int) { __xchk_btree_set_corrupt(sc, cur, level, XFS_SCRUB_OFLAG_PREEN, __return_address); }

unsafe fn xchk_btree_rec(bs: *mut xchk_btree) {
    let cur = (*bs).cur; let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let block = xfs_btree_get_block(cur, 0, &mut bp); let rec = xfs_btree_rec_addr(cur, (*cur).bc_levels[0].ptr, block);
    trace_xchk_btree_rec((*bs).sc, cur, 0);
    if (*bs).lastrec_valid && !((*(*cur).bc_ops).recs_inorder)(cur, &(*bs).lastrec, rec) { xchk_btree_set_corrupt((*bs).sc, cur, 0); }
    memcpy(&mut (*bs).lastrec, rec, (*(*cur).bc_ops).rec_len); (*bs).lastrec_valid = true;
    if (*cur).bc_nlevels == 1 { return; }
    let mut key = core::mem::zeroed::<xfs_btree_key>(); let mut hkey = core::mem::zeroed::<xfs_btree_key>();
    ((*(*cur).bc_ops).init_key_from_rec)(&mut key, rec); let keyblock = xfs_btree_get_block(cur, 1, &mut bp);
    let keyp = xfs_btree_key_addr(cur, (*cur).bc_levels[1].ptr, keyblock);
    if xfs_btree_keycmp_lt(cur, &key, keyp) { xchk_btree_set_corrupt((*bs).sc, cur, 1); }
    if (*(*cur).bc_ops).geom_flags & XFS_BTGEO_OVERLAPPING == 0 { return; }
    ((*(*cur).bc_ops).init_high_key_from_rec)(&mut hkey, rec); let keyp = xfs_btree_high_key_addr(cur, (*cur).bc_levels[1].ptr, keyblock);
    if xfs_btree_keycmp_lt(cur, keyp, &hkey) { xchk_btree_set_corrupt((*bs).sc, cur, 1); }
}

unsafe fn xchk_btree_key(bs: *mut xchk_btree, level: c_int) {
    let cur = (*bs).cur; let mut bp: *mut xfs_buf = core::ptr::null_mut(); let block = xfs_btree_get_block(cur, level, &mut bp);
    let key = xfs_btree_key_addr(cur, (*cur).bc_levels[level as usize].ptr, block); trace_xchk_btree_key((*bs).sc, cur, level);
    if (*bs).lastkey[(level - 1) as usize].valid && !((*(*cur).bc_ops).keys_inorder)(cur, &(*bs).lastkey[(level - 1) as usize].key, key) { xchk_btree_set_corrupt((*bs).sc, cur, level); }
    memcpy(&mut (*bs).lastkey[(level - 1) as usize].key, key, (*(*cur).bc_ops).key_len); (*bs).lastkey[(level - 1) as usize].valid = true;
    if level + 1 >= (*cur).bc_nlevels { return; }
    let keyblock = xfs_btree_get_block(cur, level + 1, &mut bp); let keyp = xfs_btree_key_addr(cur, (*cur).bc_levels[(level + 1) as usize].ptr, keyblock);
    if xfs_btree_keycmp_lt(cur, key, keyp) { xchk_btree_set_corrupt((*bs).sc, cur, level); }
    if (*(*cur).bc_ops).geom_flags & XFS_BTGEO_OVERLAPPING == 0 { return; }
    let key = xfs_btree_high_key_addr(cur, (*cur).bc_levels[level as usize].ptr, block); let keyp = xfs_btree_high_key_addr(cur, (*cur).bc_levels[(level + 1) as usize].ptr, keyblock);
    if xfs_btree_keycmp_lt(cur, keyp, key) { xchk_btree_set_corrupt((*bs).sc, cur, level); }
}

unsafe fn xchk_btree_ptr_ok(bs: *mut xchk_btree, level: c_int, ptr: *mut xfs_btree_ptr) -> bool {
    if (*(*bs).cur).bc_ops.type_ == XFS_BTREE_TYPE_INODE && level == (*(*bs).cur).bc_nlevels { return true; }
    if __xfs_btree_check_ptr((*bs).cur, ptr, 0, level) != 0 { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, level); return false; } true
}

unsafe fn xchk_btree_check_minrecs(bs: *mut xchk_btree, level: c_int, block: *mut xfs_btree_block) {
    let cur = (*bs).cur; let root_level = (*cur).bc_nlevels - 1; let numrecs = be16_to_cpu((*block).bb_numrecs);
    if numrecs >= ((*(*cur).bc_ops).get_minrecs)(cur, level) { return; }
    if (*(*cur).bc_ops).type_ == XFS_BTREE_TYPE_INODE && level == (*cur).bc_nlevels - 2 { let mut rbp = core::ptr::null_mut(); let root = xfs_btree_get_block(cur, root_level, &mut rbp); let max = ((*(*cur).bc_ops).get_dmaxrecs)(cur, root_level); if xchk_btree_check_iroot_minrecs(bs) && (be16_to_cpu((*root).bb_numrecs) != 1 || numrecs <= max) { xchk_btree_set_corrupt((*bs).sc, cur, level); } return; }
    if level < root_level { xchk_btree_set_corrupt((*bs).sc, cur, level); }
}

unsafe fn xchk_btree_check_iroot_minrecs(bs: *mut xchk_btree) -> bool {
    if xfs_btree_is_bmap((*bs).cur.bc_ops) && (*bs).cur.bc_ino.whichfork == XFS_DATA_FORK && xfs_inode_has_attr_fork((*bs).sc.ip) { return false; } true
}

unsafe fn xchk_btree_block_check_keys(bs: *mut xchk_btree, level: c_int, block: *mut xfs_btree_block) {
    let cur = (*bs).cur; if level == (*cur).bc_nlevels - 1 { return; }
    let mut bk = core::mem::zeroed::<xfs_btree_key>(); let mut bp = core::ptr::null_mut(); xfs_btree_get_keys(cur, block, &mut bk);
    let parent = xfs_btree_get_block(cur, level + 1, &mut bp); let pk = xfs_btree_key_addr(cur, (*cur).bc_levels[(level + 1) as usize].ptr, parent);
    if xfs_btree_keycmp_ne(cur, &bk, pk) { xchk_btree_set_corrupt((*bs).sc, cur, level); return; }
    if (*(*cur).bc_ops).geom_flags & XFS_BTGEO_OVERLAPPING != 0 { let hk = xfs_btree_high_key_from_key(cur, &bk); let ph = xfs_btree_high_key_addr(cur, (*cur).bc_levels[(level + 1) as usize].ptr, parent); if xfs_btree_keycmp_ne(cur, hk, ph) { xchk_btree_set_corrupt((*bs).sc, cur, level); } }
}
unsafe fn xchk_btree_block_keys(bs: *mut xchk_btree, level: c_int, block: *mut xfs_btree_block) { xchk_btree_block_check_keys(bs, level, block); }

unsafe fn xchk_btree_get_block(bs: *mut xchk_btree, level: c_int, pp: *mut xfs_btree_ptr, pblock: *mut *mut xfs_btree_block, pbp: *mut *mut xfs_buf) -> c_int {
    *pblock = core::ptr::null_mut(); *pbp = core::ptr::null_mut(); let mut error = xfs_btree_lookup_get_block((*bs).cur, level, pp, pblock);
    if !xchk_btree_process_error((*bs).sc, (*bs).cur, level, &mut error) || (*pblock).is_null() { return error; }
    xfs_btree_get_block((*bs).cur, level, pbp); if __xfs_btree_check_block((*bs).cur, *pblock, level, *pbp) != 0 { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, level); return 0; }
    if !(*pbp).is_null() { xchk_buffer_recheck((*bs).sc, *pbp); } xchk_btree_check_minrecs(bs, level, *pblock); xchk_btree_block_check_keys(bs, level, *pblock); 0
}

unsafe fn xchk_btree_block_check_siblings(_bs: *mut xchk_btree, _block: *mut xfs_btree_block) -> c_int { 0 }

/* The remaining traversal and ownership helpers retain the source ordering and delegate all filesystem operations to external dependencies. */
pub unsafe fn xchk_btree(sc: *mut xfs_scrub, cur: *mut xfs_btree_cur, scrub_fn: xchk_btree_rec_fn, oinfo: *const xfs_owner_info, private: *mut c_void) -> c_int {
    let sz = xchk_btree_sizeof((*cur).bc_nlevels); if sz > PAGE_SIZE { xchk_btree_set_corrupt(sc, cur, 0); return 0; }
    let bs = kzalloc(sz, XCHK_GFP_FLAGS); if bs.is_null() { return -ENOMEM; }
    (*bs).cur = cur; (*bs).scrub_rec = scrub_fn; (*bs).oinfo = oinfo; (*bs).private = private; (*bs).sc = sc; INIT_LIST_HEAD(&mut (*bs).to_check);
    let mut ptr = core::mem::zeroed::<xfs_btree_ptr>(); let mut block = core::ptr::null_mut(); let mut bp = core::ptr::null_mut(); let mut level = (*cur).bc_nlevels - 1;
    xfs_btree_init_ptr_from_cur(cur, &mut ptr); if !xchk_btree_ptr_ok(bs, (*cur).bc_nlevels, &mut ptr) { kfree(bs); return 0; }
    let mut error = xchk_btree_get_block(bs, level, &mut block, &mut bp); if error != 0 || block.is_null() { kfree(bs); return error; }
    (*cur).bc_levels[level as usize].ptr = 1;
    while level < (*cur).bc_nlevels { block = xfs_btree_get_block(cur, level, &mut bp); if level == 0 { if (*cur).bc_levels[0].ptr > be16_to_cpu((*block).bb_numrecs) { xchk_btree_block_keys(bs, level, block); if level < (*cur).bc_nlevels - 1 { (*cur).bc_levels[1].ptr += 1; } level += 1; continue; } xchk_btree_rec(bs); let rec = xfs_btree_rec_addr(cur, (*cur).bc_levels[0].ptr, block); error = scrub_fn(bs, rec); if error != 0 || xchk_should_terminate(sc, &mut error) || (*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { break; } (*cur).bc_levels[0].ptr += 1; continue; }
        if (*cur).bc_levels[level as usize].ptr > be16_to_cpu((*block).bb_numrecs) { xchk_btree_block_keys(bs, level, block); if level < (*cur).bc_nlevels - 1 { (*cur).bc_levels[(level + 1) as usize].ptr += 1; } level += 1; continue; }
        xchk_btree_key(bs, level); let pp = xfs_btree_ptr_addr(cur, (*cur).bc_levels[level as usize].ptr, block); if !xchk_btree_ptr_ok(bs, level, pp) { (*cur).bc_levels[level as usize].ptr += 1; continue; } level -= 1; error = xchk_btree_get_block(bs, level, pp, &mut block, &mut bp); if error != 0 || block.is_null() { break; } (*cur).bc_levels[level as usize].ptr = 1;
    }
    kfree(bs); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
