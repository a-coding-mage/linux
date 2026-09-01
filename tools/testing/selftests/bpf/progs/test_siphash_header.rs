// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */

/* C header guard _TEST_SIPHASH_H omitted in Rust. */

/* include/linux/bitops.h */
#[inline]
pub const fn rol64(word: u64, shift: u32) -> u64 {
    word.wrapping_shl(shift & 63) | word.wrapping_shr(0u32.wrapping_sub(shift) & 63)
}

/* include/linux/siphash.h */
#[inline]
pub fn SIPHASH_PERMUTATION(a: &mut u64, b: &mut u64, c: &mut u64, d: &mut u64) {
    *a = (*a).wrapping_add(*b);
    *b = rol64(*b, 13);
    *b ^= *a;
    *a = rol64(*a, 32);
    *c = (*c).wrapping_add(*d);
    *d = rol64(*d, 16);
    *d ^= *c;
    *a = (*a).wrapping_add(*d);
    *d = rol64(*d, 21);
    *d ^= *a;
    *c = (*c).wrapping_add(*b);
    *b = rol64(*b, 17);
    *b ^= *c;
    *c = rol64(*c, 32);
}

pub const SIPHASH_CONST_0: u64 = 0x736f6d6570736575_u64;
pub const SIPHASH_CONST_1: u64 = 0x646f72616e646f6d_u64;
pub const SIPHASH_CONST_2: u64 = 0x6c7967656e657261_u64;
pub const SIPHASH_CONST_3: u64 = 0x7465646279746573_u64;

/* lib/siphash.c */
#[inline]
pub fn SIPROUND(v0: &mut u64, v1: &mut u64, v2: &mut u64, v3: &mut u64) {
    SIPHASH_PERMUTATION(v0, v1, v2, v3);
}

#[inline]
pub unsafe fn siphash_2u64(first: u64, second: u64, key: *const siphash_key_t) -> u64 {
    let mut v0: u64 = SIPHASH_CONST_0;
    let mut v1: u64 = SIPHASH_CONST_1;
    let mut v2: u64 = SIPHASH_CONST_2;
    let mut v3: u64 = SIPHASH_CONST_3;
    let mut b: u64 = (16u64) << 56;

    v3 ^= (*key).key[1];
    v2 ^= (*key).key[0];
    v1 ^= (*key).key[1];
    v0 ^= (*key).key[0];

    v3 ^= first;
    SIPROUND(&mut v0, &mut v1, &mut v2, &mut v3);
    SIPROUND(&mut v0, &mut v1, &mut v2, &mut v3);
    v0 ^= first;
    v3 ^= second;
    SIPROUND(&mut v0, &mut v1, &mut v2, &mut v3);
    SIPROUND(&mut v0, &mut v1, &mut v2, &mut v3);
    v0 ^= second;

    v3 ^= b;
    SIPROUND(&mut v0, &mut v1, &mut v2, &mut v3);
    SIPROUND(&mut v0, &mut v1, &mut v2, &mut v3);
    v0 ^= b;
    v2 ^= 0xff;
    SIPROUND(&mut v0, &mut v1, &mut v2, &mut v3);
    SIPROUND(&mut v0, &mut v1, &mut v2, &mut v3);
    SIPROUND(&mut v0, &mut v1, &mut v2, &mut v3);
    SIPROUND(&mut v0, &mut v1, &mut v2, &mut v3);

    (v0 ^ v1) ^ (v2 ^ v3)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
