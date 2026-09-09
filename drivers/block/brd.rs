// SPDX-License-Identifier: GPL-2.0-only
/*
 * Ram backed block device driver.
 *
 * Copyright (C) 2007 Nick Piggin
 * Copyright (C) 2007 Novell Inc.
 *
 * Parts derived from drivers/block/rd.c, and drivers/block/loop.c, copyright
 * of their respective owners.
 */

// Kernel dependencies supplied by the surrounding repository.

#[repr(C)]
pub struct brd_device {
    pub brd_number: i32,
    pub brd_disk: *mut gendisk,
    pub brd_list: list_head,
    pub brd_pages: xarray,
    pub brd_nr_pages: u64,
}

unsafe fn brd_lookup_page(brd: *mut brd_device, sector: sector_t) -> *mut page {
    let mut page: *mut page;
    let mut xas = XA_STATE(&mut (*brd).brd_pages, sector >> PAGE_SECTORS_SHIFT);
    rcu_read_lock();
    'repeat: loop {
        page = xas_load(&mut xas);
        if xas_retry(&mut xas, page) {
            xas_reset(&mut xas);
            continue 'repeat;
        }
        if page.is_null() {
            break;
        }
        if !get_page_unless_zero(page) {
            xas_reset(&mut xas);
            continue 'repeat;
        }
        if page != xas_reload(&mut xas) {
            put_page(page);
            xas_reset(&mut xas);
            continue 'repeat;
        }
        break;
    }
    rcu_read_unlock();
    page
}

unsafe fn brd_insert_page(brd: *mut brd_device, sector: sector_t, opf: blk_opf_t) -> *mut page {
    let gfp: gfp_t = if opf & REQ_NOWAIT != 0 { GFP_NOWAIT } else { GFP_NOIO };
    let page = alloc_page(gfp | __GFP_ZERO | __GFP_HIGHMEM);
    if page.is_null() { return ERR_PTR(-ENOMEM); }
    xa_lock(&mut (*brd).brd_pages);
    let ret = __xa_cmpxchg(&mut (*brd).brd_pages, sector >> PAGE_SECTORS_SHIFT,
                           core::ptr::null_mut(), page, gfp);
    if ret.is_null() {
        (*brd).brd_nr_pages += 1;
        get_page(page);
        xa_unlock(&mut (*brd).brd_pages);
        return page;
    }
    if !xa_is_err(ret) {
        get_page(ret);
        xa_unlock(&mut (*brd).brd_pages);
        put_page(page);
        return ret;
    }
    xa_unlock(&mut (*brd).brd_pages);
    put_page(page);
    ERR_PTR(xa_err(ret))
}

unsafe fn brd_free_pages(brd: *mut brd_device) {
    let mut page: *mut page;
    let mut idx: pgoff_t;
    xa_for_each(&mut (*brd).brd_pages, idx, page) {
        put_page(page);
        cond_resched();
    }
    xa_destroy(&mut (*brd).brd_pages);
}

unsafe fn brd_rw_bvec(brd: *mut brd_device, bio: *mut bio) -> bool {
    let mut bv = bio_iter_iovec(bio, (*bio).bi_iter);
    let sector = (*bio).bi_iter.bi_sector;
    let offset: u32 = ((sector & (PAGE_SECTORS - 1)) << SECTOR_SHIFT) as u32;
    let opf = (*bio).bi_opf;
    let mut page = brd_lookup_page(brd, sector);
    bv.bv_len = core::cmp::min(bv.bv_len, (PAGE_SIZE as u32) - offset);
    if page.is_null() && op_is_write(opf) {
        page = brd_insert_page(brd, sector, opf);
        if IS_ERR(page) { return brd_rw_error(bio, page, opf); }
    }
    let kaddr = bvec_kmap_local(&bv);
    if op_is_write(opf) {
        memcpy_to_page(page, offset, kaddr, bv.bv_len);
    } else if !page.is_null() {
        memcpy_from_page(kaddr, page, offset, bv.bv_len);
    } else {
        memset(kaddr, 0, bv.bv_len);
    }
    kunmap_local(kaddr);
    bio_advance_iter_single(bio, &mut (*bio).bi_iter, bv.bv_len);
    if !page.is_null() { put_page(page); }
    true
}

unsafe fn brd_rw_error(bio: *mut bio, page: *mut page, opf: blk_opf_t) -> bool {
    if PTR_ERR(page) == -ENOMEM && opf & REQ_NOWAIT != 0 { bio_wouldblock_error(bio); }
    else { bio_io_error(bio); }
    false
}

unsafe fn brd_do_discard(brd: *mut brd_device, sector: sector_t, size: u32) {
    let mut aligned_sector = round_up(sector, PAGE_SECTORS);
    let aligned_end = round_down(sector + (size >> SECTOR_SHIFT), PAGE_SECTORS);
    if aligned_end <= aligned_sector { return; }
    xa_lock(&mut (*brd).brd_pages);
    while aligned_sector < aligned_end && aligned_sector < rd_size * 2 {
        let page = __xa_erase(&mut (*brd).brd_pages, aligned_sector >> PAGE_SECTORS_SHIFT);
        if !page.is_null() { put_page(page); (*brd).brd_nr_pages -= 1; }
        aligned_sector += PAGE_SECTORS;
    }
    xa_unlock(&mut (*brd).brd_pages);
}

unsafe fn brd_submit_bio(bio: *mut bio) {
    let brd = (*(*bio).bi_bdev).bd_disk.private_data as *mut brd_device;
    if op_is_discard((*bio).bi_opf) {
        brd_do_discard(brd, (*bio).bi_iter.bi_sector, (*bio).bi_iter.bi_size);
        bio_endio(bio); return;
    }
    while (*bio).bi_iter.bi_size != 0 {
        if !brd_rw_bvec(brd, bio) { return; }
    }
    bio_endio(bio);
}

static brd_fops: block_device_operations = block_device_operations {
    owner: THIS_MODULE, submit_bio: Some(brd_submit_bio),
};

static mut rd_nr: i32 = CONFIG_BLK_DEV_RAM_COUNT;
static mut rd_size: u64 = CONFIG_BLK_DEV_RAM_SIZE;
static mut max_part: i32 = 1;

// Module metadata and boot/module parameter declarations are supplied by the kernel integration.

#[cfg(not(module))]
unsafe fn ramdisk_size(str_: *mut u8) -> i32 { kstrtoul(str_, 0, &mut rd_size) == 0 }

static mut brd_devices: list_head = LIST_HEAD_INIT();
static mut brd_devices_mutex: mutex = DEFINE_MUTEX_INIT();
static mut brd_debugfs_dir: *mut dentry = core::ptr::null_mut();

unsafe fn brd_find_or_alloc_device(i: i32) -> *mut brd_device {
    let mut brd: *mut brd_device;
    mutex_lock(&mut brd_devices_mutex);
    list_for_each_entry!(brd, &mut brd_devices, brd_list) {
        if (*brd).brd_number == i { mutex_unlock(&mut brd_devices_mutex); return ERR_PTR(-EEXIST); }
    }
    brd = kzalloc_obj::<brd_device>();
    if brd.is_null() { mutex_unlock(&mut brd_devices_mutex); return ERR_PTR(-ENOMEM); }
    (*brd).brd_number = i;
    list_add_tail(&mut (*brd).brd_list, &mut brd_devices);
    mutex_unlock(&mut brd_devices_mutex);
    brd
}

unsafe fn brd_free_device(brd: *mut brd_device) {
    mutex_lock(&mut brd_devices_mutex);
    list_del(&mut (*brd).brd_list);
    mutex_unlock(&mut brd_devices_mutex);
    kfree(brd as *mut core::ffi::c_void);
}

unsafe fn brd_alloc(i: i32) -> i32 {
    let brd = brd_find_or_alloc_device(i);
    if IS_ERR(brd) { return PTR_ERR(brd); }
    xa_init(&mut (*brd).brd_pages);
    let disk = blk_alloc_disk(core::ptr::null_mut(), NUMA_NO_NODE);
    if IS_ERR(disk) { brd_free_device(brd); return PTR_ERR(disk); }
    (*brd).brd_disk = disk;
    (*disk).major = RAMDISK_MAJOR;
    (*disk).first_minor = i * max_part;
    (*disk).minors = max_part;
    (*disk).fops = &brd_fops;
    (*disk).private_data = brd as *mut core::ffi::c_void;
    set_capacity(disk, rd_size * 2);
    let err = add_disk(disk);
    if err != 0 { put_disk(disk); brd_free_device(brd); }
    err
}

unsafe fn brd_probe(dev: dev_t) { brd_alloc(MINOR(dev) / max_part); }

unsafe fn brd_cleanup() {
    debugfs_remove_recursive(brd_debugfs_dir);
    let mut brd: *mut brd_device; let mut next: *mut brd_device;
    list_for_each_entry_safe!(brd, next, &mut brd_devices, brd_list) {
        del_gendisk((*brd).brd_disk); put_disk((*brd).brd_disk);
        brd_free_pages(brd); brd_free_device(brd);
    }
}

unsafe fn brd_check_and_reset_par() {
    if max_part == 0 { max_part = 1; }
    if (1u32 << MINORBITS) % max_part as u32 != 0 { max_part = 1i32 << fls(max_part); }
    if max_part > DISK_MAX_PARTS { pr_info!("brd: max_part can't be larger than %d, reset max_part = %d.\n", DISK_MAX_PARTS, DISK_MAX_PARTS); max_part = DISK_MAX_PARTS; }
}

unsafe fn brd_init() -> i32 {
    brd_check_and_reset_par();
    brd_debugfs_dir = debugfs_create_dir(c"ramdisk_pages", core::ptr::null_mut());
    if __register_blkdev(RAMDISK_MAJOR, c"ramdisk", Some(brd_probe)) != 0 { brd_cleanup(); pr_info!("brd: module NOT loaded !!!\n"); return -EIO; }
    for i in 0..rd_nr { brd_alloc(i); }
    pr_info!("brd: module loaded\n"); 0
}

unsafe fn brd_exit() {
    unregister_blkdev(RAMDISK_MAJOR, c"ramdisk"); brd_cleanup();
    pr_info!("brd: module unloaded\n");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
