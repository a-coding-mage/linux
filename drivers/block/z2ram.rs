/*
** z2ram - Amiga pseudo-driver to access 16bit-RAM in ZorroII space
**         as a block device, to be used as a RAM disk or swap space
**
** Copyright (C) 1994 by Ingo Wilken (Ingo.Wilken@informatik.uni-oldenburg.de)
**
** Permission to use, copy, modify, and distribute this software and its
** documentation for any purpose and without fee is hereby granted, provided
** that the above copyright notice appear in all copies and that both that
** copyright notice and this permission notice appear in supporting
** documentation.  This software is provided "as is" without express or
** implied warranty.
*/

// Kernel and architecture dependencies supplied by the surrounding tree.

const DEVICE_NAME: &str = "Z2RAM";
const Z2MINOR_COMBINED: i32 = 0;
const Z2MINOR_Z2ONLY: i32 = 1;
const Z2MINOR_CHIPONLY: i32 = 2;
const Z2MINOR_MEMLIST1: i32 = 4;
const Z2MINOR_MEMLIST2: i32 = 5;
const Z2MINOR_MEMLIST3: i32 = 6;
const Z2MINOR_MEMLIST4: i32 = 7;
const Z2MINOR_COUNT: usize = 8;
const Z2RAM_CHUNK1024: usize = Z2RAM_CHUNKSIZE >> 10;

static mut z2ram_map: *mut u_long = core::ptr::null_mut();
static mut z2ram_size: u_long = 0;
static mut z2_count: i32 = 0;
static mut chip_count: i32 = 0;
static mut list_count: i32 = 0;
static mut current_device: i32 = -1;
static mut z2ram_gendisk: [*mut gendisk; Z2MINOR_COUNT] = [core::ptr::null_mut(); Z2MINOR_COUNT];

static mut z2ram_mutex: mutex = unsafe { core::mem::zeroed() };
static mut z2ram_lock: spinlock = unsafe { core::mem::zeroed() };

unsafe fn z2_queue_rq(_hctx: *mut blk_mq_hw_ctx, bd: *const blk_mq_queue_data) -> blk_status_t {
    let req = (*bd).rq;
    let mut start = blk_rq_pos(req) << 9;
    let mut len = blk_rq_cur_bytes(req);
    blk_mq_start_request(req);

    if start + len > z2ram_size {
        pr_err!("{}: bad access: block={}, count={}\n", DEVICE_NAME,
            blk_rq_pos(req) as u64, blk_rq_cur_sectors(req));
        return BLK_STS_IOERR;
    }

    spin_lock_irq(&mut z2ram_lock);
    while len != 0 {
        let mut addr = start & Z2RAM_CHUNKMASK;
        let mut size = Z2RAM_CHUNKSIZE - addr;
        let buffer = bio_data((*req).bio);
        if len < size { size = len; }
        addr += *z2ram_map.add((start >> Z2RAM_CHUNKSHIFT) as usize) as usize;
        if rq_data_dir(req) == READ {
            core::ptr::copy_nonoverlapping(addr as *const u8, buffer as *mut u8, size as usize);
        } else {
            core::ptr::copy_nonoverlapping(buffer as *const u8, addr as *mut u8, size as usize);
        }
        start += size;
        len -= size;
    }
    spin_unlock_irq(&mut z2ram_lock);
    blk_mq_end_request(req, BLK_STS_OK);
    BLK_STS_OK
}

unsafe fn get_z2ram() {
    for i in 0..(Z2RAM_SIZE / Z2RAM_CHUNKSIZE) {
        if test_bit(i, zorro_unused_z2ram) != 0 {
            z2_count += 1;
            *z2ram_map.add(z2ram_size as usize) = ZTWO_VADDR(Z2RAM_START) as u_long + ((i << Z2RAM_CHUNKSHIFT) as u_long);
            z2ram_size += 1;
            clear_bit(i, zorro_unused_z2ram);
        }
    }
}

unsafe fn get_chipram() {
    while amiga_chip_avail() > (Z2RAM_CHUNKSIZE * 4) {
        chip_count += 1;
        *z2ram_map.add(z2ram_size as usize) = amiga_chip_alloc(Z2RAM_CHUNKSIZE, b"z2ram\0".as_ptr() as *const i8) as u_long;
        if *z2ram_map.add(z2ram_size as usize) == 0 { break; }
        z2ram_size += 1;
    }
}

unsafe fn z2_open(disk: *mut gendisk, _mode: blk_mode_t) -> i32 {
    let device = (*disk).first_minor;
    let max_z2_map = (Z2RAM_SIZE / Z2RAM_CHUNKSIZE) * core::mem::size_of::<u_long>();
    let max_chip_map = (amiga_chip_size / Z2RAM_CHUNKSIZE) * core::mem::size_of::<u_long>();
    let mut rc = -ENOMEM;
    mutex_lock(&mut z2ram_mutex);
    if current_device != -1 && current_device != device { rc = -EBUSY; goto_err!(); }
    if current_device == -1 {
        z2_count = 0; chip_count = 0; list_count = 0; z2ram_size = 0;
        if device >= Z2MINOR_MEMLIST1 && device <= Z2MINOR_MEMLIST4 {
            let index = device - Z2MINOR_MEMLIST1 + 1;
            let mut size = m68k_memory[index as usize].size & !(Z2RAM_CHUNKSIZE - 1);
            let mut vaddr = z_remap_nocache_nonser(m68k_memory[index as usize].addr, size) as u_long;
            z2ram_map = kmalloc_objs(z2ram_map as usize, size / Z2RAM_CHUNKSIZE) as *mut u_long;
            if z2ram_map.is_null() { goto_err!(); }
            while size != 0 { *z2ram_map.add(z2ram_size as usize) = vaddr; z2ram_size += 1; size -= Z2RAM_CHUNKSIZE; vaddr += Z2RAM_CHUNKSIZE as u_long; list_count += 1; }
        } else {
            match device {
                Z2MINOR_COMBINED => { z2ram_map = kmalloc(max_z2_map + max_chip_map, GFP_KERNEL) as *mut u_long; if z2ram_map.is_null() { goto_err!(); } get_z2ram(); get_chipram(); }
                Z2MINOR_Z2ONLY => { z2ram_map = kmalloc(max_z2_map, GFP_KERNEL) as *mut u_long; if z2ram_map.is_null() { goto_err!(); } get_z2ram(); }
                Z2MINOR_CHIPONLY => { z2ram_map = kmalloc(max_chip_map, GFP_KERNEL) as *mut u_long; if z2ram_map.is_null() { goto_err!(); } get_chipram(); }
                _ => { rc = -ENODEV; goto_err!(); }
            }
        }
        if z2ram_size == 0 { kfree(z2ram_map as *mut core::ffi::c_void); goto_err!(); }
        current_device = device; z2ram_size <<= Z2RAM_CHUNKSHIFT; set_capacity(z2ram_gendisk[device as usize], z2ram_size >> 9);
    }
    mutex_unlock(&mut z2ram_mutex); return 0;
}

unsafe fn z2_release(_disk: *mut gendisk) { mutex_lock(&mut z2ram_mutex); if current_device == -1 { mutex_unlock(&mut z2ram_mutex); return; } mutex_unlock(&mut z2ram_mutex); /* FIXME: unmap memory */ }

#[allow(dead_code)]
unsafe fn z2ram_register_disk(minor: i32) -> i32 {
    let disk = blk_mq_alloc_disk(&mut tag_set, core::ptr::null_mut(), core::ptr::null_mut());
    if is_err(disk) { return ptr_err(disk); }
    (*disk).major = Z2RAM_MAJOR;
    (*disk).first_minor = minor;
    (*disk).minors = 1;
    (*disk).flags |= GENHD_FL_NO_PART;
    (*disk).fops = &z2_fops;
    if minor != 0 { sprintf((*disk).disk_name.as_mut_ptr(), b"z2ram%d\0".as_ptr(), minor); }
    else { sprintf((*disk).disk_name.as_mut_ptr(), b"z2ram\0".as_ptr()); }
    z2ram_gendisk[minor as usize] = disk;
    let err = add_disk(disk);
    if err != 0 { put_disk(disk); }
    err
}

unsafe fn z2_init() -> i32 {
    let mut ret: i32;
    if !MACH_IS_AMIGA { return -ENODEV; }
    if register_blkdev(Z2RAM_MAJOR, DEVICE_NAME) != 0 { return -EBUSY; }
    tag_set.ops = &z2_mq_ops;
    tag_set.nr_hw_queues = 1;
    tag_set.nr_maps = 1;
    tag_set.queue_depth = 16;
    tag_set.numa_node = NUMA_NO_NODE;
    ret = blk_mq_alloc_tag_set(&mut tag_set);
    if ret != 0 { unregister_blkdev(Z2RAM_MAJOR, DEVICE_NAME); return ret; }
    for i in 0..Z2MINOR_COUNT {
        ret = z2ram_register_disk(i as i32);
        if ret != 0 && i == 0 { blk_mq_free_tag_set(&mut tag_set); unregister_blkdev(Z2RAM_MAJOR, DEVICE_NAME); return ret; }
    }
    0
}

unsafe fn z2_exit() {
    unregister_blkdev(Z2RAM_MAJOR, DEVICE_NAME);
    for i in 0..Z2MINOR_COUNT { del_gendisk(z2ram_gendisk[i]); put_disk(z2ram_gendisk[i]); }
    blk_mq_free_tag_set(&mut tag_set);
    if current_device != -1 {
        let mut i = 0usize;
        for _ in 0..z2_count { set_bit(i as i32, zorro_unused_z2ram); i += 1; }
        for _ in 0..chip_count { if *z2ram_map.add(i) != 0 { amiga_chip_free(*z2ram_map.add(i) as *mut core::ffi::c_void); i += 1; } }
        if !z2ram_map.is_null() { kfree(z2ram_map as *mut core::ffi::c_void); }
    }
}

// module_init(z2_init); module_exit(z2_exit);
// MODULE_DESCRIPTION("Amiga Zorro II ramdisk driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
