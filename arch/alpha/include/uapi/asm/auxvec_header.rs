/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Reserve these numbers for any future use of a VDSO. */
#[cfg(any())]
pub const AT_SYSINFO: u64 = 32;
#[cfg(any())]
pub const AT_SYSINFO_EHDR: u64 = 33;

/*
 * More complete cache descriptions than AT_[DIU]CACHEBSIZE.  If the
 * value is -1, then the cache doesn't exist.  Otherwise:
 *
 *   bit 0-3:    Cache set-associativity; 0 means fully associative.
 *   bit 4-7:    Log2 of cacheline size.
 *   bit 8-31:   Size of the entire cache >> 8.
 *   bit 32-63:  Reserved.
 */
pub const AT_L1I_CACHESHAPE: u64 = 34;
pub const AT_L1D_CACHESHAPE: u64 = 35;
pub const AT_L2_CACHESHAPE: u64 = 36;
pub const AT_L3_CACHESHAPE: u64 = 37;

pub const AT_VECTOR_SIZE_ARCH: u64 = 4; /* entries in ARCH_DLINFO */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
