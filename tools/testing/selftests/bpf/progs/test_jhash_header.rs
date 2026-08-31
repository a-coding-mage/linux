// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// C dependency intent: #include <features.h>

pub type u32 = ::core::ffi::c_uint;

#[inline(always)]
pub fn rol32(word: u32, shift: ::core::ffi::c_uint) -> u32 {
    word.wrapping_shl(shift)
        | word.wrapping_shr((0u32.wrapping_sub(shift as u32) & 31) as u32)
}

#[inline(always)]
pub fn __jhash_mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = (*a).wrapping_sub(*c);
    *a ^= rol32(*c, 4);
    *c = (*c).wrapping_add(*b);
    *b = (*b).wrapping_sub(*a);
    *b ^= rol32(*a, 6);
    *a = (*a).wrapping_add(*c);
    *c = (*c).wrapping_sub(*b);
    *c ^= rol32(*b, 8);
    *b = (*b).wrapping_add(*a);
    *a = (*a).wrapping_sub(*c);
    *a ^= rol32(*c, 16);
    *c = (*c).wrapping_add(*b);
    *b = (*b).wrapping_sub(*a);
    *b ^= rol32(*a, 19);
    *a = (*a).wrapping_add(*c);
    *c = (*c).wrapping_sub(*b);
    *c ^= rol32(*b, 4);
    *b = (*b).wrapping_add(*a);
}

#[inline(always)]
pub fn __jhash_final(a: &mut u32, b: &mut u32, c: &mut u32) {
    *c ^= *b;
    *c = (*c).wrapping_sub(rol32(*b, 14));
    *a ^= *c;
    *a = (*a).wrapping_sub(rol32(*c, 11));
    *b ^= *a;
    *b = (*b).wrapping_sub(rol32(*a, 25));
    *c ^= *b;
    *c = (*c).wrapping_sub(rol32(*b, 16));
    *a ^= *c;
    *a = (*a).wrapping_sub(rol32(*c, 4));
    *b ^= *a;
    *b = (*b).wrapping_sub(rol32(*a, 14));
    *c ^= *b;
    *c = (*c).wrapping_sub(rol32(*b, 24));
}

pub const JHASH_INITVAL: u32 = 0xdeadbeef;

// C storage/attribute intent: static ATTR
pub unsafe fn jhash(key: *const ::core::ffi::c_void, mut length: u32, initval: u32) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut k: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;

    c = JHASH_INITVAL.wrapping_add(length).wrapping_add(initval);
    b = c;
    a = b;

    while length > 12 {
        a = a.wrapping_add(::core::ptr::read_volatile(k as *const u32));
        b = b.wrapping_add(::core::ptr::read_volatile(k.add(4) as *const u32));
        c = c.wrapping_add(::core::ptr::read_volatile(k.add(8) as *const u32));
        __jhash_mix(&mut a, &mut b, &mut c);
        length = length.wrapping_sub(12);
        k = k.add(12);
    }
    match length {
        12 => {
            c = c.wrapping_add((*(k.add(11)) as u32).wrapping_shl(24));
            c = c.wrapping_add((*(k.add(10)) as u32).wrapping_shl(16));
            c = c.wrapping_add((*(k.add(9)) as u32).wrapping_shl(8));
            c = c.wrapping_add(*(k.add(8)) as u32);
            b = b.wrapping_add((*(k.add(7)) as u32).wrapping_shl(24));
            b = b.wrapping_add((*(k.add(6)) as u32).wrapping_shl(16));
            b = b.wrapping_add((*(k.add(5)) as u32).wrapping_shl(8));
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32).wrapping_shl(24));
            a = a.wrapping_add((*(k.add(2)) as u32).wrapping_shl(16));
            a = a.wrapping_add((*(k.add(1)) as u32).wrapping_shl(8));
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        11 => {
            c = c.wrapping_add((*(k.add(10)) as u32).wrapping_shl(16));
            c = c.wrapping_add((*(k.add(9)) as u32).wrapping_shl(8));
            c = c.wrapping_add(*(k.add(8)) as u32);
            b = b.wrapping_add((*(k.add(7)) as u32).wrapping_shl(24));
            b = b.wrapping_add((*(k.add(6)) as u32).wrapping_shl(16));
            b = b.wrapping_add((*(k.add(5)) as u32).wrapping_shl(8));
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32).wrapping_shl(24));
            a = a.wrapping_add((*(k.add(2)) as u32).wrapping_shl(16));
            a = a.wrapping_add((*(k.add(1)) as u32).wrapping_shl(8));
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        10 => {
            c = c.wrapping_add((*(k.add(9)) as u32).wrapping_shl(8));
            c = c.wrapping_add(*(k.add(8)) as u32);
            b = b.wrapping_add((*(k.add(7)) as u32).wrapping_shl(24));
            b = b.wrapping_add((*(k.add(6)) as u32).wrapping_shl(16));
            b = b.wrapping_add((*(k.add(5)) as u32).wrapping_shl(8));
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32).wrapping_shl(24));
            a = a.wrapping_add((*(k.add(2)) as u32).wrapping_shl(16));
            a = a.wrapping_add((*(k.add(1)) as u32).wrapping_shl(8));
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        9 => {
            c = c.wrapping_add(*(k.add(8)) as u32);
            b = b.wrapping_add((*(k.add(7)) as u32).wrapping_shl(24));
            b = b.wrapping_add((*(k.add(6)) as u32).wrapping_shl(16));
            b = b.wrapping_add((*(k.add(5)) as u32).wrapping_shl(8));
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32).wrapping_shl(24));
            a = a.wrapping_add((*(k.add(2)) as u32).wrapping_shl(16));
            a = a.wrapping_add((*(k.add(1)) as u32).wrapping_shl(8));
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        8 => {
            b = b.wrapping_add((*(k.add(7)) as u32).wrapping_shl(24));
            b = b.wrapping_add((*(k.add(6)) as u32).wrapping_shl(16));
            b = b.wrapping_add((*(k.add(5)) as u32).wrapping_shl(8));
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32).wrapping_shl(24));
            a = a.wrapping_add((*(k.add(2)) as u32).wrapping_shl(16));
            a = a.wrapping_add((*(k.add(1)) as u32).wrapping_shl(8));
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        7 => {
            b = b.wrapping_add((*(k.add(6)) as u32).wrapping_shl(16));
            b = b.wrapping_add((*(k.add(5)) as u32).wrapping_shl(8));
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32).wrapping_shl(24));
            a = a.wrapping_add((*(k.add(2)) as u32).wrapping_shl(16));
            a = a.wrapping_add((*(k.add(1)) as u32).wrapping_shl(8));
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        6 => {
            b = b.wrapping_add((*(k.add(5)) as u32).wrapping_shl(8));
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32).wrapping_shl(24));
            a = a.wrapping_add((*(k.add(2)) as u32).wrapping_shl(16));
            a = a.wrapping_add((*(k.add(1)) as u32).wrapping_shl(8));
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        5 => {
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32).wrapping_shl(24));
            a = a.wrapping_add((*(k.add(2)) as u32).wrapping_shl(16));
            a = a.wrapping_add((*(k.add(1)) as u32).wrapping_shl(8));
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        4 => {
            a = a.wrapping_add((*(k.add(3)) as u32).wrapping_shl(24));
            a = a.wrapping_add((*(k.add(2)) as u32).wrapping_shl(16));
            a = a.wrapping_add((*(k.add(1)) as u32).wrapping_shl(8));
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        3 => {
            a = a.wrapping_add((*(k.add(2)) as u32).wrapping_shl(16));
            a = a.wrapping_add((*(k.add(1)) as u32).wrapping_shl(8));
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        2 => {
            a = a.wrapping_add((*(k.add(1)) as u32).wrapping_shl(8));
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        1 => {
            a = a.wrapping_add(*k as u32);
            c ^= a;
            __jhash_final(&mut a, &mut b, &mut c);
        }
        0 => {
            /* Nothing left to add */
        }
        _ => {}
    }

    c
}

#[inline(always)]
pub unsafe fn jhash2(mut k: *const u32, mut length: u32, initval: u32) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;

    /* Set up the internal state */
    c = JHASH_INITVAL
        .wrapping_add(length.wrapping_shl(2))
        .wrapping_add(initval);
    b = c;
    a = b;

    /* Handle most of the key */
    while length > 3 {
        a = a.wrapping_add(*k.add(0));
        b = b.wrapping_add(*k.add(1));
        c = c.wrapping_add(*k.add(2));
        __jhash_mix(&mut a, &mut b, &mut c);
        length = length.wrapping_sub(3);
        k = k.add(3);
    }

    /* Handle the last 3 u32's */
    match length {
        3 => {
            c = c.wrapping_add(*k.add(2));
            b = b.wrapping_add(*k.add(1));
            a = a.wrapping_add(*k.add(0));
            __jhash_final(&mut a, &mut b, &mut c);
        }
        2 => {
            b = b.wrapping_add(*k.add(1));
            a = a.wrapping_add(*k.add(0));
            __jhash_final(&mut a, &mut b, &mut c);
        }
        1 => {
            a = a.wrapping_add(*k.add(0));
            __jhash_final(&mut a, &mut b, &mut c);
        }
        0 => {
            /* Nothing left to add */
        }
        _ => {}
    }

    c
}
