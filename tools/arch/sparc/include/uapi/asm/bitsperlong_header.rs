/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* C condition: defined(__sparc__) && defined(__arch64__) */
#[cfg(all(target_arch = "sparc64", target_pointer_width = "64"))]
pub const __BITS_PER_LONG: u32 = 64;

/* C fallback for non-64-bit sparc builds. */
#[cfg(not(all(target_arch = "sparc64", target_pointer_width = "64")))]
pub const __BITS_PER_LONG: u32 = 32;

/* Depends on <asm-generic/bitsperlong.h>. */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
