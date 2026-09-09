// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2026 Micron Technology, Inc. */
// Kernel headers and local dependencies are supplied by the surrounding build.

unsafe fn fsdev_write_dax(mut addr: *mut core::ffi::c_void, mut page: *mut page,
                          mut off: u32, mut len: u32) {
    while len != 0 {
        let mem = kmap_local_page(page);
        let chunk = core::cmp::min(len, PAGE_SIZE - off);
        memcpy_flushcache(addr, (mem as *mut u8).add(off as usize) as _, chunk);
        kunmap_local(mem);
        len -= chunk;
        off = 0;
        page = page.add(1);
        addr = (addr as *mut u8).add(chunk as usize) as _;
    }
}

unsafe fn __fsdev_dax_direct_access(dax_dev: *mut dax_device, pgoff: pgoff_t,
                                    nr_pages: i64, mode: dax_access_mode,
                                    kaddr: *mut *mut core::ffi::c_void,
                                    pfn: *mut u64) -> i64 {
    let dev_dax = dax_get_private(dax_dev);
    let size = (nr_pages as usize) << PAGE_SHIFT;
    let offset = (pgoff as usize) << PAGE_SHIFT;
    let phys = dax_pgoff_to_phys(dev_dax, pgoff, size);
    if phys == !0usize {
        dev_dbg(&mut (*dev_dax).dev, "pgoff (%#lx) out of range\n", pgoff);
        return -EFAULT as i64;
    }
    if !kaddr.is_null() { *kaddr = __va(phys); }
    if !pfn.is_null() { *pfn = PHYS_PFN(phys); }
    PHYS_PFN(core::cmp::min(size, (*dev_dax).cached_size - offset)) as i64
}

unsafe fn fsdev_dax_zero_page_range(dax_dev: *mut dax_device, pgoff: pgoff_t,
                                    nr_pages: usize) -> i32 {
    WARN_ONCE(nr_pages > 1, "%s: nr_pages > 1\n", __func__);
    let mut kaddr = core::ptr::null_mut();
    let rc = __fsdev_dax_direct_access(dax_dev, pgoff, 1, DAX_ACCESS,
                                       &mut kaddr, core::ptr::null_mut());
    if rc < 0 { return rc as i32; }
    fsdev_write_dax(kaddr, ZERO_PAGE(0), 0, PAGE_SIZE as u32);
    0
}

unsafe fn fsdev_dax_direct_access(dax_dev: *mut dax_device, pgoff: pgoff_t,
                                  nr_pages: i64, mode: dax_access_mode,
                                  kaddr: *mut *mut core::ffi::c_void,
                                  pfn: *mut u64) -> i64 {
    __fsdev_dax_direct_access(dax_dev, pgoff, nr_pages, mode, kaddr, pfn)
}

unsafe fn fsdev_dax_recovery_write(_dax_dev: *mut dax_device, _pgoff: pgoff_t,
                                   addr: *mut core::ffi::c_void, bytes: usize,
                                   i: *mut iov_iter) -> usize {
    _copy_from_iter_flushcache(addr, bytes, i)
}

static dev_dax_ops: dax_operations = dax_operations {
    direct_access: Some(fsdev_dax_direct_access),
    zero_page_range: Some(fsdev_dax_zero_page_range),
    recovery_write: Some(fsdev_dax_recovery_write),
};

unsafe fn fsdev_cdev_del(cdev: *mut core::ffi::c_void) { cdev_del(cdev as *mut cdev); }
unsafe fn fsdev_kill(dev_dax: *mut core::ffi::c_void) { kill_dev_dax(dev_dax as *mut dev_dax); }
unsafe fn fsdev_clear_ops(data: *mut core::ffi::c_void) {
    let dev_dax = data as *mut dev_dax;
    dax_set_ops((*dev_dax).dax_dev, core::ptr::null());
}
unsafe fn fsdev_clear_pgmap_ops(data: *mut core::ffi::c_void) {
    let pgmap = data as *mut dev_pagemap;
    (*pgmap).ops = core::ptr::null();
    (*pgmap).owner = core::ptr::null_mut();
}

unsafe fn fsdev_pfn_to_offset(dev_dax: *mut dev_dax, pfn: usize) -> u64 {
    let phys = PFN_PHYS(pfn);
    let mut offset = 0;
    for i in 0..(*dev_dax).nr_range {
        let range = &(*dev_dax).ranges.add(i as usize).as_ref().unwrap().range;
        if phys >= range.start && phys <= range.end { return offset + phys - range.start; }
        offset += range_len(range);
    }
    !0u64
}

unsafe fn fsdev_pagemap_memory_failure(pgmap: *mut dev_pagemap, pfn: usize,
                                       nr_pages: usize, mf_flags: i32) -> i32 {
    let dev_dax = (*pgmap).owner as *mut dev_dax;
    dax_holder_notify_failure((*dev_dax).dax_dev, fsdev_pfn_to_offset(dev_dax, pfn),
                              nr_pages << PAGE_SHIFT, mf_flags)
}

static fsdev_pagemap_ops: dev_pagemap_ops = dev_pagemap_ops { memory_failure: Some(fsdev_pagemap_memory_failure) };

unsafe fn fsdev_clear_folio_state(dev_dax: *mut dev_dax) {
    for i in 0..(*dev_dax).nr_range {
        let range = &(*dev_dax).ranges.add(i as usize).as_ref().unwrap().range;
        let mut pfn = PHYS_PFN(range.start);
        let end_pfn = PHYS_PFN(range.end) + 1;
        while pfn < end_pfn { let folio = pfn_folio(pfn); let order = dax_folio_reset_order(folio); pfn += 1usize << order; }
    }
}
unsafe fn fsdev_clear_folio_state_action(data: *mut core::ffi::c_void) { fsdev_clear_folio_state(data as *mut dev_dax); }

unsafe fn fsdev_open(inode: *mut inode, filp: *mut file) -> i32 {
    let dev_dax = dax_get_private(inode_dax(inode));
    (*filp).private_data = dev_dax as _;
    0
}
unsafe fn fsdev_release(_inode: *mut inode, _filp: *mut file) -> i32 { 0 }

static fsdev_fops: file_operations = file_operations {
    llseek: Some(noop_llseek), owner: THIS_MODULE, open: Some(fsdev_open), release: Some(fsdev_release),
};

unsafe fn fsdev_acquire_pgmap(dev_dax: *mut dev_dax) -> *mut dev_pagemap {
    let dev = &mut (*dev_dax).dev;
    if static_dev_dax(dev_dax) {
        if (*dev_dax).nr_range > 1 { dev_warn(dev, "static pgmap / multi-range device conflict\n"); return ERR_PTR(-EINVAL); }
        let pgmap = (*dev_dax).pgmap;
        (*pgmap).vmemmap_shift = 0;
        return pgmap;
    }
    if !(*dev_dax).pgmap.is_null() { dev_warn(dev, "dynamic-dax with pre-populated page map\n"); return ERR_PTR(-EINVAL); }
    let size = struct_size::<dev_pagemap, _>("ranges", (*dev_dax).nr_range - 1);
    let pgmap = devm_kzalloc(dev, size, GFP_KERNEL) as *mut dev_pagemap;
    if pgmap.is_null() { return ERR_PTR(-ENOMEM); }
    (*pgmap).nr_range = (*dev_dax).nr_range;
    for i in 0..(*dev_dax).nr_range { (*pgmap).ranges.add(i as usize).write((*dev_dax).ranges.add(i as usize).read().range); }
    pgmap
}

unsafe fn fsdev_dax_probe(dev_dax: *mut dev_dax) -> i32 {
    let dax_dev = (*dev_dax).dax_dev; let dev = &mut (*dev_dax).dev;
    let pgmap = fsdev_acquire_pgmap(dev_dax); if IS_ERR(pgmap) { return PTR_ERR(pgmap); }
    for i in 0..(*dev_dax).nr_range {
        let range = &(*dev_dax).ranges.add(i as usize).as_ref().unwrap().range;
        if devm_request_mem_region(dev, range.start, range_len(range), dev_name(dev)).is_null() { dev_warn(dev, "mapping range could not reserve\n"); return -EBUSY; }
    }
    (*dev_dax).cached_size = 0;
    for i in 0..(*dev_dax).nr_range { (*dev_dax).cached_size += range_len(&(*dev_dax).ranges.add(i as usize).as_ref().unwrap().range); }
    (*pgmap).type_ = MEMORY_DEVICE_FS_DAX; (*pgmap).ops = &fsdev_pagemap_ops; (*pgmap).owner = dev_dax as _;
    let addr = devm_memremap_pages(dev, pgmap); if IS_ERR(addr) { return PTR_ERR(addr); }
    let mut rc = devm_add_action_or_reset(dev, fsdev_clear_pgmap_ops, pgmap as _); if rc != 0 { return rc; }
    fsdev_clear_folio_state(dev_dax); rc = devm_add_action_or_reset(dev, fsdev_clear_folio_state_action, dev_dax as _); if rc != 0 { return rc; }
    let inode = dax_inode(dax_dev); let cdev = (*inode).i_cdev; cdev_init(cdev, &fsdev_fops); (*cdev).owner = (*dev).driver.owner; cdev_set_parent(cdev, &(*dev).kobj);
    rc = cdev_add(cdev, (*dev).devt, 1); if rc != 0 { return rc; }
    rc = devm_add_action_or_reset(dev, fsdev_cdev_del, cdev as _); if rc != 0 { return rc; }
    rc = dax_set_ops(dax_dev, &dev_dax_ops); if rc != 0 { return rc; }
    rc = devm_add_action_or_reset(dev, fsdev_clear_ops, dev_dax as _); if rc != 0 { return rc; }
    run_dax(dax_dev); rc = devm_add_action_or_reset(dev, fsdev_kill, dev_dax as _); if rc != 0 { return rc; }
    (*dev_dax).pgmap = pgmap; 0
}

static mut fsdev_dax_driver: dax_device_driver = dax_device_driver { probe: Some(fsdev_dax_probe), type_: DAXDRV_FSDEV_TYPE };
unsafe fn dax_init() -> i32 { dax_driver_register(&mut fsdev_dax_driver) }
unsafe fn dax_exit() { dax_driver_unregister(&mut fsdev_dax_driver); }
// MODULE_AUTHOR("John Groves"); MODULE_DESCRIPTION("FS-DAX Device: fs-dax compatible devdax driver");
// MODULE_LICENSE("GPL"); module_init(dax_init); module_exit(dax_exit); MODULE_ALIAS_DAX_DEVICE(0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
