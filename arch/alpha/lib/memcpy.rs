// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/alpha/lib/memcpy.c
 *
 *  Copyright (C) 1995  Linus Torvalds
 */

/* This is a reasonably optimized memcpy() routine. */

use core::ffi::c_void;
use core::ptr;

unsafe fn align_dest_to8_up(mut d: usize, mut s: usize, mut n: isize) -> (usize, usize, isize) {
    while d & 7 != 0 {
        if n <= 0 { return (d, s, n); }
        n -= 1;
        *(d as *mut u8) = *(s as *const u8);
        d += 1;
        s += 1;
    }
    (d, s, n)
}

unsafe fn align_dest_to8_dn(mut d: usize, mut s: usize, mut n: isize) -> (usize, usize, isize) {
    while d & 7 != 0 {
        if n <= 0 { return (d, s, n); }
        n -= 1;
        d -= 1;
        s -= 1;
        *(d as *mut u8) = *(s as *const u8);
    }
    (d, s, n)
}

unsafe fn do_rest_up(mut d: usize, mut s: usize, mut n: isize) {
    while n > 0 {
        n -= 1;
        *(d as *mut u8) = *(s as *const u8);
        d += 1;
        s += 1;
    }
}

unsafe fn do_rest_dn(mut d: usize, mut s: usize, mut n: isize) {
    while n > 0 {
        n -= 1;
        d -= 1;
        s -= 1;
        *(d as *mut u8) = *(s as *const u8);
    }
}

unsafe fn memcpy_unaligned_up(mut d: usize, mut s: usize, mut n: isize) {
    (d, s, n) = align_dest_to8_up(d, s, n);
    n -= 8;
    if n >= 0 {
        let mut low_word = ptr::read_unaligned(s as *const usize);
        loop {
            let high_word = ptr::read_unaligned((s + 8) as *const usize);
            n -= 8;
            // Alpha ldq_u/extql/extqh form the naturally ordered unaligned word.
            let shift = (s & 7) * 8;
            let tmp = if shift == 0 { 0 } else { high_word << (64 - shift) };
            low_word = if shift == 0 { low_word } else { low_word >> shift };
            s += 8;
            ptr::write(d as *mut usize, low_word | tmp);
            d += 8;
            low_word = high_word;
            if n < 0 { break; }
        }
    }
    n += 8;
    do_rest_up(d, s, n);
}

unsafe fn memcpy_unaligned_dn(mut d: usize, mut s: usize, mut n: isize) {
    s = s.wrapping_add(n as usize);
    d = d.wrapping_add(n as usize);
    while n > 0 {
        n -= 1;
        d -= 1;
        s -= 1;
        *(d as *mut u8) = *(s as *const u8);
    }
}

unsafe fn memcpy_aligned_up(mut d: usize, mut s: usize, mut n: isize) {
    (d, s, n) = align_dest_to8_up(d, s, n);
    n -= 8;
    while n >= 0 {
        let tmp = ptr::read(s as *const usize);
        n -= 8;
        s += 8;
        ptr::write(d as *mut usize, tmp);
        d += 8;
    }
    n += 8;
    do_rest_up(d, s, n);
}

unsafe fn memcpy_aligned_dn(mut d: usize, mut s: usize, mut n: isize) {
    s = s.wrapping_add(n as usize);
    d = d.wrapping_add(n as usize);
    (d, s, n) = align_dest_to8_dn(d, s, n);
    n -= 8;
    while n >= 0 {
        s -= 8;
        let tmp = ptr::read(s as *const usize);
        n -= 8;
        d -= 8;
        ptr::write(d as *mut usize, tmp);
    }
    n += 8;
    do_rest_dn(d, s, n);
}

pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let d = dest as usize;
    let s = src as usize;
    if ((d ^ s) & 7) == 0 {
        memcpy_aligned_up(d, s, n as isize);
        return dest;
    }
    memcpy_unaligned_up(d, s, n as isize);
    dest
}

// EXPORT_SYMBOL(memcpy);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
