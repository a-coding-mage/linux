/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Miscellaneous registers definitions for SPEAr3xx machine family
 *
 * Copyright (C) 2009 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

// Dependency: `VA_SPEAR_ICM3_MISC_REG_BASE` is supplied by `spear.h`.

pub const MISC_BASE: usize = VA_SPEAR_ICM3_MISC_REG_BASE;
pub const DMA_CHN_CFG: usize = MISC_BASE + 0x0A0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
