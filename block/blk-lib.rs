// SPDX-License-Identifier: GPL-2.0
/*
 * Functions related to generic helpers functions
 */

// C dependencies from linux/kernel.h, linux/module.h, linux/bio.h,
// linux/blkdev.h, linux/scatterlist.h, and "blk.h" are supplied externally.

unsafe fn bio_discard_limit(bdev: *mut block_device, mut sector: sector_t) -> sector_t {
    let discard_granularity: u32 = bdev_discard_granularity(bdev);
    let granularity_aligned_sector: sector_t;

    if bdev_is_partition(bdev) {
        sector = sector.wrapping_add((*bdev).bd_start_sect);
    }

    granularity_aligned_sector = round_up(sector, (discard_granularity >> SECTOR_SHIFT) as sector_t);

    /*
     * Make sure subsequent bios start aligned to the discard granularity if
     * it needs to be split.
     */
    if granularity_aligned_sector != sector {
        return granularity_aligned_sector.wrapping_sub(sector);
    }

    /*
     * Align the bio size to the discard granularity to make splitting the bio
     * at discard granularity boundaries easier in the driver if needed.
     */
    round_down(BIO_MAX_SIZE, discard_granularity as usize) >> SECTOR_SHIFT
}

pub unsafe fn blk_alloc_discard_bio(
    bdev: *mut block_device,
    sector: *mut sector_t,
    nr_sects: *mut sector_t,
    gfp_mask: gfp_t,
) -> *mut bio {
    let bio_sects = min(*nr_sects, bio_discard_limit(bdev, *sector));
    let bio: *mut bio;

    if bio_sects == 0 { return core::ptr::null_mut(); }
    bio = bio_alloc(bdev, 0, REQ_OP_DISCARD, gfp_mask);
    if bio.is_null() { return core::ptr::null_mut(); }
    (*bio).bi_iter.bi_sector = *sector;
    (*bio).bi_iter.bi_size = bio_sects << SECTOR_SHIFT;
    *sector = (*sector).wrapping_add(bio_sects);
    *nr_sects = (*nr_sects).wrapping_sub(bio_sects);
    cond_resched();
    bio
}

pub unsafe fn __blkdev_issue_discard(
    bdev: *mut block_device, mut sector: sector_t, mut nr_sects: sector_t,
    gfp_mask: gfp_t, biop: *mut *mut bio,
) {
    let mut bio: *mut bio;
    loop {
        bio = blk_alloc_discard_bio(bdev, &mut sector, &mut nr_sects, gfp_mask);
        if bio.is_null() { break; }
        *biop = bio_chain_and_submit(*biop, bio);
    }
}

pub unsafe fn blkdev_issue_discard(
    bdev: *mut block_device, sector: sector_t, nr_sects: sector_t, gfp_mask: gfp_t,
) -> i32 {
    let mut bio: *mut bio = core::ptr::null_mut();
    let mut plug: blk_plug = core::mem::zeroed();
    let mut ret = 0;
    blk_start_plug(&mut plug);
    __blkdev_issue_discard(bdev, sector, nr_sects, gfp_mask, &mut bio);
    if !bio.is_null() {
        ret = submit_bio_wait(bio);
        if ret == -EOPNOTSUPP { ret = 0; }
        bio_put(bio);
    }
    blk_finish_plug(&mut plug);
    ret
}

unsafe fn bio_write_zeroes_limit(bdev: *mut block_device) -> sector_t {
    let bs_mask = (bdev_logical_block_size(bdev) >> 9) - 1;
    min(bdev_write_zeroes_sectors(bdev), BIO_MAX_SECTORS & !bs_mask)
}

unsafe fn __blkdev_issue_write_zeroes(
    bdev: *mut block_device, mut sector: sector_t, mut nr_sects: sector_t,
    gfp_mask: gfp_t, biop: *mut *mut bio, flags: u32, limit: sector_t,
) {
    while nr_sects != 0 {
        let len = min(nr_sects, limit);
        if (flags & BLKDEV_ZERO_KILLABLE) != 0 && fatal_signal_pending(current) { break; }
        let bio = bio_alloc(bdev, 0, REQ_OP_WRITE_ZEROES, gfp_mask);
        (*bio).bi_iter.bi_sector = sector;
        if (flags & BLKDEV_ZERO_NOUNMAP) != 0 { (*bio).bi_opf |= REQ_NOUNMAP; }
        (*bio).bi_iter.bi_size = len << SECTOR_SHIFT;
        *biop = bio_chain_and_submit(*biop, bio);
        nr_sects -= len;
        sector += len;
        cond_resched();
    }
}

unsafe fn blkdev_issue_write_zeroes(bdev: *mut block_device, sector: sector_t,
    nr_sects: sector_t, gfp: gfp_t, flags: u32) -> i32 {
    let limit = bio_write_zeroes_limit(bdev);
    let mut bio: *mut bio = core::ptr::null_mut();
    let mut plug: blk_plug = core::mem::zeroed();
    let mut ret = 0;
    blk_start_plug(&mut plug);
    __blkdev_issue_write_zeroes(bdev, sector, nr_sects, gfp, &mut bio, flags, limit);
    if !bio.is_null() { ret = bio_submit_or_kill(bio, flags); bio_put(bio); }
    blk_finish_plug(&mut plug);
    if ret != 0 && bdev_write_zeroes_sectors(bdev) == 0 { return -EOPNOTSUPP; }
    ret
}

unsafe fn __blkdev_sectors_to_bio_pages(nr_sects: sector_t) -> u32 {
    let pages = DIV_ROUND_UP_SECTOR_T(nr_sects, PAGE_SIZE / 512);
    min(pages, BIO_MAX_VECS as sector_t) as u32
}

unsafe fn __blkdev_issue_zero_pages(bdev: *mut block_device, mut sector: sector_t,
    mut nr_sects: sector_t, gfp_mask: gfp_t, biop: *mut *mut bio, flags: u32) {
    let zero_folio = largest_zero_folio();
    while nr_sects != 0 {
        let nr_vecs = __blkdev_sectors_to_bio_pages(nr_sects);
        if (flags & BLKDEV_ZERO_KILLABLE) != 0 && fatal_signal_pending(current) { break; }
        let bio = bio_alloc(bdev, nr_vecs, REQ_OP_WRITE, gfp_mask);
        (*bio).bi_iter.bi_sector = sector;
        loop {
            let len = min(folio_size(zero_folio), nr_sects << SECTOR_SHIFT);
            if bio_add_folio(bio, zero_folio, len, 0) == 0 { break; }
            nr_sects -= len >> SECTOR_SHIFT;
            sector += len >> SECTOR_SHIFT;
            if nr_sects == 0 { break; }
        }
        *biop = bio_chain_and_submit(*biop, bio);
        cond_resched();
    }
}

unsafe fn blkdev_issue_zero_pages(bdev: *mut block_device, sector: sector_t,
    nr_sects: sector_t, gfp: gfp_t, flags: u32) -> i32 {
    if flags & BLKDEV_ZERO_NOFALLBACK != 0 { return -EOPNOTSUPP; }
    let mut bio: *mut bio = core::ptr::null_mut();
    let mut plug: blk_plug = core::mem::zeroed();
    let mut ret = 0;
    blk_start_plug(&mut plug);
    __blkdev_issue_zero_pages(bdev, sector, nr_sects, gfp, &mut bio, flags);
    if !bio.is_null() { ret = bio_submit_or_kill(bio, flags); bio_put(bio); }
    blk_finish_plug(&mut plug);
    ret
}

pub unsafe fn __blkdev_issue_zeroout(bdev: *mut block_device, sector: sector_t,
    nr_sects: sector_t, gfp_mask: gfp_t, biop: *mut *mut bio, flags: u32) -> i32 {
    let limit = bio_write_zeroes_limit(bdev);
    if bdev_read_only(bdev) { return -EPERM; }
    if limit != 0 { __blkdev_issue_write_zeroes(bdev, sector, nr_sects, gfp_mask, biop, flags, limit); }
    else { if flags & BLKDEV_ZERO_NOFALLBACK != 0 { return -EOPNOTSUPP; } __blkdev_issue_zero_pages(bdev, sector, nr_sects, gfp_mask, biop, flags); }
    0
}

pub unsafe fn blkdev_issue_zeroout(bdev: *mut block_device, sector: sector_t,
    nr_sects: sector_t, gfp_mask: gfp_t, flags: u32) -> i32 {
    let mask = (bdev_logical_block_size(bdev) >> 9) - 1;
    if (sector | nr_sects) & mask != 0 { return -EINVAL; }
    if bdev_read_only(bdev) { return -EPERM; }
    if bdev_write_zeroes_sectors(bdev) != 0 {
        let ret = blkdev_issue_write_zeroes(bdev, sector, nr_sects, gfp_mask, flags);
        if ret != -EOPNOTSUPP { return ret; }
    }
    blkdev_issue_zero_pages(bdev, sector, nr_sects, gfp_mask, flags)
}

pub unsafe fn blkdev_issue_secure_erase(bdev: *mut block_device, mut sector: sector_t,
    mut nr_sects: sector_t, gfp: gfp_t) -> i32 {
    let bs_mask = (bdev_logical_block_size(bdev) >> 9) - 1;
    let mut max_sectors = bdev_max_secure_erase_sectors(bdev);
    let mut bio: *mut bio = core::ptr::null_mut();
    let mut plug: blk_plug = core::mem::zeroed();
    let mut ret = 0;
    if max_sectors > BIO_MAX_SECTORS { max_sectors = BIO_MAX_SECTORS; }
    max_sectors &= !bs_mask;
    if max_sectors == 0 { return -EOPNOTSUPP; }
    if (sector | nr_sects) & bs_mask != 0 { return -EINVAL; }
    if bdev_read_only(bdev) { return -EPERM; }
    blk_start_plug(&mut plug);
    while nr_sects != 0 {
        let len = min(nr_sects, max_sectors);
        bio = blk_next_bio(bio, bdev, 0, REQ_OP_SECURE_ERASE, gfp);
        (*bio).bi_iter.bi_sector = sector;
        (*bio).bi_iter.bi_size = len << SECTOR_SHIFT;
        sector += len;
        nr_sects -= len;
        cond_resched();
    }
    if !bio.is_null() { ret = submit_bio_wait(bio); bio_put(bio); }
    blk_finish_plug(&mut plug);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
