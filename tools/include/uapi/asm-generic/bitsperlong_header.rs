/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * In C, __BITS_PER_LONG may already be supplied before this header is included.
 *
 * If not supplied, the original header uses compiler-provided macros when both
 * __CHAR_BIT__ and __SIZEOF_LONG__ are defined:
 *
 *     __BITS_PER_LONG = __CHAR_BIT__ * __SIZEOF_LONG__
 *
 * Otherwise it falls back to 32, with 64 bit architectures expected to override
 * this in their own bitsperlong.h.
 */
#[cfg(target_pointer_width = "64")]
pub const __BITS_PER_LONG: usize = 64;

#[cfg(target_pointer_width = "32")]
pub const __BITS_PER_LONG: usize = 32;

#[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
pub const __BITS_PER_LONG: usize = 32;

pub const __BITS_PER_LONG_LONG: usize = 64;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
