/* SPDX-License-Identifier: GPL-2.0 */

/**
 * generic___fls - find last (most-significant) set bit in a long word
 * @word: the word to search
 *
 * Undefined if no set bit exists, so code should check against 0 first.
 */
#[inline(always)]
pub const fn generic___fls(mut word: usize) -> u32 {
    let mut num: u32 = usize::BITS - 1;

    // #if BITS_PER_LONG == 64
    #[cfg(target_pointer_width = "64")]
    {
        if (word & (!0usize << 32)) == 0 {
            num -= 32;
            word <<= 32;
        }
    }
    // #endif
    if (word & (!0usize << (usize::BITS - 16))) == 0 {
        num -= 16;
        word <<= 16;
    }
    if (word & (!0usize << (usize::BITS - 8))) == 0 {
        num -= 8;
        word <<= 8;
    }
    if (word & (!0usize << (usize::BITS - 4))) == 0 {
        num -= 4;
        word <<= 4;
    }
    if (word & (!0usize << (usize::BITS - 2))) == 0 {
        num -= 2;
        word <<= 2;
    }
    if (word & (!0usize << (usize::BITS - 1))) == 0 {
        num -= 1;
    }
    num
}

// Fallback for configurations without __HAVE_ARCH___FLS.
#[inline(always)]
pub const fn __fls(word: usize) -> u32 {
    generic___fls(word)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
