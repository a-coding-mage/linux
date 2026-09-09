// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2016-2025 Christoph Hellwig. */
// C dependencies supplied by the surrounding kernel translation unit.

pub static mut iomap_ioend_bioset: bio_set = bio_set { _unused: 0 };
static mut iomap_ioend_split_bioset: bio_set = bio_set { _unused: 0 };

pub unsafe fn iomap_init_ioend(inode: *mut inode, bio: *mut bio,
    file_offset: loff_t, ioend_flags: u16) -> *mut iomap_ioend {
    let ioend = iomap_ioend_from_bio(bio);
    atomic_set(&mut (*ioend).io_remaining, 1);
    (*ioend).io_error = 0;
    (*ioend).io_parent = core::ptr::null_mut();
    INIT_LIST_HEAD(&mut (*ioend).io_list);
    (*ioend).io_flags = ioend_flags;
    (*ioend).io_inode = inode;
    (*ioend).io_offset = file_offset;
    (*ioend).io_size = (*bio).bi_iter.bi_size;
    (*ioend).io_sector = (*bio).bi_iter.bi_sector;
    (*ioend).io_vi = core::ptr::null_mut();
    (*ioend).io_private = core::ptr::null_mut();
    ioend
}

// Update folio state, release bio holds, and free the ioend.
unsafe fn iomap_finish_ioend_buffered_write(ioend: *mut iomap_ioend) -> u32 {
    let inode = (*ioend).io_inode;
    let bio = &mut (*ioend).io_bio;
    let mut fi: folio_iter = core::mem::zeroed();
    let mut folio_count: u32 = 0;
    if (*ioend).io_error != 0 {
        mapping_set_error((*inode).i_mapping, (*ioend).io_error);
        if !bio_flagged(bio, BIO_QUIET) {
            pr_err_ratelimited("%s: writeback error on inode %llu, offset %lld, sector %llu",
                (*(*inode).i_sb).s_id, (*inode).i_ino, (*ioend).io_offset,
                (*ioend).io_sector);
        }
    }
    bio_for_each_folio_all!(fi, bio, {
        if (*ioend).io_error != 0 {
            fserror_report_io(inode, FSERR_BUFFERED_WRITE, folio_pos(fi.folio) + fi.offset,
                fi.length, (*ioend).io_error, GFP_ATOMIC);
        }
        iomap_finish_folio_write(inode, fi.folio, fi.length);
        folio_count += 1;
    });
    if bio_integrity(bio) { fs_bio_integrity_free(bio); }
    bio_put(bio);
    folio_count
}

static mut failed_ioend_lock: spinlock_t = spinlock_t { _unused: 0 };
static mut failed_ioend_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

unsafe fn iomap_fail_ioends(work: *mut work_struct) {
    let mut ioend: *mut iomap_ioend;
    let mut tmp: list_head = core::mem::zeroed();
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut failed_ioend_lock, &mut flags);
    list_replace_init(&mut failed_ioend_list, &mut tmp);
    spin_unlock_irqrestore(&mut failed_ioend_lock, flags);
    while { ioend = list_first_entry_or_null(&mut tmp); !ioend.is_null() } {
        list_del_init(&mut (*ioend).io_list);
        iomap_finish_ioend_buffered_write(ioend);
        cond_resched();
    }
}

static mut failed_ioend_work: work_struct = work_struct { _unused: 0 };

unsafe fn iomap_fail_ioend_buffered(ioend: *mut iomap_ioend) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut failed_ioend_lock, &mut flags);
    if list_empty(&failed_ioend_list) { WARN_ON_ONCE(!schedule_work(&mut failed_ioend_work)); }
    list_add_tail(&mut (*ioend).io_list, &mut failed_ioend_list);
    spin_unlock_irqrestore(&mut failed_ioend_lock, flags);
}

unsafe fn ioend_writeback_end_bio(bio: *mut bio) {
    let ioend = iomap_ioend_from_bio(bio);
    (*ioend).io_error = blk_status_to_errno((*bio).bi_status);
    if (*ioend).io_error != 0 { iomap_fail_ioend_buffered(ioend); return; }
    iomap_finish_ioend_buffered_write(ioend);
}

pub unsafe fn iomap_ioend_writeback_submit(wpc: *mut iomap_writepage_ctx, mut error: c_int) -> c_int {
    let ioend = (*wpc).wb_ctx;
    if (*ioend).io_bio.bi_end_io.is_none() { (*ioend).io_bio.bi_end_io = Some(ioend_writeback_end_bio); }
    if WARN_ON_ONCE((*wpc).iomap.flags & IOMAP_F_ANON_WRITE != 0) { error = -EIO; }
    if error != 0 { (*ioend).io_bio.bi_status = errno_to_blk_status(error); bio_endio(&mut (*ioend).io_bio); return error; }
    if (*wpc).iomap.flags & IOMAP_F_INTEGRITY != 0 { fs_bio_integrity_generate(&mut (*ioend).io_bio); }
    submit_bio(&mut (*ioend).io_bio); 0
}

unsafe fn iomap_alloc_ioend(wpc: *mut iomap_writepage_ctx, pos: loff_t, ioend_flags: u16) -> *mut iomap_ioend {
    let bio = bio_alloc_bioset((*wpc).iomap.bdev, BIO_MAX_VECS,
        REQ_OP_WRITE | wbc_to_write_flags((*wpc).wbc), GFP_NOFS, &mut iomap_ioend_bioset);
    (*bio).bi_iter.bi_sector = iomap_sector(&(*wpc).iomap, pos);
    (*bio).bi_write_hint = (*(*wpc).inode).i_write_hint;
    wbc_init_bio((*wpc).wbc, bio);
    (*wpc).nr_folios = 0;
    iomap_init_ioend((*wpc).inode, bio, pos, ioend_flags)
}

unsafe fn iomap_can_add_to_ioend(wpc: *mut iomap_writepage_ctx, pos: loff_t, map_len: u32, ioend_flags: u16) -> bool {
    let ioend = (*wpc).wb_ctx;
    if (*ioend).io_bio.bi_iter.bi_size > iomap_max_bio_size(&(*wpc).iomap) - map_len { return false; }
    if ioend_flags & IOMAP_IOEND_BOUNDARY != 0 { return false; }
    if ioend_flags & IOMAP_IOEND_NOMERGE_FLAGS != (*ioend).io_flags & IOMAP_IOEND_NOMERGE_FLAGS { return false; }
    if pos != (*ioend).io_offset + (*ioend).io_size { return false; }
    if (*wpc).iomap.flags & IOMAP_F_ANON_WRITE == 0 && iomap_sector(&(*wpc).iomap, pos) != bio_end_sector(&(*ioend).io_bio) { return false; }
    (*wpc).nr_folios < IOEND_BATCH_SIZE
}

pub unsafe fn iomap_add_to_ioend(wpc: *mut iomap_writepage_ctx, folio: *mut folio,
    pos: loff_t, end_pos: loff_t, dirty_len: u32) -> ssize_t {
    let mut ioend = (*wpc).wb_ctx;
    let poff = offset_in_folio(folio, pos);
    let mut ioend_flags: u16 = 0;
    let map_len = min_t(dirty_len as u64, (*wpc).iomap.offset + (*wpc).iomap.length - pos) as u32;
    let mut error: c_int;
    trace_iomap_add_to_ioend((*wpc).inode, pos, dirty_len, &(*wpc).iomap);
    WARN_ON_ONCE(!(*folio).private && map_len < dirty_len);
    match (*wpc).iomap.type_ {
        IOMAP_UNWRITTEN => ioend_flags |= IOMAP_IOEND_UNWRITTEN,
        IOMAP_MAPPED => (),
        IOMAP_HOLE => return map_len as ssize_t,
        _ => { WARN_ON_ONCE(true); return -EIO as ssize_t; }
    }
    if (*wpc).iomap.flags & IOMAP_F_SHARED != 0 { ioend_flags |= IOMAP_IOEND_SHARED; }
    if pos == (*wpc).iomap.offset && (*wpc).iomap.flags & IOMAP_F_BOUNDARY != 0 { ioend_flags |= IOMAP_IOEND_BOUNDARY; }
    if ioend.is_null() || !iomap_can_add_to_ioend(wpc, pos, map_len, ioend_flags) {
        if !ioend.is_null() { error = ((*wpc).ops).writeback_submit(wpc, 0); if error != 0 { return error as ssize_t; } }
        ioend = iomap_alloc_ioend(wpc, pos, ioend_flags); (*wpc).wb_ctx = ioend;
    }
    if bio_add_folio(&mut (*ioend).io_bio, folio, map_len, poff) == 0 { return iomap_add_to_ioend(wpc, folio, pos, end_pos, dirty_len); }
    if folio_test_dropbehind(folio) { bio_set_flag(&mut (*ioend).io_bio, BIO_COMPLETE_IN_TASK); }
    (*ioend).io_size += map_len as u64;
    if (*ioend).io_offset + (*ioend).io_size > end_pos { (*ioend).io_size = if (*ioend).io_offset >= end_pos { 0 } else { end_pos - (*ioend).io_offset }; }
    wbc_account_cgroup_owner((*wpc).wbc, folio, map_len);
    map_len as ssize_t
}

unsafe fn iomap_finish_ioend(mut ioend: *mut iomap_ioend, error: c_int) -> u32 {
    if !(*ioend).io_parent.is_null() { let bio = &mut (*ioend).io_bio; ioend = (*ioend).io_parent; bio_put(bio); }
    if error != 0 { cmpxchg(&mut (*ioend).io_error, 0, error); }
    if !atomic_dec_and_test(&mut (*ioend).io_remaining) { return 0; }
    if (*ioend).io_error == 0 && bio_integrity(&mut (*ioend).io_bio) && bio_op(&(*ioend).io_bio) == REQ_OP_READ { (*ioend).io_error = fs_bio_integrity_verify(&mut (*ioend).io_bio, (*ioend).io_sector, (*ioend).io_size); }
    if (*ioend).io_flags & IOMAP_IOEND_DIRECT != 0 { return iomap_finish_ioend_direct(ioend); }
    if bio_op(&(*ioend).io_bio) == REQ_OP_READ { return iomap_finish_ioend_buffered_read(ioend); }
    iomap_finish_ioend_buffered_write(ioend)
}

pub unsafe fn iomap_finish_ioends(ioend: *mut iomap_ioend, error: c_int) {
    let mut tmp: list_head = core::mem::zeroed(); let mut completions: u32;
    might_sleep(); list_replace_init(&mut (*ioend).io_list, &mut tmp); completions = iomap_finish_ioend(ioend, error);
    while !list_empty(&tmp) { if completions > IOEND_BATCH_SIZE * 8 { cond_resched(); completions = 0; } ioend = list_first_entry(&mut tmp); list_del_init(&mut (*ioend).io_list); completions += iomap_finish_ioend(ioend, error); }
}

unsafe fn iomap_ioend_can_merge(ioend: *mut iomap_ioend, next: *mut iomap_ioend) -> bool {
    if bio_op(&(*ioend).io_bio) == REQ_OP_READ || bio_op(&(*next).io_bio) == REQ_OP_READ { return false; }
    if (*ioend).io_bio.bi_status != (*next).io_bio.bi_status || (*ioend).io_private != (*next).io_private { return false; }
    if (*next).io_flags & IOMAP_IOEND_BOUNDARY != 0 || (*ioend).io_flags & IOMAP_IOEND_NOMERGE_FLAGS != (*next).io_flags & IOMAP_IOEND_NOMERGE_FLAGS { return false; }
    if (*ioend).io_offset + (*ioend).io_size != (*next).io_offset { return false; }
    (*ioend).io_sector + ((*ioend).io_size >> SECTOR_SHIFT) == (*next).io_sector
}

pub unsafe fn iomap_ioend_try_merge(ioend: *mut iomap_ioend, more_ioends: *mut list_head) {
    let mut next: *mut iomap_ioend; INIT_LIST_HEAD(&mut (*ioend).io_list);
    while { next = list_first_entry_or_null(more_ioends); !next.is_null() } { if !iomap_ioend_can_merge(ioend, next) { break; } list_move_tail(&mut (*next).io_list, &mut (*ioend).io_list); (*ioend).io_size += (*next).io_size; }
}

unsafe fn iomap_ioend_compare(_priv: *mut c_void, a: *const list_head, b: *const list_head) -> c_int {
    let ia = container_of(a, iomap_ioend, io_list); let ib = container_of(b, iomap_ioend, io_list);
    if (*ia).io_offset < (*ib).io_offset { -1 } else if (*ia).io_offset > (*ib).io_offset { 1 } else { 0 }
}

pub unsafe fn iomap_sort_ioends(ioend_list: *mut list_head) { list_sort(core::ptr::null_mut(), ioend_list, iomap_ioend_compare); }

pub unsafe fn iomap_split_ioend(ioend: *mut iomap_ioend, mut max_len: u32, is_append: bool) -> *mut iomap_ioend {
    let bio = &mut (*ioend).io_bio; let mut nr_segs = 0; let sector_offset: c_int;
    if is_append { let lim = bdev_limits((*bio).bi_bdev); max_len = min(max_len, lim.max_zone_append_sectors << SECTOR_SHIFT); sector_offset = bio_split_rw_at(bio, lim, &mut nr_segs, max_len); if sector_offset < 0 { return ERR_PTR(sector_offset); } if sector_offset == 0 { return core::ptr::null_mut(); } } else { if (*bio).bi_iter.bi_size <= max_len { return core::ptr::null_mut(); } sector_offset = (max_len >> SECTOR_SHIFT) as c_int; }
    let sector_offset = ALIGN_DOWN((sector_offset as u32) << SECTOR_SHIFT, i_blocksize((*ioend).io_inode)) >> SECTOR_SHIFT;
    let split = bio_split(bio, sector_offset, GFP_NOFS, &mut iomap_ioend_split_bioset); if IS_ERR(split) { return ERR_CAST(split); }
    (*split).bi_private = (*bio).bi_private; (*split).bi_end_io = (*bio).bi_end_io;
    let split_ioend = iomap_init_ioend((*ioend).io_inode, split, (*ioend).io_offset, (*ioend).io_flags); (*split_ioend).io_parent = ioend;
    atomic_inc(&mut (*ioend).io_remaining); (*ioend).io_offset += (*split_ioend).io_size; (*ioend).io_size -= (*split_ioend).io_size; (*split_ioend).io_sector = (*ioend).io_sector; if !is_append { (*ioend).io_sector += (*split_ioend).io_size >> SECTOR_SHIFT; } split_ioend
}

unsafe fn iomap_ioend_init() -> c_int {
    let nr_mempool_entries = 4 * (PAGE_SIZE / SECTOR_SIZE);
    let mut error = bioset_init(&mut iomap_ioend_bioset, nr_mempool_entries, offset_of!(iomap_ioend, io_bio), BIOSET_NEED_BVECS);
    if error != 0 { return error; }
    error = bioset_init(&mut iomap_ioend_split_bioset, nr_mempool_entries, offset_of!(iomap_ioend, io_bio), BIOSET_NEED_BVECS);
    if error != 0 { bioset_exit(&mut iomap_ioend_bioset); } error
}

// fs_initcall(iomap_ioend_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
