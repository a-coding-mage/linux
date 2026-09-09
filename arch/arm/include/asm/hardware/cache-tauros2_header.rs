/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/hardware/cache-tauros2.h
 *
 * Copyright (C) 2008 Marvell Semiconductor
 */

pub const CACHE_TAUROS2_PREFETCH_ON: u32 = 1 << 0;
pub const CACHE_TAUROS2_LINEFILL_BURST8: u32 = 1 << 1;

// C declaration included the kernel's __init attribute; its placement and
// implementation are supplied by the surrounding build environment.
unsafe extern "C" {
    pub fn tauros2_init(features: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
