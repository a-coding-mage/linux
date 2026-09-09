/* SPDX-License-Identifier: GPL-2.0 */

/**
 * ffs - find first bit set
 * @x: the word to search
 *
 * This is defined the same way as
 * the libc and compiler builtin ffs routines, therefore
 * differs in spirit from ffz (man ffs).
 */
/* Equivalent of the compiler builtin ffs: return the one-based index of the
 * least-significant set bit, or 0 when no bit is set. */
macro_rules! ffs {
    ($x:expr) => {{
        let __ffs_x = $x;
        if __ffs_x == 0 {
            0
        } else {
            (__ffs_x.trailing_zeros() as i32) + 1
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
