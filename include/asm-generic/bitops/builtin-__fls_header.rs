/* SPDX-License-Identifier: GPL-2.0 */

/**
 * __fls - find last (most-significant) set bit in a long word
 * @word: the word to search
 *
 * Undefined if no set bit exists, so code should check against 0 first.
 */
#[inline(always)]
pub const fn __fls(word: usize) -> u32 {
    (core::mem::size_of::<usize>() as u32 * 8) - 1 - word.leading_zeros()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
