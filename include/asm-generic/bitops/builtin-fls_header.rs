/* SPDX-License-Identifier: GPL-2.0 */

/**
 * fls - find last (most-significant) bit set
 * @x: the word to search
 *
 * This is defined the same way as ffs.
 * Note fls(0) = 0, fls(1) = 1, fls(0x80000000) = 32.
 */
#[inline(always)]
const fn fls(x: u32) -> i32 {
    if x != 0 {
        (core::mem::size_of::<u32>() * 8 - x.leading_zeros() as usize) as i32
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
