/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// CONFIG_L1_CACHE_SHIFT is supplied by the build configuration.
pub const L1_CACHE_SHIFT: usize = CONFIG_L1_CACHE_SHIFT;
pub const L1_CACHE_BYTES: usize = 1usize << L1_CACHE_SHIFT;

pub const ARCH_DMA_MINALIGN: usize = 16;

// C annotation macro: __read_mostly expands to __section(".data..read_mostly").
// Rust declarations requiring this placement should use the corresponding
// link-section attribute.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
