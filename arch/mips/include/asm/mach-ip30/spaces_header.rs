/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2016 Joshua Kinard <linux@kumba.dev>
 *
 */

/*
 * Memory in IP30/Octane is offset 512MB in the physical address space.
 */
pub const PHYS_OFFSET: u64 = 0x20000000u64;

/* CONFIG_64BIT build-time condition preserved from the source header. */
#[cfg(feature = "CONFIG_64BIT")]
pub const CAC_BASE: u64 = 0xA800000000000000u64;

/* Dependency supplied by the corresponding asm/mach-generic/spaces.h translation. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
