/* SPDX-License-Identifier: GPL-2.0 */

/*
 * ffz - find first zero in word.
 * @word: The word to search
 *
 * Undefined if no zero exists, so code should check against ~0UL first.
 */
#[macro_export]
macro_rules! ffz {
    ($x:expr) => {
        __ffs(!($x))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
