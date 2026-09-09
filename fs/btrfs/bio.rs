// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2007 Oracle.  All rights reserved.
 * Copyright (C) 2022 Christoph Hellwig.
 */

// Linux dependencies: blk_types.h, bio.h, bio.h, ctree.h, volumes.h,
// raid56.h, async-thread.h, dev-replace.h, zoned.h, file-item.h,
// raid-stripe-tree.h

static mut btrfs_bioset: bio_set = bio_set { };
static mut btrfs_clone_bioset: bio_set = bio_set { };
static mut btrfs_repair_bioset: bio_set = bio_set { };
static mut btrfs_failed_bio_pool: mempool_t = mempool_t { };

#[repr(C)]
struct btrfs_failed_bio {
    bbio: *mut btrfs_bio,
    num_copies: c_int,
    repair_count: atomic_t,
}

/* Is this a data path I/O that needs storage layer checksum and repair? */
#[inline]
unsafe fn is_data_bbio(bbio: *const btrfs_bio) -> bool {
    !(*bbio).inode.is_null() && is_data_inode((*bbio).inode)
}

unsafe fn bbio_has_ordered_extent(bbio: *const btrfs_bio) -> bool {
    is_data_bbio(bbio) && btrfs_op(&(*bbio).bio) == BTRFS_MAP_WRITE
}

/* Initialize a btrfs_bio structure.  This skips the embedded bio itself as it
 * is already initialized by the block layer. */
unsafe fn btrfs_bio_init(bbio: *mut btrfs_bio, inode: *mut btrfs_inode,
                         file_offset: u64, end_io: btrfs_bio_end_io_t,
                         private: *mut c_void) {
    ASSERT(!inode.is_null());
    memset(bbio as *mut c_void, 0, offset_of!(btrfs_bio, bio));
    (*bbio).inode = inode;
    (*bbio).end_io = end_io;
    (*bbio).private = private;
    (*bbio).file_offset = file_offset;
    atomic_set(&mut (*bbio).pending_ios, 1);
    WRITE_ONCE(&mut (*bbio).status, BLK_STS_OK);
}

unsafe fn btrfs_bio_alloc(nr_vecs: c_uint, opf: blk_opf_t, inode: *mut btrfs_inode,
                          file_offset: u64, end_io: btrfs_bio_end_io_t,
                          private: *mut c_void) -> *mut btrfs_bio {
    let bio = bio_alloc_bioset(core::ptr::null_mut(), nr_vecs, opf, GFP_NOFS,
                               &mut btrfs_bioset);
    let bbio = btrfs_bio(bio);
    btrfs_bio_init(bbio, inode, file_offset, end_io, private);
    bbio
}

unsafe fn btrfs_split_bio(fs_info: *mut btrfs_fs_info, orig_bbio: *mut btrfs_bio,
                          map_length: u64) -> *mut btrfs_bio {
    let bio = bio_split(&mut (*orig_bbio).bio, map_length >> SECTOR_SHIFT,
                        GFP_NOFS, &mut btrfs_clone_bioset);
    if IS_ERR(bio) { return ERR_CAST(bio); }
    let bbio = btrfs_bio(bio);
    btrfs_bio_init(bbio, (*orig_bbio).inode, (*orig_bbio).file_offset,
                   None, orig_bbio as *mut c_void);
    (*orig_bbio).file_offset += map_length;
    if bbio_has_ordered_extent(bbio) {
        refcount_inc(&mut (*(*orig_bbio).ordered).refs);
        (*bbio).ordered = (*orig_bbio).ordered;
        (*bbio).orig_logical = (*orig_bbio).orig_logical;
        (*orig_bbio).orig_logical += map_length;
    }
    (*bbio).csum_search_commit_root = (*orig_bbio).csum_search_commit_root;
    (*bbio).can_use_append = (*orig_bbio).can_use_append;
    (*bbio).is_scrub = (*orig_bbio).is_scrub;
    (*bbio).is_remap = (*orig_bbio).is_remap;
    (*bbio).async_csum = (*orig_bbio).async_csum;
    atomic_inc(&mut (*orig_bbio).pending_ios);
    bbio
}

unsafe fn btrfs_bio_end_io(bbio: *mut btrfs_bio, status: blk_status_t) {
    ASSERT(in_task());
    if (*bbio).async_csum { wait_for_completion(&mut (*bbio).csum_done); }
    (*bbio).bio.bi_status = status;
    if (*bbio).bio.bi_pool == &mut btrfs_clone_bioset {
        let orig_bbio = (*bbio).private as *mut btrfs_bio;
        if bbio_has_ordered_extent(bbio) { btrfs_put_ordered_extent((*bbio).ordered); }
        bio_put(&mut (*bbio).bio);
        bbio = orig_bbio;
    }
    if status != BLK_STS_OK { cmpxchg(&mut (*bbio).status, BLK_STS_OK, status); }
    if atomic_dec_and_test(&mut (*bbio).pending_ios) {
        if status == BLK_STS_OK { (*bbio).bio.bi_status = READ_ONCE((*bbio).status); }
        if bbio_has_ordered_extent(bbio) {
            let ordered = (*bbio).ordered;
            ((*bbio).end_io)(bbio);
            btrfs_put_ordered_extent(ordered);
        } else { ((*bbio).end_io)(bbio); }
    }
}

unsafe fn next_repair_mirror(fbio: *const btrfs_failed_bio, cur_mirror: c_int) -> c_int {
    if cur_mirror == (*fbio).num_copies { cur_mirror + 1 - (*fbio).num_copies } else { cur_mirror + 1 }
}
unsafe fn prev_repair_mirror(fbio: *const btrfs_failed_bio, cur_mirror: c_int) -> c_int {
    if cur_mirror == 1 { (*fbio).num_copies } else { cur_mirror - 1 }
}

unsafe fn btrfs_repair_done(fbio: *mut btrfs_failed_bio) {
    if atomic_dec_and_test(&mut (*fbio).repair_count) {
        btrfs_bio_end_io((*fbio).bbio, (*(*fbio).bbio).bio.bi_status);
        mempool_free(fbio as *mut c_void, &mut btrfs_failed_bio_pool);
    }
}

unsafe fn btrfs_end_repair_bio(repair_bbio: *mut btrfs_bio, dev: *mut btrfs_device) {
    let fbio = (*repair_bbio).private as *mut btrfs_failed_bio;
    let inode = (*repair_bbio).inode;
    let fs_info = (*(*inode).root).fs_info;
    let mut saved_iter = (*repair_bbio).saved_iter;
    let step = min((*fs_info).sectorsize, PAGE_SIZE);
    let logical = saved_iter.bi_sector << SECTOR_SHIFT;
    let nr_steps = saved_iter.bi_size / step;
    let mut mirror = (*repair_bbio).mirror_num;
    let mut paddrs: [phys_addr_t; BTRFS_MAX_BLOCKSIZE / PAGE_SIZE] = [0; BTRFS_MAX_BLOCKSIZE / PAGE_SIZE];
    let mut slot = 0;
    ASSERT(saved_iter.bi_size == (*fs_info).sectorsize);
    btrfs_bio_for_each_block(paddr, &mut (*repair_bbio).bio, &mut saved_iter, step) {
        ASSERT(slot < nr_steps); paddrs[slot] = paddr; slot += 1;
    }
    if (*repair_bbio).bio.bi_status != 0 || !btrfs_data_csum_ok(repair_bbio, dev, 0, paddrs.as_mut_ptr()) {
        bio_reset(&mut (*repair_bbio).bio, core::ptr::null_mut(), REQ_OP_READ);
        (*repair_bbio).bio.bi_iter = (*repair_bbio).saved_iter;
        mirror = next_repair_mirror(fbio, mirror);
        if mirror == (*(*fbio).bbio).mirror_num { btrfs_debug(fs_info, "no mirror left"); (*(*fbio).bbio).bio.bi_status = BLK_STS_IOERR; }
        else { btrfs_submit_bbio(repair_bbio, mirror); return; }
    } else {
        loop {
            mirror = prev_repair_mirror(fbio, mirror);
            btrfs_repair_io_failure(fs_info, btrfs_ino(inode), (*repair_bbio).file_offset,
                                    (*fs_info).sectorsize, logical, paddrs.as_ptr(), step, mirror);
            if mirror == (*fbio).bbio.as_ref().unwrap().mirror_num { break; }
        }
    }
    btrfs_repair_done(fbio); bio_put(&mut (*repair_bbio).bio);
}

unsafe fn repair_one_sector(failed_bbio: *mut btrfs_bio, bio_offset: u32,
                            paddrs: *mut phys_addr_t, mut fbio: *mut btrfs_failed_bio) -> *mut btrfs_failed_bio {
    let inode = (*failed_bbio).inode; let fs_info = (*(*inode).root).fs_info;
    let sectorsize = (*fs_info).sectorsize; let step = min(sectorsize, PAGE_SIZE); let nr_steps = sectorsize / step;
    let logical = round_down((*failed_bbio).saved_iter.bi_sector << SECTOR_SHIFT, sectorsize);
    let num_copies = btrfs_num_copies(fs_info, logical, sectorsize);
    if num_copies == 1 { (*failed_bbio).bio.bi_status = BLK_STS_IOERR; return fbio; }
    if fbio.is_null() { fbio = mempool_alloc(&mut btrfs_failed_bio_pool, GFP_NOFS) as *mut btrfs_failed_bio; (*fbio).bbio = failed_bbio; (*fbio).num_copies = num_copies; atomic_set(&mut (*fbio).repair_count, 1); }
    atomic_inc(&mut (*fbio).repair_count);
    let repair_bio = bio_alloc_bioset(core::ptr::null_mut(), nr_steps, REQ_OP_READ, GFP_NOFS, &mut btrfs_repair_bioset);
    (*repair_bio).bi_iter.bi_sector = logical >> SECTOR_SHIFT;
    for i in 0..nr_steps { ASSERT(offset_in_page(*paddrs.add(i)) + step <= PAGE_SIZE); ASSERT(bio_add_page(repair_bio, phys_to_page(*paddrs.add(i)), step, offset_in_page(*paddrs.add(i))) == step); }
    let repair_bbio = btrfs_bio(repair_bio);
    btrfs_bio_init(repair_bbio, (*failed_bbio).inode, (*failed_bbio).file_offset + bio_offset as u64, None, fbio as *mut c_void);
    let mirror = next_repair_mirror(fbio, (*failed_bbio).mirror_num); btrfs_submit_bbio(repair_bbio, mirror); fbio
}

unsafe fn btrfs_check_read_bio(bbio: *mut btrfs_bio, dev: *mut btrfs_device) {
    let inode = (*bbio).inode; let fs_info = (*(*inode).root).fs_info; let sectorsize = (*fs_info).sectorsize;
    let step = min(sectorsize, PAGE_SIZE); let nr_steps = sectorsize / step; let mut iter = &mut (*bbio).saved_iter;
    let status = (*bbio).bio.bi_status; let mut fbio: *mut btrfs_failed_bio = core::ptr::null_mut();
    let mut paddrs: [phys_addr_t; BTRFS_MAX_BLOCKSIZE / PAGE_SIZE] = [0; BTRFS_MAX_BLOCKSIZE / PAGE_SIZE]; let mut offset = 0;
    ASSERT(!inode.is_null());
    if (*bbio).bio.bi_pool == &mut btrfs_repair_bioset { btrfs_end_repair_bio(bbio, dev); return; }
    (*bbio).bio.bi_status = BLK_STS_OK;
    btrfs_bio_for_each_block(paddr, &mut (*bbio).bio, iter, step) { paddrs[(offset / step) % nr_steps] = paddr; offset += step; if IS_ALIGNED(offset, sectorsize) && (status != 0 || !btrfs_data_csum_ok(bbio, dev, offset - sectorsize, paddrs.as_mut_ptr())) { fbio = repair_one_sector(bbio, offset - sectorsize, paddrs.as_mut_ptr(), fbio); } }
    if (*bbio).csum != (*bbio).csum_inline { kvfree((*bbio).csum as *mut c_void); }
    if !fbio.is_null() { btrfs_repair_done(fbio); } else { btrfs_bio_end_io(bbio, (*bbio).bio.bi_status); }
}

unsafe fn btrfs_log_dev_io_error(bio: *const bio, dev: *mut btrfs_device) {
    let sts = (*bio).bi_status; if dev.is_null() || (*dev).bdev.is_null() || sts == BLK_STS_OK { return; }
    if sts != BLK_STS_IOERR && sts != BLK_STS_TARGET && sts != BLK_STS_MEDIUM && sts != BLK_STS_PROTECTION { btrfs_warn_rl((*dev).fs_info, "bdev unexpected block io error"); return; }
    if btrfs_op(bio) == BTRFS_MAP_WRITE { btrfs_dev_stat_inc_and_print(dev, BTRFS_DEV_STAT_WRITE_ERRS); } else if (*bio).bi_opf & REQ_RAHEAD == 0 { btrfs_dev_stat_inc_and_print(dev, BTRFS_DEV_STAT_READ_ERRS); }
    if (*bio).bi_opf & REQ_PREFLUSH != 0 { btrfs_dev_stat_inc_and_print(dev, BTRFS_DEV_STAT_FLUSH_ERRS); }
}

unsafe fn btrfs_submit_bbio(bbio: *mut btrfs_bio, mirror_num: c_int) { ASSERT(!(*bbio).inode.is_null() || (*bbio).file_offset == 0); assert_bbio_alignment(bbio); while !btrfs_submit_chunk(bbio, mirror_num) {} }

unsafe fn assert_bbio_alignment(_bbio: *mut btrfs_bio) { }

unsafe fn btrfs_repair_io_failure(_fs_info: *mut btrfs_fs_info, _ino: u64, _fileoff: u64, _length: u32, _logical: u64, _paddrs: *const phys_addr_t, _step: c_uint, _mirror_num: c_int) -> c_int { 0 }

unsafe fn btrfs_submit_repair_write(bbio: *mut btrfs_bio, mirror_num: c_int, _dev_replace: bool) { btrfs_submit_bbio(bbio, mirror_num); }

unsafe fn btrfs_bioset_init() -> c_int { 0 }
unsafe fn btrfs_bioset_exit() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
