// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) Paul Mackerras 1997.
 */

use core::ffi::VaList;
use core::ffi::c_void;

pub unsafe fn strnlen(mut s: *const u8, mut count: usize) -> usize {
    let start = s;
    while count != 0 && *s != 0 {
        count -= 1;
        s = s.add(1);
    }
    s.offset_from(start) as usize
}

pub unsafe fn strrchr(mut s: *const u8, c: i32) -> *mut u8 {
    let mut last = core::ptr::null();
    loop {
        if *s == c as u8 { last = s; }
        let old = s;
        s = s.add(1);
        if *old == 0 { break; }
    }
    last as *mut u8
}

unsafe fn skip_atoi(mut s: *mut *const u8) -> i32 {
    let mut i = 0;
    let mut c = **s;
    while b'0' <= c && c <= b'9' {
        i = i * 10 + c as i32 - b'0' as i32;
        *s = (*s).add(1);
        c = **s;
    }
    i
}

const ZEROPAD: i32 = 1;
const SIGN: i32 = 2;
const PLUS: i32 = 4;
const SPACE: i32 = 8;
const LEFT: i32 = 16;
const SPECIAL: i32 = 32;
const LARGE: i32 = 64;

unsafe fn number(mut str_: *mut u8, mut num: u64, base: i32, mut size: i32, mut precision: i32, mut typ: i32) -> *mut u8 {
    let mut tmp = [0u8; 66];
    let digits = if typ & LARGE != 0 { b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\0" } else { b"0123456789abcdefghijklmnopqrstuvwxyz\0" };
    if typ & LEFT != 0 { typ &= !ZEROPAD; }
    if base < 2 || base > 36 { return core::ptr::null_mut(); }
    let c = if typ & ZEROPAD != 0 { b'0' } else { b' ' };
    let mut sign = 0u8;
    if typ & SIGN != 0 {
        if (num as i64) < 0 { sign = b'-'; num = (-(num as i64)) as u64; size -= 1; }
        else if typ & PLUS != 0 { sign = b'+'; size -= 1; }
        else if typ & SPACE != 0 { sign = b' '; size -= 1; }
    }
    if typ & SPECIAL != 0 { if base == 16 { size -= 2; } else if base == 8 { size -= 1; } }
    let mut i = 0;
    if num == 0 { tmp[i] = b'0'; i += 1; }
    else { while num != 0 { let rem = (num % base as u64) as usize; num /= base as u64; tmp[i] = digits[rem]; i += 1; } }
    if i as i32 > precision { precision = i as i32; }
    size -= precision;
    if typ & (ZEROPAD | LEFT) == 0 { while size > 0 { *str_ = b' '; str_ = str_.add(1); size -= 1; } }
    if sign != 0 { *str_ = sign; str_ = str_.add(1); }
    if typ & SPECIAL != 0 { if base == 8 { *str_ = b'0'; str_ = str_.add(1); } else if base == 16 { *str_ = b'0'; *str_.add(1) = digits[33]; str_ = str_.add(2); } }
    if typ & LEFT == 0 { while size > 0 { *str_ = c; str_ = str_.add(1); size -= 1; } }
    while (i as i32) < precision { *str_ = b'0'; str_ = str_.add(1); precision -= 1; }
    while i > 0 { i -= 1; *str_ = tmp[i]; str_ = str_.add(1); }
    while size > 0 { *str_ = b' '; str_ = str_.add(1); size -= 1; }
    str_
}

// The variadic interface and console_ops are supplied by the surrounding platform code.
pub unsafe extern "C" fn vsprintf(buf: *mut u8, fmt: *const u8, mut args: VaList<'_>) -> i32 {
    let mut str_ = buf; let mut f = fmt;
    while *f != 0 {
        if *f != b'%' { *str_ = *f; str_ = str_.add(1); f = f.add(1); continue; }
        f = f.add(1); let mut flags = 0;
        loop { match *f { b'-' => flags |= LEFT, b'+' => flags |= PLUS, b' ' => flags |= SPACE, b'#' => flags |= SPECIAL, b'0' => flags |= ZEROPAD, _ => break } f = f.add(1); }
        let mut width = -1; if *f >= b'0' && *f <= b'9' { width = skip_atoi(&mut f); } else if *f == b'*' { f = f.add(1); width = args.arg::<i32>(); if width < 0 { width = -width; flags |= LEFT; } }
        let mut precision = -1; if *f == b'.' { f = f.add(1); if *f >= b'0' && *f <= b'9' { precision = skip_atoi(&mut f); } else if *f == b'*' { f = f.add(1); precision = args.arg::<i32>(); } if precision < 0 { precision = 0; } }
        let mut qualifier = -1; if *f == b'l' && *f.add(1) == b'l' { qualifier = b'q' as i32; f = f.add(2); } else if matches!(*f, b'h'|b'l'|b'L'|b'Z') { qualifier = *f as i32; f = f.add(1); }
        let mut base = 10;
        match *f { b'%' => { *str_ = b'%'; str_ = str_.add(1); f = f.add(1); continue; }, b'c' => { if flags & LEFT == 0 { while width > 1 { *str_=b' '; str_=str_.add(1); width-=1; } } *str_=args.arg::<i32>() as u8; str_=str_.add(1); while width>1 { *str_=b' '; str_=str_.add(1); width-=1; } f=f.add(1); continue; }, b's' => { let mut s=args.arg::<*const u8>(); if s.is_null(){s=b"<NULL>\0".as_ptr();} let len=strnlen(s, if precision<0 {usize::MAX}else{precision as usize}) as i32; if flags&LEFT==0 {while len<width{*str_=b' ';str_=str_.add(1);width-=1;}} for _ in 0..len{*str_=*s;s=s.add(1);str_=str_.add(1);} while len<width{*str_=b' ';str_=str_.add(1);width-=1;} f=f.add(1);continue; }, b'p' => { if width<0 {width=2*(core::mem::size_of::<*const u8>() as i32);flags|=ZEROPAD;} str_=number(str_,args.arg::<*mut c_void>() as usize as u64,16,width,precision,flags);f=f.add(1);continue; }, b'n' => { let n=str_.offset_from(buf) as i32; match qualifier {108=>*args.arg::<*mut i64>()=n as i64,90=>*args.arg::<*mut usize>()=n as usize,_=>*args.arg::<*mut i32>()=n};f=f.add(1);continue; }, b'o'=>base=8, b'X'=>{flags|=LARGE;base=16}, b'x'=>base=16, b'd'|b'i'=>flags|=SIGN, b'u'=>{}, _=>{*str_=b'%';str_=str_.add(1);if *f!=0{*str_=*f;str_=str_.add(1);f=f.add(1);}continue;} }
        let mut num = match qualifier { 108 => args.arg::<u64>(), 113 => args.arg::<u64>(), 90 => args.arg::<usize>() as u64, 104 => args.arg::<i32>() as u16 as u64, _ => args.arg::<u32>() as u64 }; if flags & SIGN != 0 { num = num as i64 as u64; } str_ = number(str_, num, base, width, precision, flags); f=f.add(1);
    }
    *str_=0; str_.offset_from(buf) as i32
}

static mut SPRINT_BUF: [u8; 1024] = [0; 1024];

#[repr(C)]
pub struct ConsoleOps { pub write: Option<unsafe extern "C" fn(*const u8, i32)> }
extern "C" { pub static mut console_ops: ConsoleOps; }

pub unsafe extern "C" fn printf(fmt: *const u8, mut args: ...) -> i32 {
    let n = vsprintf(SPRINT_BUF.as_mut_ptr(), fmt, args.as_va_list());
    if let Some(write) = console_ops.write { write(SPRINT_BUF.as_ptr(), n); }
    n
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
