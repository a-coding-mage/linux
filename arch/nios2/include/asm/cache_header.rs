/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *
 * All rights reserved.
 */

// C header guard: _ASM_NIOS2_CACHE_H

pub const NIOS2_DCACHE_SIZE: usize = CONFIG_NIOS2_DCACHE_SIZE;
pub const NIOS2_ICACHE_SIZE: usize = CONFIG_NIOS2_ICACHE_SIZE;
pub const NIOS2_DCACHE_LINE_SIZE: usize = CONFIG_NIOS2_DCACHE_LINE_SIZE;
pub const NIOS2_ICACHE_LINE_SHIFT: usize = 5;
pub const NIOS2_ICACHE_LINE_SIZE: usize = 1usize << NIOS2_ICACHE_LINE_SHIFT;

/* bytes per L1 cache line */
pub const L1_CACHE_SHIFT: usize = NIOS2_ICACHE_LINE_SHIFT;
pub const L1_CACHE_BYTES: usize = NIOS2_ICACHE_LINE_SIZE;

pub const ARCH_DMA_MINALIGN: usize = L1_CACHE_BYTES;

// C marker macros with no replacement text:
// #define __cacheline_aligned
// #define ____cacheline_aligned


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
