/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/hardware/memc.h
 *
 *  Copyright (C) Russell King.
 */

pub const VDMA_ALIGNMENT: usize = PAGE_SIZE;
pub const VDMA_XFERSIZE: usize = 16;
pub const VDMA_INIT: u32 = 0;
pub const VDMA_START: u32 = 1;
pub const VDMA_END: u32 = 2;

extern "C" {
    pub fn memc_write(reg: u32, val: usize);
}

pub unsafe fn video_set_dma(start: usize, end: usize, offset: usize) {
    memc_write(VDMA_START, start >> 2);
    memc_write(VDMA_END, (end - VDMA_XFERSIZE) >> 2);
    memc_write(VDMA_INIT, offset >> 2);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
