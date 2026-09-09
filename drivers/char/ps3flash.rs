// SPDX-License-Identifier: GPL-2.0-only
/*
 * PS3 FLASH ROM Storage Driver
 *
 * Copyright (C) 2007 Sony Computer Entertainment Inc.
 * Copyright 2007 Sony Corp.
 */

// Linux kernel dependencies and architecture-specific declarations are supplied
// by the surrounding kernel translation.

const DEVICE_NAME: &str = "ps3flash";
const FLASH_BLOCK_SIZE: usize = 256 * 1024;

#[repr(C)]
struct Ps3flashPrivate {
    mutex: mutex,
    chunk_sectors: u64,
    tag: i32,
    dirty: bool,
}

static mut ps3flash_dev: *mut ps3_storage_device = core::ptr::null_mut();

unsafe fn ps3flash_read_write_sectors(dev: *mut ps3_storage_device,
                                      start_sector: u64, write: i32) -> i32 {
    let priv_ = ps3_system_bus_get_drvdata((*dev).sbd) as *mut Ps3flashPrivate;
    let res = ps3stor_read_write_sectors(dev, (*dev).bounce_lpar,
                                         start_sector, (*priv_).chunk_sectors,
                                         write);
    if res != 0 {
        dev_err((*dev).sbd.core, "{}:{}: {} failed 0x{:x}\n",
                "ps3flash_read_write_sectors", line!(),
                if write != 0 { "write" } else { "read" }, res);
        return -EIO;
    }
    0
}

unsafe fn ps3flash_writeback(dev: *mut ps3_storage_device) -> i32 {
    let priv_ = ps3_system_bus_get_drvdata((*dev).sbd) as *mut Ps3flashPrivate;
    if !(*priv_).dirty || (*priv_).tag < 0 { return 0; }
    let res = ps3flash_read_write_sectors(dev, (*priv_).tag as u64, 1);
    if res != 0 { return res; }
    (*priv_).dirty = false;
    0
}

unsafe fn ps3flash_fetch(dev: *mut ps3_storage_device, start_sector: u64) -> i32 {
    let priv_ = ps3_system_bus_get_drvdata((*dev).sbd) as *mut Ps3flashPrivate;
    if start_sector == (*priv_).tag as u64 { return 0; }
    let res = ps3flash_writeback(dev);
    if res != 0 { return res; }
    (*priv_).tag = -1;
    let res = ps3flash_read_write_sectors(dev, start_sector, 0);
    if res != 0 { return res; }
    (*priv_).tag = start_sector as i32;
    0
}

unsafe fn ps3flash_llseek(file: *mut file, offset: loff_t, origin: i32) -> loff_t {
    let dev = ps3flash_dev;
    generic_file_llseek_size(file, offset, origin, MAX_LFS_FILESIZE,
        (*dev).regions[(*dev).region_idx].size * (*dev).blk_size)
}

unsafe fn ps3flash_read(mut userbuf: *mut u8, mut kernelbuf: *mut u8,
                        mut count: usize, pos: *mut loff_t) -> isize {
    let dev = ps3flash_dev;
    let priv_ = ps3_system_bus_get_drvdata((*dev).sbd) as *mut Ps3flashPrivate;
    let size = (*dev).regions[(*dev).region_idx].size * (*dev).blk_size;
    if *pos >= size as loff_t || count == 0 { return 0; }
    if *pos + count as loff_t > size as loff_t { count = (size as loff_t - *pos) as usize; }
    let mut sector = (*pos as u64 / (*dev).bounce_size) * (*priv_).chunk_sectors;
    let mut offset = *pos as u64 % (*dev).bounce_size;
    let mut remaining = count;
    while remaining > 0 {
        let n = core::cmp::min(remaining as u64, (*dev).bounce_size - offset) as usize;
        let src = (*dev).bounce_buf.add(offset as usize);
        mutex_lock(&mut (*priv_).mutex);
        let res = ps3flash_fetch(dev, sector);
        if res != 0 { mutex_unlock(&mut (*priv_).mutex); return res as isize; }
        if !userbuf.is_null() {
            if copy_to_user(userbuf, src, n) != 0 { mutex_unlock(&mut (*priv_).mutex); return -EFAULT as isize; }
            userbuf = userbuf.add(n);
        }
        if !kernelbuf.is_null() { memcpy(kernelbuf, src, n); kernelbuf = kernelbuf.add(n); }
        mutex_unlock(&mut (*priv_).mutex);
        *pos += n as loff_t; remaining -= n; sector += (*priv_).chunk_sectors; offset = 0;
    }
    count as isize
}

unsafe fn ps3flash_write(mut userbuf: *const u8, mut kernelbuf: *const u8,
                         mut count: usize, pos: *mut loff_t) -> isize {
    let dev = ps3flash_dev;
    let priv_ = ps3_system_bus_get_drvdata((*dev).sbd) as *mut Ps3flashPrivate;
    let size = (*dev).regions[(*dev).region_idx].size * (*dev).blk_size;
    if *pos >= size as loff_t || count == 0 { return 0; }
    if *pos + count as loff_t > size as loff_t { count = (size as loff_t - *pos) as usize; }
    let mut sector = (*pos as u64 / (*dev).bounce_size) * (*priv_).chunk_sectors;
    let mut offset = *pos as u64 % (*dev).bounce_size;
    let mut remaining = count;
    while remaining > 0 {
        let n = core::cmp::min(remaining as u64, (*dev).bounce_size - offset) as usize;
        let dst = (*dev).bounce_buf.add(offset as usize);
        mutex_lock(&mut (*priv_).mutex);
        let res = if n != (*dev).bounce_size as usize { ps3flash_fetch(dev, sector) } else if sector != (*priv_).tag as u64 { ps3flash_writeback(dev) } else { 0 };
        if res != 0 { mutex_unlock(&mut (*priv_).mutex); return res as isize; }
        if !userbuf.is_null() { if copy_from_user(dst, userbuf, n) != 0 { mutex_unlock(&mut (*priv_).mutex); return -EFAULT as isize; } userbuf = userbuf.add(n); }
        if !kernelbuf.is_null() { memcpy(dst, kernelbuf, n); kernelbuf = kernelbuf.add(n); }
        (*priv_).tag = sector as i32; (*priv_).dirty = true;
        mutex_unlock(&mut (*priv_).mutex);
        *pos += n as loff_t; remaining -= n; sector += (*priv_).chunk_sectors; offset = 0;
    }
    count as isize
}

unsafe fn ps3flash_user_read(_: *mut file, buf: *mut u8, count: usize, pos: *mut loff_t) -> isize { ps3flash_read(buf, core::ptr::null_mut(), count, pos) }
unsafe fn ps3flash_user_write(_: *mut file, buf: *const u8, count: usize, pos: *mut loff_t) -> isize { ps3flash_write(buf, core::ptr::null(), count, pos) }
unsafe fn ps3flash_kernel_read(buf: *mut u8, count: usize, mut pos: loff_t) -> isize { ps3flash_read(core::ptr::null_mut(), buf, count, &mut pos) }
unsafe fn ps3flash_kernel_write(buf: *const u8, count: usize, mut pos: loff_t) -> isize {
    let res = ps3flash_write(core::ptr::null(), buf, count, &mut pos);
    if res < 0 { return res; }
    let wb = ps3flash_writeback(ps3flash_dev);
    if wb != 0 { return wb as isize; }
    res
}

unsafe fn ps3flash_flush(_: *mut file, _: fl_owner_t) -> i32 { ps3flash_writeback(ps3flash_dev) }
unsafe fn ps3flash_fsync(file: *mut file, _: loff_t, _: loff_t, _: i32) -> i32 {
    let inode = file_inode(file); inode_lock(inode); let err = ps3flash_writeback(ps3flash_dev); inode_unlock(inode); err
}

unsafe fn ps3flash_interrupt(_: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = data as *mut ps3_storage_device; let mut tag = 0; let mut status = 0;
    let res = lv1_storage_get_async_status((*dev).sbd.dev_id, &mut tag, &mut status);
    if res != 0 { dev_err((*dev).sbd.core, "ps3flash interrupt: res={} status=0x{:x}\n", res, status); }
    else { (*dev).lv1_status = status; complete(&mut (*dev).done); }
    IRQ_HANDLED
}

static mut ps3flash_fops: file_operations = file_operations {
    owner: THIS_MODULE, llseek: Some(ps3flash_llseek), read: Some(ps3flash_user_read),
    write: Some(ps3flash_user_write), flush: Some(ps3flash_flush), fsync: Some(ps3flash_fsync),
};
static mut ps3flash_kernel_ops: ps3_os_area_flash_ops = ps3_os_area_flash_ops {
    read: Some(ps3flash_kernel_read), write: Some(ps3flash_kernel_write),
};
static mut ps3flash_misc: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR, name: DEVICE_NAME.as_ptr() as *const i8,
    fops: &ps3flash_fops, parent: core::ptr::null_mut(),
};

unsafe fn ps3flash_probe(_dev: *mut ps3_system_bus_device) -> i32 {
    let dev = to_ps3_storage_device(&mut (*_dev).core);
    let mut tmp = (*dev).regions[(*dev).region_idx].start * (*dev).blk_size;
    if tmp % FLASH_BLOCK_SIZE as u64 != 0 { return -EINVAL; }
    tmp = (*dev).regions[(*dev).region_idx].size * (*dev).blk_size;
    if tmp % FLASH_BLOCK_SIZE as u64 != 0 { return -EINVAL; }
    if ps3flash_bounce_buffer.address.is_null() { return -ENODEV; }
    if !ps3flash_dev.is_null() { return -EBUSY; }
    ps3flash_dev = dev;
    let priv_ = kzalloc_obj::<Ps3flashPrivate>();
    if priv_.is_null() { ps3flash_dev = core::ptr::null_mut(); return -ENOMEM; }
    ps3_system_bus_set_drvdata((*dev).sbd, priv_ as *mut core::ffi::c_void);
    mutex_init(&mut (*priv_).mutex); (*priv_).tag = -1;
    (*dev).bounce_size = ps3flash_bounce_buffer.size;
    (*dev).bounce_buf = ps3flash_bounce_buffer.address;
    (*priv_).chunk_sectors = (*dev).bounce_size / (*dev).blk_size;
    let error = ps3stor_setup(dev, ps3flash_interrupt);
    if error != 0 { kfree(priv_); ps3_system_bus_set_drvdata((*dev).sbd, core::ptr::null_mut()); ps3flash_dev = core::ptr::null_mut(); return error; }
    ps3flash_misc.parent = &mut (*dev).sbd.core;
    let error = misc_register(&mut ps3flash_misc);
    if error != 0 { ps3stor_teardown(dev); kfree(priv_); ps3_system_bus_set_drvdata((*dev).sbd, core::ptr::null_mut()); ps3flash_dev = core::ptr::null_mut(); return error; }
    ps3_os_area_flash_register(&ps3flash_kernel_ops); 0
}

unsafe fn ps3flash_remove(_dev: *mut ps3_system_bus_device) {
    let dev = to_ps3_storage_device(&mut (*_dev).core);
    ps3_os_area_flash_register(core::ptr::null()); misc_deregister(&mut ps3flash_misc);
    ps3stor_teardown(dev); kfree(ps3_system_bus_get_drvdata((*dev).sbd));
    ps3_system_bus_set_drvdata((*dev).sbd, core::ptr::null_mut()); ps3flash_dev = core::ptr::null_mut();
}

static mut ps3flash: ps3_system_bus_driver = ps3_system_bus_driver {
    match_id: PS3_MATCH_ID_STOR_FLASH, core: device_driver { name: DEVICE_NAME.as_ptr() as *const i8, owner: THIS_MODULE },
    probe: Some(ps3flash_probe), remove: Some(ps3flash_remove), shutdown: Some(ps3flash_remove),
};
unsafe fn ps3flash_init() -> i32 { ps3_system_bus_driver_register(&mut ps3flash) }
unsafe fn ps3flash_exit() { ps3_system_bus_driver_unregister(&mut ps3flash); }

module_init!(ps3flash_init);
module_exit!(ps3flash_exit);
module_license!("GPL");
module_description!("PS3 FLASH ROM Storage Driver");
module_author!("Sony Corporation");
module_alias!(PS3_MODULE_ALIAS_STOR_FLASH);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
