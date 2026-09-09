/* gf128mul.c - GF(2^128) multiplication functions
 *
 * Copyright (c) 2003, Dr Brian Gladman, Worcester, UK.
 * Copyright (c) 2006, Rik Snel <rsnel@cube.dyndns.org>
 *
 * Based on Dr Brian Gladman's (GPL'd) work published at
 * http://gladman.plushost.co.uk/oldsite/cryptography_technology/index.php
 * See the original copyright notice below.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 of the License, or (at your option)
 * any later version.
 */

/* The original C source includes crypto/gf128mul.h and Linux allocator,
 * endian, export, and module facilities. These are supplied externally. */

#[repr(C)]
pub struct be128 { pub a: u64, pub b: u64 }
#[repr(C)]
pub struct le128 { pub a: u64, pub b: u64 }
#[repr(C)]
pub struct gf128mul_4k { pub t: [be128; 256] }
#[repr(C)]
pub struct gf128mul_64k { pub t: [*mut gf128mul_4k; 16] }

extern "C" {
    fn gf128mul_x_lle(r: *mut be128, x: *const be128);
    fn gf128mul_x_bbe(r: *mut be128, x: *const be128);
    fn be128_xor(r: *mut be128, a: *const be128, b: *const be128);
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree_sensitive(p: *mut core::ffi::c_void);
}

const fn xda_be(i: u64) -> u16 {
    (if i & 0x80 != 0 { 0x4380 } else { 0 }) ^
    (if i & 0x40 != 0 { 0x21c0 } else { 0 }) ^
    (if i & 0x20 != 0 { 0x10e0 } else { 0 }) ^
    (if i & 0x10 != 0 { 0x0870 } else { 0 }) ^
    (if i & 0x08 != 0 { 0x0438 } else { 0 }) ^
    (if i & 0x04 != 0 { 0x021c } else { 0 }) ^
    (if i & 0x02 != 0 { 0x010e } else { 0 }) ^
    (if i & 0x01 != 0 { 0x0087 } else { 0 })
}

const fn xda_le(i: u64) -> u64 {
    (if i & 0x80 != 0 { 0xe100 } else { 0 }) ^
    (if i & 0x40 != 0 { 0x7080 } else { 0 }) ^
    (if i & 0x20 != 0 { 0x3840 } else { 0 }) ^
    (if i & 0x10 != 0 { 0x1c20 } else { 0 }) ^
    (if i & 0x08 != 0 { 0x0e10 } else { 0 }) ^
    (if i & 0x04 != 0 { 0x0708 } else { 0 }) ^
    (if i & 0x02 != 0 { 0x0384 } else { 0 }) ^
    (if i & 0x01 != 0 { 0x01c2 } else { 0 })
}

const fn make_table() -> [u16; 256] {
    let mut a = [0u16; 256]; let mut i = 0;
    while i < 256 { a[i] = xda_be(i as u64); i += 1; }
    a
}
static GF128MUL_TABLE_BE: [u16; 256] = make_table();

unsafe fn gf128mul_x8_lle_ti(x: *mut be128) {
    let a = u64::from_be((*x).a); let b = u64::from_be((*x).b);
    let tt = xda_le(b & 0xff);
    (*x).b = ((b >> 8) | (a << 56)).to_be();
    (*x).a = ((a >> 8) ^ (tt << 48)).to_be();
}

unsafe fn gf128mul_x8_bbe(x: *mut be128) {
    let a = u64::from_be((*x).a); let b = u64::from_be((*x).b);
    let tt = GF128MUL_TABLE_BE[(a >> 56) as usize] as u64;
    (*x).a = ((a << 8) | (b >> 56)).to_be();
    (*x).b = ((b << 8) ^ tt).to_be();
}

pub unsafe fn gf128mul_x8_ble(r: *mut le128, x: *const le128) {
    let a = u64::from_le((*x).a); let b = u64::from_le((*x).b);
    let tt = GF128MUL_TABLE_BE[(a >> 56) as usize] as u64;
    (*r).a = ((a << 8) | (b >> 56)).to_le();
    (*r).b = ((b << 8) ^ tt).to_le();
}

pub unsafe fn gf128mul_lle(r: *mut be128, b: *const be128) {
    let mut array = [be128 { a: 0, b: 0 }; 19];
    let p = array.as_mut_ptr().add(0);
    *p = *r;
    let mut i = 0;
    while i < 7 { gf128mul_x_lle(p.add(2 * i + 2), p.add(2 * i)); i += 1; }
    *r = be128 { a: 0, b: 0 };
    i = 0;
    loop {
        let ch = *((b as *const u8).add(15 - i));
        be128_xor(r, r, p.add(0 + if ch & 0x80 != 0 { 0 } else { 1 }));
        be128_xor(r, r, p.add(2 + if ch & 0x40 != 0 { 0 } else { 1 }));
        be128_xor(r, r, p.add(4 + if ch & 0x20 != 0 { 0 } else { 1 }));
        be128_xor(r, r, p.add(6 + if ch & 0x10 != 0 { 0 } else { 1 }));
        be128_xor(r, r, p.add(8 + if ch & 0x08 != 0 { 0 } else { 1 }));
        be128_xor(r, r, p.add(10 + if ch & 0x04 != 0 { 0 } else { 1 }));
        be128_xor(r, r, p.add(12 + if ch & 0x02 != 0 { 0 } else { 1 }));
        be128_xor(r, r, p.add(14 + if ch & 0x01 != 0 { 0 } else { 1 }));
        i += 1; if i >= 16 { break; } gf128mul_x8_lle_ti(r);
    }
}

pub unsafe fn gf128mul_init_64k_bbe(g: *const be128) -> *mut gf128mul_64k {
    /* kzalloc_obj and kfree_sensitive are external C allocation facilities. */
    let t = kzalloc(core::mem::size_of::<gf128mul_64k>(), 0) as *mut gf128mul_64k;
    if t.is_null() { return core::ptr::null_mut(); }
    let mut i = 0;
    while i < 16 {
        (*t).t[i] = kzalloc(core::mem::size_of::<gf128mul_4k>(), 0) as *mut gf128mul_4k;
        if (*t).t[i].is_null() { gf128mul_free_64k(t); return core::ptr::null_mut(); }
        i += 1;
    }
    (*(*t).t[0]).t[1] = *g;
    let mut j = 1; while j <= 64 { gf128mul_x_bbe(&mut (*(*t).t[0]).t[j + j], &(*(*t).t[0]).t[j]); j <<= 1; }
    i = 0;
    loop {
        j = 2; while j < 256 { let mut k = 1; while k < j { be128_xor(&mut (*(*t).t[i]).t[j+k], &(*(*t).t[i]).t[j], &(*(*t).t[i]).t[k]); k += 1; } j += j; }
        i += 1; if i >= 16 { break; }
        j = 128; while j > 0 { (*(*t).t[i]).t[j] = (*(*t).t[i-1]).t[j]; gf128mul_x8_bbe(&mut (*(*t).t[i]).t[j]); j >>= 1; }
    }
    t
}

pub unsafe fn gf128mul_free_64k(t: *mut gf128mul_64k) {
    let mut i = 0;
    while i < 16 { kfree_sensitive((*t).t[i] as *mut core::ffi::c_void); i += 1; }
    kfree_sensitive(t as *mut core::ffi::c_void);
}

pub unsafe fn gf128mul_64k_bbe(a: *mut be128, t: *const gf128mul_64k) {
    let ap = a as *const u8; let mut r = (*(*t).t[0]).t[*ap.add(15) as usize];
    let mut i = 1; while i < 16 { be128_xor(&mut r, &r, &(*(*t).t[i]).t[*ap.add(15-i) as usize]); i += 1; }
    *a = r;
}

/* MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("Functions for multiplying elements of GF(2^128)"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
