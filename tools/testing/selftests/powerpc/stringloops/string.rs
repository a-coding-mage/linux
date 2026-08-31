// SPDX-License-Identifier: GPL-2.0
/*
 * Copied from linux/lib/string.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

// C dependency intent: #include <stddef.h> for size_t.

/**
 * strlen - Find the length of a string
 * @s: The string to be sized
 */
#[no_mangle]
pub unsafe extern "C" fn test_strlen(s: *const ::core::ffi::c_char) -> usize {
    let mut sc: *const ::core::ffi::c_char;

    sc = s;
    while *sc != 0 {
        sc = sc.add(1);
    }
    sc.offset_from(s) as usize
}
