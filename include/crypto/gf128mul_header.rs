/* gf128mul.h - GF(2^128) multiplication functions
 *
 * Copyright (c) 2003, Dr Brian Gladman, Worcester, UK.
 * Copyright (c) 2006 Rik Snel <rsnel@cube.dyndns.org>
 *
 * Based on Dr Brian Gladman's (GPL'd) work published at
 * http://fp.gladman.plus.com/cryptography_technology/index.htm
 * See the original copyright notice below.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 of the License, or (at your option)
 * any later version.
 */
/*
 ---------------------------------------------------------------------------
 Copyright (c) 2003, Dr Brian Gladman, Worcester, UK.   All rights reserved.

 LICENSE TERMS

 The free distribution and use of this software in both source and binary
 form is allowed (with or without changes) provided that:

   1. distributions of this source code include the above copyright
      notice, this list of conditions and the following disclaimer;

   2. distributions in binary form include the above copyright
      notice, this list of conditions and the following disclaimer
      in the documentation and/or other associated materials;

   3. the copyright holder's name is not used to endorse products
      built using this software without specific written permission.

 ALTERNATIVELY, provided that this notice is retained in full, this product
 may be distributed under the terms of the GNU General Public License (GPL),
 in which case the provisions of the GPL apply INSTEAD OF those given above.

 DISCLAIMER

 This software is provided 'as is' with no explicit or implied warranties
 in respect of its properties, including, but not limited to, correctness
 and/or fitness for purpose.
 ---------------------------------------------------------------------------
 Issue Date: 31/01/2006

 An implementation of field multiplication in Galois Field GF(2^128)
*/

// C dependencies: asm/byteorder.h, crypto/b128ops.h, linux/slab.h

/* The extensive representation and licensing comments from the C header
 * apply unchanged to this translation. */

extern "C" {
    pub fn gf128mul_lle(a: *mut be128, b: *const be128);
    pub fn gf128mul_x8_ble(r: *mut le128, x: *const le128);
    pub fn gf128mul_init_64k_bbe(g: *const be128) -> *mut gf128mul_64k;
    pub fn gf128mul_free_64k(t: *mut gf128mul_64k);
    pub fn gf128mul_64k_bbe(a: *mut be128, t: *const gf128mul_64k);
}

/* External types and byte-order operations are supplied by the translated
 * dependencies. */
extern "C" {
    fn be64_to_cpu(x: u64) -> u64;
    fn cpu_to_be64(x: u64) -> u64;
    fn le64_to_cpu(x: u64) -> u64;
    fn cpu_to_le64(x: u64) -> u64;
}

#[inline]
pub unsafe fn gf128mul_mask_from_bit(x: u64, which: i32) -> u64 {
    /* a constant-time version of 'x & ((u64)1 << which) ? (u64)-1 : 0' */
    (((x << (63 - which)) as i64) >> 63) as u64
}

#[inline]
pub unsafe fn gf128mul_x_lle(r: *mut be128, x: *const be128) {
    let a = be64_to_cpu((*x).a);
    let b = be64_to_cpu((*x).b);

    /* equivalent to gf128mul_table_le[(b << 7) & 0xff] << 48
     * (see crypto/gf128mul.c): */
    let _tt = gf128mul_mask_from_bit(b, 0) & (0xe1u64 << 56);

    (*r).b = cpu_to_be64((b >> 1) | (a << 63));
    (*r).a = cpu_to_be64((a >> 1) ^ _tt);
}

#[inline]
pub unsafe fn gf128mul_x_bbe(r: *mut be128, x: *const be128) {
    let a = be64_to_cpu((*x).a);
    let b = be64_to_cpu((*x).b);

    /* equivalent to gf128mul_table_be[a >> 63] (see crypto/gf128mul.c): */
    let _tt = gf128mul_mask_from_bit(a, 63) & 0x87;

    (*r).a = cpu_to_be64((a << 1) | (b >> 63));
    (*r).b = cpu_to_be64((b << 1) ^ _tt);
}

/* needed by XTS */
#[inline]
pub unsafe fn gf128mul_x_ble(r: *mut le128, x: *const le128) {
    let a = le64_to_cpu((*x).a);
    let b = le64_to_cpu((*x).b);

    /* equivalent to gf128mul_table_be[b >> 63] (see crypto/gf128mul.c): */
    let _tt = gf128mul_mask_from_bit(a, 63) & 0x87;

    (*r).a = cpu_to_le64((a << 1) | (b >> 63));
    (*r).b = cpu_to_le64((b << 1) ^ _tt);
}

#[repr(C)]
pub struct gf128mul_64k {
    pub t: [*mut gf128mul_64k_table; 16],
}

#[repr(C)]
pub struct gf128mul_64k_table {
    pub t: [be128; 256],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
