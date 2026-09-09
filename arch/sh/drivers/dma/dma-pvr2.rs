// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/drivers/dma/dma-pvr2.c
 *
 * NEC PowerVR 2 (Dreamcast) DMA support
 *
 * Copyright (C) 2003, 2004  Paul Mundt
 */

// The following types, constants, macros, and functions are supplied by the
// corresponding kernel and architecture dependencies.
use core::ffi::c_void;

extern "C" {
    fn get_dma_residue(channel: i32) -> i32;
    fn dma_wait_for_completion(channel: i32);
    fn printk(format: *const u8, ...);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut c_void) -> i32,
                   flags: u32, name: *const u8, dev_id: *mut c_void) -> i32;
    fn request_dma(channel: i32, name: *const u8) -> i32;
    fn register_dmac(info: *mut dma_info) -> i32;
    fn free_dma(channel: i32);
    fn free_irq(irq: i32, dev_id: u32);
    fn unregister_dmac(info: *mut dma_info);
    fn __raw_readl(address: usize) -> u32;
    fn __raw_writel(value: u32, address: usize);
}

#[repr(C)]
pub struct dma_channel {
    pub sar: u32,
    pub dar: u32,
    pub count: u32,
    pub mode: u32,
}

#[repr(C)]
pub struct dma_ops {
    pub request: Option<unsafe extern "C" fn(*mut dma_channel) -> i32>,
    pub get_residue: Option<unsafe extern "C" fn(*mut dma_channel) -> i32>,
    pub xfer: Option<unsafe extern "C" fn(*mut dma_channel) -> i32>,
}

#[repr(C)]
pub struct dma_info {
    pub name: *const u8,
    pub nr_channels: i32,
    pub ops: *mut dma_ops,
    pub flags: u32,
}

// Architecture-provided constants/macros.
extern "C" {
    static PVR2_CASCADE_CHAN: i32;
    static HW_EVENT_PVR2_DMA: i32;
    static PVR2_DMA_MODE: usize;
    static PVR2_DMA_LMMODE0: usize;
    static PVR2_DMA_ADDR: usize;
    static PVR2_DMA_COUNT: usize;
    static DMA_MODE_MASK: u32;
    static DMAC_CHANNELS_TEI_CAPABLE: u32;
}

static mut XFER_COMPLETE: u32 = 0;
static mut COUNT: i32 = 0;

unsafe extern "C" fn pvr2_dma_interrupt(irq: i32, _dev_id: *mut c_void) -> i32 {
    if get_dma_residue(PVR2_CASCADE_CHAN) != 0 {
        printk(b"DMA: SH DMAC did not complete transfer on channel %d, waiting..\n\0".as_ptr(),
               PVR2_CASCADE_CHAN);
        dma_wait_for_completion(PVR2_CASCADE_CHAN);
    }

    if COUNT < 10 {
        COUNT += 1;
        // pr_debug("Got a pvr2 dma interrupt for channel %d\n", irq - HW_EVENT_PVR2_DMA);
    }

    XFER_COMPLETE = 1;
    1 // IRQ_HANDLED
}

unsafe extern "C" fn pvr2_request_dma(_chan: *mut dma_channel) -> i32 {
    if __raw_readl(PVR2_DMA_MODE) != 0 {
        return -16; // -EBUSY
    }

    __raw_writel(0, PVR2_DMA_LMMODE0);
    0
}

unsafe extern "C" fn pvr2_get_dma_residue(_chan: *mut dma_channel) -> i32 {
    (XFER_COMPLETE == 0) as i32
}

unsafe extern "C" fn pvr2_xfer_dma(chan: *mut dma_channel) -> i32 {
    if (*chan).sar != 0 || (*chan).dar == 0 {
        return -22; // -EINVAL
    }

    XFER_COMPLETE = 0;
    __raw_writel((*chan).dar, PVR2_DMA_ADDR);
    __raw_writel((*chan).count, PVR2_DMA_COUNT);
    __raw_writel((*chan).mode & DMA_MODE_MASK, PVR2_DMA_MODE);
    0
}

static mut PVR2_DMA_OPS: dma_ops = dma_ops {
    request: Some(pvr2_request_dma),
    get_residue: Some(pvr2_get_dma_residue),
    xfer: Some(pvr2_xfer_dma),
};

static mut PVR2_DMA_INFO: dma_info = dma_info {
    name: b"pvr2_dmac\0".as_ptr(),
    nr_channels: 1,
    ops: &raw mut PVR2_DMA_OPS,
    flags: DMAC_CHANNELS_TEI_CAPABLE,
};

unsafe extern "C" fn pvr2_dma_init() -> i32 {
    if request_irq(HW_EVENT_PVR2_DMA, pvr2_dma_interrupt, 0,
                   b"pvr2 DMA handler\0".as_ptr(), core::ptr::null_mut()) != 0 {
        // pr_err("Failed to register pvr2 DMA handler interrupt\n");
    }
    request_dma(PVR2_CASCADE_CHAN, b"pvr2 cascade\0".as_ptr());
    register_dmac(&raw mut PVR2_DMA_INFO)
}

unsafe extern "C" fn pvr2_dma_exit() {
    free_dma(PVR2_CASCADE_CHAN);
    free_irq(HW_EVENT_PVR2_DMA, 0);
    unregister_dmac(&raw mut PVR2_DMA_INFO);
}

// subsys_initcall(pvr2_dma_init);
// module_exit(pvr2_dma_exit);
// MODULE_AUTHOR("Paul Mundt <lethal@linux-sh.org>");
// MODULE_DESCRIPTION("NEC PowerVR 2 DMA driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
