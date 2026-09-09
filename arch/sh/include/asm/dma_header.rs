/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/dma.h
 *
 * Copyright (C) 2003, 2004  Paul Mundt
 */

/* Dependencies supplied by the surrounding kernel translation are intentionally
 * referenced here rather than reimplemented. */

/*
 * Read and write modes can mean drastically different things depending on the
 * channel configuration. Consult your DMAC documentation and module
 * implementation for further clues.
 */
pub const DMA_MODE_READ: u32 = 0x00;
pub const DMA_MODE_WRITE: u32 = 0x01;
pub const DMA_MODE_MASK: u32 = 0x01;

pub const DMA_AUTOINIT: u32 = 0x10;

/* DMAC (dma_info) flags */
pub const DMAC_CHANNELS_CONFIGURED: u32 = 0x01;
pub const DMAC_CHANNELS_TEI_CAPABLE: u32 = 0x02; /* Transfer end interrupt */

/* DMA channel capabilities / flags */
pub const DMA_CONFIGURED: u32 = 0x01;

/*
 * Transfer end interrupt, inherited from DMAC.
 * wait_queue used in dma_wait_for_completion.
 */
pub const DMA_TEI_CAPABLE: u32 = 0x02;

extern "C" {
    pub static mut dma_spin_lock: spinlock_t;
}

#[repr(C)]
pub struct dma_channel {
    pub dev_id: [core::ffi::c_char; 16],
    pub chan: u32,
    pub vchan: u32,
    pub mode: u32,
    pub count: u32,
    pub sar: c_ulong,
    pub dar: c_ulong,
    pub caps: *const *const core::ffi::c_char,
    pub flags: c_ulong,
    pub busy: atomic_t,
    pub wait_queue: wait_queue_head_t,
    pub dev: device,
    pub priv_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct dma_ops {
    pub request: Option<unsafe extern "C" fn(*mut dma_channel) -> i32>,
    pub free: Option<unsafe extern "C" fn(*mut dma_channel)>,
    pub get_residue: Option<unsafe extern "C" fn(*mut dma_channel) -> i32>,
    pub xfer: Option<unsafe extern "C" fn(*mut dma_channel) -> i32>,
    pub configure: Option<unsafe extern "C" fn(*mut dma_channel, c_ulong) -> i32>,
}

#[repr(C)]
pub struct dma_info {
    pub pdev: *mut platform_device,
    pub name: *const core::ffi::c_char,
    pub nr_channels: u32,
    pub flags: c_ulong,
    pub ops: *mut dma_ops,
    pub channels: *mut dma_channel,
    pub list: list_head,
    pub first_channel_nr: i32,
    pub first_vchannel_nr: i32,
}

#[repr(C)]
pub struct dma_chan_caps {
    pub ch_num: i32,
    pub caplist: *const *const core::ffi::c_char,
}

/* Equivalent to: container_of(channel, struct dma_channel, dev). */
#[macro_export]
macro_rules! to_dma_channel {
    ($channel:expr) => {
        container_of!($channel, dma_channel, dev)
    };
}

/* arch/sh/drivers/dma/dma-api.c */
extern "C" {
    pub fn dma_xfer(chan: u32, from: c_ulong, to: c_ulong, size: usize, mode: u32) -> i32;
    pub fn get_dma_residue(chan: u32) -> i32;
    pub fn get_dma_info(chan: u32) -> *mut dma_info;
    pub fn get_dma_channel(chan: u32) -> *mut dma_channel;
    pub fn dma_wait_for_completion(chan: u32);
    pub fn dma_configure_channel(chan: u32, flags: c_ulong);
    pub fn register_dmac(info: *mut dma_info) -> i32;
    pub fn unregister_dmac(info: *mut dma_info);
    pub fn dma_create_sysfs_files(channel: *mut dma_channel, info: *mut dma_info) -> i32;
    pub fn dma_remove_sysfs_files(channel: *mut dma_channel, info: *mut dma_info);
}

#[inline]
pub unsafe fn dma_write(chan: u32, from: c_ulong, to: c_ulong, size: usize) -> i32 {
    dma_xfer(chan, from, to, size, DMA_MODE_WRITE)
}

#[inline]
pub unsafe fn dma_write_page(chan: u32, from: c_ulong, to: c_ulong) -> i32 {
    dma_write(chan, from, to, PAGE_SIZE)
}

#[inline]
pub unsafe fn dma_read(chan: u32, from: c_ulong, to: c_ulong, size: usize) -> i32 {
    dma_xfer(chan, from, to, size, DMA_MODE_READ)
}

#[inline]
pub unsafe fn dma_read_page(chan: u32, from: c_ulong, to: c_ulong) -> i32 {
    dma_read(chan, from, to, PAGE_SIZE)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
