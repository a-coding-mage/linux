/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2008, Michael Ellerman, IBM Corporation.
 */

// Dependencies supplied by the surrounding kernel translation:
// <linux/of.h>, <asm/irq.h>

#[repr(C)]
pub struct msi_bitmap {
    pub of_node: *mut device_node,
    pub bitmap: *mut ::kernel::ffi::c_ulong,
    pub lock: spinlock_t,
    pub irq_count: ::kernel::ffi::c_uint,
    pub bitmap_from_slab: bool,
}

extern "C" {
    pub fn msi_bitmap_alloc_hwirqs(bmp: *mut msi_bitmap, num: ::kernel::ffi::c_int)
        -> ::kernel::ffi::c_int;
    pub fn msi_bitmap_free_hwirqs(
        bmp: *mut msi_bitmap,
        offset: ::kernel::ffi::c_uint,
        num: ::kernel::ffi::c_uint,
    );
    pub fn msi_bitmap_reserve_hwirq(bmp: *mut msi_bitmap, hwirq: ::kernel::ffi::c_uint);

    pub fn msi_bitmap_reserve_dt_hwirqs(bmp: *mut msi_bitmap) -> ::kernel::ffi::c_int;

    pub fn msi_bitmap_alloc(
        bmp: *mut msi_bitmap,
        irq_count: ::kernel::ffi::c_uint,
        of_node: *mut device_node,
    ) -> ::kernel::ffi::c_int;
    pub fn msi_bitmap_free(bmp: *mut msi_bitmap);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
