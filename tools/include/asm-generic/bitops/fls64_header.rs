// SPDX-License-Identifier: GPL-2.0
// Translated from include/asm-generic/bitops/fls64.h.
// C dependency: <asm/types.h> supplies __u64 and __u32.

/*
 * fls64 - find last set bit in a 64-bit word
 * @x: the word to search
 *
 * This is defined in a similar way as the libc and compiler builtin
 * ffsll, but returns the position of the most significant set bit.
 *
 * fls64(value) returns 0 if value is 0 or the position of the last
 * set bit if value is nonzero. The last (most significant) bit is
 * at position 64.
 */

// Original C condition: #if BITS_PER_LONG == 32
#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub unsafe fn fls64(x: __u64) -> core::ffi::c_int {
    let h: __u32 = (x >> 32) as __u32;
    if h != 0 {
        return fls(h) + 32;
    }
    fls(x as __u32)
}

// Original C condition: #elif BITS_PER_LONG == 64
#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub unsafe fn fls64(x: __u64) -> core::ffi::c_int {
    if x == 0 {
        return 0;
    }
    (__fls(x) + 1) as core::ffi::c_int
}

// Original C fallback: #error BITS_PER_LONG not 32 or 64

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
