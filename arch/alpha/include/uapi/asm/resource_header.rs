/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Alpha/Linux-specific ordering of these four resource limit IDs,
 * the rest comes from the generic header:
 */
pub const RLIMIT_NOFILE: u32 = 6; /* max number of open files */
pub const RLIMIT_AS: u32 = 7; /* address space limit */
pub const RLIMIT_NPROC: u32 = 8; /* max number of processes */
pub const RLIMIT_MEMLOCK: u32 = 9; /* max locked-in-memory address space */

/*
 * SuS says limits have to be unsigned.  Fine, it's unsigned, but
 * we retain the old value for compatibility, especially with DU.
 * When you run into the 2^63 barrier, you call me.
 */
pub const RLIM_INFINITY: u64 = 0x7fffffffffffffffu64;

/* The remaining declarations come from asm-generic/resource.h. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
