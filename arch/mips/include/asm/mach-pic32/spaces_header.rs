/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

// C: #ifdef CONFIG_PIC32MZDA
#[cfg(feature = "CONFIG_PIC32MZDA")]
pub const PHYS_OFFSET: u32 = 0x08000000u32;

// C dependency: <asm/mach-generic/spaces.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
