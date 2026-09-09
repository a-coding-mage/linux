/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2017 Chen Liqin <liqin.chen@sunplusct.com>
 * Copyright (C) 2012 Regents of the University of California
 */

pub const L1_CACHE_SHIFT: u32 = 6;

pub const L1_CACHE_BYTES: usize = 1usize << L1_CACHE_SHIFT;

#[cfg(CONFIG_RISCV_DMA_NONCOHERENT)]
pub const ARCH_DMA_MINALIGN: usize = L1_CACHE_BYTES;

#[cfg(CONFIG_RISCV_DMA_NONCOHERENT)]
pub const ARCH_KMALLOC_MINALIGN: usize = 8;

/*
 * RISC-V requires the stack pointer to be 16-byte aligned, so ensure that
 * the flat loader aligns it accordingly.
 */
#[cfg(not(CONFIG_MMU))]
pub const ARCH_SLAB_MINALIGN: usize = 16;

/* The C declaration `extern int dma_cache_alignment` is an external global. */
extern "C" {
    pub static mut dma_cache_alignment: ::core::ffi::c_int;
}

#[cfg(CONFIG_RISCV_DMA_NONCOHERENT)]
#[inline]
pub unsafe fn dma_get_cache_alignment() -> ::core::ffi::c_int {
    dma_cache_alignment
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
