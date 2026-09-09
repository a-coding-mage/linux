// SPDX-License-Identifier: GPL-2.0-only
/*
 * A generic implementation of binary search for the Linux kernel
 *
 * Copyright (C) 2008-2009 Ksplice, Inc.
 * Author: Tim Abbott <tabbott@ksplice.com>
 */

use core::ffi::c_void;

/*
 * bsearch - binary search an array of elements
 * @key: pointer to item being searched for
 * @base: pointer to first element to search
 * @num: number of elements
 * @size: size of each element
 * @cmp: pointer to comparison function
 *
 * This function does a binary search on the given array.  The
 * contents of the array should already be in ascending sorted order
 * under the provided comparison function.
 *
 * Note that the key need not have the same type as the elements in
 * the array, e.g. key could be a string and the comparison function
 * could compare the string with the struct's name field.  However, if
 * the key and elements in the array are of the same type, you can use
 * the same comparison function for both sort() and bsearch().
 */
pub unsafe fn bsearch(
    key: *const c_void,
    base: *const c_void,
    num: usize,
    size: usize,
    cmp: cmp_func_t,
) -> *mut c_void {
    __inline_bsearch(key, base, num, size, cmp)
}

// EXPORT_SYMBOL(bsearch);
// NOKPROBE_SYMBOL(bsearch);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
