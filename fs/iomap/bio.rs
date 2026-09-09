// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2010 Red Hat, Inc.
 * Copyright (C) 2016-2023 Christoph Hellwig.
 */
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

static failed_read_lock: SpinLock = DEFINE_SPINLOCK();
static mut failed_read_list: bio_list = BIO_EMPTY_LIST;

unsafe fn __iomap_read_end_io(bio: *mut bio, error: c_int) -> u32 {
    let mut fi: folio_iter = core::mem::zeroed();
    let mut folio_count: u32 = 0;

    bio_for_each_folio_all!(fi, bio, {
        iomap_finish_folio_read(fi.folio, fi.offset, fi.length, error);
        folio_count += 1;
    });
    if bio_integrity(bio) {
        fs_bio_integrity_free(bio);
    }
    bio_put(bio);
    folio_count
}

unsafe extern "C" fn iomap_fail_reads(work: *mut work_struct) {
    let bio: *mut bio;
    let mut tmp: bio_list = BIO_EMPTY_LIST;
    let mut flags: c_ulong;

    spin_lock_irqsave(&failed_read_lock, &mut flags);
    bio_list_merge_init(&mut tmp, &mut failed_read_list);
    spin_unlock_irqrestore(&failed_read_lock, flags);

    loop {
        bio = bio_list_pop(&mut tmp);
        if bio.is_null() {
            break;
        }
        __iomap_read_end_io(bio, blk_status_to_errno((*bio).bi_status));
        cond_resched();
    }
}

static mut failed_read_work: work_struct = DECLARE_WORK!(iomap_fail_reads);

unsafe fn iomap_fail_buffered_read(bio: *mut bio) {
    let mut flags: c_ulong;

    /*
     * Bounce I/O errors to a workqueue to avoid nested i_lock acquisitions
     * in the fserror code.  The caller no longer owns the bio reference
     * after the spinlock drops.
     */
    spin_lock_irqsave(&failed_read_lock, &mut flags);
    if bio_list_empty(&failed_read_list) {
        WARN_ON_ONCE(!schedule_work(&mut failed_read_work));
    }
    bio_list_add(&mut failed_read_list, bio);
    spin_unlock_irqrestore(&failed_read_lock, flags);
}

unsafe extern "C" fn iomap_read_end_io(bio: *mut bio) {
    if (*bio).bi_status != 0 {
        iomap_fail_buffered_read(bio);
        return;
    }

    __iomap_read_end_io(bio, 0);
}

pub unsafe fn iomap_finish_ioend_buffered_read(ioend: *mut iomap_ioend) -> u32 {
    __iomap_read_end_io(&mut (*ioend).io_bio, (*ioend).io_error)
}

pub unsafe fn iomap_bio_submit_read_endio(
    iter: *const iomap_iter,
    ctx: *mut iomap_read_folio_ctx,
    end_io: bio_end_io_t,
) {
    let bio = (*ctx).read_ctx;

    (*bio).bi_end_io = end_io;
    if (*iter).iomap.flags & IOMAP_F_INTEGRITY != 0 {
        fs_bio_integrity_alloc(bio);
    }
    submit_bio(bio);

    (*ctx).read_ctx = core::ptr::null_mut();
}

// EXPORT_SYMBOL_GPL(iomap_bio_submit_read_endio);

unsafe extern "C" fn iomap_bio_submit_read(
    iter: *const iomap_iter,
    ctx: *mut iomap_read_folio_ctx,
) {
    iomap_bio_submit_read_endio(iter, ctx, iomap_read_end_io);
}

unsafe fn iomap_read_bio_set(ctx: *mut iomap_read_folio_ctx) -> *mut bio_set {
    if !(*ctx).ops.is_null() && !(*(*ctx).ops).bio_set.is_null() {
        return (*(*ctx).ops).bio_set;
    }
    &mut fs_bio_set
}

unsafe fn iomap_read_alloc_bio(
    iter: *const iomap_iter,
    ctx: *mut iomap_read_folio_ctx,
    plen: usize,
) {
    let iomap = &(*iter).iomap;
    let nr_vecs: c_uint = DIV_ROUND_UP!(iomap_length(iter), PAGE_SIZE);
    let bio_set = iomap_read_bio_set(ctx);
    let folio = (*ctx).cur_folio;
    let gfp: gfp_t = mapping_gfp_constraint((*folio).mapping, GFP_KERNEL);
    let orig_gfp = gfp;
    let mut bio: *mut bio;

    /* Submit the existing range if there was one. */
    if !(*ctx).read_ctx.is_null() {
        ((*(*ctx).ops).submit_read)(iter, ctx);
    }

    /* Same as readahead_gfp_mask: */
    let mut alloc_gfp = gfp;
    if !(*ctx).rac.is_null() {
        alloc_gfp |= __GFP_NORETRY | __GFP_NOWARN;
    }

    /*
     * If the bio_alloc fails, try it again for a single page to avoid
     * having to deal with partial page reads.  This emulates what
     * do_mpage_read_folio does.
     */
    bio = bio_alloc_bioset(iomap.bdev, bio_max_segs(nr_vecs), REQ_OP_READ,
                           alloc_gfp, bio_set);
    if bio.is_null() {
        bio = bio_alloc_bioset(iomap.bdev, 1, REQ_OP_READ, orig_gfp, bio_set);
    }
    if !(*ctx).rac.is_null() {
        (*bio).bi_opf |= REQ_RAHEAD;
    }
    (*bio).bi_iter.bi_sector = iomap_sector(iomap, (*iter).pos);
    bio_add_folio_nofail(bio, folio, plen, offset_in_folio(folio, (*iter).pos));
    (*ctx).read_ctx = bio;
    (*ctx).read_ctx_file_offset = (*iter).pos;
}

pub unsafe fn iomap_bio_read_folio_range(
    iter: *const iomap_iter,
    ctx: *mut iomap_read_folio_ctx,
    plen: usize,
) -> c_int {
    let folio = (*ctx).cur_folio;
    let bio = (*ctx).read_ctx;

    if bio.is_null()
        || bio_end_sector(bio) != iomap_sector(&(*iter).iomap, (*iter).pos)
        || (*bio).bi_iter.bi_size > iomap_max_bio_size(&(*iter).iomap) - plen
        || !bio_add_folio(bio, folio, plen, offset_in_folio(folio, (*iter).pos))
    {
        iomap_read_alloc_bio(iter, ctx, plen);
    }
    0
}

// EXPORT_SYMBOL_GPL(iomap_bio_read_folio_range);

pub static iomap_bio_read_ops: iomap_read_ops = iomap_read_ops {
    read_folio_range: iomap_bio_read_folio_range,
    submit_read: iomap_bio_submit_read,
};

pub unsafe fn iomap_bio_read_folio_range_sync(
    iter: *const iomap_iter,
    folio: *mut folio,
    pos: loff_t,
    len: usize,
) -> c_int {
    let srcmap = iomap_iter_srcmap(iter);
    let sector = iomap_sector(srcmap, pos);
    let mut bvec: bio_vec = core::mem::zeroed();
    let mut bio: bio = core::mem::zeroed();
    let mut error: c_int;

    bio_init(&mut bio, (*srcmap).bdev, &mut bvec, 1, REQ_OP_READ);
    bio.bi_iter.bi_sector = sector;
    bio_add_folio_nofail(&mut bio, folio, len, offset_in_folio(folio, pos));
    if (*srcmap).flags & IOMAP_F_INTEGRITY != 0 {
        fs_bio_integrity_alloc(&mut bio);
    }
    error = submit_bio_wait(&mut bio);
    if bio_integrity(&mut bio) {
        if error == 0 {
            error = fs_bio_integrity_verify(&mut bio, sector, len);
        }
        fs_bio_integrity_free(&mut bio);
    }
    bio_uninit(&mut bio);
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
