// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cryptographic API.
 *
 * Blowfish Cipher Algorithm, by Bruce Schneier.
 * http://www.counterpane.com/blowfish.html
 *
 * Adapted from Kerneli implementation.
 *
 * Copyright (c) Herbert Valerio Riedel <hvr@hvrlab.org>
 * Copyright (c) Kyle McMartin <kyle@debian.org>
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 */

// Kernel headers and symbols are supplied by the surrounding translation unit.

#[repr(C)]
pub struct bf_ctx {
    pub p: [u32; 18],
    pub s: [u32; 1024],
}

#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_alg {
    _private: [u8; 0],
}

extern "C" {
    fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut bf_ctx;
    fn crypto_register_alg(alg: *mut crypto_alg) -> i32;
    fn crypto_unregister_alg(alg: *mut crypto_alg);
    fn blowfish_setkey();
}

const BF_BLOCK_SIZE: usize = 8;
const BF_MIN_KEY_SIZE: usize = 4;
const BF_MAX_KEY_SIZE: usize = 56;

#[inline]
unsafe fn get_unaligned_be32(src: *const u8) -> u32 {
    u32::from_be_bytes([*src, *src.add(1), *src.add(2), *src.add(3)])
}

#[inline]
unsafe fn put_unaligned_be32(value: u32, dst: *mut u8) {
    let bytes = value.to_be_bytes();
    *dst = bytes[0];
    *dst.add(1) = bytes[1];
    *dst.add(2) = bytes[2];
    *dst.add(3) = bytes[3];
}

/* Round loop unrolling macros, S is a pointer to a S-Box array
 * organized in 4 unsigned longs at a row.
 */

#[inline]
unsafe fn bf_f(s: *const u32, x: u32) -> u32 {
    let get32_3 = (x & 0xff) as usize;
    let get32_2 = ((x >> 8) & 0xff) as usize;
    let get32_1 = ((x >> 16) & 0xff) as usize;
    let get32_0 = ((x >> 24) & 0xff) as usize;
    ((*s.add(get32_0)).wrapping_add(*s.add(256 + get32_1)) ^ *s.add(512 + get32_2))
        .wrapping_add(*s.add(768 + get32_3))
}

#[inline]
unsafe fn round(a: &mut u32, b: &mut u32, p: *const u32, s: *const u32, n: usize) {
    *b ^= *p.add(n);
    *a ^= bf_f(s, *b);
}

unsafe fn bf_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    let ctx = crypto_tfm_ctx(tfm);
    let p = (*ctx).p.as_ptr();
    let s = (*ctx).s.as_ptr();
    let mut yl = get_unaligned_be32(src);
    let mut yr = get_unaligned_be32(src.add(4));

    round(&mut yr, &mut yl, p, s, 0); round(&mut yl, &mut yr, p, s, 1);
    round(&mut yr, &mut yl, p, s, 2); round(&mut yl, &mut yr, p, s, 3);
    round(&mut yr, &mut yl, p, s, 4); round(&mut yl, &mut yr, p, s, 5);
    round(&mut yr, &mut yl, p, s, 6); round(&mut yl, &mut yr, p, s, 7);
    round(&mut yr, &mut yl, p, s, 8); round(&mut yl, &mut yr, p, s, 9);
    round(&mut yr, &mut yl, p, s, 10); round(&mut yl, &mut yr, p, s, 11);
    round(&mut yr, &mut yl, p, s, 12); round(&mut yl, &mut yr, p, s, 13);
    round(&mut yr, &mut yl, p, s, 14); round(&mut yl, &mut yr, p, s, 15);
    yl ^= *p.add(16); yr ^= *p.add(17);
    put_unaligned_be32(yr, dst); put_unaligned_be32(yl, dst.add(4));
}

unsafe fn bf_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    let ctx = crypto_tfm_ctx(tfm);
    let p = (*ctx).p.as_ptr();
    let s = (*ctx).s.as_ptr();
    let mut yl = get_unaligned_be32(src);
    let mut yr = get_unaligned_be32(src.add(4));

    round(&mut yr, &mut yl, p, s, 17); round(&mut yl, &mut yr, p, s, 16);
    round(&mut yr, &mut yl, p, s, 15); round(&mut yl, &mut yr, p, s, 14);
    round(&mut yr, &mut yl, p, s, 13); round(&mut yl, &mut yr, p, s, 12);
    round(&mut yr, &mut yl, p, s, 11); round(&mut yl, &mut yr, p, s, 10);
    round(&mut yr, &mut yl, p, s, 9); round(&mut yl, &mut yr, p, s, 8);
    round(&mut yr, &mut yl, p, s, 7); round(&mut yl, &mut yr, p, s, 6);
    round(&mut yr, &mut yl, p, s, 5); round(&mut yl, &mut yr, p, s, 4);
    round(&mut yr, &mut yl, p, s, 3); round(&mut yl, &mut yr, p, s, 2);
    yl ^= *p.add(1); yr ^= *p;
    put_unaligned_be32(yr, dst); put_unaligned_be32(yl, dst.add(4));
}

// The crypto_alg initializer and module registration are provided by the
// surrounding kernel-compatibility layer; these declarations preserve their
// externally visible entry points.
extern "C" {
    fn blowfish_mod_init() -> i32;
    fn blowfish_mod_fini();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
