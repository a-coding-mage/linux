/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// PAGE_OFFSET and PAGE_SHIFT are supplied by the surrounding build environment.
pub const MAX_DMA_ADDRESS: usize = PAGE_OFFSET;
pub const MAX_DMA32_PFN: usize = 1usize << (32 - PAGE_SHIFT);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
