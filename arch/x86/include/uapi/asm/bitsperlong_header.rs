/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Equivalent to the C condition:
// defined(__x86_64__) && !defined(__ILP32__)
#[cfg(all(target_arch = "x86_64", not(target_pointer_width = "32")))]
pub const __BITS_PER_LONG: u32 = 64;

#[cfg(not(all(target_arch = "x86_64", not(target_pointer_width = "32"))))]
pub const __BITS_PER_LONG: u32 = 32;

// C dependency: #include <asm-generic/bitsperlong.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
