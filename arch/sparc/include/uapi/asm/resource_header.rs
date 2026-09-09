/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * resource_header.rs: Resource definitions.
 *
 * Rust translation of the SPARC resource header.
 */

/*
 * These two resource limit IDs have a Sparc/Linux-specific ordering,
 * the rest comes from the generic header:
 */
pub const RLIMIT_NOFILE: i32 = 6; /* max number of open files */
pub const RLIMIT_NPROC: i32 = 7; /* max number of processes */

/* On 64-bit SPARC, use the generic version. */
#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
/*
 * SuS says limits have to be unsigned.
 * We make this unsigned, but keep the
 * old value for compatibility:
 */
pub const RLIM_INFINITY: i32 = 0x7fffffff;

/* Definitions from asm-generic/resource.h are supplied by the generic header. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
