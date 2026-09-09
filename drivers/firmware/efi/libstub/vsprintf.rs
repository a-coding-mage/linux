// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/* Oh, it's a waste of space, but oh-so-yummy for debugging. */

use core::ffi::VaList;
use core::ptr;

const ZEROPAD: i32 = 1;
const SIGN: i32 = 2;
const PLUS: i32 = 4;
const SPACE: i32 = 8;
const LEFT: i32 = 16;
const SMALL: i32 = 32;
const SPECIAL: i32 = 64;
const WIDE: i32 = 128;

unsafe fn skip_atoi(mut s: *mut *const u8) -> i32 {
    let mut i = 0;
    while (**s).is_ascii_digit() {
        i = i * 10 + (**s - b'0') as i32;
        *s = (*s).add(1);
    }
    i
}

unsafe fn put_dec_full4(mut end: *mut u8, mut r: u32) {
    for _ in 0..3 {
        let q = ((r as u64 * 0xccd) >> 15) as u32;
        end = end.sub(1);
        *end = b'0' + (r - q * 10) as u8;
        r = q;
    }
    *end.sub(1) = b'0' + r as u8;
}

unsafe fn put_dec_helper4(end: *mut u8, x: u32) -> u32 {
    let q = ((x as u64 * 0x346dc5d7) >> 43) as u32;
    put_dec_full4(end, x - q * 10000);
    q
}

unsafe fn put_dec(end: *mut u8, n: u64) -> *mut u8 {
    let mut d1 = (n >> 16) as u32;
    let h = n >> 32;
    let d2 = (h & 0xffff) as u32;
    let d3 = (h >> 16) as u32;
    let mut q = 656 * d3 + 7296 * d2 + 5536 * d1 + (n as u32 & 0xffff);
    put_dec_helper4(end, q);
    let mut p = end.sub(4);
    q = 7671 * d3 + 9496 * d2 + 6 * d1 + put_dec_helper4(p, q);
    p = p.sub(4);
    q = 4749 * d3 + 42 * d2 + put_dec_helper4(p, q);
    p = p.sub(4);
    q = 281 * d3 + put_dec_helper4(p, q);
    p = p.sub(4);
    put_dec_full4(p, q);
    p = p.sub(4);
    while p < end && *p == b'0' { p = p.add(1); }
    p
}

unsafe fn number(mut end: *mut u8, mut num: u64, base: i32, locase: i32) -> *mut u8 {
    static DIGITS: &[u8; 16] = b"0123456789ABCDEF";
    match base {
        10 => if num != 0 { end = put_dec(end, num); },
        8 => while num != 0 { end = end.sub(1); *end = b'0' + (num & 7) as u8; num >>= 3; },
        16 => while num != 0 { end = end.sub(1); *end = DIGITS[(num & 0xf) as usize] | locase as u8; num >>= 4; },
        _ => core::hint::unreachable_unchecked(),
    }
    end
}

unsafe fn get_flags(fmt: &mut *const u8) -> i32 {
    let mut flags = 0;
    loop {
        match **fmt {
            b'-' => flags |= LEFT, b'+' => flags |= PLUS, b' ' => flags |= SPACE,
            b'#' => flags |= SPECIAL, b'0' => flags |= ZEROPAD,
            _ => return flags,
        }
        *fmt = (*fmt).add(1);
    }
}

unsafe fn get_int(fmt: &mut *const u8, ap: &mut VaList<'_>) -> i32 {
    if (**fmt).is_ascii_digit() { return skip_atoi(fmt); }
    if **fmt == b'*' { *fmt = (*fmt).add(1); return ap.arg::<i32>(); }
    0
}

unsafe fn get_number(sign: i32, qualifier: i32, ap: &mut VaList<'_>) -> u64 {
    if sign != 0 {
        match qualifier { b'L' as i32 => ap.arg::<i64>() as u64, b'l' as i32 => ap.arg::<i64>() as u64,
            b'h' as i32 => ap.arg::<i32>() as i16 as i64 as u64, b'H' as i32 => ap.arg::<i32>() as i8 as i64 as u64,
            _ => ap.arg::<i32>() as i64 as u64 }
    } else { match qualifier { b'L' as i32 => ap.arg::<u64>(), b'l' as i32 => ap.arg::<u64>(),
        b'h' as i32 => ap.arg::<i32>() as u16 as u64, b'H' as i32 => ap.arg::<i32>() as u8 as u64,
        _ => ap.arg::<u32>() as u64 } }
}

unsafe fn get_sign(num: &mut i64, flags: i32) -> u8 {
    if flags & SIGN == 0 { return 0; }
    if *num < 0 { *num = num.wrapping_neg(); return b'-'; }
    if flags & PLUS != 0 { return b'+'; }
    if flags & SPACE != 0 { return b' '; }
    0
}

unsafe fn utf16s_utf8nlen(mut s: *const u16, maxlen: usize) -> usize {
    let mut len = 0;
    while len < maxlen && *s != 0 {
        let c0 = *s; s = s.add(1);
        let mut clen = 1 + (c0 >= 0x80) as usize + (c0 >= 0x800) as usize;
        if len + clen > maxlen { break; }
        if c0 & 0xfc00 == 0xd800 {
            if len + clen == maxlen { break; }
            if *s & 0xfc00 == 0xdc00 { s = s.add(1); clen += 1; }
        }
        len += clen;
    }
    len
}

unsafe fn utf16_to_utf32(s: &mut *const u16) -> u32 {
    let c0 = **s; *s = (*s).add(1);
    if c0 & 0xf800 != 0xd800 { return c0 as u32; }
    if c0 & 0x0400 != 0 { return 0xfffd; }
    let c1 = **s;
    if c1 & 0xfc00 != 0xdc00 { return 0xfffd; }
    *s = (*s).add(1);
    (0x10000 - (0xd800 << 10) - 0xdc00 + ((c0 as u32) << 10) + c1 as u32)
}

pub unsafe extern "C" fn vsnprintf(buf: *mut u8, size: usize, mut fmt: *const u8, mut ap: VaList<'_>) -> i32 {
    let mut tmp = [0u8; (core::mem::size_of::<u64>() * 8 + 2) / 3];
    let tmp_end = tmp.as_mut_ptr().add(tmp.len());
    let mut args = ap;
    let mut pos = 0usize;
    while *fmt != 0 {
        if *fmt != b'%' || { fmt = fmt.add(1); *fmt == b'%' } {
            if pos < size { *buf.add(pos) = *fmt; } pos += 1; continue;
        }
        let mut flags = get_flags(&mut fmt);
        let mut field_width = get_int(&mut fmt, &mut args);
        if field_width < 0 { field_width = -field_width; flags |= LEFT; }
        if flags & LEFT != 0 { flags &= !ZEROPAD; }
        let mut precision = -1;
        if *fmt == b'.' { fmt = fmt.add(1); precision = get_int(&mut fmt, &mut args); if precision >= 0 { flags &= !ZEROPAD; } }
        let mut qualifier = -1;
        if *fmt == b'h' || *fmt == b'l' { qualifier = *fmt as i32; fmt = fmt.add(1); if qualifier == *fmt as i32 { qualifier -= b'a' as i32 - b'A' as i32; fmt = fmt.add(1); } }
        let mut sign = 0u8;
        let mut s: *const u8;
        let mut len: usize;
        let mut base = 0;
        match *fmt {
            b'c' => { flags &= LEFT; s = tmp.as_ptr(); if qualifier == b'l' as i32 { *(tmp.as_mut_ptr() as *mut u16) = args.arg::<u32>() as u16; *((tmp.as_mut_ptr() as *mut u16).add(1)) = 0; precision = i32::MAX; } else { tmp[0] = args.arg::<i32>() as u8; precision = 1; } len = 1; }
            b's' => { flags &= LEFT; if precision < 0 { precision = i32::MAX; } s = args.arg::<*const u8>(); if s.is_null() { s = if precision < 6 { b"\0".as_ptr() } else { b"(null)\0".as_ptr() }; } if qualifier == b'l' as i32 { flags |= WIDE; len = utf16s_utf8nlen(s as *const u16, precision as usize); } else { len = libc_strnlen(s, precision as usize); } }
            b'o' => { base = 8; s = ptr::null(); len = 0; }
            b'p' => { if precision < 0 { precision = (2 * core::mem::size_of::<*const u8>) as i32; } flags |= SMALL; base = 16; s = ptr::null(); len = 0; }
            b'x' => { flags |= SMALL; base = 16; s = ptr::null(); len = 0; }
            b'X' => { base = 16; s = ptr::null(); len = 0; }
            b'd' | b'i' => { flags |= SIGN; base = 10; s = ptr::null(); len = 0; }
            b'u' => { flags &= !SPECIAL; base = 10; s = ptr::null(); len = 0; }
            _ => break,
        }
        if base != 0 { let mut n = if *fmt == b'p' { args.arg::<*const u8>() as usize as u64 } else { get_number(flags & SIGN, qualifier, &mut args) }; if flags & SIGN != 0 { sign = get_sign(&mut (n as i64), flags); } s = number(tmp_end, n, base, flags & SMALL); len = tmp_end.offset_from(s) as usize; if precision < 0 { precision = 1; } if precision < len as i32 { precision = len as i32; } }
        let mut width = field_width - precision; while flags & LEFT == 0 && width > 0 { if pos < size { *buf.add(pos) = b' '; } pos += 1; width -= 1; }
        if sign != 0 { if pos < size { *buf.add(pos) = sign; } pos += 1; }
        while precision > len as i32 { if pos < size { *buf.add(pos) = b'0'; } pos += 1; precision -= 1; }
        while len > 0 { if pos < size { *buf.add(pos) = *s; } pos += 1; s = s.add(1); len -= 1; }
        while width > 0 { if pos < size { *buf.add(pos) = b' '; } pos += 1; width -= 1; }
        fmt = fmt.add(1);
    }
    if size != 0 { *buf.add(core::cmp::min(pos, size - 1)) = 0; }
    pos as i32
}

unsafe fn libc_strnlen(mut s: *const u8, max: usize) -> usize { let mut n = 0; while n < max && *s != 0 { n += 1; s = s.add(1); } n }

// C variadic wrapper; the platform ABI supplies the variadic argument list.
pub unsafe extern "C" fn snprintf(buf: *mut u8, size: usize, fmt: *const u8, mut args: ...) -> i32 {
    vsnprintf(buf, size, fmt, args)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
