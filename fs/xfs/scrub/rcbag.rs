// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// External types, constants, and functions are supplied by the translated
// XFS dependencies.

#[repr(C)]
pub struct rcbag {
    pub mp: *mut xfs_mount,
    pub xfbtree: xfbtree,
    pub nr_items: u64,
}

extern "C" {
    pub fn kzalloc_obj<T>(flags: i32) -> *mut T;
    pub fn kfree(ptr: *mut core::ffi::c_void);
    pub fn rcbagbt_mem_init(mp: *mut xfs_mount, tree: *mut xfbtree, btp: *mut xfs_buftarg) -> i32;
    pub fn xfbtree_destroy(tree: *mut xfbtree);
    pub fn rcbagbt_mem_cursor(mp: *mut xfs_mount, tp: *mut xfs_trans, tree: *mut xfbtree) -> *mut xfs_btree_cur;
    pub fn rcbagbt_lookup_eq(cur: *mut xfs_btree_cur, rmap: *const xfs_rmap_irec, has: *mut i32) -> i32;
    pub fn rcbagbt_get_rec(cur: *mut xfs_btree_cur, rec: *mut rcbag_rec, has: *mut i32) -> i32;
    pub fn rcbagbt_update(cur: *mut xfs_btree_cur, rec: *const rcbag_rec) -> i32;
    pub fn rcbagbt_insert(cur: *mut xfs_btree_cur, rec: *const rcbag_rec, has: *mut i32) -> i32;
    pub fn xfs_btree_del_cursor(cur: *mut xfs_btree_cur, error: i32);
    pub fn xfbtree_trans_commit(tree: *mut xfbtree, tp: *mut xfs_trans) -> i32;
    pub fn xfbtree_trans_cancel(tree: *mut xfbtree, tp: *mut xfs_trans);
    pub fn xfs_btree_goto_left_edge(cur: *mut xfs_btree_cur) -> i32;
    pub fn xfs_btree_increment(cur: *mut xfs_btree_cur, level: i32, has: *mut i32) -> i32;
    pub fn xfs_btree_decrement(cur: *mut xfs_btree_cur, level: i32, has: *mut i32) -> i32;
    pub fn xfs_btree_lookup(cur: *mut xfs_btree_cur, op: i32, has: *mut i32) -> i32;
    pub fn xfs_btree_delete(cur: *mut xfs_btree_cur, has: *mut i32) -> i32;
    pub fn xfs_err(mp: *mut xfs_mount, fmt: *const core::ffi::c_char, ...);
}

#[repr(C)] pub struct xfs_mount;
#[repr(C)] pub struct xfs_buftarg;
#[repr(C)] pub struct xfs_trans;
#[repr(C)] pub struct xfs_btree_cur { pub bc_rec: [u8; 0] }
#[repr(C)] pub struct xfbtree;
#[repr(C)] pub struct xfs_rmap_irec {
    pub rm_startblock: u32,
    pub rm_blockcount: u32,
}
#[repr(C)] pub struct rcbag_rec {
    pub rbg_startblock: u32,
    pub rbg_blockcount: u32,
    pub rbg_refcount: u64,
}

pub const NULLAGBLOCK: u32 = u32::MAX;

pub unsafe fn rcbag_init(mp: *mut xfs_mount, btp: *mut xfs_buftarg, bagp: *mut *mut rcbag) -> i32 {
    let bag = kzalloc_obj::<rcbag>(XCHK_GFP_FLAGS);
    if bag.is_null() { return -ENOMEM; }

    (*bag).nr_items = 0;
    (*bag).mp = mp;

    let error = rcbagbt_mem_init(mp, &mut (*bag).xfbtree, btp);
    if error != 0 {
        kfree(bag.cast());
        return error;
    }

    *bagp = bag;
    0
}

pub unsafe fn rcbag_free(bagp: *mut *mut rcbag) {
    let bag = *bagp;
    xfbtree_destroy(&mut (*bag).xfbtree);
    kfree(bag.cast());
    *bagp = core::ptr::null_mut();
}

/* Track an rmap in the refcount bag. */
pub unsafe fn rcbag_add(bag: *mut rcbag, tp: *mut xfs_trans, rmap: *const xfs_rmap_irec) -> i32 {
    let mut bagrec = core::mem::zeroed::<rcbag_rec>();
    let mp = (*bag).mp;
    let cur = rcbagbt_mem_cursor(mp, tp, &mut (*bag).xfbtree);
    let mut has = 0;
    let mut error = rcbagbt_lookup_eq(cur, rmap, &mut has);
    if error != 0 { xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }

    if has != 0 {
        error = rcbagbt_get_rec(cur, &mut bagrec, &mut has);
        if error != 0 { xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }
        if has == 0 { error = -EFSCORRUPTED; xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }
        bagrec.rbg_refcount = bagrec.rbg_refcount.wrapping_add(1);
        error = rcbagbt_update(cur, &bagrec);
        if error != 0 { xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }
    } else {
        bagrec.rbg_startblock = (*rmap).rm_startblock;
        bagrec.rbg_blockcount = (*rmap).rm_blockcount;
        bagrec.rbg_refcount = 1;
        error = rcbagbt_insert(cur, &bagrec, &mut has);
        if error != 0 { xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }
        if has == 0 { error = -EFSCORRUPTED; xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }
    }
    xfs_btree_del_cursor(cur, 0);
    error = xfbtree_trans_commit(&mut (*bag).xfbtree, tp);
    if error != 0 { return error; }
    (*bag).nr_items = (*bag).nr_items.wrapping_add(1);
    0
}

/* Return the number of records in the bag. */
pub unsafe fn rcbag_count(rcbag: *const rcbag) -> u64 { (*rcbag).nr_items }

unsafe fn rcbag_rec_next_bno(r: *const rcbag_rec) -> u32 { (*r).rbg_startblock.wrapping_add((*r).rbg_blockcount) }

/* Find the next block where the refcount changes, given the next rmap we looked at and the ones we're already tracking. */
pub unsafe fn rcbag_next_edge(bag: *mut rcbag, tp: *mut xfs_trans, next_rmap: *const xfs_rmap_irec, next_valid: bool, next_bnop: *mut u32) -> i32 {
    let mut next_bno = if next_valid { (*next_rmap).rm_startblock } else { NULLAGBLOCK };
    let cur = rcbagbt_mem_cursor((*bag).mp, tp, &mut (*bag).xfbtree);
    let mut has = 0; let mut error = xfs_btree_goto_left_edge(cur);
    if error != 0 { xfs_btree_del_cursor(cur, error); return error; }
    loop {
        error = xfs_btree_increment(cur, 0, &mut has); if error != 0 { xfs_btree_del_cursor(cur, error); return error; }
        if has == 0 { break; }
        let mut rec = core::mem::zeroed(); error = rcbagbt_get_rec(cur, &mut rec, &mut has);
        if error != 0 { xfs_btree_del_cursor(cur, error); return error; }
        if has == 0 { error = -EFSCORRUPTED; xfs_btree_del_cursor(cur, error); return error; }
        next_bno = core::cmp::min(next_bno, rcbag_rec_next_bno(&rec));
    }
    if next_bno == NULLAGBLOCK { error = -EFSCORRUPTED; xfs_btree_del_cursor(cur, error); return error; }
    xfs_btree_del_cursor(cur, 0); *next_bnop = next_bno; 0
}

/* Pop all refcount bag records that end at next_bno */
pub unsafe fn rcbag_remove_ending_at(bag: *mut rcbag, tp: *mut xfs_trans, next_bno: u32) -> i32 {
    let cur = rcbagbt_mem_cursor((*bag).mp, tp, &mut (*bag).xfbtree);
    let mut has = 0; let mut error = xfs_btree_lookup(cur, XFS_LOOKUP_GE, &mut has);
    if error != 0 { xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }
    loop {
        error = xfs_btree_decrement(cur, 0, &mut has); if error != 0 { xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }
        if has == 0 { break; }
        let mut rec = core::mem::zeroed(); error = rcbagbt_get_rec(cur, &mut rec, &mut has);
        if error != 0 { xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }
        if has == 0 { error = -EFSCORRUPTED; xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }
        if rcbag_rec_next_bno(&rec) != next_bno { continue; }
        error = xfs_btree_delete(cur, &mut has); if error != 0 { xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }
        if has == 0 { error = -EFSCORRUPTED; xfs_btree_del_cursor(cur, error); xfbtree_trans_cancel(&mut (*bag).xfbtree, tp); return error; }
        (*bag).nr_items = (*bag).nr_items.wrapping_sub(rec.rbg_refcount);
    }
    xfs_btree_del_cursor(cur, 0); xfbtree_trans_commit(&mut (*bag).xfbtree, tp)
}

/* Dump the rcbag. */
pub unsafe fn rcbag_dump(bag: *mut rcbag, tp: *mut xfs_trans) {
    let cur = rcbagbt_mem_cursor((*bag).mp, tp, &mut (*bag).xfbtree);
    let mut has = 0; let mut error = xfs_btree_goto_left_edge(cur);
    if error != 0 { xfs_btree_del_cursor(cur, error); return; }
    let mut nr: u64 = 0;
    loop {
        error = xfs_btree_increment(cur, 0, &mut has); if error != 0 { break; }
        if has == 0 { break; }
        let mut rec = core::mem::zeroed(); error = rcbagbt_get_rec(cur, &mut rec, &mut has);
        if error != 0 { break; }
        if has == 0 { error = -EFSCORRUPTED; break; }
        xfs_err((*bag).mp, b"[%llu]: bno 0x%x fsbcount 0x%x refcount 0x%llx\0".as_ptr() as *const _, nr, rec.rbg_startblock, rec.rbg_blockcount, rec.rbg_refcount);
        nr = nr.wrapping_add(1);
    }
    xfs_btree_del_cursor(cur, error);
}

// External build-time constants.
extern "C" { static XCHK_GFP_FLAGS: i32; static ENOMEM: i32; static EFSCORRUPTED: i32; static XFS_LOOKUP_GE: i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
