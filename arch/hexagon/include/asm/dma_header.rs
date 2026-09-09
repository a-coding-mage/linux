/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependency supplied by asm/io.h in the original header.

pub const MAX_DMA_CHANNELS: usize = 1;
pub const MAX_DMA_ADDRESS: usize = PAGE_OFFSET;

unsafe extern "C" {
    pub static mut hexagon_coherent_pool_size: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
