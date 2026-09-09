/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: <asm/types.h> supplies the architecture word-size
// configuration used by the original header.

/**
 * generic___ffs - find first bit in word.
 * @word: The word to search
 *
 * Undefined if no bit exists, so code should check against 0 first.
 */
#[inline(always)]
fn generic___ffs(mut word: usize) -> u32 {
    let mut num: u32 = 0;

    // Corresponds to: #if BITS_PER_LONG == 64
    #[cfg(target_pointer_width = "64")]
    {
        if (word & 0xffff_ffff) == 0 {
            num += 32;
            word >>= 32;
        }
    }

    if (word & 0xffff) == 0 {
        num += 16;
        word >>= 16;
    }
    if (word & 0xff) == 0 {
        num += 8;
        word >>= 8;
    }
    if (word & 0xf) == 0 {
        num += 4;
        word >>= 4;
    }
    if (word & 0x3) == 0 {
        num += 2;
        word >>= 2;
    }
    if (word & 0x1) == 0 {
        num += 1;
    }
    num
}

// The original __ffs macro is defined only when __HAVE_ARCH___FFS is absent.
// This local equivalent assumes no architecture-specific override is present.
#[macro_export]
macro_rules! __ffs {
    ($word:expr) => {
        $crate::generic___ffs($word)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
