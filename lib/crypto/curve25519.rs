// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * This is an implementation of the Curve25519 ECDH algorithm, using either an
 * architecture-optimized implementation or a generic implementation. The
 * generic implementation is either 32-bit, or 64-bit with 128-bit integers,
 * depending on what is supported by the target compiler.
 *
 * Information: https://cr.yp.to/ecdh.html
 */

// C dependencies: crypto/curve25519.h, crypto/utils.h, linux/export.h,
// linux/init.h, and linux/module.h.

extern "C" {
    static fn curve25519_generic(
        mypublic: *mut u8,
        secret: *const u8,
        basepoint: *const u8,
    );

    fn crypto_memneq(a: *const u8, b: *const u8, size: usize) -> bool;
}

const CURVE25519_KEY_SIZE: usize = 32;

static CURVE25519_NULL_POINT: [u8; CURVE25519_KEY_SIZE] = [0; CURVE25519_KEY_SIZE];
static CURVE25519_BASE_POINT: [u8; CURVE25519_KEY_SIZE] = {
    let mut point = [0u8; CURVE25519_KEY_SIZE];
    point[0] = 9;
    point
};

// CONFIG_CRYPTO_LIB_CURVE25519_ARCH selects the architecture implementation.
// The generic implementation is used here when it is not selected.
#[cfg(not(feature = "CONFIG_CRYPTO_LIB_CURVE25519_ARCH"))]
unsafe fn curve25519_arch(
    mypublic: *mut u8,
    secret: *const u8,
    basepoint: *const u8,
) {
    unsafe { curve25519_generic(mypublic, secret, basepoint) };
}

#[cfg(not(feature = "CONFIG_CRYPTO_LIB_CURVE25519_ARCH"))]
unsafe fn curve25519_base_arch(pub_: *mut u8, secret: *const u8) {
    unsafe { curve25519_generic(pub_, secret, CURVE25519_BASE_POINT.as_ptr()) };
}

#[cfg(feature = "CONFIG_CRYPTO_LIB_CURVE25519_ARCH")]
extern "C" {
    fn curve25519_arch(
        mypublic: *mut u8,
        secret: *const u8,
        basepoint: *const u8,
    );
    fn curve25519_base_arch(pub_: *mut u8, secret: *const u8);
}

#[must_use]
pub unsafe fn curve25519(
    mypublic: *mut u8,
    secret: *const u8,
    basepoint: *const u8,
) -> bool {
    unsafe { curve25519_arch(mypublic, secret, basepoint) };
    unsafe {
        crypto_memneq(
            mypublic as *const u8,
            CURVE25519_NULL_POINT.as_ptr(),
            CURVE25519_KEY_SIZE,
        )
    }
}

#[must_use]
pub unsafe fn curve25519_generate_public(pub_: *mut u8, secret: *const u8) -> bool {
    if unsafe {
        !crypto_memneq(
            secret,
            CURVE25519_NULL_POINT.as_ptr(),
            CURVE25519_KEY_SIZE,
        )
    } {
        return false;
    }
    unsafe { curve25519_base_arch(pub_, secret) };
    unsafe {
        crypto_memneq(
            pub_ as *const u8,
            CURVE25519_NULL_POINT.as_ptr(),
            CURVE25519_KEY_SIZE,
        )
    }
}

// CONFIG_CRYPTO_LIB_CURVE25519_ARCH may provide curve25519_mod_init_arch.
// The Linux module initialization and export annotations have no direct
// file-local Rust equivalent and are preserved here as dependency intent.

// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("Curve25519 algorithm");
// MODULE_AUTHOR("Jason A. Donenfeld <Jason@zx2c4.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
