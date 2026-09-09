// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2025 Christoph Hellwig.
 */

// Dependencies supplied by the surrounding kernel tree:
// linux/blk-integrity.h, linux/bio-integrity.h, and blk.h.

#[repr(C)]
pub struct FsBioIntegrityBuf {
    pub bip: bio_integrity_payload,
    pub bvec: bio_vec,
}

static mut fs_bio_integrity_cache: *mut kmem_cache = core::ptr::null_mut();
static mut fs_bio_integrity_pool: mempool_t = mempool_t::UNINIT;

pub unsafe fn fs_bio_integrity_alloc(bio: *mut bio) -> u32 {
    let iib: *mut FsBioIntegrityBuf;
    let action: u32;

    action = bio_integrity_action(bio);
    if action == 0 {
        return 0;
    }

    iib = mempool_alloc(&raw mut fs_bio_integrity_pool, GFP_NOFS) as *mut FsBioIntegrityBuf;
    bio_integrity_init(bio, &mut (*iib).bip, &mut (*iib).bvec, 1);

    bio_integrity_alloc_buf(bio, GFP_NOFS, action & BI_ACT_ZERO);
    if action & BI_ACT_CHECK != 0 {
        bio_integrity_setup_default(bio);
    }
    action
}

pub unsafe fn fs_bio_integrity_free(bio: *mut bio) {
    let bip: *mut bio_integrity_payload = bio_integrity(bio);

    bio_integrity_free_buf(bip);
    mempool_free(
        bip as *mut FsBioIntegrityBuf,
        &raw mut fs_bio_integrity_pool,
    );

    (*bio).bi_integrity = core::ptr::null_mut();
    (*bio).bi_opf &= !REQ_INTEGRITY;
}

pub unsafe fn fs_bio_integrity_generate(bio: *mut bio) {
    if fs_bio_integrity_alloc(bio) != 0
        && ((*bio).bi_integrity as *mut bio_integrity_payload).as_ref().unwrap().bip_flags
            & BIP_CHECK_FLAGS != 0
    {
        bio_integrity_generate(bio);
    }
}

// EXPORT_SYMBOL_GPL(fs_bio_integrity_generate);

pub unsafe fn fs_bio_integrity_verify(
    bio: *mut bio,
    sector: sector_t,
    size: u32,
) -> i32 {
    let bi: *mut blk_integrity = blk_get_integrity((*(*bio).bi_bdev).bd_disk);
    let bip: *mut bio_integrity_payload = bio_integrity(bio);
    let mut data_iter = bvec_iter {
        bi_sector: sector,
        bi_size: size,
    };

    if bip.is_null() || (*bip).bip_flags & BIP_CHECK_FLAGS == 0 {
        return 0;
    }

    /*
     * Reinitialize bip->bip_iter.
     *
     * This is for use in the submitter after the driver is done with the
     * bio.  Requires the submitter to remember the sector and the size.
     */
    core::ptr::write_bytes(
        &mut (*bip).bip_iter as *mut _,
        0,
        core::mem::size_of_val(&(*bip).bip_iter),
    );
    (*bip).bip_iter.bi_sector = sector;
    (*bip).bip_iter.bi_size = bio_integrity_bytes(bi, size >> SECTOR_SHIFT);
    blk_status_to_errno(bio_integrity_verify(bio, &mut data_iter))
}

unsafe fn fs_bio_integrity_init() -> i32 {
    fs_bio_integrity_cache = kmem_cache_create(
        c"fs_bio_integrity",
        core::mem::size_of::<FsBioIntegrityBuf>(),
        0,
        SLAB_HWCACHE_ALIGN | SLAB_PANIC,
        None,
    );
    if mempool_init_slab_pool(
        &raw mut fs_bio_integrity_pool,
        BIO_POOL_SIZE,
        fs_bio_integrity_cache,
    ) != 0
    {
        panic!("fs_bio_integrity: can't create pool\n");
    }
    0
}

// fs_initcall(fs_bio_integrity_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
