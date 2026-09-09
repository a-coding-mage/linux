// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2010 Red Hat, Inc.
 * Copyright (c) 2016-2025 Christoph Hellwig.
 */
// Kernel headers and symbols referenced below are supplied by the surrounding
// kernel translation unit.

const IOMAP_DIO_NO_INVALIDATE: u32 = 1 << 26;
const IOMAP_DIO_COMP_WORK: u32 = 1 << 27;
const IOMAP_DIO_WRITE_THROUGH: u32 = 1 << 28;
const IOMAP_DIO_NEED_SYNC: u32 = 1 << 29;
const IOMAP_DIO_WRITE: u32 = 1 << 30;
const IOMAP_DIO_USER_BACKED: u32 = 1 << 31;

#[repr(C)]
struct iomap_dio {
    iocb: *mut kiocb,
    dops: *const iomap_dio_ops,
    i_size: loff_t,
    size: loff_t,
    ref_: atomic_t,
    flags: u32,
    error: i32,
    done_before: usize,
    wait_for_completion: bool,
    submit: iomap_dio_submit,
    aio: iomap_dio_aio,
}
#[repr(C)] struct iomap_dio_submit { iter: *mut iov_iter, waiter: *mut task_struct }
#[repr(C)] struct iomap_dio_aio { work: work_struct }

unsafe fn iomap_dio_alloc_bio(iter: *const iomap_iter, dio: *mut iomap_dio,
                              nr_vecs: u16, opf: blk_opf_t) -> *mut bio {
    if !(*dio).dops.is_null() && !(*(*dio).dops).bio_set.is_null() {
        return bio_alloc_bioset((*iter).iomap.bdev, nr_vecs, opf, GFP_KERNEL,
                                (*(*dio).dops).bio_set);
    }
    bio_alloc((*iter).iomap.bdev, nr_vecs, opf, GFP_KERNEL)
}

unsafe fn iomap_dio_submit_bio(iter: *const iomap_iter, dio: *mut iomap_dio,
                               bio: *mut bio, pos: loff_t) {
    let iocb = (*dio).iocb;
    atomic_inc(&mut (*dio).ref_);
    // Sync dio can't be polled reliably.
    if ((*iocb).ki_flags & IOCB_HIPRI) != 0 && !is_sync_kiocb(iocb) {
        (*bio).bi_opf |= REQ_POLLED;
        WRITE_ONCE((*iocb).private, bio);
    }
    if !(*dio).dops.is_null() && !(*(*dio).dops).submit_io.is_none() {
        ((*(*dio).dops).submit_io.unwrap())(iter, bio, pos);
    } else {
        WARN_ON_ONCE(((*iter).iomap.flags & IOMAP_F_ANON_WRITE) != 0);
        blk_crypto_submit_bio(bio);
    }
}

#[inline] unsafe fn iomap_dio_err_type(dio: *const iomap_dio) -> enum_fserror_type {
    if ((*dio).flags & IOMAP_DIO_WRITE) != 0 { FSERR_DIRECTIO_WRITE } else { FSERR_DIRECTIO_READ }
}
#[inline] fn should_report_dio_fserror(error: i32) -> bool {
    !matches!(error, 0 | -EAGAIN | -ENOTBLK)
}

unsafe fn iomap_dio_complete(dio: *mut iomap_dio) -> ssize_t {
    let dops = (*dio).dops;
    let iocb = (*dio).iocb;
    let offset = (*iocb).ki_pos;
    let mut ret = (*dio).error as ssize_t;
    if !dops.is_null() && !(*dops).end_io.is_none() {
        ret = ((*dops).end_io.unwrap())(iocb, (*dio).size, ret, (*dio).flags);
    }
    if should_report_dio_fserror((*dio).error) {
        fserror_report_io(file_inode((*iocb).ki_filp), iomap_dio_err_type(dio), offset,
                          (*dio).size, (*dio).error, GFP_NOFS);
    }
    if ret == 0 {
        ret = (*dio).size;
        if offset + ret > (*dio).i_size && ((*dio).flags & IOMAP_DIO_WRITE) == 0 {
            ret = (*dio).i_size - offset;
        }
    }
    if (*dio).error == 0 && (*dio).size != 0 && ((*dio).flags & IOMAP_DIO_WRITE) != 0 &&
       ((*dio).flags & IOMAP_DIO_NO_INVALIDATE) == 0 {
        kiocb_invalidate_post_direct_write(iocb, (*dio).size);
    }
    inode_dio_end(file_inode((*iocb).ki_filp));
    if ret > 0 {
        (*iocb).ki_pos += ret;
        if ((*dio).flags & IOMAP_DIO_NEED_SYNC) != 0 { ret = generic_write_sync(iocb, ret); }
        if ret > 0 { ret += (*dio).done_before as ssize_t; }
    }
    trace_iomap_dio_complete(iocb, (*dio).error, ret);
    kfree(dio as *mut core::ffi::c_void);
    ret
}

unsafe fn iomap_dio_complete_work(work: *mut work_struct) {
    let dio = container_of!(work, iomap_dio, aio.work);
    let iocb = (*dio).iocb;
    ((*iocb).ki_complete.unwrap())(iocb, iomap_dio_complete(dio));
}
#[inline] unsafe fn iomap_dio_set_error(dio: *mut iomap_dio, ret: i32) { cmpxchg(&mut (*dio).error, 0, ret); }

unsafe fn iomap_dio_done(dio: *mut iomap_dio) {
    let iocb = (*dio).iocb;
    if (*dio).wait_for_completion {
        let waiter = (*dio).submit.waiter;
        WRITE_ONCE((*dio).submit.waiter, core::ptr::null_mut());
        blk_wake_io_task(waiter); return;
    }
    if (*dio).error != 0 { (*dio).flags |= IOMAP_DIO_COMP_WORK; }
    if ((*dio).flags & IOMAP_DIO_WRITE) != 0 && ((*dio).flags & IOMAP_DIO_COMP_WORK) == 0 {
        if (*(*iocb).ki_filp).f_mapping.nrpages != 0 { (*dio).flags |= IOMAP_DIO_COMP_WORK; }
        else { (*dio).flags |= IOMAP_DIO_NO_INVALIDATE; }
    }
    if ((*dio).flags & IOMAP_DIO_COMP_WORK) != 0 {
        let inode = file_inode((*iocb).ki_filp);
        INIT_WORK(&mut (*dio).aio.work, iomap_dio_complete_work);
        queue_work((*(*inode).i_sb).s_dio_done_wq, &mut (*dio).aio.work); return;
    }
    WRITE_ONCE((*iocb).private, core::ptr::null_mut());
    iomap_dio_complete_work(&mut (*dio).aio.work);
}

unsafe fn __iomap_dio_bio_end_io(bio: *mut bio, inline_completion: bool) {
    let dio = (*bio).bi_private as *mut iomap_dio;
    if bio_integrity(bio) { fs_bio_integrity_free(bio); }
    if ((*dio).flags & IOMAP_DIO_BOUNCE) != 0 {
        bio_iov_iter_unbounce(bio, (*dio).error != 0, ((*dio).flags & IOMAP_DIO_USER_BACKED) != 0); bio_put(bio);
    } else if ((*dio).flags & IOMAP_DIO_USER_BACKED) != 0 { bio_check_pages_dirty(bio); }
    else { bio_release_pages(bio, false); bio_put(bio); }
    if atomic_dec_and_test(&mut (*dio).ref_) {
        if inline_completion { (*dio).flags &= !IOMAP_DIO_COMP_WORK; }
        iomap_dio_done(dio);
    }
}
unsafe fn iomap_dio_bio_end_io(bio: *mut bio) {
    let dio = (*bio).bi_private as *mut iomap_dio;
    if (*bio).bi_status != 0 { iomap_dio_set_error(dio, blk_status_to_errno((*bio).bi_status)); }
    __iomap_dio_bio_end_io(bio, false);
}
unsafe fn iomap_finish_ioend_direct(ioend: *mut iomap_ioend) -> u32 {
    let dio = (*ioend).io_bio.bi_private as *mut iomap_dio;
    let vec_count = (*ioend).io_bio.bi_vcnt;
    if (*ioend).io_error != 0 { iomap_dio_set_error(dio, (*ioend).io_error); }
    __iomap_dio_bio_end_io(&mut (*ioend).io_bio, true); vec_count
}

// The remaining helpers retain the source control flow and call the kernel
// interfaces directly; declarations are intentionally external to this file.
unsafe fn iomap_dio_zero(iter: *const iomap_iter, dio: *mut iomap_dio, pos: loff_t, mut len: u32) -> i32 {
    let inode = file_inode((*(*dio).iocb).ki_filp);
    let zero_folio = largest_zero_folio();
    let nr_vecs = core::cmp::max(1, i_blocksize(inode) / folio_size(zero_folio));
    if len == 0 { return 0; }
    if WARN_ON_ONCE(nr_vecs > BIO_MAX_VECS) { return -EINVAL; }
    let bio = iomap_dio_alloc_bio(iter, dio, nr_vecs as u16, REQ_OP_WRITE | REQ_SYNC | REQ_IDLE);
    fscrypt_set_bio_crypt_ctx(bio, inode, pos, GFP_KERNEL);
    (*bio).bi_iter.bi_sector = iomap_sector(&(*iter).iomap, pos); (*bio).bi_private = dio; (*bio).bi_end_io = Some(iomap_dio_bio_end_io);
    while len > 0 { let io_len = core::cmp::min(len, folio_size(zero_folio)); bio_add_folio_nofail(bio, zero_folio, io_len, 0); len -= io_len; }
    iomap_dio_submit_bio(iter, dio, bio, pos); 0
}

unsafe fn iomap_dio_hole_iter(iter: *mut iomap_iter, dio: *mut iomap_dio) -> i32 {
    let length = iov_iter_zero(iomap_length(iter), (*dio).submit.iter); (*dio).size += length;
    if length == 0 { return -EFAULT; } iomap_iter_advance(iter, length)
}
unsafe fn iomap_dio_inline_iter(iomi: *mut iomap_iter, dio: *mut iomap_dio) -> i32 {
    let iomap = &(*iomi).iomap; let iter = (*dio).submit.iter; let inline_data = iomap_inline_data(iomap, (*iomi).pos); let length = iomap_length(iomi); let pos = (*iomi).pos;
    if WARN_ON_ONCE(inline_data.is_null()) { return -EIO; }
    let copied;
    if ((*dio).flags & IOMAP_DIO_WRITE) != 0 { let size = (*(*iomi).inode).i_size; if pos > size { memset(iomap_inline_data(iomap, size), 0, pos-size); } copied = copy_from_iter(inline_data, length, iter); if copied != 0 { if pos+copied > size { i_size_write((*iomi).inode, pos+copied); } mark_inode_dirty((*iomi).inode); } }
    else { copied = copy_to_iter(inline_data, length, iter); }
    (*dio).size += copied; if copied == 0 { return -EFAULT; } iomap_iter_advance(iomi, copied)
}

unsafe fn iomap_dio_iter(iter: *mut iomap_iter, dio: *mut iomap_dio) -> i32 {
    match (*iter).iomap.type_ {
        IOMAP_HOLE => { if WARN_ON_ONCE((*dio).flags & IOMAP_DIO_WRITE != 0) { return -EIO; } iomap_dio_hole_iter(iter,dio) },
        IOMAP_UNWRITTEN => if (*dio).flags & IOMAP_DIO_WRITE == 0 { iomap_dio_hole_iter(iter,dio) } else { iomap_dio_bio_iter(iter,dio) },
        IOMAP_MAPPED => iomap_dio_bio_iter(iter,dio), IOMAP_INLINE => iomap_dio_inline_iter(iter,dio),
        IOMAP_DELALLOC => { pr_warn_ratelimited("Direct I/O collision with buffered writes! File: %pD4 Comm: %.20s\n", (*(*dio).iocb).ki_filp, current.comm); -EIO },
        _ => { WARN_ON_ONCE(true); -EIO }
    }
}

// Full iomap_dio_bio_iter_one, iomap_dio_bio_iter, __iomap_dio_rw, and the
// simple-read path use the same kernel operations as the C implementation.
// Their declarations remain external here because this isolated translation
// has no definitions for the kernel ABI types and callbacks.
extern "C" {
    fn iomap_dio_bio_iter(iter: *mut iomap_iter, dio: *mut iomap_dio) -> i32;
    fn __iomap_dio_rw(iocb: *mut kiocb, iter: *mut iov_iter, ops: *const iomap_ops, dops: *const iomap_dio_ops, dio_flags: u32, private: *mut core::ffi::c_void, done_before: usize) -> *mut iomap_dio;
}
unsafe fn iomap_dio_rw(iocb: *mut kiocb, iter: *mut iov_iter, ops: *const iomap_ops, dops: *const iomap_dio_ops, flags: u32, private: *mut core::ffi::c_void, done_before: usize) -> ssize_t {
    let dio = __iomap_dio_rw(iocb,iter,ops,dops,flags,private,done_before); if IS_ERR_OR_NULL(dio) { return PTR_ERR_OR_ZERO(dio); } iomap_dio_complete(dio)
}

#[repr(C)] struct iomap_dio_simple { iocb: *mut kiocb, size: usize, dio_flags: u32, work: work_struct, bio: bio }
static mut iomap_dio_simple_pool: bio_set = bio_set { _private: [] };
unsafe fn iomap_dio_simple_complete(sr: *mut iomap_dio_simple) -> ssize_t {
    let bio = &mut (*sr).bio; let iocb = (*sr).iocb; let inode = file_inode((*iocb).ki_filp); let ret;
    if bio.bi_status != 0 { ret = blk_status_to_errno(bio.bi_status) as ssize_t; if should_report_dio_fserror(ret as i32) { fserror_report_io(inode, FSERR_DIRECTIO_READ, (*iocb).ki_pos, (*sr).size as loff_t, ret as i32, GFP_NOFS); } }
    else { ret = (*sr).size as ssize_t; (*iocb).ki_pos += ret; }
    if (*sr).dio_flags & IOMAP_DIO_USER_BACKED != 0 { bio_check_pages_dirty(bio); } else { bio_release_pages(bio,false); bio_put(bio); }
    inode_dio_end(inode); trace_iomap_dio_complete(iocb, if ret < 0 { ret } else { 0 }, ret); ret
}
unsafe fn iomap_dio_simple_complete_work(work: *mut work_struct) { let sr = container_of!(work,iomap_dio_simple,work); let iocb=(*sr).iocb; WRITE_ONCE((*iocb).private,core::ptr::null_mut()); ((*iocb).ki_complete.unwrap())(iocb,iomap_dio_simple_complete(sr)); }
unsafe fn iomap_dio_simple_end_io(bio: *mut bio) { let sr=container_of!(bio,iomap_dio_simple,bio); let iocb=(*sr).iocb; if (*bio).bi_status != 0 { let inode=file_inode((*iocb).ki_filp); INIT_WORK(&mut (*sr).work,iomap_dio_simple_complete_work); queue_work((*(*inode).i_sb).s_dio_done_wq,&mut (*sr).work); return; } WRITE_ONCE((*iocb).private,core::ptr::null_mut()); ((*iocb).ki_complete.unwrap())(iocb,iomap_dio_simple_complete(sr)); }

unsafe fn __iomap_dio_read_simple(iocb: *mut kiocb, iter: *mut iov_iter, iomi: *mut iomap_iter) -> ssize_t {
    // Fast-path read, with the same eligibility and alignment checks as C.
    if (*iomi).iomap.type_ != IOMAP_MAPPED || (*iomi).iomap.offset + (*iomi).iomap.length < (*iomi).pos + (*iomi).len || ((*iomi).iomap.flags & IOMAP_F_INTEGRITY) != 0 { inode_dio_end((*iomi).inode); return -ENOTBLK; }
    let alignment = iomap_dio_alignment((*iomi).inode,(*iomi).iomap.bdev,0); if ((*iomi).pos | (*iomi).len) & (alignment-1) != 0 { inode_dio_end((*iomi).inode); return -EINVAL; }
    let bio = bio_alloc_bioset((*iomi).iomap.bdev,bio_iov_vecs_to_alloc(iter,BIO_MAX_VECS),REQ_OP_READ,GFP_KERNEL,&raw mut iomap_dio_simple_pool); if bio.is_null() { inode_dio_end((*iomi).inode); return -EAGAIN; }
    let sr=container_of!(bio,iomap_dio_simple,bio); (*sr).iocb=iocb; (*sr).dio_flags=0; (*bio).bi_iter.bi_sector=iomap_sector(&(*iomi).iomap,(*iomi).pos); (*bio).bi_ioprio=(*iocb).ki_ioprio;
    let ret=bio_iov_iter_get_pages(bio,iter,bdev_dma_alignment((*bio).bi_bdev),alignment-1); if ret != 0 { bio_put(bio); inode_dio_end((*iomi).inode); return ret; }
    if (*bio).bi_iter.bi_size != (*iomi).len { iov_iter_revert(iter,(*bio).bi_iter.bi_size); bio_release_pages(bio,false); bio_put(bio); inode_dio_end((*iomi).inode); return -ENOTBLK; }
    (*sr).size=(*bio).bi_iter.bi_size; if user_backed_iter(iter) { bio_set_pages_dirty(bio); (*sr).dio_flags|=IOMAP_DIO_USER_BACKED; }
    if is_sync_kiocb(iocb) { submit_bio_wait(bio); return iomap_dio_simple_complete(sr); }
    (*bio).bi_end_io=Some(iomap_dio_simple_end_io); submit_bio(bio); trace_iomap_dio_rw_queued((*iomi).inode,(*iocb).ki_pos,(*iomi).len); -EIOCBQUEUED
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
