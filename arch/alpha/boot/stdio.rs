// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) Paul Mackerras 1997.
 */

use core::ffi::VaList;
use core::ptr;

pub unsafe fn strnlen(s: *const i8, mut count: usize) -> usize {
    let mut sc = s;
    while count != 0 && *sc != 0 {
        count -= 1;
        sc = sc.add(1);
    }
    sc.offset_from(s) as usize
}

const ZEROPAD: i32 = 1;
const SIGN: i32 = 2;
const PLUS: i32 = 4;
const SPACE: i32 = 8;
const LEFT: i32 = 16;
const SPECIAL: i32 = 32;
const LARGE: i32 = 64;

unsafe fn skip_atoi(mut s: *mut *const i8) -> i32 {
    let mut i = 0;
    let mut c = **s;
    while b'0' as i8 <= c && c <= b'9' as i8 {
        i = i * 10 + c as i32 - b'0' as i32;
        *s = (*s).add(1);
        c = **s;
    }
    i
}

unsafe fn number(mut str_: *mut i8, mut num: u64, base: i32, mut size: i32, mut precision: i32, mut typ: i32) -> *mut i8 {
    let mut tmp = [0i8; 66];
    let digits = if typ & LARGE != 0 { b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\0" } else { b"0123456789abcdefghijklmnopqrstuvwxyz\0" };
    if typ & LEFT != 0 { typ &= !ZEROPAD; }
    if base < 2 || base > 36 { return ptr::null_mut(); }
    let c = if typ & ZEROPAD != 0 { b'0' as i8 } else { b' ' as i8 };
    let mut sign = 0i8;
    if typ & SIGN != 0 {
        if (num as i64) < 0 { sign = b'-' as i8; num = (-(num as i64)) as u64; size -= 1; }
        else if typ & PLUS != 0 { sign = b'+' as i8; size -= 1; }
        else if typ & SPACE != 0 { sign = b' ' as i8; size -= 1; }
    }
    if typ & SPECIAL != 0 { if base == 16 { size -= 2; } else if base == 8 { size -= 1; } }
    let mut i = 0usize;
    if num == 0 { tmp[i] = b'0' as i8; i += 1; }
    while num != 0 { let rem = (num % base as u64) as usize; num /= base as u64; tmp[i] = digits[rem] as i8; i += 1; }
    if i as i32 > precision { precision = i as i32; }
    size -= precision;
    if typ & (ZEROPAD | LEFT) == 0 { while size > 0 { *str_ = b' ' as i8; str_ = str_.add(1); size -= 1; } }
    if sign != 0 { *str_ = sign; str_ = str_.add(1); }
    if typ & SPECIAL != 0 { if base == 8 { *str_ = b'0' as i8; str_ = str_.add(1); } else if base == 16 { *str_ = b'0' as i8; *str_.add(1) = digits[33] as i8; str_ = str_.add(2); } }
    if typ & LEFT == 0 { while size > 0 { *str_ = c; str_ = str_.add(1); size -= 1; } }
    while (i as i32) < precision { *str_ = b'0' as i8; str_ = str_.add(1); precision -= 1; }
    while i > 0 { i -= 1; *str_ = tmp[i]; str_ = str_.add(1); }
    while size > 0 { *str_ = b' '; str_ = str_.add(1); size -= 1; }
    str_
}

pub unsafe fn vsprintf(buf: *mut i8, fmt: *const i8, mut args: VaList<'_>) -> i32 {
    let mut str_ = buf;
    let mut f = fmt;
    while *f != 0 {
        if *f != b'%' as i8 { *str_ = *f; str_ = str_.add(1); f = f.add(1); continue; }
        f = f.add(1);
        let mut flags = 0;
        loop { match *f as u8 { b'-' => flags |= LEFT, b'+' => flags |= PLUS, b' ' => flags |= SPACE, b'#' => flags |= SPECIAL, b'0' => flags |= ZEROPAD, _ => break } f = f.add(1); }
        let mut width = -1;
        if (*f as u8) >= b'0' && (*f as u8) <= b'9' { width = skip_atoi(&mut f); } else if *f == b'*' as i8 { f = f.add(1); width = args.arg::<i32>(); if width < 0 { width = -width; flags |= LEFT; } }
        let mut precision = -1;
        if *f == b'.' as i8 { f = f.add(1); if (*f as u8) >= b'0' && (*f as u8) <= b'9' { precision = skip_atoi(&mut f); } else if *f == b'*' as i8 { f = f.add(1); precision = args.arg::<i32>(); } if precision < 0 { precision = 0; } }
        let mut qualifier = -1;
        if *f == b'l' as i8 && *f.add(1) == b'l' as i8 { qualifier = b'q' as i32; f = f.add(2); } else if [b'h', b'l', b'L', b'Z'].contains(&(*f as u8)) { qualifier = *f as i32; f = f.add(1); }
        let mut base = 10;
        match *f as u8 { b'c' => { if flags & LEFT == 0 { while width > 1 { *str_ = b' '; str_ = str_.add(1); width -= 1; } } *str_ = args.arg::<i32>() as u8 as i8; str_ = str_.add(1); while width > 1 { *str_ = b' '; str_ = str_.add(1); width -= 1; } }, b'%' => { *str_ = b'%'; str_ = str_.add(1); }, b'o' => { base = 8; }, b'X' => { flags |= LARGE; base = 16; }, b'x' => { base = 16; }, b'd' | b'i' => { flags |= SIGN; }, b'u' => {}, _ => { *str_ = b'%'; str_ = str_.add(1); if *f != 0 { *str_ = *f; str_ = str_.add(1); } } }
        if (*f as u8) == b'c' || (*f as u8) == b'%' || !matches!(*f as u8, b'o'|b'X'|b'x'|b'd'|b'i'|b'u') { f = f.add(1); continue; }
        let mut num = if qualifier == b'l' as i32 { args.arg::<u64>() } else if qualifier == b'q' as i32 { args.arg::<u64>() } else if qualifier == b'Z' as i32 { args.arg::<usize>() as u64 } else { args.arg::<u32>() as u64 };
        if flags & SIGN != 0 { num = num as i64 as u64; }
        str_ = number(str_, num, base, width, precision, flags); f = f.add(1);
    }
    *str_ = 0; str_.offset_from(buf) as i32
}

pub unsafe fn sprintf(buf: *mut i8, fmt: *const i8, mut args: VaList<'_>) -> i32 { vsprintf(buf, fmt, args) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
