// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext4/page-io.c
 *
 * This contains the new page_io functions for ext4
 *
 * Written by Theodore Ts'o, 2010.
 */

// Kernel includes and local headers from the C source are external Rust dependencies.

static mut io_end_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut io_end_vec_cachep: *mut kmem_cache = core::ptr::null_mut();

pub unsafe fn ext4_init_pageio() -> i32 {
    io_end_cachep = KMEM_CACHE(ext4_io_end, SLAB_RECLAIM_ACCOUNT);
    if io_end_cachep.is_null() { return -ENOMEM; }
    io_end_vec_cachep = KMEM_CACHE(ext4_io_end_vec, 0);
    if io_end_vec_cachep.is_null() {
        kmem_cache_destroy(io_end_cachep);
        return -ENOMEM;
    }
    0
}

pub unsafe fn ext4_exit_pageio() {
    kmem_cache_destroy(io_end_cachep);
    kmem_cache_destroy(io_end_vec_cachep);
}

pub unsafe fn ext4_alloc_io_end_vec(io_end: *mut ext4_io_end_t) -> *mut ext4_io_end_vec {
    let io_end_vec = kmem_cache_zalloc(io_end_vec_cachep, GFP_NOFS);
    if io_end_vec.is_null() { return ERR_PTR(-ENOMEM); }
    INIT_LIST_HEAD(&mut (*io_end_vec).list);
    list_add_tail(&mut (*io_end_vec).list, &mut (*io_end).list_vec);
    io_end_vec
}

unsafe fn ext4_free_io_end_vec(io_end: *mut ext4_io_end_t) {
    if list_empty(&(*io_end).list_vec) { return; }
    let mut pos = (*io_end).list_vec.next;
    while pos != &mut (*io_end).list_vec {
        let next = (*pos).next;
        let io_end_vec = container_of(pos, ext4_io_end_vec, list);
        list_del(pos);
        kmem_cache_free(io_end_vec_cachep, io_end_vec);
        pos = next;
    }
}

pub unsafe fn ext4_last_io_end_vec(io_end: *mut ext4_io_end_t) -> *mut ext4_io_end_vec {
    BUG_ON(list_empty(&(*io_end).list_vec));
    list_last_entry(&mut (*io_end).list_vec, ext4_io_end_vec, list)
}

/* Print a buffer I/O error compatible with fs/buffer.c. */
unsafe fn buffer_io_error(bh: *mut buffer_head) {
    printk_ratelimited!(KERN_ERR, "Buffer I/O error on device %pg, logical block %llu\n",
        (*bh).b_bdev, (*bh).b_blocknr as u64);
}

unsafe fn ext4_finish_bio(bio: *mut bio) {
    let mut fi: folio_iter = core::mem::zeroed();
    bio_for_each_folio_all!(fi, bio, {
        let folio = fi.folio;
        let mut bh = folio_buffers(folio);
        let head = bh;
        let bio_start = fi.offset;
        let bio_end = bio_start + fi.length;
        let mut under_io = 0u32;
        let mut flags = 0ul;
        if (*bio).bi_status != 0 {
            mapping_set_error((*folio).mapping, blk_status_to_errno((*bio).bi_status));
        }
        spin_lock_irqsave(&mut (*head).b_uptodate_lock, &mut flags);
        loop {
            if bh_offset(bh) < bio_start || bh_offset(bh) + (*bh).b_size > bio_end {
                if buffer_async_write(bh) { under_io += 1; }
            } else {
                clear_buffer_async_write(bh);
                if (*bio).bi_status != 0 {
                    set_buffer_write_io_error(bh);
                    buffer_io_error(bh);
                }
            }
            bh = (*bh).b_this_page;
            if bh == head { break; }
        }
        spin_unlock_irqrestore(&mut (*head).b_uptodate_lock, flags);
        if under_io == 0 { folio_end_writeback(folio); }
    });
}

unsafe fn ext4_release_io_end(io_end: *mut ext4_io_end_t) {
    BUG_ON(!list_empty(&(*io_end).list));
    BUG_ON((*io_end).flag & EXT4_IO_END_UNWRITTEN != 0);
    WARN_ON(!(*io_end).handle.is_null());
    let mut bio = (*io_end).bio;
    while !bio.is_null() {
        let next_bio = (*bio).bi_private as *mut bio;
        ext4_finish_bio(bio);
        bio_put(bio);
        bio = next_bio;
    }
    ext4_free_io_end_vec(io_end);
    kmem_cache_free(io_end_cachep, io_end);
}

unsafe fn ext4_end_io_end(io_end: *mut ext4_io_end_t) -> i32 {
    let inode = (*io_end).inode;
    let handle = (*io_end).handle;
    let sb = (*inode).i_sb;
    let mut ret = 0;
    ext4_debug!("ext4_end_io_nolock: io_end 0x%p from inode %llu,list->next 0x%p,list->prev 0x%p\n", io_end, (*inode).i_ino, (*io_end).list.next, (*io_end).list.prev);
    (*io_end).handle = core::ptr::null_mut();
    if ((*io_end).flag & EXT4_IO_END_FAILED) != 0 {
        ret = -EIO;
        if !handle.is_null() { jbd2_journal_free_reserved(handle); }
        if test_opt(sb, DATA_ERR_ABORT) { jbd2_journal_abort((*EXT4_SB(sb)).s_journal, ret); }
    } else { ret = ext4_convert_unwritten_io_end_vec(handle, io_end); }
    if ret < 0 && !ext4_emergency_state(sb) && ((*io_end).flag & EXT4_IO_END_UNWRITTEN) != 0 {
        ext4_msg!(sb, KERN_EMERG, "failed to convert unwritten extents to written extents -- potential data loss!  (inode %llu, error %d)", (*inode).i_ino, ret);
    }
    ext4_clear_io_unwritten_flag(io_end);
    ext4_release_io_end(io_end);
    ret
}

unsafe fn dump_completed_IO(inode: *mut inode, head: *mut list_head) {
    // EXT4FS_DEBUG conditional code retained in the source; debug-only traversal is external.
    if list_empty(&*head) { return; }
    ext4_debug!("Dump inode %llu completed io list\n", (*inode).i_ino);
}

unsafe fn ext4_io_end_defer_completion(io_end: *mut ext4_io_end_t) -> bool {
    if ((*io_end).flag & EXT4_IO_END_UNWRITTEN) != 0 && !list_empty(&(*io_end).list_vec) { return true; }
    if test_opt((*(*io_end).inode).i_sb, DATA_ERR_ABORT) && ((*io_end).flag & EXT4_IO_END_FAILED) != 0 && !ext4_emergency_state((*(*io_end).inode).i_sb) { return true; }
    false
}

unsafe fn ext4_add_complete_io(io_end: *mut ext4_io_end_t) {
    let ei = EXT4_I((*io_end).inode);
    let sbi = EXT4_SB((*(*io_end).inode).i_sb);
    let mut flags = 0ul;
    WARN_ON((*io_end).flag & EXT4_IO_END_DEFER_COMPLETION == 0);
    WARN_ON((*io_end).flag & EXT4_IO_END_UNWRITTEN != 0 && (*io_end).handle.is_null() && !(*sbi).s_journal.is_null());
    WARN_ON((*io_end).bio.is_null());
    spin_lock_irqsave(&mut (*ei).i_completed_io_lock, &mut flags);
    if list_empty(&(*ei).i_rsv_conversion_list) { queue_work((*sbi).rsv_conversion_wq, &mut (*ei).i_rsv_conversion_work); }
    list_add_tail(&mut (*io_end).list, &mut (*ei).i_rsv_conversion_list);
    spin_unlock_irqrestore(&mut (*ei).i_completed_io_lock, flags);
}

unsafe fn ext4_do_flush_completed_IO(inode: *mut inode, head: *mut list_head) -> i32 {
    let ei = EXT4_I(inode); let mut unwritten: list_head = core::mem::zeroed(); let mut flags = 0ul; let mut ret = 0;
    spin_lock_irqsave(&mut (*ei).i_completed_io_lock, &mut flags); dump_completed_IO(inode, head); list_replace_init(head, &mut unwritten); spin_unlock_irqrestore(&mut (*ei).i_completed_io_lock, flags);
    while !list_empty(&unwritten) { let io_end = list_entry(unwritten.next, ext4_io_end_t, list); BUG_ON((*io_end).flag & EXT4_IO_END_DEFER_COMPLETION == 0); list_del_init(&mut (*io_end).list); let err = ext4_end_io_end(io_end); if ret == 0 && err != 0 { ret = err; } }
    ret
}

pub unsafe fn ext4_end_io_rsv_work(work: *mut work_struct) { let ei = container_of(work, ext4_inode_info, i_rsv_conversion_work); ext4_do_flush_completed_IO(&mut (*ei).vfs_inode, &mut (*ei).i_rsv_conversion_list); }

pub unsafe fn ext4_init_io_end(inode: *mut inode, flags: gfp_t) -> *mut ext4_io_end_t {
    let io_end = kmem_cache_zalloc(io_end_cachep, flags);
    if !io_end.is_null() { (*io_end).inode = inode; INIT_LIST_HEAD(&mut (*io_end).list); INIT_LIST_HEAD(&mut (*io_end).list_vec); refcount_set(&mut (*io_end).count, 1); }
    io_end
}

pub unsafe fn ext4_put_io_end_defer(io_end: *mut ext4_io_end_t) { if refcount_dec_and_test(&mut (*io_end).count) { if ext4_io_end_defer_completion(io_end) { ext4_add_complete_io(io_end); } else { ext4_release_io_end(io_end); } } }
pub unsafe fn ext4_put_io_end(io_end: *mut ext4_io_end_t) -> i32 { if refcount_dec_and_test(&mut (*io_end).count) { if ext4_io_end_defer_completion(io_end) { return ext4_end_io_end(io_end); } ext4_release_io_end(io_end); } 0 }
pub unsafe fn ext4_get_io_end(io_end: *mut ext4_io_end_t) -> *mut ext4_io_end_t { refcount_inc(&mut (*io_end).count); io_end }

unsafe fn ext4_end_bio(bio: *mut bio) {
    let io_end = (*bio).bi_private as *mut ext4_io_end_t; let bi_sector = (*bio).bi_iter.bi_sector;
    if WARN_ONCE(io_end.is_null(), "io_end is NULL: %pg: sector %Lu len %u err %d\n") { ext4_finish_bio(bio); bio_put(bio); return; }
    (*bio).bi_end_io = None;
    if (*bio).bi_status != 0 { let inode = (*io_end).inode; ext4_warning!((*inode).i_sb, "I/O error %d writing to inode %llu starting block %llu)", (*bio).bi_status, (*inode).i_ino, bi_sector >> ((*inode).i_blkbits - 9)); (*io_end).flag |= EXT4_IO_END_FAILED; mapping_set_error((*inode).i_mapping, blk_status_to_errno((*bio).bi_status)); }
    if ext4_io_end_defer_completion(io_end) { (*bio).bi_private = xchg(&mut (*io_end).bio, bio); ext4_put_io_end_defer(io_end); } else { ext4_put_io_end_defer(io_end); ext4_finish_bio(bio); bio_put(bio); }
}

pub unsafe fn ext4_io_submit(io: *mut ext4_io_submit) { let bio = (*io).io_bio; if !bio.is_null() { if (*(*io).io_wbc).sync_mode == WB_SYNC_ALL { (*bio).bi_opf |= REQ_SYNC; } blk_crypto_submit_bio(bio); } (*io).io_bio = core::ptr::null_mut(); }
pub unsafe fn ext4_io_submit_init(io: *mut ext4_io_submit, wbc: *mut writeback_control) { (*io).io_wbc = wbc; (*io).io_bio = core::ptr::null_mut(); (*io).io_end = core::ptr::null_mut(); }

unsafe fn io_submit_init_bio(io: *mut ext4_io_submit, inode: *mut inode, folio: *mut folio, bh: *mut buffer_head) {
    let bio = bio_alloc((*bh).b_bdev, BIO_MAX_VECS, REQ_OP_WRITE, GFP_NOIO); fscrypt_set_bio_crypt_ctx(bio, inode, folio_pos(folio) + bh_offset(bh), GFP_NOIO); (*bio).bi_iter.bi_sector = (*bh).b_blocknr * ((*bh).b_size >> 9); (*bio).bi_end_io = Some(ext4_end_bio); (*bio).bi_private = ext4_get_io_end((*io).io_end) as *mut core::ffi::c_void; (*bio).bi_write_hint = (*inode).i_write_hint; (*io).io_bio = bio; (*io).io_next_block = (*bh).b_blocknr; wbc_init_bio((*io).io_wbc, bio);
}
unsafe fn io_submit_need_new_bio(io: *mut ext4_io_submit, inode: *mut inode, folio: *mut folio, bh: *mut buffer_head) -> bool { (*bh).b_blocknr != (*io).io_next_block || !fscrypt_mergeable_bio((*io).io_bio, inode, folio_pos(folio) + bh_offset(bh)) }
unsafe fn io_submit_add_bh(io: *mut ext4_io_submit, inode: *mut inode, folio: *mut folio, bh: *mut buffer_head) { if !(*io).io_bio.is_null() && io_submit_need_new_bio(io, inode, folio, bh) { ext4_io_submit(io); } if (*io).io_bio.is_null() { io_submit_init_bio(io, inode, folio, bh); } if bio_add_folio((*io).io_bio, folio, (*bh).b_size, bh_offset(bh)) == 0 { ext4_io_submit(io); io_submit_init_bio(io, inode, folio, bh); let _ = bio_add_folio((*io).io_bio, folio, (*bh).b_size, bh_offset(bh)); } wbc_account_cgroup_owner((*io).io_wbc, folio, (*bh).b_size); (*io).io_next_block += 1; }

pub unsafe fn ext4_bio_write_folio(io: *mut ext4_io_submit, folio: *mut folio, len: usize) {
    let inode = (*(*folio).mapping).host; let mut bh = folio_buffers(folio); let head = bh; let mut nr_to_submit = 0; let wbc = (*io).io_wbc; let mut keep_towrite = false;
    BUG_ON(!folio_test_locked(folio)); BUG_ON(folio_test_writeback(folio)); if len < folio_size(folio) { folio_zero_segment(folio, len, folio_size(folio)); }
    loop { let block_start = bh_offset(bh); if block_start >= len { clear_buffer_dirty(bh); set_buffer_uptodate(bh); } else if !buffer_dirty(bh) || buffer_delay(bh) || !buffer_mapped(bh) || buffer_unwritten(bh) { if !buffer_mapped(bh) { clear_buffer_dirty(bh); } if buffer_dirty(bh) || (buffer_jbd(bh) && buffer_jbddirty(bh)) { if !folio_test_dirty(folio) { folio_redirty_for_writepage(wbc, folio); } keep_towrite = true; } } else { if buffer_new(bh) { clear_buffer_new(bh); } set_buffer_async_write(bh); clear_buffer_dirty(bh); nr_to_submit += 1; } bh = (*bh).b_this_page; if bh == head { break; } }
    if nr_to_submit == 0 { __folio_start_writeback(folio, keep_towrite); folio_end_writeback(folio); return; }
    bh = head; __folio_start_writeback(folio, keep_towrite); loop { if buffer_async_write(bh) { io_submit_add_bh(io, inode, folio, bh); } bh = (*bh).b_this_page; if bh == head { break; } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
