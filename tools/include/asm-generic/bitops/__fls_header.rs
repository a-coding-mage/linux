/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from C header: #include <asm/types.h>

/**
 * generic___fls - find last (most-significant) set bit in a long word
 * @word: the word to search
 *
 * Undefined if no set bit exists, so code should check against 0 first.
 */
#[inline(always)]
pub const fn generic___fls(mut word: core::ffi::c_ulong) -> u32 {
    let mut num: u32 = (BITS_PER_LONG - 1) as u32;

    // C preprocessor condition: #if BITS_PER_LONG == 64
    if BITS_PER_LONG == 64 {
        if word & ((!0 as core::ffi::c_ulong) << 32) == 0 {
            num -= 32;
            word <<= 32;
        }
    }

    if word & ((!0 as core::ffi::c_ulong) << (BITS_PER_LONG - 16)) == 0 {
        num -= 16;
        word <<= 16;
    }
    if word & ((!0 as core::ffi::c_ulong) << (BITS_PER_LONG - 8)) == 0 {
        num -= 8;
        word <<= 8;
    }
    if word & ((!0 as core::ffi::c_ulong) << (BITS_PER_LONG - 4)) == 0 {
        num -= 4;
        word <<= 4;
    }
    if word & ((!0 as core::ffi::c_ulong) << (BITS_PER_LONG - 2)) == 0 {
        num -= 2;
        word <<= 2;
    }
    if word & ((!0 as core::ffi::c_ulong) << (BITS_PER_LONG - 1)) == 0 {
        num -= 1;
    }
    num
}

// C conditional alias:
// #ifndef __HAVE_ARCH___FLS
// #define __fls(word) generic___fls(word)
// #endif
#[inline(always)]
pub const fn __fls(word: core::ffi::c_ulong) -> u32 {
    generic___fls(word)
}
