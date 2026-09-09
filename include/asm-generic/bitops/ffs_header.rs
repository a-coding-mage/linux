/* SPDX-License-Identifier: GPL-2.0 */

/**
 * generic_ffs - find first bit set
 * @x: the word to search
 *
 * This is defined the same way as
 * the libc and compiler builtin ffs routines, therefore
 * differs in spirit from ffz (man ffs).
 */
#[inline]
pub fn generic_ffs(mut x: i32) -> i32 {
    let mut r: i32 = 1;

    if x == 0 {
        return 0;
    }
    if (x & 0xffff) == 0 {
        x >>= 16;
        r += 16;
    }
    if (x & 0xff) == 0 {
        x >>= 8;
        r += 8;
    }
    if (x & 0xf) == 0 {
        x >>= 4;
        r += 4;
    }
    if (x & 3) == 0 {
        x >>= 2;
        r += 2;
    }
    if (x & 1) == 0 {
        x >>= 1;
        r += 1;
    }
    r
}

// The C header defines ffs(x) as generic_ffs(x) unless __HAVE_ARCH_FFS is set.
#[inline]
pub fn ffs(x: i32) -> i32 {
    generic_ffs(x)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
