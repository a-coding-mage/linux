// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of linux/mm/page_io.c. Kernel dependencies are external. */

pub unsafe fn generic_swapfile_activate(sis: *mut swap_info_struct, swap_file: *mut file, span: *mut sector_t) -> i32 {
    let mapping = (*swap_file).f_mapping;
    let inode = (*mapping).host;
    let blocks_per_page: u32;
    let mut page_no: c_ulong = 0;
    let blkbits = (*inode).i_blkbits;
    let mut probe_block: sector_t = 0;
    let last_block = i_size_read(inode) >> blkbits;
    let mut lowest_block: sector_t = !0;
    let mut highest_block: sector_t = 0;
    let mut nr_extents = 0;
    let mut ret;

    blocks_per_page = (PAGE_SIZE >> blkbits) as u32;
    while probe_block + blocks_per_page as u64 <= last_block && page_no < (*sis).max {
        cond_resched();
        let mut first_block = probe_block;
        ret = bmap(inode, &mut first_block);
        if ret != 0 || first_block == 0 { break 'bad_bmap; }
        if first_block & (blocks_per_page as u64 - 1) != 0 {
            probe_block += 1;
            continue;
        }
        let mut discontinuous = false;
        for block_in_page in 1..blocks_per_page {
            let mut block = probe_block + block_in_page as u64;
            ret = bmap(inode, &mut block);
            if ret != 0 || block == 0 { break 'bad_bmap; }
            if block != first_block + block_in_page as u64 {
                probe_block += 1;
                discontinuous = true;
                break;
            }
        }
        if discontinuous { continue; }
        first_block >>= PAGE_SHIFT - blkbits;
        if page_no != 0 {
            if first_block < lowest_block { lowest_block = first_block; }
            if first_block > highest_block { highest_block = first_block; }
        }
        ret = add_swap_extent(sis, page_no, 1, first_block);
        if ret < 0 { break; }
        nr_extents += ret;
        page_no += 1;
        probe_block += blocks_per_page as u64;
    }
    ret = nr_extents;
    *span = 1 + highest_block - lowest_block;
    if page_no == 0 { page_no = 1; }
    (*sis).max = page_no;
    (*sis).pages = page_no - 1;
    return ret;
    'bad_bmap: {
        pr_err!("swapon: swapfile has holes\n");
        ret = -EINVAL;
        *span = 1 + highest_block - lowest_block;
        if page_no == 0 { page_no = 1; }
        (*sis).max = page_no;
        (*sis).pages = page_no - 1;
        ret
    }
}

unsafe fn is_folio_zero_filled(folio: *mut folio) -> bool {
    let last_pos = PAGE_SIZE / core::mem::size_of::<*mut c_ulong>() - 1;
    for i in 0..folio_nr_pages(folio) {
        let data = kmap_local_folio(folio, i * PAGE_SIZE);
        if *data.add(last_pos) != 0 { kunmap_local(data); return false; }
        for pos in 0..last_pos { if *data.add(pos) != 0 { kunmap_local(data); return false; } }
        kunmap_local(data);
    }
    true
}

unsafe fn swap_zeromap_folio_set(folio: *mut folio) {
    let objcg = get_obj_cgroup_from_folio(folio);
    let nr_pages = folio_nr_pages(folio);
    VM_WARN_ON_ONCE_FOLIO!(!folio_test_swapcache(folio), folio);
    VM_WARN_ON_ONCE_FOLIO!(!folio_test_locked(folio), folio);
    let ci = swap_cluster_get_and_lock(folio);
    for i in 0..folio_nr_pages(folio) {
        let entry = page_swap_entry(folio_page(folio, i));
        __swap_table_set_zero(ci, swp_cluster_offset(entry));
    }
    swap_cluster_unlock(ci);
    count_vm_events!(SWPOUT_ZERO, nr_pages);
    if !objcg.is_null() { count_objcg_events!(objcg, SWPOUT_ZERO, nr_pages); obj_cgroup_put(objcg); }
}

unsafe fn swap_zeromap_folio_clear(folio: *mut folio) {
    VM_WARN_ON_ONCE_FOLIO!(!folio_test_swapcache(folio), folio);
    VM_WARN_ON_ONCE_FOLIO!(!folio_test_locked(folio), folio);
    let ci = swap_cluster_get_and_lock(folio);
    for i in 0..folio_nr_pages(folio) {
        let entry = page_swap_entry(folio_page(folio, i));
        __swap_table_clear_zero(ci, swp_cluster_offset(entry));
    }
    swap_cluster_unlock(ci);
}

pub unsafe fn swap_writeout(ctx: *mut swap_io_ctx, folio: *mut folio) -> i32 {
    if folio_free_swap(folio) { folio_unlock(folio); return 0; }
    let mut ret = arch_prepare_to_swap(folio);
    if ret != 0 { folio_mark_dirty(folio); folio_unlock(folio); return ret; }
    if is_folio_zero_filled(folio) { swap_zeromap_folio_set(folio); folio_unlock(folio); return 0; }
    swap_zeromap_folio_clear(folio);
    if zswap_store(folio) { count_mthp_stat(folio_order(folio), MTHP_STAT_ZSWPOUT); folio_unlock(folio); return 0; }
    rcu_read_lock();
    if !mem_cgroup_zswap_writeback_enabled(folio_memcg(folio)) {
        rcu_read_unlock(); folio_mark_dirty(folio); return AOP_WRITEPAGE_ACTIVATE;
    }
    rcu_read_unlock();
    __swap_writepage(ctx, folio);
    ret = 0;
    ret
}

#[cfg(all(CONFIG_MEMCG, CONFIG_BLK_CGROUP))]
unsafe fn folio_blkg_can_merge(folio: *mut folio, prev_folio: *mut folio) -> bool {
    if folio_memcg_charged(folio) != folio_memcg_charged(prev_folio) { return false; }
    if folio_memcg_charged(folio) {
        rcu_read_lock();
        let result = folio_memcg_blkg_css(folio) == folio_memcg_blkg_css(prev_folio);
        rcu_read_unlock();
        return result;
    }
    true
}
#[cfg(not(all(CONFIG_MEMCG, CONFIG_BLK_CGROUP)))]
unsafe fn folio_blkg_can_merge(_: *mut folio, _: *mut folio) -> bool { true }

static mut sio_pool: *mut mempool_t = core::ptr::null_mut();

pub unsafe fn sio_pool_init() -> i32 {
    if sio_pool.is_null() {
        let pool = mempool_create_kmalloc_pool(SWAP_CLUSTER_MAX, core::mem::size_of::<swap_iocb>());
        if cmpxchg(&mut sio_pool, core::ptr::null_mut(), pool) != core::ptr::null_mut() { mempool_destroy(pool); }
    }
    if sio_pool.is_null() { return -ENOMEM; }
    0
}

unsafe fn swap_can_merge(ctx: *mut swap_io_ctx, folio: *mut folio, rw: i32) -> bool {
    let sis = __swap_entry_to_info((*folio).swap);
    let sio = (*ctx).sio;
    let last_bv = &(*sio).bvecs[(*sio).nr_bvecs - 1];
    let prev_folio = bvec_folio(last_bv);
    if (*ctx).sis != sis { return false; }
    ((*sis).ops).as_ref().unwrap().can_merge.unwrap()(folio, prev_folio, folio_size(prev_folio), rw)
}

unsafe fn swap_add_folio(ctx: *mut swap_io_ctx, folio: *mut folio, rw: i32) {
    let sis = __swap_entry_to_info((*folio).swap);
    let mut sio = (*ctx).sio;
    if !sio.is_null() && !swap_can_merge(ctx, folio, rw) {
        if rw == WRITE { swap_write_submit(ctx); } else { swap_read_submit(ctx); }
        sio = (*ctx).sio;
    }
    if sio.is_null() {
        (*ctx).sis = sis;
        sio = mempool_alloc(sio_pool, GFP_NOIO);
        (*ctx).sio = sio;
        (*sio).nr_bvecs = 0;
        (*sio).len = 0;
    }
    bvec_set_folio(&mut (*sio).bvecs[(*sio).nr_bvecs], folio, folio_size(folio), 0);
    (*sio).len += folio_size(folio);
    (*sio).nr_bvecs += 1;
    if (*sio).nr_bvecs == core::mem::size_of_val(&(*sio).bvecs) / core::mem::size_of::<bio_vec>() || (rw == WRITE && ((*sis).flags & SWP_SYNCHRONOUS_IO) != 0) {
        if rw == WRITE { swap_write_submit(ctx); } else { swap_read_submit(ctx); }
    }
}

pub unsafe fn __swap_writepage(ctx: *mut swap_io_ctx, folio: *mut folio) {
    VM_BUG_ON_FOLIO!(!folio_test_swapcache(folio), folio);
    count_mthp_stat(folio_order(folio), MTHP_STAT_SWPOUT);
    count_memcg_folio_events(folio, PSWPOUT, folio_nr_pages(folio));
    count_vm_events!(PSWPOUT, folio_nr_pages(folio));
    folio_start_writeback(folio);
    folio_unlock(folio);
    swap_add_folio(ctx, folio, WRITE);
}

unsafe fn swap_zeromap_batch(entry: swp_entry_t, max_nr: i32, is_zerop: *mut bool) -> i32 {
    let ci_start = swp_cluster_offset(entry);
    let ci = __swap_entry_to_cluster(entry);
    rcu_read_lock();
    let is_zero = __swap_table_test_zero(ci, ci_start);
    let mut i = 1;
    while i < max_nr && is_zero == __swap_table_test_zero(ci, ci_start + i as u32) { i += 1; }
    rcu_read_unlock();
    if !is_zerop.is_null() { *is_zerop = is_zero; }
    i
}

unsafe fn swap_read_folio_zeromap(folio: *mut folio) -> bool {
    let nr_pages = folio_nr_pages(folio) as i32;
    let mut is_zeromap = false;
    if WARN_ON_ONCE!(swap_zeromap_batch((*folio).swap, nr_pages, &mut is_zeromap) != nr_pages) { return true; }
    if !is_zeromap { return false; }
    let objcg = get_obj_cgroup_from_folio(folio);
    count_vm_events!(SWPIN_ZERO, nr_pages);
    if !objcg.is_null() { count_objcg_events!(objcg, SWPIN_ZERO, nr_pages); obj_cgroup_put(objcg); }
    folio_zero_range(folio, 0, folio_size(folio));
    folio_mark_uptodate(folio);
    true
}

pub unsafe fn swap_read_folio(ctx: *mut swap_io_ctx, folio: *mut folio) {
    let sis = __swap_entry_to_info((*folio).swap);
    let synchronous = ((*sis).flags & SWP_SYNCHRONOUS_IO) != 0;
    let workingset = folio_test_workingset(folio);
    let mut pflags = 0;
    let mut in_thrashing = false;
    if workingset { delayacct_thrashing_start(&mut in_thrashing); psi_memstall_enter(&mut pflags); }
    delayacct_swapin_start();
    if swap_read_folio_zeromap(folio) { folio_unlock(folio); }
    else if zswap_load(folio) == -ENOENT { zswap_folio_swapin(folio); swap_add_folio(ctx, folio, READ); }
    if workingset { delayacct_thrashing_end(&mut in_thrashing); psi_memstall_leave(&mut pflags); }
    delayacct_swapin_end();
}

pub unsafe fn swap_write_submit(ctx: *mut swap_io_ctx) {
    if (*ctx).sio.is_null() { return; }
    count_vm_events!(NRSWPOUT, 1);
    ((*(*ctx).sis).ops).as_ref().unwrap().submit_write.unwrap()(ctx);
    (*ctx).sio = core::ptr::null_mut(); (*ctx).sis = core::ptr::null_mut();
}
pub unsafe fn swap_read_submit(ctx: *mut swap_io_ctx) {
    if (*ctx).sio.is_null() { return; }
    count_vm_events!(NRSWPIN, 1);
    ((*(*ctx).sis).ops).as_ref().unwrap().submit_read.unwrap()(ctx);
    (*ctx).sio = core::ptr::null_mut(); (*ctx).sis = core::ptr::null_mut();
}

unsafe fn swap_write_end(sio: *mut swap_iocb, failed: bool) {
    for p in 0..(*sio).nr_bvecs {
        let page = (*sio).bvecs[p].bv_page;
        if failed { set_page_dirty(page); ClearPageReclaim(page); }
        end_page_writeback(page);
    }
    mempool_free(sio, sio_pool);
}
unsafe fn swap_read_end(sio: *mut swap_iocb, failed: bool) {
    for p in 0..(*sio).nr_bvecs {
        let folio = bvec_folio(&(*sio).bvecs[p]);
        if !failed { count_mthp_stat(folio_order(folio), MTHP_STAT_SWPIN); count_memcg_folio_events(folio, PSWPIN, folio_nr_pages(folio)); folio_mark_uptodate(folio); }
        folio_unlock(folio);
    }
    if !failed { count_vm_events!(PSWPIN, (*sio).len >> PAGE_SHIFT); }
    mempool_free(sio, sio_pool);
}

unsafe fn swap_fs_write_complete(iocb: *mut kiocb, ret: c_long) {
    let sio = container_of!(iocb, swap_iocb, iocb);
    swap_write_end(sio, ret != (*sio).len);
}
unsafe fn swap_fs_read_complete(iocb: *mut kiocb, ret: c_long) {
    let sio = container_of!(iocb, swap_iocb, iocb);
    swap_read_end(sio, ret != (*sio).len);
}
unsafe fn end_swap_bio_write(bio: *mut bio) {
    let sio = container_of!(bio, swap_iocb, bio);
    let failed = (*bio).bi_status != 0;
    bio_uninit(bio); swap_write_end(sio, failed);
}
unsafe fn swap_bio_read_end_io(bio: *mut bio) {
    let sio = container_of!(bio, swap_iocb, bio);
    let failed = (*bio).bi_status != 0;
    bio_uninit(bio); swap_read_end(sio, failed);
}

unsafe fn swap_bdev_submit_write(ctx: *mut swap_io_ctx) {
    let sio = (*ctx).sio; let bio = &mut (*sio).bio;
    bio_init(bio, (*(*ctx).sis).bdev, (*sio).bvecs.as_mut_ptr(), (*sio).bvecs.len(), REQ_OP_WRITE | REQ_SWAP);
    (*bio).bi_iter.bi_size = (*sio).len;
    (*bio).bi_iter.bi_sector = swap_folio_sector(bio_first_folio_all(bio));
    if ((*(*ctx).sis).flags & SWP_SYNCHRONOUS_IO) != 0 { submit_bio_wait(bio); end_swap_bio_write(bio); }
    else { (*bio).bi_end_io = Some(end_swap_bio_write); submit_bio(bio); }
}
unsafe fn swap_bdev_submit_read(ctx: *mut swap_io_ctx) {
    let sio = (*ctx).sio; let bio = &mut (*sio).bio;
    bio_init(bio, (*(*ctx).sis).bdev, (*sio).bvecs.as_mut_ptr(), (*sio).bvecs.len(), REQ_OP_READ);
    (*bio).bi_iter.bi_size = (*sio).len;
    (*bio).bi_iter.bi_sector = swap_folio_sector(bio_first_folio_all(bio));
    if ((*(*ctx).sis).flags & SWP_SYNCHRONOUS_IO) != 0 { submit_bio_wait(bio); swap_bio_read_end_io(bio); }
    else { (*bio).bi_end_io = Some(swap_bio_read_end_io); submit_bio(bio); }
}
unsafe fn swap_bdev_can_merge(folio: *mut folio, prev: *mut folio, prev_size: usize, rw: i32) -> bool {
    swap_folio_sector(folio) == swap_folio_sector(prev) + (prev_size >> SECTOR_SHIFT) as u64 && (rw != WRITE || folio_blkg_can_merge(folio, prev))
}

pub static swap_bdev_ops: swap_ops = swap_ops { submit_write: Some(swap_bdev_submit_write), submit_read: Some(swap_bdev_submit_read), can_merge: Some(swap_bdev_can_merge) };

pub unsafe fn swap_fs_prepare_rw(ctx: *mut swap_io_ctx, rw: i32, iter: *mut iov_iter) {
    let sio = (*ctx).sio;
    init_sync_kiocb(&mut (*sio).iocb, (*(*ctx).sis).swap_file);
    (*sio).iocb.ki_pos = swap_dev_pos(bvec_folio(&(*sio).bvecs[0]).swap);
    (*sio).iocb.ki_complete = if rw == WRITE { Some(swap_fs_write_complete) } else { Some(swap_fs_read_complete) };
    iov_iter_bvec(iter, if rw == WRITE { ITER_SOURCE } else { ITER_DEST }, (*sio).bvecs.as_mut_ptr(), (*sio).nr_bvecs, (*sio).len);
}
pub unsafe fn swap_fs_can_merge(folio: *mut folio, prev: *mut folio, prev_size: usize, _: i32) -> bool {
    swap_dev_pos((*folio).swap) == swap_dev_pos((*prev).swap) + prev_size as u64
}
pub unsafe fn swap_fs_activate(sis: *mut swap_info_struct, ops: *const swap_ops) -> i32 { (*sis).ops = ops; add_swap_extent(sis, 0, (*sis).max, 0) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
