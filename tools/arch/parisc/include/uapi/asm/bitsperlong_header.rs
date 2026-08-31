// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// C header guard removed: __ASM_PARISC_BITSPERLONG_H.

#[cfg(target_pointer_width = "64")]
pub const __BITS_PER_LONG: usize = 64;
#[cfg(target_pointer_width = "64")]
pub const SHIFT_PER_LONG: usize = 6;

#[cfg(not(target_pointer_width = "64"))]
pub const __BITS_PER_LONG: usize = 32;
#[cfg(not(target_pointer_width = "64"))]
pub const SHIFT_PER_LONG: usize = 5;

// Depends on declarations/constants from <asm-generic/bitsperlong.h>.
