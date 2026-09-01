// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/tools/lib/string.c
 *
 *  Copied from linux/lib/string.c, where it is:
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  More specifically, the first copied function was strtobool, which
 *  was introduced by:
 *
 *  d0f1fed29e6e ("Add a strtobool function matching semantics of existing in kernel equivalents")
 *  Author: Jonathan Cameron <jic23@cam.ac.uk>
 */

use core::ptr;
use std::os::raw::{c_char, c_int, c_void};

type size_t = usize;
type u8_t = u8;
type u64_t = u64;

const EINVAL: c_int = 22;

extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn isspace(c: c_int) -> c_int;
}

/**
 * memdup - duplicate region of memory
 *
 * @src: memory region to duplicate
 * @len: memory region length
 */
#[no_mangle]
pub unsafe extern "C" fn memdup(src: *const c_void, len: size_t) -> *mut c_void {
    let p = malloc(len);

    if !p.is_null() {
        memcpy(p, src, len);
    }

    p
}

/**
 * strtobool - convert common user inputs into boolean values
 * @s: input string
 * @res: result
 *
 * This routine returns 0 iff the first character is one of 'Yy1Nn0', or
 * [oO][NnFf] for "on" and "off". Otherwise it will return -EINVAL.  Value
 * pointed to by res is updated upon finding a match.
 */
#[no_mangle]
pub unsafe extern "C" fn strtobool(s: *const c_char, res: *mut bool) -> c_int {
    if s.is_null() {
        return -EINVAL;
    }

    match *s as u8 {
        b'y' | b'Y' | b'1' => {
            *res = true;
            return 0;
        }
        b'n' | b'N' | b'0' => {
            *res = false;
            return 0;
        }
        b'o' | b'O' => match *s.add(1) as u8 {
            b'n' | b'N' => {
                *res = true;
                return 0;
            }
            b'f' | b'F' => {
                *res = false;
                return 0;
            }
            _ => {}
        },
        _ => {}
    }

    -EINVAL
}

/**
 * strlcpy - Copy a C-string into a sized buffer
 * @dest: Where to copy the string to
 * @src: Where to copy the string from
 * @size: size of destination buffer
 *
 * Compatible with *BSD: the result is always a valid
 * NUL-terminated string that fits in the buffer (unless,
 * of course, the buffer size is zero). It does not pad
 * out the result like strncpy() does.
 *
 * If libc has strlcpy() then that version will override this
 * implementation:
 */
/* C source uses __weak here; preserve the C ABI name for this fallback body. */
#[no_mangle]
pub unsafe extern "C" fn strlcpy(dest: *mut c_char, src: *const c_char, size: size_t) -> size_t {
    let ret = strlen(src);

    if size != 0 {
        let len = if ret >= size { size - 1 } else { ret };
        memcpy(dest as *mut c_void, src as *const c_void, len);
        *dest.add(len) = b'\0' as c_char;
    }
    ret
}

/**
 * skip_spaces - Removes leading whitespace from @str.
 * @str: The string to be stripped.
 *
 * Returns a pointer to the first non-whitespace character in @str.
 */
#[no_mangle]
pub unsafe extern "C" fn skip_spaces(mut str_: *const c_char) -> *mut c_char {
    while isspace(*str_ as c_int) != 0 {
        str_ = str_.add(1);
    }
    str_ as *mut c_char
}

/**
 * strim - Removes leading and trailing whitespace from @s.
 * @s: The string to be stripped.
 *
 * Note that the first trailing whitespace is replaced with a %NUL-terminator
 * in the given string @s. Returns a pointer to the first non-whitespace
 * character in @s.
 */
#[no_mangle]
pub unsafe extern "C" fn strim(s: *mut c_char) -> *mut c_char {
    let size: size_t;
    let mut end: *mut c_char;

    size = strlen(s);
    if size == 0 {
        return s;
    }

    end = s.add(size - 1);
    while (end as usize) >= (s as usize) && isspace(*end as c_int) != 0 {
        end = end.sub(1);
    }
    *end.add(1) = b'\0' as c_char;

    skip_spaces(s)
}

/*
 * remove_spaces - Removes whitespaces from @s
 */
#[no_mangle]
pub unsafe extern "C" fn remove_spaces(s: *mut c_char) {
    let mut s = s;
    let mut d = s;

    loop {
        while *d == b' ' as c_char {
            d = d.add(1);
        }

        let ch = *d;
        *s = ch;
        s = s.add(1);
        d = d.add(1);

        if ch == 0 {
            break;
        }
    }
}

/**
 * strreplace - Replace all occurrences of character in string.
 * @s: The string to operate on.
 * @old: The character being replaced.
 * @new: The character @old is replaced with.
 *
 * Returns pointer to the nul byte at the end of @s.
 */
#[no_mangle]
pub unsafe extern "C" fn strreplace(mut s: *mut c_char, old: c_char, new: c_char) -> *mut c_char {
    while *s != 0 {
        if *s == old {
            *s = new;
        }
        s = s.add(1);
    }
    s
}

unsafe fn check_bytes8(mut start: *const u8_t, value: u8_t, mut bytes: c_uint) -> *mut c_void {
    while bytes != 0 {
        if *start != value {
            return start as *mut c_void;
        }
        start = start.add(1);
        bytes -= 1;
    }
    ptr::null_mut()
}

type c_uint = std::os::raw::c_uint;

/**
 * memchr_inv - Find an unmatching character in an area of memory.
 * @start: The memory area
 * @c: Find a character other than c
 * @bytes: The size of the area.
 *
 * returns the address of the first character other than @c, or %NULL
 * if the whole buffer contains just @c.
 */
#[no_mangle]
pub unsafe extern "C" fn memchr_inv(start: *const c_void, c: c_int, bytes: size_t) -> *mut c_void {
    let mut start = start as *const u8_t;
    let value: u8_t = c as u8_t;
    let mut value64: u64_t;
    let mut words: c_uint;
    let mut prefix: c_uint;

    if bytes <= 16 {
        return check_bytes8(start, value, bytes as c_uint);
    }

    value64 = value as u64_t;
    value64 |= value64 << 8;
    value64 |= value64 << 16;
    value64 |= value64 << 32;

    prefix = (start as usize % 8) as c_uint;
    if prefix != 0 {
        let r: *mut c_void;

        prefix = 8 - prefix;
        r = check_bytes8(start, value, prefix);
        if !r.is_null() {
            return r;
        }
        start = start.add(prefix as usize);
        let bytes = bytes - prefix as size_t;

        words = (bytes / 8) as c_uint;

        while words != 0 {
            if *(start as *const u64_t) != value64 {
                return check_bytes8(start, value, 8);
            }
            start = start.add(8);
            words -= 1;
        }

        return check_bytes8(start, value, (bytes % 8) as c_uint);
    }

    words = (bytes / 8) as c_uint;

    while words != 0 {
        if *(start as *const u64_t) != value64 {
            return check_bytes8(start, value, 8);
        }
        start = start.add(8);
        words -= 1;
    }

    check_bytes8(start, value, (bytes % 8) as c_uint)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
