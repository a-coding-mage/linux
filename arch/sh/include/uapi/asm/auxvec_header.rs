/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Architecture-neutral AT_ values in 0-17, leave some room
 * for more of them.
 */

/*
 * This entry gives some information about the FPU initialization
 * performed by the kernel.
 */
pub const AT_FPUCW: i32 = 18; /* Used FPU control word.  */

/*
 * The entry point to the vsyscall page gets placed here.
 */
pub const AT_SYSINFO_EHDR: i32 = 33;

/*
 * More complete cache descriptions than AT_[DIU]CACHEBSIZE.  If the
 * value is -1, then the cache doesn't exist.  Otherwise:
 *
 *    bit 0-3:    Cache set-associativity; 0 means fully associative.
 *    bit 4-7:    Log2 of cacheline size.
 *    bit 8-31:   Size of the entire cache >> 8.
 */
pub const AT_L1I_CACHESHAPE: i32 = 34;
pub const AT_L1D_CACHESHAPE: i32 = 35;
pub const AT_L2_CACHESHAPE: i32 = 36;

pub const AT_VECTOR_SIZE_ARCH: i32 = 5; /* entries in ARCH_DLINFO */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
