/* SPDX-License-Identifier: GPL-2.0 */

/**
 * __ffs - find first bit in word.
 * @word: The word to search
 *
 * Undefined if no bit exists, so code should check against 0 first.
 */
#[inline]
pub const fn __ffs(word: usize) -> u32 {
    word.trailing_zeros()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
