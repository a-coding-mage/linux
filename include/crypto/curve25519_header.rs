/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

// Translated from curve25519.h.
// The original header includes Linux definitions for `u8`, `bool`, and
// `get_random_bytes_wait`; those names are supplied by the surrounding code.

pub const CURVE25519_KEY_SIZE: usize = 32;

extern "C" {
    pub fn curve25519_generic(
        out: *mut u8,
        scalar: *const u8,
        point: *const u8,
    );

    pub fn curve25519(
        mypublic: *mut u8,
        secret: *const u8,
        basepoint: *const u8,
    ) -> bool;

    pub fn curve25519_generate_public(
        pub_: *mut u8,
        secret: *const u8,
    ) -> bool;

    fn get_random_bytes_wait(buf: *mut u8, len: usize);
}

#[inline]
pub unsafe fn curve25519_clamp_secret(secret: *mut u8) {
    // C: secret[0] &= 248;
    *secret &= 248;
    // C: secret[31] = (secret[31] & 127) | 64;
    *secret.add(31) = (*secret.add(31) & 127) | 64;
}

#[inline]
pub unsafe fn curve25519_generate_secret(secret: *mut u8) {
    get_random_bytes_wait(secret, CURVE25519_KEY_SIZE);
    curve25519_clamp_secret(secret);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
