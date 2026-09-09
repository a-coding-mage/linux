/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SiFive FU540 Platform DMA driver
 * Copyright (C) 2019 SiFive
 *
 * Based partially on:
 * - drivers/dma/fsl-edma.c
 * - drivers/dma/dw-edma/
 * - drivers/dma/pxa-dma.c
 *
 * See the following sources for further documentation:
 * - Chapter 12 "Platform DMA Engine (PDMA)" of
 *   SiFive FU540-C000 v1.0
 *   https://static.dev.sifive.com/FU540-C000-v1.0.pdf
 */

use core::ffi::c_void;

pub const PDMA_MAX_NR_CH: u32 = 4;
pub const PDMA_BASE_ADDR: usize = 0x3000000;
pub const PDMA_CHAN_OFFSET: usize = 0x1000;

/* Register Offset */
pub const PDMA_CTRL: usize = 0x000;
pub const PDMA_XFER_TYPE: usize = 0x004;
pub const PDMA_XFER_SIZE: usize = 0x008;
pub const PDMA_DST_ADDR: usize = 0x010;
pub const PDMA_SRC_ADDR: usize = 0x018;
pub const PDMA_ACT_TYPE: usize = 0x104; /* Read-only */
pub const PDMA_REMAINING_BYTE: usize = 0x108; /* Read-only */
pub const PDMA_CUR_DST_ADDR: usize = 0x110; /* Read-only*/
pub const PDMA_CUR_SRC_ADDR: usize = 0x118; /* Read-only*/

/* CTRL */
pub const PDMA_CLEAR_CTRL: u32 = 0x0;
pub const PDMA_CLAIM_MASK: u32 = 1 << 0;
pub const PDMA_RUN_MASK: u32 = 1 << 1;
pub const PDMA_ENABLE_DONE_INT_MASK: u32 = 1 << 14;
pub const PDMA_ENABLE_ERR_INT_MASK: u32 = 1 << 15;
pub const PDMA_DONE_STATUS_MASK: u32 = 1 << 30;
pub const PDMA_ERR_STATUS_MASK: u32 = 1 << 31;

/* Transfer Type */
pub const PDMA_FULL_SPEED: u32 = 0xFF000000;
pub const PDMA_STRICT_ORDERING: u32 = 1 << 3;

/* Error Recovery */
pub const MAX_RETRY: i32 = 1;

pub unsafe fn sf_pdma_reg_base(pdma: *mut sf_pdma, ch: usize) -> *mut c_void {
    ((*pdma).membase as *mut u8).add(PDMA_CHAN_OFFSET * ch) as *mut c_void
}

#[repr(C)]
pub struct pdma_regs {
    /* read-write regs */
    pub ctrl: *mut c_void, /* 4 bytes */
    pub xfer_type: *mut c_void, /* 4 bytes */
    pub xfer_size: *mut c_void, /* 8 bytes */
    pub dst_addr: *mut c_void, /* 8 bytes */
    pub src_addr: *mut c_void, /* 8 bytes */

    /* read-only */
    pub act_type: *mut c_void, /* 4 bytes */
    pub residue: *mut c_void, /* 8 bytes */
    pub cur_dst_addr: *mut c_void, /* 8 bytes */
    pub cur_src_addr: *mut c_void, /* 8 bytes */
}

#[repr(C)]
pub struct sf_pdma_desc {
    pub xfer_type: u32,
    pub xfer_size: u64,
    pub dst_addr: u64,
    pub src_addr: u64,
    pub vdesc: virt_dma_desc,
    pub chan: *mut sf_pdma_chan,
    pub dirn: dma_transfer_direction,
    pub async_tx: *mut dma_async_tx_descriptor,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sf_pdma_pm_state {
    RUNNING = 0,
    SUSPENDED,
}

#[repr(C)]
pub struct sf_pdma_chan {
    pub vchan: virt_dma_chan,
    pub status: dma_status,
    pub pm_state: sf_pdma_pm_state,
    pub slave_id: u32,
    pub pdma: *mut sf_pdma,
    pub desc: *mut sf_pdma_desc,
    pub cfg: dma_slave_config,
    pub attr: u32,
    pub dma_dev_addr: dma_addr_t,
    pub dma_dev_size: u32,
    pub done_tasklet: tasklet_struct,
    pub err_tasklet: tasklet_struct,
    pub regs: pdma_regs,
    pub lock: spinlock_t, /* protect chan data */
    pub xfer_err: bool,
    pub txirq: i32,
    pub errirq: i32,
    pub retries: i32,
}

#[repr(C)]
pub struct sf_pdma {
    pub dma_dev: dma_device,
    pub membase: *mut c_void,
    pub mappedbase: *mut c_void,
    pub transfer_type: u32,
    pub n_chans: u32,
    pub chans: [sf_pdma_chan; 0], /* __counted_by(n_chans) */
}

#[repr(C)]
pub struct sf_pdma_driver_platdata {
    pub quirks: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
