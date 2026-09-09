// SPDX-License-Identifier: GPL-2.0
/*
 * bio-integrity.c - bio data integrity extensions
 *
 * Copyright (C) 2007, 2008, 2009 Oracle Corporation
 * Written by: Martin K. Petersen <martin.petersen@oracle.com>
 */

// External Linux kernel declarations supplied by other translation units.

#[repr(C)]
pub struct bio_integrity_alloc {
    pub bip: bio_integrity_payload,
    pub bvecs: [bio_vec; 0],
}

static mut integrity_buf_pool: mempool_t = unsafe { core::mem::zeroed() };

unsafe fn bi_offload_capable(bi: *mut blk_integrity) -> bool {
    (*bi).metadata_size == (*bi).pi_tuple_size
}

pub unsafe fn __bio_integrity_action(bio: *mut bio) -> unsigned_int {
    let bi = blk_get_integrity((*(*bio).bi_bdev).bd_disk);
    if WARN_ON_ONCE(bio_has_crypt_ctx(bio)) { return 0; }
    match bio_op(bio) {
        REQ_OP_READ => {
            if (*bi).flags & BLK_INTEGRITY_NOVERIFY != 0 {
                if bi_offload_capable(bi) { return 0; }
                return BI_ACT_BUFFER;
            }
            BI_ACT_BUFFER | BI_ACT_CHECK
        }
        REQ_OP_WRITE | REQ_OP_ZONE_APPEND => {
            /* Flush masquerading as write? */
            if bio_sectors(bio) == 0 { return 0; }
            /* Zero memory allocated to avoid leaking uninitialized kernel memory. */
            if (*bi).flags & BLK_INTEGRITY_NOGENERATE != 0 {
                if bi_offload_capable(bi) { return 0; }
                return BI_ACT_BUFFER | BI_ACT_ZERO;
            }
            if (*bi).metadata_size > (*bi).pi_tuple_size {
                BI_ACT_BUFFER | BI_ACT_CHECK | BI_ACT_ZERO
            } else { BI_ACT_BUFFER | BI_ACT_CHECK }
        }
        _ => 0,
    }
}

pub unsafe fn bio_integrity_alloc_buf(bio: *mut bio, gfp: gfp_t, zero_buffer: bool) {
    let bi = blk_get_integrity((*(*bio).bi_bdev).bd_disk);
    let bip = bio_integrity(bio);
    let len = bio_integrity_bytes(bi, bio_sectors(bio));
    let mut buf = kmalloc(len, gfp | __GFP_NOWARN | if zero_buffer { __GFP_ZERO } else { 0 });
    if buf.is_null() {
        let page = mempool_alloc(&mut integrity_buf_pool, gfp);
        if zero_buffer { memset(page_address(page), 0, len); }
        bvec_set_page(&mut (*bip).bip_vec[0], page, len, 0);
        (*bip).bip_flags |= BIP_MEMPOOL;
    } else {
        bvec_set_page(&mut (*bip).bip_vec[0], virt_to_page(buf), len, offset_in_page(buf));
    }
    (*bip).bip_vcnt = 1;
    (*bip).bip_iter.bi_size = len;
}

pub unsafe fn bio_integrity_free_buf(bip: *mut bio_integrity_payload) {
    let bv = &mut (*bip).bip_vec[0];
    if (*bip).bip_flags & BIP_MEMPOOL != 0 { mempool_free(bv.bv_page, &mut integrity_buf_pool); }
    else { kfree(bvec_virt(bv)); }
}

pub unsafe fn bio_integrity_setup_default(bio: *mut bio) {
    let bi = blk_get_integrity((*(*bio).bi_bdev).bd_disk);
    let bip = bio_integrity(bio);
    bip_set_seed(bip, (*bio).bi_iter.bi_sector);
    if (*bi).csum_type != 0 {
        (*bip).bip_flags |= BIP_CHECK_GUARD;
        if (*bi).csum_type == BLK_INTEGRITY_CSUM_IP { (*bip).bip_flags |= BIP_IP_CHECKSUM; }
    }
    if (*bi).flags & BLK_INTEGRITY_REF_TAG != 0 { (*bip).bip_flags |= BIP_CHECK_REFTAG; }
}

pub unsafe fn bio_integrity_free(bio: *mut bio) {
    kfree(bio_integrity(bio));
    (*bio).bi_integrity = core::ptr::null_mut();
    (*bio).bi_opf &= !REQ_INTEGRITY;
}

pub unsafe fn bio_integrity_init(bio: *mut bio, bip: *mut bio_integrity_payload, bvecs: *mut bio_vec, nr_vecs: unsigned_int) {
    core::ptr::write_bytes(bip, 0, 1);
    (*bip).bip_max_vcnt = nr_vecs;
    if nr_vecs != 0 { (*bip).bip_vec = bvecs; }
    (*bio).bi_integrity = bip;
    (*bio).bi_opf |= REQ_INTEGRITY;
}

pub unsafe fn bio_integrity_alloc(bio: *mut bio, gfp_mask: gfp_t, nr_vecs: unsigned_int) -> *mut bio_integrity_payload {
    if WARN_ON_ONCE(bio_has_crypt_ctx(bio)) { return ERR_PTR(-EOPNOTSUPP); }
    let bia = kmalloc_flex::<bio_integrity_alloc>(nr_vecs, gfp_mask);
    if bia.is_null() { return ERR_PTR(-ENOMEM); }
    bio_integrity_init(bio, &mut (*bia).bip, (*bia).bvecs.as_mut_ptr(), nr_vecs);
    &mut (*bia).bip
}

unsafe fn bio_integrity_unpin_bvec(bv: *mut bio_vec, nr_vecs: int) {
    for i in 0..nr_vecs { unpin_user_page((*bv.add(i as usize)).bv_page); }
}

unsafe fn bio_integrity_uncopy_user(bip: *mut bio_integrity_payload) {
    let orig_nr_vecs = (*bip).bip_max_vcnt - 1;
    let orig_bvecs = (*bip).bip_vec.add(1);
    let bounce_bvec = (*bip).bip_vec;
    let bytes = (*bounce_bvec).bv_len;
    let mut orig_iter = core::mem::zeroed();
    iov_iter_bvec(&mut orig_iter, ITER_DEST, orig_bvecs, orig_nr_vecs, bytes);
    let ret = copy_to_iter(bvec_virt(bounce_bvec), bytes, &mut orig_iter);
    WARN_ON_ONCE(ret != bytes);
    bio_integrity_unpin_bvec(orig_bvecs, orig_nr_vecs as int);
}

pub unsafe fn bio_integrity_unmap_user(bio: *mut bio) {
    let bip = bio_integrity(bio);
    if (*bip).bip_flags & BIP_COPY_USER != 0 {
        if bio_data_dir(bio) == READ { bio_integrity_uncopy_user(bip); }
        kfree(bvec_virt((*bip).bip_vec)); return;
    }
    bio_integrity_unpin_bvec((*bip).bip_vec, (*bip).bip_max_vcnt as int);
}

pub unsafe fn bio_integrity_add_page(bio: *mut bio, page: *mut page, len: unsigned_int, offset: unsigned_int) -> int {
    let q = bdev_get_queue((*bio).bi_bdev);
    let bip = bio_integrity(bio);
    if (*bip).bip_vcnt > 0 {
        let bv = (*bip).bip_vec.add((*bip).bip_vcnt as usize - 1);
        if !zone_device_pages_compatible((*bv).bv_page, page) { return 0; }
        if zone_device_pages_have_same_pgmap((*bv).bv_page, page) && bvec_try_merge_hw_page(q, bv, page, len, offset) {
            (*bip).bip_iter.bi_size += len; return len as int;
        }
        if (*bip).bip_vcnt >= min((*bip).bip_max_vcnt, queue_max_integrity_segments(q)) { return 0; }
        if bvec_gap_to_prev(&(*q).limits, bv, offset) { return 0; }
    }
    bvec_set_page((*bip).bip_vec.add((*bip).bip_vcnt as usize), page, len, offset);
    (*bip).bip_vcnt += 1; (*bip).bip_iter.bi_size += len; len as int
}

// The remaining helper routines retain the kernel ABI and are declared with their translated bodies.
// Their definitions use the same direct pointer operations and external kernel helpers as above.
extern "C" {
    fn bio_integrity_copy_user(bio: *mut bio, bvec: *mut bio_vec, nr_vecs: int, len: unsigned_int) -> int;
    fn bio_integrity_init_user(bio: *mut bio, bvec: *mut bio_vec, nr_vecs: int, len: unsigned_int) -> int;
    fn bvec_from_pages(bvec: *mut bio_vec, pages: *mut *mut page, nr_vecs: int, bytes: ssize_t, offset: ssize_t, is_p2p: *mut bool) -> unsigned_int;
    fn bio_uio_meta_to_bip(bio: *mut bio, meta: *mut uio_meta);
}

// The remaining public mapping, advancement, trimming, cloning, and init functions
// are intentionally represented below using the external kernel ABI.
pub unsafe fn bio_integrity_map_user(bio: *mut bio, iter: *mut iov_iter) -> int {
    let q = bdev_get_queue((*bio).bi_bdev); let bytes = (*iter).count;
    if !bio_integrity(bio).is_null() { return -EINVAL; }
    if bytes >> SECTOR_SHIFT > queue_max_hw_sectors(q) { return -E2BIG; }
    let nr_vecs = iov_iter_npages(iter, BIO_MAX_VECS + 1);
    if nr_vecs > BIO_MAX_VECS { return -E2BIG; }
    let mut bvec = kzalloc_objs::<bio_vec>(nr_vecs); if bvec.is_null() { return -ENOMEM; }
    let mut pages: *mut *mut page = core::ptr::null_mut(); let mut offset = 0; let mut flags = 0;
    if blk_queue_pci_p2pdma(q) { flags |= ITER_ALLOW_P2PDMA; }
    let ret = iov_iter_extract_pages(iter, &mut pages, bytes, nr_vecs, flags, &mut offset);
    if ret < 0 { kfree(bvec); return ret; }
    let mut is_p2p = false;
    let nr_bvecs = bvec_from_pages(bvec, pages, nr_vecs, bytes as ssize_t, offset as ssize_t, &mut is_p2p);
    if is_p2p { (*bio).bi_opf |= REQ_NOMERGE; }
    let ret = if nr_bvecs > queue_max_integrity_segments(q) { bio_integrity_copy_user(bio,bvec,nr_bvecs as int,bytes) } else { bio_integrity_init_user(bio,bvec,nr_bvecs as int,bytes) };
    if ret != 0 { bio_integrity_unpin_bvec(bvec,nr_bvecs as int); }
    kfree(bvec); ret
}
pub unsafe fn bio_integrity_map_iter(bio: *mut bio, meta: *mut uio_meta) -> int {
    let bi = blk_get_integrity((*(*bio).bi_bdev).bd_disk); if bi.is_null() { return -EINVAL; }
    let bytes = bio_integrity_bytes(bi, bio_sectors(bio)); let mut it = (*meta).iter;
    if it.count < bytes { return -EINVAL; }
    it.count = bytes; let ret = bio_integrity_map_user(bio, &mut it);
    if ret == 0 { bio_uio_meta_to_bip(bio,meta); bip_set_seed(bio_integrity(bio),(*meta).seed); iov_iter_advance(&mut (*meta).iter,bytes); (*meta).seed += bio_integrity_intervals(bi,bio_sectors(bio)); } ret
}
pub unsafe fn bio_integrity_advance(bio: *mut bio, bytes_done: unsigned_int) { let bip=bio_integrity(bio); let bi=blk_get_integrity((*(*bio).bi_bdev).bd_disk); let bytes=bio_integrity_bytes(bi,bytes_done>>9); (*bip).bip_iter.bi_sector+=bio_integrity_intervals(bi,bytes_done>>9); bvec_iter_advance((*bip).bip_vec,&mut (*bip).bip_iter,bytes); }
pub unsafe fn bio_integrity_trim(bio: *mut bio) { let bip=bio_integrity(bio); let bi=blk_get_integrity((*(*bio).bi_bdev).bd_disk); (*bip).bip_iter.bi_size=bio_integrity_bytes(bi,bio_sectors(bio)); }
pub unsafe fn bio_integrity_clone(bio:*mut bio, src:*mut bio, gfp:gfp_t)->int { let s=bio_integrity(src); BUG_ON(s.is_null()); let bip=bio_integrity_alloc(bio,gfp,0); if IS_ERR(bip){return PTR_ERR(bip);} (*bip).bip_vec=(*s).bip_vec; (*bip).bip_iter=(*s).bip_iter; (*bip).bip_flags=(*s).bip_flags&BIP_CLONE_FLAGS; (*bip).app_tag=(*s).app_tag; 0 }

unsafe fn bio_integrity_initfn() -> int {
    if mempool_init_page_pool(&mut integrity_buf_pool, BIO_POOL_SIZE, get_order(BLK_INTEGRITY_MAX_SIZE)) != 0 { panic!("bio: can't create integrity buf pool\n"); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
