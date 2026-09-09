// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 Christoph Hellwig.
 */

// Types, constants, and functions referenced below are supplied by the
// corresponding platform and kernel interfaces.

use core::ffi::c_char;

#[repr(C)]
pub struct block_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bio {
    pub bi_iter: bio_iter,
    pub bi_bdev: *mut block_device,
    pub bi_opf: u32,
}

#[repr(C)]
pub struct bio_iter {
    pub bi_sector: u64,
}

pub type sector_t = u64;
pub type req_op = u32;

unsafe extern "C" {
    fn bio_max_segs(nr_pages: u32) -> u32;
    fn howmany(count: u32, divisor: u32) -> u32;
    fn is_vmalloc_addr(addr: *const core::ffi::c_void) -> bool;
    fn bdev_rw_virt(
        bdev: *mut block_device,
        sector: sector_t,
        data: *mut c_char,
        count: u32,
        op: req_op,
    ) -> i32;
    fn bio_alloc(
        bdev: *mut block_device,
        nr_vecs: u32,
        op: req_op,
        gfp_mask: u32,
    ) -> *mut bio;
    fn bio_add_vmalloc_chunk(
        bio: *mut bio,
        data: *mut c_char,
        count: u32,
    ) -> u32;
    fn bio_end_sector(bio: *mut bio) -> sector_t;
    fn bio_chain(prev: *mut bio, next: *mut bio);
    fn submit_bio(bio: *mut bio);
    fn submit_bio_wait(bio: *mut bio) -> i32;
    fn bio_put(bio: *mut bio);
    fn invalidate_kernel_vmap_range(data: *mut c_char, count: u32);
}

const PAGE_SIZE: u32 = 4096;
const GFP_KERNEL: u32 = 0;
const REQ_META: req_op = 1 << 16;
const REQ_SYNC: req_op = 1 << 17;
pub const REQ_OP_READ: req_op = 0;

#[inline]
unsafe fn bio_max_vecs(count: u32) -> u32 {
    bio_max_segs(howmany(count, PAGE_SIZE))
}

pub unsafe fn xfs_rw_bdev(
    bdev: *mut block_device,
    sector: sector_t,
    count: u32,
    data: *mut c_char,
    mut op: req_op,
) -> i32 {
    let mut done: u32 = 0;
    let mut added: u32;
    let error: i32;
    let mut bio: *mut bio;

    op |= REQ_META | REQ_SYNC;
    if !is_vmalloc_addr(data.cast()) {
        return bdev_rw_virt(bdev, sector, data, count, op);
    }

    bio = bio_alloc(bdev, bio_max_vecs(count), op, GFP_KERNEL);
    (*bio).bi_iter.bi_sector = sector;

    loop {
        added = bio_add_vmalloc_chunk(bio, data.add(done as usize), count - done);
        if added == 0 {
            let prev: *mut bio = bio;

            bio = bio_alloc(
                (*prev).bi_bdev,
                bio_max_vecs(count - done),
                (*prev).bi_opf,
                GFP_KERNEL,
            );
            (*bio).bi_iter.bi_sector = bio_end_sector(prev);
            bio_chain(prev, bio);
            submit_bio(prev);
        }
        done += added;
        if done >= count {
            break;
        }
    }

    error = submit_bio_wait(bio);
    bio_put(bio);

    if op == REQ_OP_READ {
        invalidate_kernel_vmap_range(data, count);
    }
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
