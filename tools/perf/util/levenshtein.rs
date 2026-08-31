// SPDX-License-Identifier: GPL-2.0
// C dependencies: "levenshtein.h", <errno.h>, <stdlib.h>, <string.h>

use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

/*
 * This function implements the Damerau-Levenshtein algorithm to
 * calculate a distance between strings.
 *
 * Basically, it says how many letters need to be swapped, substituted,
 * deleted from, or added to string1, at least, to get string2.
 *
 * The idea is to build a distance matrix for the substrings of both
 * strings.  To avoid a large space complexity, only the last three rows
 * are kept in memory (if swaps had the same or higher cost as one deletion
 * plus one insertion, only two rows would be needed).
 *
 * At any stage, "i + 1" denotes the length of the current substring of
 * string1 that the distance is calculated for.
 *
 * row2 holds the current row, row1 the previous row (i.e. for the substring
 * of string1 of length "i"), and row0 the row before that.
 *
 * In other words, at the start of the big loop, row2[j + 1] contains the
 * Damerau-Levenshtein distance between the substring of string1 of length
 * "i" and the substring of string2 of length "j + 1".
 *
 * All the big loop does is determine the partial minimum-cost paths.
 *
 * It does so by calculating the costs of the path ending in characters
 * i (in string1) and j (in string2), respectively, given that the last
 * operation is a substitution, a swap, a deletion, or an insertion.
 *
 * This implementation allows the costs to be weighted:
 *
 * - w (as in "sWap")
 * - s (as in "Substitution")
 * - a (for insertion, AKA "Add")
 * - d (as in "Deletion")
 *
 * Note that this algorithm calculates a distance _iff_ d == a.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn levenshtein(
    string1: *const c_char,
    string2: *const c_char,
    w: c_int,
    s: c_int,
    a: c_int,
    d: c_int,
) -> c_int {
    unsafe {
        let len1 = strlen(string1) as c_int;
        let len2 = strlen(string2) as c_int;
        let mut row0 = malloc(core::mem::size_of::<c_int>() * ((len2 + 1) as usize)) as *mut c_int;
        let mut row1 = malloc(core::mem::size_of::<c_int>() * ((len2 + 1) as usize)) as *mut c_int;
        let mut row2 = malloc(core::mem::size_of::<c_int>() * ((len2 + 1) as usize)) as *mut c_int;
        let mut i: c_int;
        let mut j: c_int;

        j = 0;
        while j <= len2 {
            *row1.offset(j as isize) = j * a;
            j += 1;
        }
        i = 0;
        while i < len1 {
            let dummy: *mut c_int;

            *row2.offset(0) = (i + 1) * d;
            j = 0;
            while j < len2 {
                /* substitution */
                *row2.offset((j + 1) as isize) = *row1.offset(j as isize)
                    + s * ((*string1.offset(i as isize) != *string2.offset(j as isize)) as c_int);
                /* swap */
                if i > 0
                    && j > 0
                    && *string1.offset((i - 1) as isize) == *string2.offset(j as isize)
                    && *string1.offset(i as isize) == *string2.offset((j - 1) as isize)
                    && *row2.offset((j + 1) as isize) > *row0.offset((j - 1) as isize) + w
                {
                    *row2.offset((j + 1) as isize) = *row0.offset((j - 1) as isize) + w;
                }
                /* deletion */
                if *row2.offset((j + 1) as isize) > *row1.offset((j + 1) as isize) + d {
                    *row2.offset((j + 1) as isize) = *row1.offset((j + 1) as isize) + d;
                }
                /* insertion */
                if *row2.offset((j + 1) as isize) > *row2.offset(j as isize) + a {
                    *row2.offset((j + 1) as isize) = *row2.offset(j as isize) + a;
                }
                j += 1;
            }

            dummy = row0;
            row0 = row1;
            row1 = row2;
            row2 = dummy;
            i += 1;
        }

        i = *row1.offset(len2 as isize);
        free(row0 as *mut c_void);
        free(row1 as *mut c_void);
        free(row2 as *mut c_void);

        i
    }
}
