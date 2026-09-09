// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2008 Red Hat, Inc.  All rights reserved.
 */

// Linux and GFS2 dependencies are supplied by the surrounding translation unit.

unsafe fn gfs2_aspace_write_folio(folio: *mut folio, wbc: *mut writeback_control) {
    let mut bh: *mut buffer_head;
    let head: *mut buffer_head;
    let mut nr_underway: i32 = 0;
    let write_flags: blk_opf_t = REQ_META | REQ_PRIO | wbc_to_write_flags(wbc);

    BUG_ON(!folio_test_locked(folio));

    head = folio_buffers(folio);
    bh = head;
    loop {
        if !buffer_mapped(bh) {
            bh = (*bh).b_this_page;
            if bh == head { break; }
            continue;
        }
        /*
         * If it's a fully non-blocking write attempt and we cannot
         * lock the buffer then redirty the page.  Note that this can
         * potentially cause a busy-wait loop from flusher thread and kswapd
         * activity, but those code paths have their own higher-level
         * throttling.
         */
        if (*wbc).sync_mode != WB_SYNC_NONE {
            lock_buffer(bh);
        } else if !trylock_buffer(bh) {
            folio_redirty_for_writepage(wbc, folio);
            bh = (*bh).b_this_page;
            if bh == head { break; }
            continue;
        }
        if test_clear_buffer_dirty(bh) {
            set_buffer_async_write(bh);
        } else {
            unlock_buffer(bh);
        }
        bh = (*bh).b_this_page;
        if bh == head { break; }
    }

    BUG_ON(folio_test_writeback(folio));
    folio_start_writeback(folio);

    loop {
        let next: *mut buffer_head = (*bh).b_this_page;
        if buffer_async_write(bh) {
            bh_submit(bh, REQ_OP_WRITE | write_flags, Some(bh_end_async_write));
            nr_underway += 1;
        }
        bh = next;
        if bh == head { break; }
    }
    folio_unlock(folio);

    if nr_underway == 0 {
        folio_end_writeback(folio);
    }
}

unsafe fn gfs2_aspace_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32 {
    let mut folio: *mut folio = core::ptr::null_mut();
    let mut error: i32 = 0;
    while {
        folio = writeback_iter(mapping, wbc, folio, &mut error);
        !folio.is_null()
    } {
        gfs2_aspace_write_folio(folio, wbc);
    }
    error
}

pub static gfs2_meta_aops: address_space_operations = address_space_operations {
    dirty_folio: Some(block_dirty_folio), invalidate_folio: Some(block_invalidate_folio),
    writepages: Some(gfs2_aspace_writepages), release_folio: Some(gfs2_release_folio),
    migrate_folio: Some(buffer_migrate_folio_norefs),
};

pub static gfs2_rgrp_aops: address_space_operations = address_space_operations {
    dirty_folio: Some(block_dirty_folio), invalidate_folio: Some(block_invalidate_folio),
    writepages: Some(gfs2_aspace_writepages), release_folio: Some(gfs2_release_folio),
    migrate_folio: Some(buffer_migrate_folio_norefs),
};

pub unsafe fn gfs2_getbuf(gl: *mut gfs2_glock, blkno: u64, create: i32) -> *mut buffer_head {
    let mut mapping = gfs2_glock2aspace(gl);
    let sdp = glock_sbd(gl);
    let mut folio: *mut folio;
    let mut bh: *mut buffer_head;
    let shift = PAGE_SHIFT - (*sdp).sd_sb.sb_bsize_shift;
    let index = blkno >> shift;
    let bufnum = blkno - (index << shift);
    if mapping.is_null() { mapping = gfs2_aspace(sdp); }
    if create != 0 {
        folio = __filemap_get_folio(mapping, index, FGP_LOCK | FGP_ACCESSED | FGP_CREAT,
                                    mapping_gfp_mask(mapping) | __GFP_NOFAIL);
        bh = folio_buffers(folio);
        if bh.is_null() { bh = create_empty_buffers(folio, (*sdp).sd_sb.sb_bsize, 0); }
    } else {
        folio = __filemap_get_folio(mapping, index, FGP_LOCK | FGP_ACCESSED, 0);
        if IS_ERR(folio) { return core::ptr::null_mut(); }
        bh = folio_buffers(folio);
    }
    if !bh.is_null() {
        bh = get_nth_bh(bh, bufnum);
        if !buffer_mapped(bh) { map_bh(bh, (*sdp).sd_vfs, blkno); }
    }
    folio_unlock(folio);
    folio_put(folio);
    bh
}

unsafe fn meta_prep_new(bh: *mut buffer_head) {
    let mh = (*bh).b_data as *mut gfs2_meta_header;
    lock_buffer(bh); clear_buffer_dirty(bh); set_buffer_uptodate(bh); unlock_buffer(bh);
    (*mh).mh_magic = cpu_to_be32(GFS2_MAGIC);
}

pub unsafe fn gfs2_meta_new(gl: *mut gfs2_glock, blkno: u64) -> *mut buffer_head {
    let bh = gfs2_getbuf(gl, blkno, CREATE); meta_prep_new(bh); bh
}

unsafe fn gfs2_meta_read_endio(bio: *mut bio) {
    let mut fi: folio_iter;
    bio_for_each_folio_all!(fi, bio, {
        let folio = fi.folio;
        let mut bh = folio_buffers(folio);
        let mut len = fi.length;
        while bh_offset(bh) < fi.offset { bh = (*bh).b_this_page; }
        loop {
            let next = (*bh).b_this_page;
            len -= (*bh).b_size;
            end_buffer_read_sync(bh, (*bio).bi_status == BLK_STS_OK);
            bh = next;
            if bh.is_null() || len == 0 { break; }
        }
    });
    bio_put(bio);
}

/* Submit several consecutive buffer head I/O requests as a single bio I/O request. */
unsafe fn gfs2_submit_bhs(opf: blk_opf_t, mut bhs: *mut *mut buffer_head, mut num: i32) {
    while num > 0 {
        let first = *bhs;
        let bio = bio_alloc((*first).b_bdev, num, opf, GFP_NOIO);
        (*bio).bi_iter.bi_sector = (*first).b_blocknr * ((*first).b_size >> SECTOR_SHIFT);
        while num > 0 {
            let bh = *bhs;
            if !bio_add_folio(bio, (*bh).b_folio, (*bh).b_size, bh_offset(bh)) {
                BUG_ON((*bio).bi_iter.bi_size == 0); break;
            }
            bhs = bhs.add(1); num -= 1;
        }
        (*bio).bi_end_io = Some(gfs2_meta_read_endio);
        submit_bio(bio);
    }
}

pub unsafe fn gfs2_meta_read(gl: *mut gfs2_glock, blkno: u64, mut flags: i32,
                             rahead: i32, bhp: *mut *mut buffer_head) -> i32 {
    let sdp = glock_sbd(gl); let mut bh: *mut buffer_head; let mut bhs = [core::ptr::null_mut(); 2];
    let mut num = 0;
    if gfs2_withdrawn(sdp) { *bhp = core::ptr::null_mut(); return -EIO; }
    *bhp = gfs2_getbuf(gl, blkno, CREATE); bh = *bhp;
    lock_buffer(bh);
    if buffer_uptodate(bh) { unlock_buffer(bh); flags &= !DIO_WAIT; }
    else { get_bh(bh); bhs[num as usize] = bh; num += 1; }
    if rahead != 0 {
        bh = gfs2_getbuf(gl, blkno + 1, CREATE); lock_buffer(bh);
        if buffer_uptodate(bh) { unlock_buffer(bh); } else { bhs[num as usize] = bh; num += 1; }
        brelse(bh);
    }
    gfs2_submit_bhs(REQ_OP_READ | REQ_META | REQ_PRIO, bhs.as_mut_ptr(), num);
    if flags & DIO_WAIT == 0 { return 0; }
    bh = *bhp; wait_on_buffer(bh);
    if !buffer_uptodate(bh) {
        let tr = (*current).journal_info;
        if !tr.is_null() && test_bit(TR_TOUCHED, &(*tr).tr_flags) { gfs2_io_error_bh(sdp, bh); }
        brelse(bh); *bhp = core::ptr::null_mut(); return -EIO;
    }
    0
}

pub unsafe fn gfs2_meta_wait(sdp: *mut gfs2_sbd, bh: *mut buffer_head) -> i32 {
    if gfs2_withdrawn(sdp) { return -EIO; }
    wait_on_buffer(bh);
    if !buffer_uptodate(bh) {
        let tr = (*current).journal_info;
        if !tr.is_null() && test_bit(TR_TOUCHED, &(*tr).tr_flags) { gfs2_io_error_bh(sdp, bh); }
        return -EIO;
    }
    if gfs2_withdrawn(sdp) { return -EIO; } 0
}

unsafe fn gfs2_ail1_wipe(sdp: *mut gfs2_sbd, bstart: u64, blen: u32) {
    let end = bstart + blen as u64;
    spin_lock(&mut (*sdp).sd_log_lock); spin_lock(&mut (*sdp).sd_ail_lock);
    list_for_each_entry_safe!(tr, s, &(*sdp).sd_ail1_list, tr_list, {
        list_for_each_entry_safe!(bd, bs, &(*tr).tr_ail1_list, bd_ail_st_list, {
            let bh = (*bd).bd_bh;
            if (*bh).b_blocknr >= bstart && (*bh).b_blocknr < end { gfs2_remove_from_journal(bh, REMOVE_JDATA); }
        });
    });
    spin_unlock(&mut (*sdp).sd_ail_lock); spin_unlock(&mut (*sdp).sd_log_lock);
}

unsafe fn gfs2_getjdatabuf(ip: *mut gfs2_inode, blkno: u64) -> *mut buffer_head {
    let mapping = (*ip).i_inode.i_mapping; let sdp = GFS2_SB(&(*ip).i_inode);
    let shift = PAGE_SHIFT - (*sdp).sd_sb.sb_bsize_shift; let index = blkno >> shift;
    let bufnum = blkno - (index << shift);
    let folio = __filemap_get_folio(mapping, index, FGP_LOCK | FGP_ACCESSED, 0);
    if IS_ERR(folio) { return core::ptr::null_mut(); }
    let mut bh = folio_buffers(folio); if !bh.is_null() { bh = get_nth_bh(bh, bufnum); }
    folio_unlock(folio); folio_put(folio); bh
}

pub unsafe fn gfs2_journal_wipe(ip: *mut gfs2_inode, mut bstart: u64, mut blen: u32) {
    let sdp = GFS2_SB(&(*ip).i_inode); if (*ip).i_gl.is_null() { return; }
    gfs2_ail1_wipe(sdp, bstart, blen);
    while blen != 0 {
        let mut ty = REMOVE_META; let mut bh = gfs2_getbuf((*ip).i_gl, bstart, NO_CREATE);
        if bh.is_null() && gfs2_is_jdata(ip) { bh = gfs2_getjdatabuf(ip, bstart); ty = REMOVE_JDATA; }
        if !bh.is_null() {
            lock_buffer(bh); spin_lock(&mut (*sdp).sd_log_lock); spin_lock(&mut (*sdp).sd_ail_lock);
            gfs2_remove_from_journal(bh, ty); spin_unlock(&mut (*sdp).sd_ail_lock);
            spin_unlock(&mut (*sdp).sd_log_lock); unlock_buffer(bh); brelse(bh);
        }
        bstart += 1; blen -= 1;
    }
}

pub unsafe fn gfs2_meta_buffer(ip: *mut gfs2_inode, mtype: u32, num: u64,
                               bhp: *mut *mut buffer_head) -> i32 {
    let sdp = GFS2_SB(&(*ip).i_inode); let mut bh: *mut buffer_head; let mut ret = 0;
    let rahead = if num == (*ip).i_no_addr { (*ip).i_rahead } else { 0 };
    ret = gfs2_meta_read((*ip).i_gl, num, DIO_WAIT, rahead, &mut bh);
    if ret == 0 && gfs2_metatype_check(sdp, bh, mtype) { brelse(bh); ret = -EIO; }
    else { *bhp = bh; } ret
}

pub unsafe fn gfs2_meta_ra(gl: *mut gfs2_glock, mut dblock: u64, mut extlen: u32) -> *mut buffer_head {
    let sdp = glock_sbd(gl);
    let mut max_ra = gfs2_tune_get(sdp, gt_max_readahead) >> (*sdp).sd_sb.sb_bsize_shift;
    BUG_ON(extlen == 0); if max_ra < 1 { max_ra = 1; } if extlen > max_ra { extlen = max_ra; }
    let first_bh = gfs2_getbuf(gl, dblock, CREATE);
    if buffer_uptodate(first_bh) { return first_bh; }
    bh_read_nowait(first_bh, REQ_META | REQ_PRIO); dblock += 1; extlen -= 1;
    while extlen != 0 {
        let bh = gfs2_getbuf(gl, dblock, CREATE); bh_readahead(bh, REQ_RAHEAD | REQ_META | REQ_PRIO);
        brelse(bh); dblock += 1; extlen -= 1;
        if !buffer_locked(first_bh) && buffer_uptodate(first_bh) { return first_bh; }
    }
    wait_on_buffer(first_bh); first_bh
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
