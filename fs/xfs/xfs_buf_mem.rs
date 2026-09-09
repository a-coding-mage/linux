// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2023-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Dependencies supplied by the corresponding kernel/XFS headers.

/*
 * Buffer Cache for In-Memory Files
 * ================================
 *
 * Online fsck wants to create ephemeral ordered recordsets.  The existing
 * btree infrastructure can do this, but we need the buffer cache to target
 * memory instead of block devices.
 *
 * When CONFIG_TMPFS=y, shmemfs is enough of a filesystem to meet those
 * requirements.  Therefore, the xmbuf mechanism uses an unlinked shmem file to
 * store our staging data.  This file is not installed in the file descriptor
 * table so that user programs cannot access the data, which means that the
 * xmbuf must be freed with xmbuf_destroy.
 *
 * xmbufs assume that the caller will handle all required concurrency
 * management; standard vfs locks (freezer and inode) are not taken.  Reads
 * and writes are satisfied directly from the page cache.
 *
 * The only supported block size is PAGE_SIZE, and we cannot use highmem.
 */

static mut xmbuf_i_mutex_key: lock_class_key = lock_class_key {};

pub unsafe fn xmbuf_alloc(
    mp: *mut xfs_mount,
    descr: *const c_char,
    btpp: *mut *mut xfs_buftarg,
) -> c_int {
    let mut file: *mut file;
    let mut inode: *mut inode;
    let btp: *mut xfs_buftarg = kzalloc_obj::<xfs_buftarg>();
    let mut error: c_int;

    if btp.is_null() {
        return -ENOMEM;
    }

    file = shmem_kernel_file_setup(descr, 0, EMPTY_VMA_FLAGS);
    if IS_ERR(file) {
        error = PTR_ERR(file);
        goto out_free_btp;
    }
    inode = file_inode(file);

    // private file, private locking
    lockdep_set_class(&mut (*inode).i_rwsem, &mut xmbuf_i_mutex_key);

    // We don't want to bother with kmapping data during repair, so don't allow
    // highmem folios to back this mapping.
    mapping_set_gfp_mask((*inode).i_mapping, GFP_KERNEL);

    // ensure all writes are below EOF to avoid pagecache zeroing
    i_size_write(inode, (*(*inode).i_sb).s_maxbytes);

    // Initialize buffer target
    (*btp).bt_mount = mp;
    (*btp).bt_dev = (-1i32) as dev_t;
    (*btp).bt_bdev = core::ptr::null_mut(); // in-memory buftargs have no bdev
    (*btp).bt_file = file;
    (*btp).bt_meta_sectorsize = XMBUF_BLOCKSIZE;
    (*btp).bt_meta_sectormask = XMBUF_BLOCKSIZE - 1;

    error = xfs_init_buftarg(btp, XMBUF_BLOCKSIZE, descr);
    if error != 0 {
        goto out_file;
    }

    trace_xmbuf_create(btp);
    *btpp = btp;
    return 0;

out_file:
    fput(file);
out_free_btp:
    kfree(btp);
    return error;
}

pub unsafe fn xmbuf_free(btp: *mut xfs_buftarg) {
    ASSERT(xfs_buftarg_is_mem(btp));
    ASSERT(percpu_counter_sum(&(*btp).bt_readahead_count) == 0);

    trace_xmbuf_free(btp);
    xfs_destroy_buftarg(btp);
    fput((*btp).bt_file);
    kfree(btp);
}

pub unsafe fn xmbuf_map_backing_mem(bp: *mut xfs_buf) -> c_int {
    let inode: *mut inode = file_inode((*(*bp).b_target).bt_file);
    let mut folio: *mut folio = core::ptr::null_mut();
    let pos: loff_t = BBTOB(xfs_buf_daddr(bp));
    let mut error: c_int;

    ASSERT(xfs_buftarg_is_mem((*bp).b_target));
    if (*bp).b_map_count != 1 || BBTOB((*bp).b_length) != XMBUF_BLOCKSIZE {
        return -ENOMEM;
    }
    if offset_in_page(pos) != 0 {
        ASSERT(offset_in_page(pos));
        return -ENOMEM;
    }

    error = shmem_get_folio(inode, pos >> PAGE_SHIFT, 0, &mut folio, SGP_CACHE);
    if error != 0 {
        return error;
    }
    if filemap_check_wb_err((*inode).i_mapping, 0) {
        folio_unlock(folio);
        folio_put(folio);
        return -EIO;
    }
    // Mark the folio dirty so that it won't be reclaimed once we drop the
    // (potentially last) reference in xfs_buf_free.
    folio_set_dirty(folio);
    folio_unlock(folio);
    (*bp).b_addr = folio_address(folio).add(offset_in_folio(folio, pos) as usize);
    0
}

pub unsafe fn xmbuf_verify_daddr(btp: *mut xfs_buftarg, daddr: xfs_daddr_t) -> bool {
    let inode: *mut inode = file_inode((*btp).bt_file);
    ASSERT(xfs_buftarg_is_mem(btp));
    daddr < ((*(*inode).i_sb).s_maxbytes >> BBSHIFT)
}

unsafe fn xmbuf_stale(bp: *mut xfs_buf) {
    let inode = file_inode((*(*bp).b_target).bt_file);
    let pos = BBTOB(xfs_buf_daddr(bp));
    ASSERT(xfs_buftarg_is_mem((*bp).b_target));
    shmem_truncate_range(inode, pos, pos + BBTOB((*bp).b_length) - 1);
}

pub unsafe fn xmbuf_finalize(bp: *mut xfs_buf) -> c_int {
    if (*bp).b_flags & XBF_STALE != 0 {
        xmbuf_stale(bp);
        return 0;
    }
    let fa: xfs_failaddr_t = ((*(*bp).b_ops).verify_struct)(bp);
    if !fa.is_null() {
        let error = -EFSCORRUPTED;
        xfs_verifier_error(bp, error, fa);
        return error;
    }
    0
}

pub unsafe fn xmbuf_trans_bdetach(tp: *mut xfs_trans, bp: *mut xfs_buf) {
    let bli: *mut xfs_buf_log_item = (*bp).b_log_item;
    ASSERT(!bli.is_null());
    (*bli).bli_flags &= !(XFS_BLI_DIRTY | XFS_BLI_ORDERED | XFS_BLI_LOGGED | XFS_BLI_STALE);
    clear_bit(XFS_LI_DIRTY, &mut (*bli).bli_item.li_flags);
    while !(*bp).b_log_item.is_null() {
        xfs_trans_bdetach(tp, bp);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
