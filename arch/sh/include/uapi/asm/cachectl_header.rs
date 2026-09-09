/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Definitions for the cacheflush system call. */

pub const CACHEFLUSH_D_INVAL: i32 = 0x1; /* invalidate (without write back) */
pub const CACHEFLUSH_D_WB: i32 = 0x2; /* write back (without invalidate) */
pub const CACHEFLUSH_D_PURGE: i32 = 0x3; /* writeback and invalidate */

pub const CACHEFLUSH_I: i32 = 0x4;

/*
 * Options for cacheflush system call
 */
pub const ICACHE: i32 = CACHEFLUSH_I; /* flush instruction cache */
pub const DCACHE: i32 = CACHEFLUSH_D_PURGE; /* writeback and flush data cache */
pub const BCACHE: i32 = ICACHE | DCACHE; /* flush both caches */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
