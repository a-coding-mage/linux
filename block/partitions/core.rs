// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1991-1998  Linus Torvalds
 * Re-organised Feb 1998 Russell King
 * Copyright (C) 2020 Christoph Hellwig
 */
// Linux kernel dependencies: <linux/fs.h>, <linux/major.h>, <linux/slab.h>,
// <linux/string.h>, <linux/sysfs.h>, <linux/ctype.h>, <linux/vmalloc.h>,
// <linux/raid/detect.h>, and "check.h".

#[allow(non_camel_case_types, non_snake_case, dead_code)]
static CHECK_PART: &[Option<unsafe extern "C" fn(*mut parsed_partitions) -> i32>] = &[
    // Partition-format probes and configuration-gated entries from check.h.
    #[cfg(CONFIG_ACORN_PARTITION_ICS)] Some(adfspart_check_ICS),
    #[cfg(CONFIG_ACORN_PARTITION_POWERTEC)] Some(adfspart_check_POWERTEC),
    #[cfg(CONFIG_ACORN_PARTITION_EESOX)] Some(adfspart_check_EESOX),
    #[cfg(CONFIG_ACORN_PARTITION_CUMANA)] Some(adfspart_check_CUMANA),
    #[cfg(CONFIG_ACORN_PARTITION_ADFS)] Some(adfspart_check_ADFS),
    #[cfg(CONFIG_CMDLINE_PARTITION)] Some(cmdline_partition),
    #[cfg(CONFIG_OF_PARTITION)] Some(of_partition),
    #[cfg(CONFIG_EFI_PARTITION)] Some(efi_partition),
    #[cfg(CONFIG_SGI_PARTITION)] Some(sgi_partition),
    #[cfg(CONFIG_LDM_PARTITION)] Some(ldm_partition),
    #[cfg(CONFIG_MSDOS_PARTITION)] Some(msdos_partition),
    #[cfg(CONFIG_OSF_PARTITION)] Some(osf_partition),
    #[cfg(CONFIG_SUN_PARTITION)] Some(sun_partition),
    #[cfg(CONFIG_AMIGA_PARTITION)] Some(amiga_partition),
    #[cfg(CONFIG_ATARI_PARTITION)] Some(atari_partition),
    #[cfg(CONFIG_MAC_PARTITION)] Some(mac_partition),
    #[cfg(CONFIG_ULTRIX_PARTITION)] Some(ultrix_partition),
    #[cfg(CONFIG_IBM_PARTITION)] Some(ibm_partition),
    #[cfg(CONFIG_KARMA_PARTITION)] Some(karma_partition),
    #[cfg(CONFIG_SYSV68_PARTITION)] Some(sysv68_partition),
    None,
];

unsafe fn allocate_partitions(hd: *mut gendisk) -> *mut parsed_partitions {
    let nr: i32 = DISK_MAX_PARTS;
    let state = kzalloc_obj::<parsed_partitions>();
    if state.is_null() { return core::ptr::null_mut(); }
    (*state).parts = vzalloc(array_size(nr as usize, core::mem::size_of::<partition>()));
    if (*state).parts.is_null() {
        kfree(state as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    (*state).limit = nr;
    state
}

unsafe fn free_partitions(state: *mut parsed_partitions) {
    vfree((*state).parts);
    kfree(state as *mut core::ffi::c_void);
}

unsafe fn check_partition(hd: *mut gendisk) -> *mut parsed_partitions {
    let state = allocate_partitions(hd);
    if state.is_null() { return core::ptr::null_mut(); }
    (*state).pp_buf.buffer = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (*state).pp_buf.buffer.is_null() { free_partitions(state); return core::ptr::null_mut(); }
    seq_buf_init(&mut (*state).pp_buf, (*state).pp_buf.buffer, PAGE_SIZE);
    (*state).disk = hd;
    strscpy((*state).name.as_mut_ptr(), (*hd).disk_name.as_ptr());
    seq_buf_printf(&mut (*state).pp_buf, " %s:", (*state).name.as_ptr());
    if isdigit((*state).name[strlen((*state).name.as_ptr()) - 1]) != 0 {
        sprintf((*state).name.as_mut_ptr(), "p");
    }
    let mut i = 0;
    let mut res = 0;
    let mut err = 0;
    while res == 0 && CHECK_PART[i].is_some() {
        memset((*state).parts, 0, (*state).limit as usize * core::mem::size_of::<partition>());
        res = CHECK_PART[i].unwrap()(state);
        i += 1;
        if res < 0 { err = res; res = 0; }
    }
    if res > 0 {
        printk(KERN_INFO, seq_buf_str(&(*state).pp_buf));
        kfree((*state).pp_buf.buffer);
        return state;
    }
    if (*state).access_beyond_eod { err = -ENOSPC; }
    if err != 0 { res = err; }
    if res != 0 {
        seq_buf_puts(&mut (*state).pp_buf, " unable to read partition table\n");
        printk(KERN_INFO, seq_buf_str(&(*state).pp_buf));
    }
    kfree((*state).pp_buf.buffer);
    free_partitions(state);
    ERR_PTR(res)
}

unsafe fn part_partition_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize { sysfs_emit(buf, "%d\n", bdev_partno(dev_to_bdev(dev))) }
unsafe fn part_start_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize { sysfs_emit(buf, "%llu\n", (*dev_to_bdev(dev)).bd_start_sect) }
unsafe fn part_ro_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize { sysfs_emit(buf, "%d\n", bdev_read_only(dev_to_bdev(dev))) }
unsafe fn part_alignment_offset_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize { sysfs_emit(buf, "%u\n", bdev_alignment_offset(dev_to_bdev(dev))) }
unsafe fn part_discard_alignment_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize { sysfs_emit(buf, "%u\n", bdev_discard_alignment(dev_to_bdev(dev))) }

static DEVICE_ATTR_PARTITION: device_attribute = DEVICE_ATTR!(partition, 0o444, part_partition_show, None);
static DEVICE_ATTR_START: device_attribute = DEVICE_ATTR!(start, 0o444, part_start_show, None);
static DEVICE_ATTR_SIZE: device_attribute = DEVICE_ATTR!(size, 0o444, part_size_show, None);
static DEVICE_ATTR_RO: device_attribute = DEVICE_ATTR!(ro, 0o444, part_ro_show, None);
static DEVICE_ATTR_ALIGNMENT_OFFSET: device_attribute = DEVICE_ATTR!(alignment_offset, 0o444, part_alignment_offset_show, None);
static DEVICE_ATTR_DISCARD_ALIGNMENT: device_attribute = DEVICE_ATTR!(discard_alignment, 0o444, part_discard_alignment_show, None);
static DEVICE_ATTR_STAT: device_attribute = DEVICE_ATTR!(stat, 0o444, part_stat_show, None);
static DEVICE_ATTR_INFLIGHT: device_attribute = DEVICE_ATTR!(inflight, 0o444, part_inflight_show, None);

static PART_ATTRS: &[*mut attribute] = &[
    &DEVICE_ATTR_PARTITION.attr, &DEVICE_ATTR_START.attr, &DEVICE_ATTR_SIZE.attr,
    &DEVICE_ATTR_RO.attr, &DEVICE_ATTR_ALIGNMENT_OFFSET.attr,
    &DEVICE_ATTR_DISCARD_ALIGNMENT.attr, &DEVICE_ATTR_STAT.attr,
    &DEVICE_ATTR_INFLIGHT.attr, core::ptr::null_mut(),
];
static PART_ATTR_GROUP: attribute_group = attribute_group { attrs: PART_ATTRS.as_ptr() };
static PART_ATTR_GROUPS: &[*const attribute_group] = &[&PART_ATTR_GROUP, core::ptr::null(),];

unsafe fn part_release(dev: *mut device) { put_disk((*dev_to_bdev(dev)).bd_disk); bdev_drop(dev_to_bdev(dev)); }

unsafe fn part_uevent(dev: *const device, env: *mut kobj_uevent_env) -> i32 {
    let part = dev_to_bdev(dev as *mut device);
    add_uevent_var(env, "PARTN=%u", bdev_partno(part));
    if !(*part).bd_meta_info.is_null() && (*(*part).bd_meta_info).volname[0] != 0 { add_uevent_var(env, "PARTNAME=%s", (*part).bd_meta_info.volname.as_ptr()); }
    if !(*part).bd_meta_info.is_null() && (*(*part).bd_meta_info).uuid[0] != 0 { add_uevent_var(env, "PARTUUID=%s", (*part).bd_meta_info.uuid.as_ptr()); }
    0
}

static PART_TYPE: device_type = device_type { name: "partition", groups: PART_ATTR_GROUPS.as_ptr(), release: Some(part_release), uevent: Some(part_uevent) };

unsafe fn drop_partition(part: *mut block_device) {
    lockdep_assert_held(&(*(*part).bd_disk).open_mutex);
    xa_erase(&mut (*(*part).bd_disk).part_tbl, bdev_partno(part));
    kobject_put((*part).bd_holder_dir);
    device_del(&mut (*part).bd_device);
    put_device(&mut (*part).bd_device);
}

unsafe fn whole_disk_show(_dev: *mut device, _attr: *mut device_attribute, _buf: *mut i8) -> isize { 0 }
static DEVICE_ATTR_WHOLE_DISK: device_attribute = DEVICE_ATTR!(whole_disk, 0o444, whole_disk_show, None);

unsafe fn add_partition(disk: *mut gendisk, partno: i32, start: sector_t, len: sector_t, flags: i32, info: *mut partition_meta_info) -> *mut block_device {
    let mut devt = MKDEV(0, 0);
    let ddev = disk_to_dev(disk);
    let mut err: i32;
    lockdep_assert_held(&(*disk).open_mutex);
    if partno >= DISK_MAX_PARTS { return ERR_PTR(-EINVAL); }
    if bdev_is_zoned((*disk).part0) { pr_warn("%s: partitions not supported on host managed zoned block device\n", (*disk).disk_name.as_ptr()); return ERR_PTR(-ENXIO); }
    if !xa_load(&(*disk).part_tbl, partno).is_null() { return ERR_PTR(-EBUSY); }
    get_device(ddev);
    err = -ENOMEM;
    let bdev = bdev_alloc(disk, partno);
    if bdev.is_null() { put_disk(disk); return ERR_PTR(err); }
    (*bdev).bd_start_sect = start; bdev_set_nr_sectors(bdev, len);
    let pdev = &mut (*bdev).bd_device;
    let dname = dev_name(ddev);
    if isdigit(*dname.add(strlen(dname) - 1)) != 0 { dev_set_name(pdev, "%sp%d", dname, partno); } else { dev_set_name(pdev, "%s%d", dname, partno); }
    device_initialize(pdev); pdev.class = &mut block_class; pdev.r#type = &PART_TYPE; pdev.parent = ddev;
    if bdev_partno(bdev) < (*disk).minors { devt = MKDEV((*disk).major, (*disk).first_minor + bdev_partno(bdev)); } else { err = blk_alloc_ext_minor(); if err < 0 { put_device(pdev); return ERR_PTR(err); } devt = MKDEV(BLOCK_EXT_MAJOR, err); }
    pdev.devt = devt;
    if !info.is_null() { err = -ENOMEM; (*bdev).bd_meta_info = kmemdup(info, core::mem::size_of::<partition_meta_info>(), GFP_KERNEL); if (*bdev).bd_meta_info.is_null() { put_device(pdev); return ERR_PTR(err); } }
    dev_set_uevent_suppress(pdev, true); err = device_add(pdev); if err != 0 { put_device(pdev); return ERR_PTR(err); }
    err = -ENOMEM; (*bdev).bd_holder_dir = kobject_create_and_add("holders", &pdev.kobj); if (*bdev).bd_holder_dir.is_null() { kobject_put((*bdev).bd_holder_dir); device_del(pdev); put_device(pdev); return ERR_PTR(err); }
    dev_set_uevent_suppress(pdev, false);
    if flags & ADDPART_FLAG_WHOLEDISK != 0 { err = device_create_file(pdev, &DEVICE_ATTR_WHOLE_DISK); if err != 0 { kobject_put((*bdev).bd_holder_dir); device_del(pdev); put_device(pdev); return ERR_PTR(err); } }
    if flags & ADDPART_FLAG_READONLY != 0 { bdev_set_flag(bdev, BD_READ_ONLY); }
    err = xa_insert(&mut (*disk).part_tbl, partno, bdev, GFP_KERNEL); if err != 0 { kobject_put((*bdev).bd_holder_dir); device_del(pdev); put_device(pdev); return ERR_PTR(err); }
    bdev_add(bdev, devt); if !dev_get_uevent_suppress(ddev) { kobject_uevent(&mut pdev.kobj, KOBJ_ADD); } bdev
}

unsafe fn partition_overlaps(disk: *mut gendisk, start: sector_t, length: sector_t, skip_partno: i32) -> bool {
    let mut overlap = false; let mut idx: c_ulong = 0; let mut part: *mut block_device = core::ptr::null_mut();
    rcu_read_lock(); xa_for_each_start(&(*disk).part_tbl, &mut idx, &mut part, 1, { if bdev_partno(part) != skip_partno && start < (*part).bd_start_sect + bdev_nr_sectors(part) && start + length > (*part).bd_start_sect { overlap = true; } }); rcu_read_unlock(); overlap
}

pub unsafe fn bdev_add_partition(disk: *mut gendisk, partno: i32, start: sector_t, length: sector_t) -> i32 {
    mutex_lock(&mut (*disk).open_mutex); let ret; if !disk_live(disk) { ret = -ENXIO; } else if (*disk).flags & GENHD_FL_NO_PART != 0 { ret = -EINVAL; } else if partition_overlaps(disk, start, length, -1) { ret = -EBUSY; } else { ret = PTR_ERR_OR_ZERO(add_partition(disk, partno, start, length, ADDPART_FLAG_NONE, core::ptr::null_mut())); } mutex_unlock(&mut (*disk).open_mutex); ret
}

pub unsafe fn bdev_del_partition(disk: *mut gendisk, partno: i32) -> i32 { mutex_lock(&mut (*disk).open_mutex); let part = xa_load(&(*disk).part_tbl, partno); let mut ret = -ENXIO; if !part.is_null() { ret = -EBUSY; if atomic_read(&(*part).bd_openers) == 0 { bdev_unhash(part); invalidate_bdev(part); drop_partition(part); ret = 0; } } mutex_unlock(&mut (*disk).open_mutex); ret }

pub unsafe fn bdev_resize_partition(disk: *mut gendisk, partno: i32, start: sector_t, length: sector_t) -> i32 { mutex_lock(&mut (*disk).open_mutex); let part = xa_load(&(*disk).part_tbl, partno); let mut ret = -ENXIO; if !part.is_null() { ret = -EINVAL; if start == (*part).bd_start_sect { ret = -EBUSY; if !partition_overlaps(disk, start, length, partno) { bdev_set_nr_sectors(part, length); ret = 0; } } } mutex_unlock(&mut (*disk).open_mutex); ret }

unsafe fn disk_unlock_native_capacity(disk: *mut gendisk) -> bool { if (*disk).fops.is_null() || (*(*disk).fops).unlock_native_capacity.is_none() || test_and_set_bit(GD_NATIVE_CAPACITY, &mut (*disk).state) { printk(KERN_CONT, "truncated\n"); false } else { printk(KERN_CONT, "enabling native capacity\n"); (*(*disk).fops).unlock_native_capacity.unwrap()(disk); true } }

unsafe fn blk_add_partition(disk: *mut gendisk, state: *mut parsed_partitions, p: i32) -> bool {
    let mut size = (*state).parts[p as usize].size; let from = (*state).parts[p as usize].from; if size == 0 { return true; }
    if from >= get_capacity(disk) { printk(KERN_WARNING, "%s: p%d start %llu is beyond EOD, ", (*disk).disk_name.as_ptr(), p, from as u64); return !disk_unlock_native_capacity(disk); }
    if from + size > get_capacity(disk) { printk(KERN_WARNING, "%s: p%d size %llu extends beyond EOD, ", (*disk).disk_name.as_ptr(), p, size as u64); if disk_unlock_native_capacity(disk) { return false; } size = get_capacity(disk) - from; }
    let part = add_partition(disk, p, from, size, (*state).parts[p as usize].flags, &mut (*state).parts[p as usize].info); if IS_ERR(part) { if PTR_ERR(part) != -ENXIO { printk(KERN_ERR, " %s: p%d could not be added: %pe\n", (*disk).disk_name.as_ptr(), p, part); } return true; }
    if IS_BUILTIN(CONFIG_BLK_DEV_MD) && (*state).parts[p as usize].flags & ADDPART_FLAG_RAID != 0 { md_autodetect_dev((*part).bd_dev); } true
}

unsafe fn blk_add_partitions(disk: *mut gendisk) -> i32 {
    if !disk_has_partscan(disk) { return 0; }
    let state = check_partition(disk); if state.is_null() { return 0; } if IS_ERR(state) { if PTR_ERR(state) == -ENOSPC { printk(KERN_WARNING, "%s: partition table beyond EOD, ", (*disk).disk_name.as_ptr()); if disk_unlock_native_capacity(disk) { return -EAGAIN; } } return -EIO; }
    if bdev_is_zoned((*disk).part0) { pr_warn("%s: ignoring partition table on host managed zoned block device\n", (*disk).disk_name.as_ptr()); free_partitions(state); return 0; }
    if (*state).access_beyond_eod { printk(KERN_WARNING, "%s: partition table partially beyond EOD, ", (*disk).disk_name.as_ptr()); if disk_unlock_native_capacity(disk) { free_partitions(state); return -EAGAIN; } }
    kobject_uevent(&mut (*disk_to_dev(disk)).kobj, KOBJ_CHANGE);
    let mut p = 1; while p < (*state).limit { if !blk_add_partition(disk, state, p) { break; } p += 1; }
    free_partitions(state); 0
}

pub unsafe fn bdev_disk_changed(disk: *mut gendisk, invalidate: bool) -> i32 {
    lockdep_assert_held(&(*disk).open_mutex); if !disk_live(disk) { return -ENXIO; }
    loop { if (*disk).open_partitions != 0 { return -EBUSY; } sync_blockdev((*disk).part0); invalidate_bdev((*disk).part0); let mut idx: c_ulong = 0; let mut part: *mut block_device = core::ptr::null_mut(); xa_for_each_start(&(*disk).part_tbl, &mut idx, &mut part, 1, { bdev_unhash(part); WARN_ON_ONCE(atomic_read(&(*part).bd_openers) != 0); invalidate_bdev(part); drop_partition(part); }); clear_bit(GD_NEED_PART_SCAN, &mut (*disk).state); if invalidate && ((*disk).flags & GENHD_FL_NO_PART == 0 || (*disk).flags & GENHD_FL_REMOVABLE == 0) { set_capacity(disk, 0); } let mut ret = 0; if get_capacity(disk) != 0 { ret = blk_add_partitions(disk); if ret == -EAGAIN { continue; } } else if invalidate { kobject_uevent(&mut (*disk_to_dev(disk)).kobj, KOBJ_CHANGE); } return ret; }
}

// Only exported for loop and dasd for historic reasons. Don't use in new code.
EXPORT_SYMBOL_GPL!(bdev_disk_changed);

pub unsafe fn read_part_sector(state: *mut parsed_partitions, n: sector_t, p: *mut Sector) -> *mut core::ffi::c_void {
    let mapping = (*(*state).disk).part0.bd_mapping; if n >= get_capacity((*state).disk) { (*state).access_beyond_eod = true; (*p).v = core::ptr::null_mut(); return core::ptr::null_mut(); }
    let folio = read_mapping_folio(mapping, n >> PAGE_SECTORS_SHIFT, core::ptr::null_mut()); if IS_ERR(folio) { (*p).v = core::ptr::null_mut(); return core::ptr::null_mut(); }
    (*p).v = folio; folio_address(folio).add(offset_in_folio(folio, n * SECTOR_SIZE))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
