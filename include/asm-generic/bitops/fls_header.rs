/* SPDX-License-Identifier: GPL-2.0 */

/**
 * generic_fls - find last (most-significant) bit set
 * @x: the word to search
 *
 * This is defined the same way as ffs.
 * Note fls(0) = 0, fls(1) = 1, fls(0x80000000) = 32.
 */
#[inline(always)]
pub const fn generic_fls(mut x: u32) -> i32 {
    let mut r: i32 = 32;

    if x == 0 {
        return 0;
    }
    if (x & 0xffff0000u32) == 0 {
        x <<= 16;
        r -= 16;
    }
    if (x & 0xff000000u32) == 0 {
        x <<= 8;
        r -= 8;
    }
    if (x & 0xf0000000u32) == 0 {
        x <<= 4;
        r -= 4;
    }
    if (x & 0xc0000000u32) == 0 {
        x <<= 2;
        r -= 2;
    }
    if (x & 0x80000000u32) == 0 {
        x <<= 1;
        r -= 1;
    }
    r
}

/* The C header defines fls(x) as generic_fls(x) when __HAVE_ARCH_FLS is absent. */
#[macro_export]
macro_rules! fls {
    ($x:expr) => {
        $crate::generic_fls($x)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
