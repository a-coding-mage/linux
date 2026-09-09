// SPDX-License-Identifier: GPL-2.0
/*
 * Support for the N64 cart.
 *
 * Copyright (c) 2021 Lauri Kasanen
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

enum {
    PI_DRAM_REG = 0,
    PI_CART_REG,
    PI_READ_REG,
    PI_WRITE_REG,
    PI_STATUS_REG,
}

const PI_STATUS_DMA_BUSY: u32 = 1 << 0;
const PI_STATUS_IO_BUSY: u32 = 1 << 1;

const CART_DOMAIN: u32 = 0x10000000;
const CART_MAX: u32 = 0x1FFFFFFF;

const MIN_ALIGNMENT: usize = 8;

static mut reg_base: *mut u32 = core::ptr::null_mut();

static mut start: u32 = 0;
static mut size: u32 = 0;

unsafe fn n64cart_write_reg(reg: u8, value: u32) {
    // writel(value, reg_base + reg)
    core::ptr::write_volatile(reg_base.add(reg as usize), value);
}

unsafe fn n64cart_read_reg(reg: u8) -> u32 {
    core::ptr::read_volatile(reg_base.add(reg as usize))
}

unsafe fn n64cart_wait_dma() {
    while n64cart_read_reg(PI_STATUS_REG as u8)
        & (PI_STATUS_DMA_BUSY | PI_STATUS_IO_BUSY) != 0
    {
        core::hint::spin_loop();
    }
}

/*
 * Process a single bvec of a bio.
 */
unsafe fn n64cart_do_bvec(
    dev: *mut device,
    bv: *mut bio_vec,
    pos: u32,
) -> bool {
    let bstart = pos.wrapping_add(start);

    /* Alignment check */
    // WARN_ON_ONCE((bv->bv_offset & (MIN_ALIGNMENT - 1)) ||
    //              (bv->bv_len & (MIN_ALIGNMENT - 1)));

    let dma_addr = dma_map_bvec(dev, bv, DMA_FROM_DEVICE, 0);
    if dma_mapping_error(dev, dma_addr) {
        return false;
    }

    n64cart_wait_dma();

    n64cart_write_reg(PI_DRAM_REG as u8, dma_addr);
    n64cart_write_reg(
        PI_CART_REG as u8,
        (bstart | CART_DOMAIN) & CART_MAX,
    );
    n64cart_write_reg(
        PI_WRITE_REG as u8,
        (*bv).bv_len.wrapping_sub(1),
    );

    n64cart_wait_dma();

    dma_unmap_page(dev, dma_addr, (*bv).bv_len, DMA_FROM_DEVICE);
    true
}

unsafe fn n64cart_submit_bio(bio: *mut bio) {
    let dev = (*(*bio).bi_bdev).bd_disk.private_data as *mut device;
    let mut pos = (*bio).bi_iter.bi_sector << SECTOR_SHIFT;
    let mut iter = bvec_iter::default();
    let mut bvec = bio_vec::default();

    // bio_for_each_segment(bvec, bio, iter)
    while bio_for_each_segment(&mut bvec, bio, &mut iter) {
        if !n64cart_do_bvec(dev, &mut bvec, pos) {
            bio_io_error(bio);
            return;
        }
        pos = pos.wrapping_add(bvec.bv_len);
    }

    bio_endio(bio);
}

static n64cart_fops: block_device_operations = block_device_operations {
    owner: THIS_MODULE,
    submit_bio: Some(n64cart_submit_bio),
};

/*
 * The target device is embedded and RAM-constrained. We save RAM
 * by initializing in __init code that gets dropped late in boot.
 * For the same reason there is no module or unloading support.
 */
unsafe fn n64cart_probe(pdev: *mut platform_device) -> i32 {
    let lim = queue_limits {
        physical_block_size: 4096,
        logical_block_size: 4096,
    };
    let mut disk: *mut gendisk;
    let mut err: i32 = -ENOMEM;

    if start == 0 || size == 0 {
        pr_err!("start or size not specified\n");
        return -ENODEV;
    }

    if size & 4095 != 0 {
        pr_err!("size must be a multiple of 4K\n");
        return -ENODEV;
    }

    reg_base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg_base) {
        return PTR_ERR(reg_base);
    }

    disk = blk_alloc_disk(&lim, NUMA_NO_NODE);
    if IS_ERR(disk) {
        err = PTR_ERR(disk);
        goto out;
    }

    (*disk).first_minor = 0;
    (*disk).flags = GENHD_FL_NO_PART;
    (*disk).fops = &n64cart_fops;
    (*disk).private_data = &mut (*pdev).dev as *mut device as *mut core::ffi::c_void;
    strscpy((*disk).disk_name.as_mut_ptr(), b"n64cart\0".as_ptr(), (*disk).disk_name.len());

    set_capacity(disk, size >> SECTOR_SHIFT);
    set_disk_ro(disk, 1);

    err = add_disk(disk);
    if err != 0 {
        goto out_cleanup_disk;
    }

    pr_info!("n64cart: %u kb disk\n", size / 1024);

    return 0;

out_cleanup_disk:
    put_disk(disk);
out:
    err
}

static mut n64cart_driver: platform_driver = platform_driver {
    driver: driver {
        name: b"n64cart\0".as_ptr(),
    },
};

unsafe fn n64cart_init() -> i32 {
    platform_driver_probe(&mut n64cart_driver, n64cart_probe)
}

// module_init(n64cart_init)

// MODULE_AUTHOR("Lauri Kasanen <cand@gmx.com>");
// MODULE_DESCRIPTION("Driver for the N64 cart");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
