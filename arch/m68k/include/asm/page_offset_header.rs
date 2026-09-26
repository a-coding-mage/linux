/* SPDX-License-Identifier: GPL-2.0 */
/* This handles the memory map.. */

/* CONFIG_RAMBASE is a build-time configuration value supplied externally. */
#[cfg(CONFIG_RAMBASE)]
pub const PAGE_OFFSET_RAW: usize = CONFIG_RAMBASE;

/* Build-time CONFIG_SUN3 selection. */
#[cfg(all(not(CONFIG_RAMBASE), CONFIG_SUN3))]
pub const PAGE_OFFSET_RAW: usize = 0x0E000000;

#[cfg(all(not(CONFIG_RAMBASE), not(CONFIG_SUN3)))]
pub const PAGE_OFFSET_RAW: usize = 0x00000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
