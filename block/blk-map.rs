// SPDX-License-Identifier: GPL-2.0
/* Functions related to mapping data to requests */
// Kernel headers and "blk.h" are supplied by the surrounding translation unit.

#[repr(C)]
pub struct bio_map_data {
    pub is_our_pages: bool,
    pub is_null_mapped: bool,
    pub iter: iov_iter,
    pub iov: [iovec; 0],
}

unsafe fn bio_alloc_map_data(data: *mut iov_iter, gfp_mask: gfp_t) -> *mut bio_map_data {
    if (*data).nr_segs > UIO_MAXIOV { return core::ptr::null_mut(); }
    let bmd = kmalloc_flex_bio_map_data_iov((*data).nr_segs, gfp_mask);
    if bmd.is_null() { return core::ptr::null_mut(); }
    (*bmd).iter = *data;
    if iter_is_iovec(data) {
        memcpy((*bmd).iov.as_mut_ptr(), iter_iov(data), core::mem::size_of::<iovec>() * (*data).nr_segs);
        (*bmd).iter.__iov = (*bmd).iov.as_mut_ptr();
    }
    bmd
}

#[inline]
unsafe fn blk_mq_map_bio_put(bio: *mut bio) { bio_put(bio); }

unsafe fn blk_rq_map_bio_alloc(rq: *mut request, nr_vecs: c_uint, gfp_mask: gfp_t) -> *mut bio {
    let bdev = if !(*(*rq).q).disk.is_null() { (*(*(*rq).q).disk).part0 } else { core::ptr::null_mut() };
    bio_alloc_bioset(bdev, nr_vecs, (*rq).cmd_flags, gfp_mask, &raw mut fs_bio_set)
}

unsafe fn bio_copy_from_iter(bio: *mut bio, iter: *mut iov_iter) -> c_int {
    let mut bvec = core::ptr::null_mut();
    let mut iter_all = bvec_iter_all::default();
    bio_for_each_segment_all!(bvec, bio, iter_all, {
        let ret = copy_page_from_iter((*bvec).bv_page, (*bvec).bv_offset, (*bvec).bv_len, iter);
        if iov_iter_count(iter) == 0 { break; }
        if ret < (*bvec).bv_len as isize { return -EFAULT; }
    });
    0
}

unsafe fn bio_copy_to_iter(bio: *mut bio, mut iter: iov_iter) -> c_int {
    let mut bvec = core::ptr::null_mut();
    let mut iter_all = bvec_iter_all::default();
    bio_for_each_segment_all!(bvec, bio, iter_all, {
        let ret = copy_page_to_iter((*bvec).bv_page, (*bvec).bv_offset, (*bvec).bv_len, &mut iter);
        if iov_iter_count(&mut iter) == 0 { break; }
        if ret < (*bvec).bv_len as isize { return -EFAULT; }
    });
    0
}

unsafe fn bio_uncopy_user(bio: *mut bio) -> c_int {
    let bmd = (*bio).bi_private as *mut bio_map_data;
    let mut ret = 0;
    if !(*bmd).is_null_mapped {
        if (*current).mm.is_null() { ret = -EINTR; }
        else if bio_data_dir(bio) == READ { ret = bio_copy_to_iter(bio, (*bmd).iter); }
        if (*bmd).is_our_pages { bio_free_pages(bio); }
    }
    kfree(bmd as *mut core::ffi::c_void);
    ret
}

unsafe fn bio_copy_user_iov(rq: *mut request, map_data: *mut rq_map_data, iter: *mut iov_iter, gfp_mask: gfp_t) -> c_int {
    let bmd = bio_alloc_map_data(iter, gfp_mask);
    if bmd.is_null() { return -ENOMEM; }
    (*bmd).is_our_pages = map_data.is_null();
    (*bmd).is_null_mapped = !map_data.is_null() && (*map_data).null_mapped;
    let mut i = 0usize;
    let mut len = (*iter).count as c_uint;
    let mut offset = if map_data.is_null() { 0 } else { offset_in_page((*map_data).offset) };
    let mut nr_pages = bio_max_segs(DIV_ROUND_UP(offset + len, PAGE_SIZE));
    let mut bio = blk_rq_map_bio_alloc(rq, nr_pages, gfp_mask);
    if bio.is_null() { kfree(bmd as *mut _); return -ENOMEM; }
    if !map_data.is_null() { nr_pages = 1u32 << (*map_data).page_order; i = (*map_data).offset / PAGE_SIZE; }
    while len != 0 {
        let mut bytes = PAGE_SIZE - offset;
        if bytes > len { bytes = len; }
        let page;
        if !map_data.is_null() {
            if i == (*map_data).nr_entries * nr_pages { bio_free_pages(bio); blk_mq_map_bio_put(bio); kfree(bmd as *mut _); return -ENOMEM; }
            page = (*map_data).pages[i / nr_pages].add(i % nr_pages); i += 1;
        } else {
            page = alloc_page(GFP_NOIO | gfp_mask);
            if page.is_null() { bio_free_pages(bio); blk_mq_map_bio_put(bio); kfree(bmd as *mut _); return -ENOMEM; }
        }
        if bio_add_page(bio, page, bytes, offset) < bytes { if map_data.is_null() { __free_page(page); } break; }
        len -= bytes; offset = 0;
    }
    if !map_data.is_null() { (*map_data).offset += (*bio).bi_iter.bi_size; }
    let mut ret;
    if iov_iter_rw(iter) == WRITE && (map_data.is_null() || !(*map_data).null_mapped) { ret = bio_copy_from_iter(bio, iter); }
    else if !map_data.is_null() && (*map_data).from_user { let mut iter2 = *iter; iter2.data_source = ITER_SOURCE; ret = bio_copy_from_iter(bio, &mut iter2); }
    else { if (*bmd).is_our_pages { zero_fill_bio(bio); } iov_iter_advance(iter, (*bio).bi_iter.bi_size); ret = 0; }
    if ret != 0 { bio_free_pages(bio); blk_mq_map_bio_put(bio); kfree(bmd as *mut _); return ret; }
    (*bio).bi_private = bmd as *mut _;
    ret = blk_rq_append_bio(rq, bio);
    if ret != 0 { bio_free_pages(bio); blk_mq_map_bio_put(bio); kfree(bmd as *mut _); }
    ret
}

unsafe fn bio_map_user_iov(rq: *mut request, iter: *mut iov_iter, gfp_mask: gfp_t) -> c_int {
    let nr_vecs = iov_iter_npages(iter, BIO_MAX_VECS);
    if iov_iter_count(iter) == 0 { return -EINVAL; }
    let bio = blk_rq_map_bio_alloc(rq, nr_vecs, gfp_mask);
    if bio.is_null() { return -ENOMEM; }
    let mut ret = bio_iov_iter_get_pages(bio, iter, 0, 0);
    if ret == 0 { ret = blk_rq_append_bio(rq, bio); }
    if ret != 0 { bio_release_pages(bio, false); blk_mq_map_bio_put(bio); }
    ret
}

unsafe fn bio_invalidate_vmalloc_pages(bio: *mut bio) {
    #[cfg(ARCH_IMPLEMENTS_FLUSH_KERNEL_VMAP_RANGE)]
    if !(*bio).bi_private.is_null() && !op_is_write(bio_op(bio)) {
        let mut len = 0; for i in 0..(*bio).bi_vcnt { len += (*bio).bi_io_vec.add(i).read().bv_len; }
        invalidate_kernel_vmap_range((*bio).bi_private, len);
    }
}
unsafe fn bio_map_kern_endio(bio: *mut bio) { bio_invalidate_vmalloc_pages(bio); blk_mq_map_bio_put(bio); }

unsafe fn bio_map_kern(rq: *mut request, data: *mut c_void, len: c_uint, gfp_mask: gfp_t) -> *mut bio {
    let bio = blk_rq_map_bio_alloc(rq, bio_add_max_vecs(data, len), gfp_mask);
    if bio.is_null() { return ERR_PTR(-ENOMEM); }
    if is_vmalloc_addr(data) { (*bio).bi_private = data; if bio_add_vmalloc(bio, data, len) == 0 { blk_mq_map_bio_put(bio); return ERR_PTR(-EINVAL); } }
    else { bio_add_virt_nofail(bio, data, len); }
    (*bio).bi_end_io = Some(bio_map_kern_endio); bio
}

unsafe fn bio_copy_kern_endio(bio: *mut bio) { bio_free_pages(bio); blk_mq_map_bio_put(bio); }
unsafe fn bio_copy_kern_endio_read(bio: *mut bio) { let mut p = (*bio).bi_private as *mut c_char; let mut bvec = core::ptr::null_mut(); let mut all = bvec_iter_all::default(); bio_for_each_segment_all!(bvec, bio, all, { memcpy_from_bvec(p, bvec); p = p.add((*bvec).bv_len as usize); }); bio_copy_kern_endio(bio); }

unsafe fn bio_copy_kern(rq: *mut request, data: *mut c_void, mut len: c_uint, gfp_mask: gfp_t) -> *mut bio {
    let kaddr = data as usize; let end = (kaddr + len as usize + PAGE_SIZE - 1) >> PAGE_SHIFT; let start = kaddr >> PAGE_SHIFT;
    if end < start { return ERR_PTR(-EINVAL); }
    let bio = blk_rq_map_bio_alloc(rq, (end - start) as c_uint, gfp_mask); if bio.is_null() { return ERR_PTR(-ENOMEM); }
    let op = req_op(rq); let mut p = data as *mut u8;
    while len != 0 { let bytes = core::cmp::min(PAGE_SIZE, len); let page = alloc_page(GFP_NOIO | __GFP_ZERO | gfp_mask); if page.is_null() { bio_free_pages(bio); blk_mq_map_bio_put(bio); return ERR_PTR(-ENOMEM); } if op_is_write(op) { memcpy(page_address(page), p, bytes); } __bio_add_page(bio, page, bytes, 0); len -= bytes; p = p.add(bytes as usize); }
    if op_is_write(op) { (*bio).bi_end_io = Some(bio_copy_kern_endio); } else { (*bio).bi_end_io = Some(bio_copy_kern_endio_read); (*bio).bi_private = data; } bio
}

unsafe fn blk_rq_append_bio(rq: *mut request, bio: *mut bio) -> c_int {
    let lim = &(*(*rq).q).limits;
    let max_bytes = lim.max_hw_sectors << SECTOR_SHIFT;
    let mut nr_segs = 0;
    let mut ret = bio_split_io_at(bio, lim, &mut nr_segs, max_bytes, 0);
    if ret != 0 { if ret > 0 { ret = -EREMOTEIO; } return ret; }
    if !(*rq).bio.is_null() {
        if !ll_back_merge_fn(rq, bio, nr_segs) { return -EINVAL; }
        (*rq).phys_gap_bit = bio_seg_gap((*rq).q, (*rq).biotail, bio, (*rq).phys_gap_bit);
        (*(*rq).biotail).bi_next = bio; (*rq).biotail = bio;
        (*rq).__data_len += (*bio).bi_iter.bi_size; bio_crypt_free_ctx(bio); return 0;
    }
    (*rq).nr_phys_segments = nr_segs; (*rq).bio = bio; (*rq).biotail = bio;
    (*rq).__data_len = (*bio).bi_iter.bi_size; (*rq).phys_gap_bit = (*bio).bi_bvec_gap_bit; 0
}

unsafe fn blk_rq_map_user_bvec(rq: *mut request, iter: *const iov_iter) -> c_int {
    let max_bytes = (*(*rq).q).limits.max_hw_sectors << SECTOR_SHIFT;
    if iov_iter_count(iter) == 0 || iov_iter_count(iter) > max_bytes as usize { return -EINVAL; }
    let bio = blk_rq_map_bio_alloc(rq, 0, GFP_KERNEL); if bio.is_null() { return -ENOMEM; }
    bio_iov_iter_set(bio, iter); let ret = blk_rq_append_bio(rq, bio);
    if ret != 0 { blk_mq_map_bio_put(bio); } ret
}

unsafe fn blk_rq_map_user_iov(q: *mut request_queue, rq: *mut request, map_data: *mut rq_map_data, iter: *const iov_iter, gfp_mask: gfp_t) -> c_int {
    let align = blk_lim_dma_alignment_and_pad(&(*q).limits); let mut copy = !map_data.is_null();
    let mut map_bvec = false; if !copy && iov_iter_alignment(iter) & align != 0 { copy = true; }
    else if !copy && iov_iter_is_bvec(iter) { map_bvec = true; } else if !copy && !user_backed_iter(iter) { copy = true; }
    else if !copy && queue_virt_boundary(q) != 0 { copy = queue_virt_boundary(q) & iov_iter_gap_alignment(iter) != 0; }
    if map_bvec { let ret = blk_rq_map_user_bvec(rq, iter); if ret == 0 { return 0; } if ret != -EREMOTEIO { return ret; } copy = true; }
    let mut i = *iter; let mut bio = core::ptr::null_mut();
    while iov_iter_count(&i) != 0 { let ret = if copy { bio_copy_user_iov(rq, map_data, &mut i, gfp_mask) } else { bio_map_user_iov(rq, &mut i, gfp_mask) }; if ret != 0 { if !bio.is_null() { blk_rq_unmap_user(bio); } (*rq).bio = core::ptr::null_mut(); return if ret == -EREMOTEIO { -EINVAL } else { ret }; } if bio.is_null() { bio = (*rq).bio; } }
    0
}

unsafe fn blk_rq_map_user(rq: *mut request, map_data: *mut rq_map_data, ubuf: *mut c_void, len: c_ulong, gfp_mask: gfp_t) -> c_int { let mut i = iov_iter::default(); let ret = import_ubuf(rq_data_dir(rq), ubuf, len, &mut i); if ret < 0 { ret } else { blk_rq_map_user_iov((*rq).q, rq, map_data, &i, gfp_mask) } }

unsafe fn blk_rq_unmap_user(mut bio: *mut bio) -> c_int { let mut ret = 0; while !bio.is_null() { let next = (*bio).bi_next; if !(*bio).bi_private.is_null() { let r = bio_uncopy_user(bio); if r != 0 && ret == 0 { ret = r; } } else { bio_release_pages(bio, bio_data_dir(bio) == READ); } if bio_integrity(bio) { bio_integrity_unmap_user(bio); } blk_mq_map_bio_put(bio); bio = next; } ret }

unsafe fn blk_rq_map_kern(rq: *mut request, kbuf: *mut c_void, len: c_uint, gfp_mask: gfp_t) -> c_int { if len > queue_max_hw_sectors((*rq).q) << SECTOR_SHIFT || len == 0 || kbuf.is_null() { return -EINVAL; } let addr = kbuf as usize; let copy = !blk_rq_aligned((*rq).q, addr, len) || object_is_on_stack(kbuf); let bio = if copy { bio_copy_kern(rq, kbuf, len, gfp_mask) } else { bio_map_kern(rq, kbuf, len, gfp_mask) }; if IS_ERR(bio) { return PTR_ERR(bio); } let ret = blk_rq_append_bio(rq, bio); if ret != 0 { if copy { bio_free_pages(bio); } blk_mq_map_bio_put(bio); } ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
