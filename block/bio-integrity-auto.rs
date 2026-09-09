// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2007, 2008, 2009 Oracle Corporation
 * Written by: Martin K. Petersen <martin.petersen@oracle.com>
 *
 * Automatically generate and verify integrity data on PI capable devices if the
 * bio submitter didn't provide PI itself.  This ensures that kernel verifies
 * data integrity even if the file system (or other user of the block device) is
 * not aware of PI.
 */
// Dependencies supplied by the Linux block-integrity, T10 PI, workqueue, and
// local block-layer headers are intentionally left as external Rust symbols.

#[repr(C)]
struct bio_integrity_data {
    bio: *mut bio,
    saved_bio_iter: bvec_iter,
    work: work_struct,
    bip: bio_integrity_payload,
    bvec: bio_vec,
}

static mut bid_slab: *mut kmem_cache = core::ptr::null_mut();
static mut bid_pool: mempool_t = unsafe { core::mem::zeroed() };
static mut kintegrityd_wq: *mut workqueue_struct = core::ptr::null_mut();

unsafe fn bio_integrity_finish(bid: *mut bio_integrity_data) {
    (*(*bid).bio).bi_integrity = core::ptr::null_mut();
    (*(*bid).bio).bi_opf &= !REQ_INTEGRITY;
    bio_integrity_free_buf(&mut (*bid).bip);
    mempool_free(bid, &raw mut bid_pool);
}

unsafe extern "C" fn bio_integrity_verify_fn(work: *mut work_struct) {
    let bid = container_of!(work, bio_integrity_data, work);
    let bio = (*bid).bio;

    (*bio).bi_status = bio_integrity_verify(bio, &mut (*bid).saved_bio_iter);
    bio_integrity_finish(bid);
    bio_endio(bio);
}

/**
 * __bio_integrity_endio - Integrity I/O completion function
 * @bio:       Protected bio
 *
 * Normally I/O completion is done in interrupt context.  However, verifying I/O
 * integrity is a time-consuming task which must be run in process context.
 *
 * This function postpones completion accordingly.
 */
unsafe extern "C" fn __bio_integrity_endio(bio: *mut bio) -> bool {
    let bip = bio_integrity(bio);
    let bid = container_of!(bip, bio_integrity_data, bip);

    if bio_op(bio) == REQ_OP_READ && (*bio).bi_status == 0
        && ((*bip).bip_flags & BIP_CHECK_FLAGS) != 0
    {
        INIT_WORK(&mut (*bid).work, bio_integrity_verify_fn);
        queue_work(kintegrityd_wq, &mut (*bid).work);
        return false;
    }

    bio_integrity_finish(bid);
    true
}

/**
 * bio_integrity_prep - Prepare bio for integrity I/O
 * @bio:      bio to prepare
 * @action:   preparation action needed (BI_ACT_*)
 *
 * Allocate the integrity payload.  For writes, generate the integrity metadata
 * and for reads, setup the completion handler to verify the metadata.
 *
 * This is used for bios that do not have user integrity payloads attached.
 */
unsafe extern "C" fn bio_integrity_prep(bio: *mut bio, action: c_uint) {
    let bid: *mut bio_integrity_data = mempool_alloc(&raw mut bid_pool, GFP_NOIO);
    bio_integrity_init(bio, &mut (*bid).bip, &mut (*bid).bvec, 1);
    (*bid).bio = bio;
    (*bid).bip.bip_flags |= BIP_BLOCK_INTEGRITY;
    bio_integrity_alloc_buf(bio, GFP_NOIO, action & BI_ACT_ZERO);
    if action & BI_ACT_CHECK != 0 {
        bio_integrity_setup_default(bio);
    }

    // Auto-generate integrity metadata if this is a write
    if bio_data_dir(bio) == WRITE && ((*bid).bip.bip_flags & BIP_CHECK_FLAGS) != 0 {
        bio_integrity_generate(bio);
    } else {
        (*bid).saved_bio_iter = (*bio).bi_iter;
    }
}

unsafe extern "C" fn blk_flush_integrity() {
    flush_workqueue(kintegrityd_wq);
}

unsafe extern "C" fn blk_integrity_auto_init() -> c_int {
    bid_slab = kmem_cache_create(
        c"bio_integrity_data".as_ptr(),
        core::mem::size_of::<bio_integrity_data>(),
        0,
        SLAB_HWCACHE_ALIGN | SLAB_PANIC,
        None,
    );

    if mempool_init_slab_pool(&raw mut bid_pool, BIO_POOL_SIZE, bid_slab) != 0 {
        panic!("bio: can't create integrity pool\n");
    }

    // kintegrityd won't block much but may burn a lot of CPU cycles.
    // Make it highpri CPU intensive wq with max concurrency of 1.
    kintegrityd_wq = alloc_workqueue(
        c"kintegrityd".as_ptr(),
        WQ_MEM_RECLAIM | WQ_HIGHPRI | WQ_CPU_INTENSIVE | WQ_PERCPU,
        1,
    );
    if kintegrityd_wq.is_null() {
        panic!("Failed to create kintegrityd\n");
    }
    0
}

// Equivalent to EXPORT_SYMBOL(bio_integrity_prep);
// Equivalent to subsys_initcall(blk_integrity_auto_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
