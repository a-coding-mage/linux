/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <uapi/asm-generic/bitsperlong.h>, which supplies
// __BITS_PER_LONG. That external dependency is intentionally not redefined here.

#[cfg(feature = "CONFIG_64BIT")]
pub const BITS_PER_LONG: usize = 64;

#[cfg(not(feature = "CONFIG_64BIT"))]
pub const BITS_PER_LONG: usize = 32;

/*
 * FIXME: The check currently breaks x86-64 build, so it's
 * temporarily disabled. Please fix x86-64 and reenable
 */
// The disabled C check was:
// #if 0 && BITS_PER_LONG != __BITS_PER_LONG
// #error Inconsistent word size. Check asm/bitsperlong.h
// #endif

// The C preprocessor check involving __CHAR_BIT__, __SIZEOF_LONG__, and
// __BITS_PER_LONG is preserved as intent; those build-time C macros have no
// direct file-local Rust equivalent.

// The C _Static_assert(sizeof(long) * 8 == __BITS_PER_LONG) is likewise a
// build-time assertion over target ABI properties and external definitions.

pub const BITS_PER_LONG_LONG: usize = 64;

/*
 * small_const_nbits(n) is true precisely when it is known at compile-time
 * that BITMAP_SIZE(n) is 1, i.e. 1 <= n <= BITS_PER_LONG. This allows
 * various bit/bitmap APIs to provide a fast inline implementation. Bitmaps
 * of size 0 are very rare, and a compile-time-known-size 0 is most likely
 * a sign of error. They will be handled correctly by the bit/bitmap APIs,
 * but using the out-of-line functions, so that the inline implementations
 * can unconditionally dereference the pointer(s).
 */
// Rust has no direct equivalent of GCC's __builtin_constant_p; this macro
// preserves the range test and is intended for compile-time expressions.
#[macro_export]
macro_rules! small_const_nbits {
    ($nbits:expr) => {
        ($nbits) <= $crate::BITS_PER_LONG && ($nbits) > 0
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
