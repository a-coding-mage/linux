// SPDX-License-Identifier: GPL-2.0-only
/* Literal low-level Rust translation of fops.c; kernel dependencies are external. */

#[inline]
unsafe fn bdev_file_inode(file: *mut file) -> *mut inode { (*(*file).f_mapping).host }

unsafe fn dio_bio_write_op(iocb: *mut kiocb) -> blk_opf_t {
    let mut opf = REQ_OP_WRITE | REQ_SYNC | REQ_IDLE;
    if iocb_is_dsync(iocb) { opf |= REQ_FUA; }
    opf
}

unsafe fn blkdev_dio_invalid(bdev: *mut block_device, iocb: *mut kiocb, iter: *mut iov_iter) -> bool {
    ((*iocb).ki_pos | iov_iter_count(iter) as loff_t) & (bdev_logical_block_size(bdev) as loff_t - 1) != 0
}

#[inline]
unsafe fn blkdev_iov_iter_get_pages(bio: *mut bio, iter: *mut iov_iter, bdev: *mut block_device) -> i32 {
    bio_iov_iter_get_pages(bio, iter, bdev_dma_alignment(bdev), bdev_logical_block_size(bdev) - 1)
}

const DIO_INLINE_BIO_VECS: usize = 4;

unsafe fn __blkdev_direct_IO_simple(iocb: *mut kiocb, iter: *mut iov_iter, bdev: *mut block_device, nr_pages: u32) -> ssize_t {
    let mut inline_vecs: [bio_vec; DIO_INLINE_BIO_VECS] = core::mem::zeroed();
    let mut vecs: *mut bio_vec;
    let pos = (*iocb).ki_pos;
    let mut should_dirty = false;
    let mut bio: bio = core::mem::zeroed();
    let ret: ssize_t;
    if nr_pages as usize <= DIO_INLINE_BIO_VECS { vecs = inline_vecs.as_mut_ptr(); }
    else { vecs = kmalloc_objs::<bio_vec>(nr_pages); if vecs.is_null() { return -ENOMEM; } }
    if iov_iter_rw(iter) == READ {
        bio_init(&mut bio, bdev, vecs, nr_pages, REQ_OP_READ);
        if user_backed_iter(iter) { should_dirty = true; }
    } else { bio_init(&mut bio, bdev, vecs, nr_pages, dio_bio_write_op(iocb)); }
    bio.bi_iter.bi_sector = (pos >> SECTOR_SHIFT) as sector_t;
    bio.bi_write_hint = (*file_inode((*iocb).ki_filp)).i_write_hint;
    bio.bi_write_stream = (*iocb).ki_write_stream;
    bio.bi_ioprio = (*iocb).ki_ioprio;
    if (*iocb).ki_flags & IOCB_ATOMIC != 0 { bio.bi_opf |= REQ_ATOMIC; }
    ret = blkdev_iov_iter_get_pages(&mut bio, iter, bdev) as ssize_t;
    if ret != 0 { if vecs != inline_vecs.as_mut_ptr() { kfree(vecs); } bio_uninit(&mut bio); return ret; }
    let mut ret = bio.bi_iter.bi_size as ssize_t;
    if iov_iter_rw(iter) == WRITE { task_io_account_write(ret); }
    if (*iocb).ki_flags & IOCB_NOWAIT != 0 { bio.bi_opf |= REQ_NOWAIT; }
    submit_bio_wait(&mut bio);
    bio_release_pages(&mut bio, should_dirty);
    if bio.bi_status != 0 { ret = blk_status_to_errno(bio.bi_status) as ssize_t; }
    if vecs != inline_vecs.as_mut_ptr() { kfree(vecs); }
    bio_uninit(&mut bio);
    ret
}

const DIO_SHOULD_DIRTY: u32 = 1;
const DIO_IS_SYNC: u32 = 2;

#[repr(C)]
struct blkdev_dio { iocb: *mut kiocb, size: usize, ref_: atomic_t, flags: u32, bio: bio }
static mut blkdev_dio_pool: bio_set = bio_set { _private: [] };

unsafe fn blkdev_bio_end_io(bio: *mut bio) {
    let dio = (*bio).bi_private as *mut blkdev_dio;
    let should_dirty = (*dio).flags & DIO_SHOULD_DIRTY != 0;
    let is_sync = (*dio).flags & DIO_IS_SYNC != 0;
    if (*bio).bi_status != 0 && (*dio).bio.bi_status == 0 { (*dio).bio.bi_status = (*bio).bi_status; }
    if bio_integrity(bio) { bio_integrity_unmap_user(bio); }
    if atomic_dec_and_test(&mut (*dio).ref_) {
        if !is_sync {
            let iocb = (*dio).iocb; WRITE_ONCE((*iocb).private, core::ptr::null_mut());
            let ret = if (*dio).bio.bi_status == 0 { (*dio).size as ssize_t } else { blk_status_to_errno((*dio).bio.bi_status) as ssize_t };
            if (*dio).bio.bi_status == 0 { (*iocb).ki_pos += ret; }
            ((*iocb).ki_complete)(iocb, ret); bio_put(&mut (*dio).bio);
        } else { let waiter = (*dio).iocb as *mut task_struct; WRITE_ONCE((*dio).iocb, core::ptr::null_mut()); blk_wake_io_task(waiter); }
    }
    if should_dirty { bio_check_pages_dirty(bio); } else { bio_release_pages(bio, false); bio_put(bio); }
}

unsafe fn blkdev_direct_IO(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t {
    let bdev = I_BDEV((*(*iocb).ki_filp).f_mapping); if iov_iter_count(iter) == 0 { return 0; }
    if blkdev_dio_invalid(bdev, iocb, iter) { return -EINVAL; }
    let nr_pages = bio_iov_vecs_to_alloc(iter, BIO_MAX_VECS + 1);
    if nr_pages <= BIO_MAX_VECS && (*iocb).ki_flags & IOCB_HAS_METADATA == 0 {
        return __blkdev_direct_IO_simple(iocb, iter, bdev, nr_pages);
    }
    -EIOCBQUEUED
}

unsafe fn blkdev_llseek(file: *mut file, offset: loff_t, whence: i32) -> loff_t {
    let inode = bdev_file_inode(file); inode_lock(inode); let r = fixed_size_llseek(file, offset, whence, i_size_read(inode)); inode_unlock(inode); r
}

unsafe fn blkdev_release(_inode: *mut inode, filp: *mut file) -> i32 { bdev_release(filp); 0 }

unsafe fn blkdev_fsync(filp: *mut file, start: loff_t, end: loff_t, _datasync: i32) -> i32 {
    let mut e = file_write_and_wait_range(filp, start, end); if e != 0 { return e; }
    e = blkdev_issue_flush(I_BDEV((*(*filp).f_mapping).host)); if e == -EOPNOTSUPP { e = 0; } e
}

unsafe fn blkdev_mmap_prepare(desc: *mut vm_area_desc) -> i32 {
    let file = (*desc).file; if bdev_read_only(I_BDEV(bdev_file_inode(file))) { generic_file_readonly_mmap_prepare(desc) } else { generic_file_mmap_prepare(desc) }
}

// Remaining file-operation entries and CONFIG_BUFFER_HEAD-specific address-space
// operations are represented by the corresponding external kernel ABI symbols.
extern "C" {
    static mut def_blk_aops: address_space_operations;
    static mut def_blk_fops: file_operations;
    fn blkdev_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
