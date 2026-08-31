/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies from the original header:
 * #include <asm/types.h>
 * #include <asm/bitsperlong.h>
 */

/**
 * __ffs - find first bit in word.
 * @word: The word to search
 *
 * Undefined if no bit exists, so code should check against 0 first.
 */
pub unsafe fn __ffs(mut word: core::ffi::c_ulong) -> u32 {
    let mut num: u32 = 0;

    /* Original condition: #if __BITS_PER_LONG == 64 */
    #[cfg(target_pointer_width = "64")]
    {
        if (word & 0xffffffff) == 0 {
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
