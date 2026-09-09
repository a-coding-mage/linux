/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding architecture/type definitions.
// The original header includes <asm/types.h> for __u32, __u64, and
// BITS_PER_LONG.

unsafe extern "C" {
    fn fls(x: u32) -> i32;
    fn __fls(x: u64) -> i32;
}

/**
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
#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub fn fls64(x: u64) -> i32 {
    let h: u32 = (x >> 32) as u32;
    if h != 0 {
        // SAFETY: `fls` is the externally supplied bit-operation primitive.
        unsafe { fls(h) + 32 }
    } else {
        // SAFETY: `fls` is the externally supplied bit-operation primitive.
        unsafe { fls(x as u32) }
    }
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn fls64(x: u64) -> i32 {
    if x == 0 {
        return 0;
    }
    // SAFETY: `__fls` is the externally supplied bit-operation primitive.
    unsafe { __fls(x) + 1 }
}

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("BITS_PER_LONG not 32 or 64");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
