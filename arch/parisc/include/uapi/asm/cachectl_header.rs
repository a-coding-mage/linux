/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Options for cacheflush system call
 */
pub const ICACHE: u32 = 1 << 0; // flush instruction cache
pub const DCACHE: u32 = 1 << 1; // writeback and flush data cache
pub const BCACHE: u32 = ICACHE | DCACHE; // flush both caches

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
