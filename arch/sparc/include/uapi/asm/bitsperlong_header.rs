/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Equivalent to the C preprocessor condition:
// #if defined(__sparc__) && defined(__arch64__)
#[cfg(all(target_arch = "sparc64", target_pointer_width = "64"))]
pub const __BITS_PER_LONG: u32 = 64;

#[cfg(not(all(target_arch = "sparc64", target_pointer_width = "64")))]
pub const __BITS_PER_LONG: u32 = 32;

// Dependency corresponding to: #include <asm-generic/bitsperlong.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
