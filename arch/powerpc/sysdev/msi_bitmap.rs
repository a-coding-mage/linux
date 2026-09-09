// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2006-2008, Michael Ellerman, IBM Corporation.
 */

// Kernel dependencies supplied by other translation units are intentionally
// left as external names.

pub unsafe fn msi_bitmap_alloc_hwirqs(bmp: *mut msi_bitmap, num: i32) -> i32 {
    let mut flags: c_ulong = 0;
    let order: i32 = get_count_order(num);

    spin_lock_irqsave(&mut (*bmp).lock, &mut flags);

    let offset: i32 = bitmap_find_next_zero_area(
        (*bmp).bitmap,
        (*bmp).irq_count,
        0,
        num as usize,
        ((1i32 << order) - 1) as usize,
    ) as i32;
    if offset >= (*bmp).irq_count as i32 {
        spin_unlock_irqrestore(&mut (*bmp).lock, flags);
        return -ENOMEM;
    }

    bitmap_set((*bmp).bitmap, offset as usize, num as usize);
    spin_unlock_irqrestore(&mut (*bmp).lock, flags);

    pr_debug!("msi_bitmap: allocated 0x{:x} at offset 0x{:x}\n", num, offset);

    offset
}

pub unsafe fn msi_bitmap_free_hwirqs(
    bmp: *mut msi_bitmap,
    offset: c_uint,
    num: c_uint,
) {
    let mut flags: c_ulong = 0;

    pr_debug!(
        "msi_bitmap: freeing 0x{:x} at offset 0x{:x}\n",
        num,
        offset
    );

    spin_lock_irqsave(&mut (*bmp).lock, &mut flags);
    bitmap_clear((*bmp).bitmap, offset as usize, num as usize);
    spin_unlock_irqrestore(&mut (*bmp).lock, flags);
}

pub unsafe fn msi_bitmap_reserve_hwirq(bmp: *mut msi_bitmap, hwirq: c_uint) {
    let mut flags: c_ulong = 0;

    pr_debug!("msi_bitmap: reserving hwirq 0x{:x}\n", hwirq);

    spin_lock_irqsave(&mut (*bmp).lock, &mut flags);
    bitmap_allocate_region((*bmp).bitmap, hwirq as usize, 0);
    spin_unlock_irqrestore(&mut (*bmp).lock, flags);
}

/**
 * msi_bitmap_reserve_dt_hwirqs - Reserve irqs specified in the device tree.
 * @bmp: pointer to the MSI bitmap.
 *
 * Looks in the device tree to see if there is a property specifying which
 * irqs can be used for MSI. If found those irqs reserved in the device tree
 * are reserved in the bitmap.
 *
 * Returns 0 for success, < 0 if there was an error, and > 0 if no property
 * was found in the device tree.
 **/
pub unsafe fn msi_bitmap_reserve_dt_hwirqs(bmp: *mut msi_bitmap) -> i32 {
    let mut len: i32 = 0;
    let mut p: *const u32;

    if (*bmp).of_node.is_null() {
        return 1;
    }

    p = of_get_property((*bmp).of_node, c"msi-available-ranges".as_ptr(), &mut len);
    if p.is_null() {
        pr_debug!(
            "msi_bitmap: no msi-available-ranges property found on %pOF\n",
            (*bmp).of_node
        );
        return 1;
    }

    if len % (2 * size_of::<u32>() as i32) != 0 {
        printk!("msi_bitmap: Malformed msi-available-ranges property on %pOF\n", (*bmp).of_node);
        return -EINVAL;
    }

    bitmap_allocate_region(
        (*bmp).bitmap,
        0,
        get_count_order((*bmp).irq_count as i32),
    );

    spin_lock(&mut (*bmp).lock);

    /* Format is: (<u32 start> <u32 count>)+ */
    len /= 2 * size_of::<u32>() as i32;
    for _i in 0..len {
        for j in 0..*p.add(1) {
            bitmap_release_region((*bmp).bitmap, (*p as usize) + j as usize, 0);
        }
        p = p.add(2);
    }

    spin_unlock(&mut (*bmp).lock);

    0
}

pub unsafe fn msi_bitmap_alloc(
    bmp: *mut msi_bitmap,
    irq_count: c_uint,
    of_node: *mut device_node,
) -> i32 {
    let size: usize;

    if irq_count == 0 {
        return -EINVAL;
    }

    size = bits_to_longs(irq_count as usize) * size_of::<c_ulong>();
    pr_debug!("msi_bitmap: allocator bitmap size is 0x{:x} bytes\n", size);

    (*bmp).bitmap_from_slab = slab_is_available();
    if (*bmp).bitmap_from_slab {
        (*bmp).bitmap = kzalloc(size, GFP_KERNEL);
    } else {
        (*bmp).bitmap = memblock_alloc_or_panic(size, SMP_CACHE_BYTES);
        /* the bitmap won't be freed from memblock allocator */
        kmemleak_not_leak((*bmp).bitmap);
    }

    if (*bmp).bitmap.is_null() {
        pr_debug!("msi_bitmap: ENOMEM allocating allocator bitmap!\n");
        return -ENOMEM;
    }

    /* We zalloc'ed the bitmap, so all irqs are free by default */
    spin_lock_init(&mut (*bmp).lock);
    (*bmp).of_node = of_node_get(of_node);
    (*bmp).irq_count = irq_count;

    0
}

pub unsafe fn msi_bitmap_free(bmp: *mut msi_bitmap) {
    if (*bmp).bitmap_from_slab {
        kfree((*bmp).bitmap);
    }
    of_node_put((*bmp).of_node);
    (*bmp).bitmap = core::ptr::null_mut();
}

// The following block is retained under the original build-time condition.
#[cfg(CONFIG_MSI_BITMAP_SELFTEST)]
mod selftest {
    use super::*;

    unsafe fn test_basics() {
        let mut bmp: msi_bitmap = core::mem::zeroed();
        let size: i32 = 512;

        warn_on!(msi_bitmap_alloc(&mut bmp, 0, core::ptr::null_mut()) == 0);
        warn_on!(msi_bitmap_alloc(&mut bmp, size as c_uint, core::ptr::null_mut()) != 0);
        warn_on!(bitmap_find_free_region(bmp.bitmap, size as usize, get_count_order(size)) != 0);
        bitmap_release_region(bmp.bitmap, 0, get_count_order(size));
        warn_on!(msi_bitmap_reserve_dt_hwirqs(&mut bmp) <= 0);
        warn_on!(bitmap_find_free_region(bmp.bitmap, size as usize, get_count_order(size)) != 0);
        bitmap_release_region(bmp.bitmap, 0, get_count_order(size));

        for _i in 0..size {
            warn_on!(msi_bitmap_alloc_hwirqs(&mut bmp, 1) < 0);
        }
        warn_on!(msi_bitmap_alloc_hwirqs(&mut bmp, 1) >= 0);
        warn_on!(bitmap_find_free_region(bmp.bitmap, size as usize, 0) >= 0);
        msi_bitmap_free_hwirqs(&mut bmp, (size / 2) as c_uint, 1);
        warn_on!(msi_bitmap_alloc_hwirqs(&mut bmp, 1) != size / 2);
        msi_bitmap_free_hwirqs(&mut bmp, 3, (size - 3) as c_uint);

        let mut rc = msi_bitmap_alloc_hwirqs(&mut bmp, 2);
        warn_on!(rc < 0 && rc % 2 != 0);
        rc = msi_bitmap_alloc_hwirqs(&mut bmp, 4);
        warn_on!(rc < 0 && rc % 4 != 0);
        rc = msi_bitmap_alloc_hwirqs(&mut bmp, 8);
        warn_on!(rc < 0 && rc % 8 != 0);
        rc = msi_bitmap_alloc_hwirqs(&mut bmp, 9);
        warn_on!(rc < 0 && rc % 16 != 0);
        rc = msi_bitmap_alloc_hwirqs(&mut bmp, 3);
        warn_on!(rc < 0 && rc % 4 != 0);
        rc = msi_bitmap_alloc_hwirqs(&mut bmp, 7);
        warn_on!(rc < 0 && rc % 8 != 0);
        rc = msi_bitmap_alloc_hwirqs(&mut bmp, 121);
        warn_on!(rc < 0 && rc % 128 != 0);
        msi_bitmap_free(&mut bmp);
        warn_on!(!bmp.bitmap.is_null());
    }

    unsafe fn test_of_node() {
        let prop_data: [u32; 10] = [10, 10, 25, 3, 40, 1, 100, 100, 200, 20];
        let expected_str = "0-9,20-24,28-39,41-99,220-255";
        let prop_name = "msi-available-ranges";
        let node_name = "/fakenode";
        let mut of_node: device_node = core::mem::zeroed();
        let mut prop: property = core::mem::zeroed();
        let mut bmp: msi_bitmap = core::mem::zeroed();
        const SIZE_EXPECTED: usize = 256;
        let mut expected = [0 as c_ulong; (SIZE_EXPECTED + (8 * size_of::<c_ulong>()) - 1)
            / (8 * size_of::<c_ulong>())];

        core::ptr::write_bytes(
            &mut of_node as *mut device_node as *mut u8,
            0,
            size_of::<device_node>(),
        );
        of_node_init(&mut of_node);
        of_node.full_name = node_name.as_ptr() as *mut c_char;

        warn_on!(msi_bitmap_alloc(&mut bmp, SIZE_EXPECTED as c_uint, &mut of_node) != 0);
        warn_on!(msi_bitmap_reserve_dt_hwirqs(&mut bmp) <= 0);
        warn_on!(bitmap_find_free_region(
            bmp.bitmap,
            SIZE_EXPECTED,
            get_count_order(SIZE_EXPECTED as i32)
        ) != 0);
        bitmap_release_region(bmp.bitmap, 0, get_count_order(SIZE_EXPECTED as i32));

        /* Now create a fake msi-available-ranges property */
        /* There should really .. oh whatever */
        core::ptr::write_bytes(
            &mut prop as *mut property as *mut u8,
            0,
            size_of::<property>(),
        );
        prop.name = prop_name.as_ptr() as *mut c_char;
        prop.value = prop_data.as_ptr() as *const c_void;
        prop.length = size_of_val(&prop_data) as c_int;
        of_node.properties = &mut prop;

        /* msi-available-ranges, so expect == 0 */
        warn_on!(msi_bitmap_reserve_dt_hwirqs(&mut bmp) != 0);

        warn_on!(bitmap_parselist(expected_str, &mut expected, SIZE_EXPECTED) != 0);
        warn_on!(!bitmap_equal(expected.as_ptr(), bmp.bitmap, SIZE_EXPECTED));

        msi_bitmap_free(&mut bmp);
        kfree(bmp.bitmap);
    }

    pub unsafe fn msi_bitmap_selftest() -> i32 {
        printk!("Running MSI bitmap self-tests ...\n");
        test_basics();
        test_of_node();
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
