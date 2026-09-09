/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Definitions for the cacheflush system call. */

pub const FLUSH_SCOPE_LINE: i32 = 1; /* Flush a cache line */
pub const FLUSH_SCOPE_PAGE: i32 = 2; /* Flush a page */
pub const FLUSH_SCOPE_ALL: i32 = 3; /* Flush the whole cache -- superuser only */

pub const FLUSH_CACHE_DATA: i32 = 1; /* Writeback and flush data cache */
pub const FLUSH_CACHE_INSN: i32 = 2; /* Flush instruction cache */
pub const FLUSH_CACHE_BOTH: i32 = 3; /* Flush both caches */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
