// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * io.c
 *
 * Buffer cache handling
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// Linux and OCFS2 headers supply the types, constants, and functions used here.

#[repr(C)]
pub enum ocfs2_state_bits {
    BH_NeedsValidate = BH_JBDPrivateStart,
}

pub unsafe fn ocfs2_write_block(
    osb: *mut ocfs2_super,
    bh: *mut buffer_head,
    ci: *mut ocfs2_caching_info,
) -> i32 {
    let mut ret: i32 = 0;
    trace_ocfs2_write_block((*bh).b_blocknr as u64, ci);
    BUG_ON((*bh).b_blocknr < OCFS2_SUPER_BLOCK_BLKNO);
    BUG_ON(buffer_jbd(bh));
    if ocfs2_is_hard_readonly(osb) {
        ret = -EROFS;
        mlog_errno(ret);
        return ret;
    }
    ocfs2_metadata_cache_io_lock(ci);
    lock_buffer(bh);
    set_buffer_uptodate(bh);
    clear_buffer_dirty(bh);
    bh_submit(bh, REQ_OP_WRITE, bh_end_write);
    wait_on_buffer(bh);
    if buffer_uptodate(bh) {
        ocfs2_set_buffer_uptodate(ci, bh);
    } else {
        ret = -EIO;
        mlog_errno(ret);
    }
    ocfs2_metadata_cache_io_unlock(ci);
    ret
}

pub unsafe fn ocfs2_read_blocks_sync(
    osb: *mut ocfs2_super, mut block: u64, nr: u32,
    bhs: *mut *mut buffer_head,
) -> i32 {
    let mut status: i32 = 0;
    let mut new_bh: i32 = 0;
    trace_ocfs2_read_blocks_sync(block, nr);
    if nr == 0 { return status; }
    new_bh = if (*bhs).is_null() { 1 } else { 0 };
    for i in 0..nr as usize {
        if (*bhs.add(i)).is_null() {
            *bhs.add(i) = sb_getblk((*osb).sb, block);
            block = block.wrapping_add(1);
            if (*bhs.add(i)).is_null() {
                status = -ENOMEM; mlog_errno(status); break;
            }
        }
        let bh = *bhs.add(i);
        if buffer_jbd(bh) { trace_ocfs2_read_blocks_sync_jbd((*bh).b_blocknr as u64); continue; }
        if buffer_dirty(bh) { mlog(ML_ERROR, "trying to sync read a dirty buffer! (blocknr = %llu), skipping\n", (*bh).b_blocknr as u64); continue; }
        lock_buffer(bh);
        if buffer_jbd(bh) {
            // CATCH_BH_JBD_RACES: the diagnostic BUG branch is build-time conditional.
            unlock_buffer(bh); continue;
        }
        bh_submit(bh, REQ_OP_READ, bh_end_read);
    }
    let mut i = nr as isize;
    while i > 0 {
        i -= 1;
        let bh = *bhs.offset(i);
        if unlikely(status != 0) {
            if new_bh != 0 && !bh.is_null() {
                if !buffer_jbd(bh) { wait_on_buffer(bh); }
                put_bh(bh); *bhs.offset(i) = core::ptr::null_mut();
            } else if !bh.is_null() && buffer_uptodate(bh) { clear_buffer_uptodate(bh); }
            continue;
        }
        if !buffer_jbd(bh) { wait_on_buffer(bh); }
        if !buffer_uptodate(bh) { status = -EIO; continue; }
    }
    status
}

pub unsafe fn ocfs2_read_blocks(
    ci: *mut ocfs2_caching_info, mut block: u64, nr: i32,
    bhs: *mut *mut buffer_head, flags: i32,
    validate: Option<unsafe extern "C" fn(*mut super_block, *mut buffer_head) -> i32>,
) -> i32 {
    let mut status: i32 = 0;
    let mut ignore_cache: i32 = 0;
    let sb = ocfs2_metadata_cache_get_super(ci);
    let mut new_bh: i32 = 0;
    trace_ocfs2_read_blocks_begin(ci, block, nr, flags);
    BUG_ON(ci.is_null());
    BUG_ON((flags & OCFS2_BH_READAHEAD) != 0 && (flags & OCFS2_BH_IGNORE_CACHE) != 0);
    if bhs.is_null() { status = -EINVAL; mlog_errno(status); return status; }
    if nr < 0 { mlog(ML_ERROR, "asked to read %d blocks!\n", nr); status = -EINVAL; mlog_errno(status); return status; }
    if nr == 0 { return 0; }
    new_bh = if (*bhs).is_null() { 1 } else { 0 };
    ocfs2_metadata_cache_io_lock(ci);
    for i in 0..nr as usize {
        if (*bhs.add(i)).is_null() {
            *bhs.add(i) = sb_getblk(sb, block); block = block.wrapping_add(1);
            if (*bhs.add(i)).is_null() { status = -ENOMEM; mlog_errno(status); break; }
        }
        let bh = *bhs.add(i);
        ignore_cache = flags & OCFS2_BH_IGNORE_CACHE;
        if ignore_cache == 0 && !ocfs2_buffer_uptodate(ci, bh) { trace_ocfs2_read_blocks_from_disk((*bh).b_blocknr as u64, ocfs2_metadata_cache_owner(ci)); ignore_cache = 1; }
        trace_ocfs2_read_blocks_bh((*bh).b_blocknr as u64, ignore_cache, buffer_jbd(bh), buffer_dirty(bh));
        if buffer_jbd(bh) { continue; }
        if ignore_cache != 0 {
            if buffer_dirty(bh) { continue; }
            if (flags & OCFS2_BH_READAHEAD) != 0 && ocfs2_buffer_read_ahead(ci, bh) { continue; }
            lock_buffer(bh);
            if buffer_jbd(bh) { unlock_buffer(bh); continue; }
            if (flags & (OCFS2_BH_IGNORE_CACHE | OCFS2_BH_READAHEAD)) == 0 && ocfs2_buffer_uptodate(ci, bh) { unlock_buffer(bh); continue; }
            if validate.is_some() { set_buffer_needs_validate(bh); }
            bh_submit(bh, REQ_OP_READ, bh_end_read);
        }
    }
    let mut i = nr as isize;
    while i > 0 {
        i -= 1; let bh = *bhs.offset(i);
        if (flags & OCFS2_BH_READAHEAD) == 0 {
            if unlikely(status != 0) { if new_bh != 0 && !bh.is_null() { if !buffer_jbd(bh) { wait_on_buffer(bh); } put_bh(bh); *bhs.offset(i) = core::ptr::null_mut(); } continue; }
            if !buffer_jbd(bh) { wait_on_buffer(bh); }
            if !buffer_uptodate(bh) { status = -EIO; clear_buffer_needs_validate(bh); continue; }
            if buffer_needs_validate(bh) { BUG_ON(buffer_jbd(bh)); clear_buffer_needs_validate(bh); status = validate.unwrap()(sb, bh); if status != 0 { if buffer_uptodate(bh) { clear_buffer_uptodate(bh); } continue; } }
        }
        if !bh.is_null() { ocfs2_set_buffer_uptodate(ci, bh); }
    }
    ocfs2_metadata_cache_io_unlock(ci);
    trace_ocfs2_read_blocks_end(block, nr, flags, ignore_cache);
    status
}

unsafe fn ocfs2_check_super_or_backup(sb: *mut super_block, blkno: sector_t) {
    if blkno == OCFS2_SUPER_BLOCK_BLKNO { return; }
    for i in 0..OCFS2_MAX_BACKUP_SUPERBLOCKS { if ocfs2_backup_super_blkno(sb, i) == blkno { return; } }
    BUG();
}

pub unsafe fn ocfs2_write_super_or_backup(osb: *mut ocfs2_super, bh: *mut buffer_head) -> i32 {
    let mut ret = 0;
    let di = (*bh).b_data as *mut ocfs2_dinode;
    BUG_ON(buffer_jbd(bh)); ocfs2_check_super_or_backup((*osb).sb, (*bh).b_blocknr);
    if unlikely(ocfs2_emergency_state(osb)) { ret = -EROFS; mlog_errno(ret); return ret; }
    lock_buffer(bh); set_buffer_uptodate(bh); clear_buffer_dirty(bh);
    ocfs2_compute_meta_ecc((*osb).sb, (*bh).b_data, &mut (*di).i_check);
    bh_submit(bh, REQ_OP_WRITE, bh_end_write); wait_on_buffer(bh);
    if !buffer_uptodate(bh) { ret = -EIO; mlog_errno(ret); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
