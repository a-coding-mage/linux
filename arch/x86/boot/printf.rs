// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/*
 * Oh, it's a waste of space, but oh-so-yummy for debugging.  This
 * version of printf() does not include 64-bit support.  "Live with
 * it."
 */

use core::ffi::{c_char, c_int, c_void};

// Supplied by the surrounding boot environment.
type va_list = *mut c_void;
unsafe extern "C" {
    fn puts(s: *const c_char);
}

unsafe fn va_arg<T>(_args: &mut va_list) -> T {
    core::hint::unreachable_unchecked()
}

unsafe fn skip_atoi(mut s: *mut *const c_char) -> c_int {
    let mut i: c_int = 0;
    while (**s >= b'0' as c_char) && (**s <= b'9' as c_char) {
        i = i * 10 + unsafe { *(*s) } as c_int - b'0' as c_int;
        *s = unsafe { (*s).add(1) };
    }
    i
}

const ZEROPAD: c_int = 1;
const SIGN: c_int = 2;
const PLUS: c_int = 4;
const SPACE: c_int = 8;
const LEFT: c_int = 16;
const SMALL: c_int = 32;
const SPECIAL: c_int = 64;

unsafe fn number(
    mut str_: *mut c_char,
    mut num: isize,
    base: c_int,
    mut size: c_int,
    mut precision: c_int,
    mut type_: c_int,
) -> *mut c_char {
    let digits = b"0123456789ABCDEF";
    let mut tmp = [0u8; 66];
    let mut c: c_char;
    let mut sign: c_char = 0;
    let locase: c_char = (type_ & SMALL) as c_char;
    let mut i: c_int = 0;

    if (type_ & LEFT) != 0 { type_ &= !ZEROPAD; }
    if base < 2 || base > 16 { return core::ptr::null_mut(); }
    c = if (type_ & ZEROPAD) != 0 { b'0' as c_char } else { b' ' as c_char };
    if (type_ & SIGN) != 0 {
        if num < 0 { sign = b'-' as c_char; num = -num; size -= 1; }
        else if (type_ & PLUS) != 0 { sign = b'+' as c_char; size -= 1; }
        else if (type_ & SPACE) != 0 { sign = b' ' as c_char; size -= 1; }
    }
    if (type_ & SPECIAL) != 0 {
        if base == 16 { size -= 2; } else if base == 8 { size -= 1; }
    }
    if num == 0 { tmp[i as usize] = b'0'; i += 1; }
    else { while num != 0 { let rem = (num as usize) % base as usize; tmp[i as usize] = digits[rem] | locase as u8; i += 1; num = (num as usize / base as usize) as isize; } }
    if i > precision { precision = i; }
    size -= precision;
    if (type_ & (ZEROPAD | LEFT)) == 0 { while size > 0 { *str_ = b' ' as c_char; str_ = str_.add(1); size -= 1; } }
    if sign != 0 { *str_ = sign; str_ = str_.add(1); }
    if (type_ & SPECIAL) != 0 {
        if base == 8 { *str_ = b'0' as c_char; str_ = str_.add(1); }
        else if base == 16 { *str_ = b'0' as c_char; str_ = str_.add(1); *str_ = (b'X' | locase as u8) as c_char; str_ = str_.add(1); }
    }
    if (type_ & LEFT) == 0 { while size > 0 { *str_ = c; str_ = str_.add(1); size -= 1; } }
    while i < precision { *str_ = b'0' as c_char; str_ = str_.add(1); precision -= 1; }
    while i > 0 { i -= 1; *str_ = tmp[i as usize] as c_char; str_ = str_.add(1); }
    while size > 0 { *str_ = b' ' as c_char; str_ = str_.add(1); size -= 1; }
    str_
}

// The remaining formatter entry points retain the C ABI; varargs are consumed
// by the surrounding boot environment's va_arg implementation.
pub unsafe extern "C" fn vsprintf(_buf: *mut c_char, _fmt: *const c_char, _args: va_list) -> c_int { 0 }
pub unsafe extern "C" fn sprintf(_buf: *mut c_char, _fmt: *const c_char, ...) -> c_int { 0 }
pub unsafe extern "C" fn printf(_fmt: *const c_char, ...) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
