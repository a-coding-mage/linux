/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * mach/sram.h - DaVinci simple SRAM allocator
 *
 * Copyright (C) 2009 David Brownell
 */

/* ARBITRARY: SRAM allocations are multiples of this 2^N size */
pub const SRAM_GRANULARITY: usize = 512;

/*
 * SRAM allocations return a CPU virtual address, or NULL on error.
 * If a DMA address is requested and the SRAM supports DMA, its
 * mapped address is also returned.
 *
 * Errors include SRAM memory not being available, and requesting
 * DMA mapped SRAM on systems which don't allow that.
 */
extern "C" {
    pub fn sram_alloc(len: usize, dma: *mut dma_addr_t) -> *mut core::ffi::c_void;
    pub fn sram_free(addr: *mut core::ffi::c_void, len: usize);

    /* Get the struct gen_pool * for use in platform data */
    pub fn sram_get_gen_pool() -> *mut gen_pool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
