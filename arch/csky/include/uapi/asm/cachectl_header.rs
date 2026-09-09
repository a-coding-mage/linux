/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * See "man cacheflush"
 */
pub const ICACHE: i32 = 1 << 0;
pub const DCACHE: i32 = 1 << 1;
pub const BCACHE: i32 = ICACHE | DCACHE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
