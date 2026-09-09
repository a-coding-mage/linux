/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * PS3 Storage Devices
 *
 * Copyright (C) 2007 Sony Computer Entertainment Inc.
 * Copyright 2007 Sony Corp.
 */

/* C dependencies: linux/interrupt.h and asm/ps3.h. */

#[repr(C)]
pub struct ps3_storage_region {
    pub id: u32,
    pub start: u64,
    pub size: u64,
}

#[repr(C)]
pub struct ps3_storage_device {
    pub sbd: ps3_system_bus_device,

    pub dma_region: ps3_dma_region,
    pub irq: u32,
    pub blk_size: u64,

    pub tag: u64,
    pub lv1_status: u64,
    pub done: completion,

    pub bounce_size: usize,
    pub bounce_buf: *mut core::ffi::c_void,
    pub bounce_lpar: u64,
    pub bounce_dma: dma_addr_t,

    pub num_regions: u32,
    pub accessible_regions: usize,
    pub region_idx: u32, /* first accessible region */
    pub regions: [ps3_storage_region; 0], /* Must be last */
}

#[inline]
pub unsafe fn to_ps3_storage_device(dev: *mut device) -> *mut ps3_storage_device {
    /* Equivalent to container_of(dev, struct ps3_storage_device, sbd.core). */
    (dev as *mut u8).sub(core::mem::offset_of!(ps3_storage_device, sbd))
        as *mut ps3_storage_device
}

extern "C" {
    pub fn ps3stor_setup(dev: *mut ps3_storage_device, handler: irq_handler_t) -> i32;
    pub fn ps3stor_teardown(dev: *mut ps3_storage_device);
    pub fn ps3stor_read_write_sectors(
        dev: *mut ps3_storage_device,
        lpar: u64,
        start_sector: u64,
        sectors: u64,
        write: i32,
    ) -> u64;
    pub fn ps3stor_send_command(
        dev: *mut ps3_storage_device,
        cmd: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
    ) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
