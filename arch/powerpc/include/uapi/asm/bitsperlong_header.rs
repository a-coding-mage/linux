/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Equivalent of the source condition on __powerpc64__.
#[cfg(target_arch = "powerpc64")]
pub const __BITS_PER_LONG: usize = 64;

#[cfg(not(target_arch = "powerpc64"))]
pub const __BITS_PER_LONG: usize = 32;

// Dependency supplied by asm-generic/bitsperlong.h in the source repository.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
