/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// __BITS_PER_LONG is selected by the build-time LP64 condition in the source.
#[cfg(target_pointer_width = "64")]
pub const __BITS_PER_LONG: u32 = 64;

#[cfg(not(target_pointer_width = "64"))]
pub const __BITS_PER_LONG: u32 = 32;

// Dependency intent preserved from: #include <asm-generic/bitsperlong.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
