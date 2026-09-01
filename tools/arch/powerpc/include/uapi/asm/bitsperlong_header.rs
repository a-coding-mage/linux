/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C header guard removed: __ASM_POWERPC_BITSPERLONG_H.

#[cfg(target_arch = "powerpc64")]
pub const __BITS_PER_LONG: i32 = 64;

#[cfg(not(target_arch = "powerpc64"))]
pub const __BITS_PER_LONG: i32 = 32;

// Requires declarations/constants from <asm-generic/bitsperlong.h>.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
