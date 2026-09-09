/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * GF(2^128) polynomial hashing: GHASH and POLYVAL
 *
 * Copyright 2025 Google LLC
 */

// C dependencies: crypto/ghash.h, linux/string.h, linux/types.h

pub const POLYVAL_BLOCK_SIZE: usize = 16;
pub const POLYVAL_DIGEST_SIZE: usize = 16;

/**
 * struct polyval_elem - An element of the POLYVAL finite field
 * @bytes: View of the element as a byte array (unioned with @lo and @hi)
 * @lo: The low 64 terms of the element's polynomial
 * @hi: The high 64 terms of the element's polynomial
 *
 * This represents an element of the finite field GF(2^128), using the POLYVAL
 * convention: little-endian byte order and natural bit order.
 */
#[repr(C)]
pub union polyval_elem {
    pub bytes: [u8; POLYVAL_BLOCK_SIZE],
    pub words: polyval_elem_words,
}

#[repr(C)]
pub struct polyval_elem_words {
    pub lo: u64,
    pub hi: u64,
}

/**
 * struct ghash_key - Prepared key for GHASH
 *
 * Use ghash_preparekey() to initialize this.
 */
#[repr(C)]
pub struct ghash_key {
    // C conditional fields preserved below. The active layout depends on the
    // CONFIG_CRYPTO_LIB_GF128HASH_ARCH, CONFIG_PPC64, CONFIG_RISCV, and
    // CONFIG_S390 build configuration symbols.
    #[cfg(all(feature = "CONFIG_CRYPTO_LIB_GF128HASH_ARCH", feature = "CONFIG_PPC64"))]
    pub htable: [[u64; 2]; 4],
    #[cfg(all(
        feature = "CONFIG_CRYPTO_LIB_GF128HASH_ARCH",
        any(feature = "CONFIG_RISCV", feature = "CONFIG_S390")
    ))]
    pub h_raw: [u8; GHASH_BLOCK_SIZE],
    pub h: polyval_elem,
}

/**
 * struct polyval_key - Prepared key for POLYVAL
 *
 * This may contain just the raw key H, or it may contain precomputed key
 * powers, depending on the platform's POLYVAL implementation.  Use
 * polyval_preparekey() to initialize this.
 *
 * By H^i we mean H^(i-1) * H * x^-128, with base case H^1 = H.  I.e. the
 * exponentiation repeats the POLYVAL dot operation, with its "extra" x^-128.
 */
#[repr(C)]
pub struct polyval_key {
    // The active field depends on CONFIG_CRYPTO_LIB_GF128HASH_ARCH and
    // CONFIG_ARM64/CONFIG_X86.
    #[cfg(all(
        feature = "CONFIG_CRYPTO_LIB_GF128HASH_ARCH",
        any(feature = "CONFIG_ARM64", feature = "CONFIG_X86")
    ))]
    pub h_powers: [polyval_elem; 8],
    #[cfg(not(all(
        feature = "CONFIG_CRYPTO_LIB_GF128HASH_ARCH",
        any(feature = "CONFIG_ARM64", feature = "CONFIG_X86")
    )))]
    pub h: polyval_elem,
}

/** struct ghash_ctx - Context for computing a GHASH value */
#[repr(C)]
pub struct ghash_ctx {
    pub key: *const ghash_key,
    pub acc: polyval_elem,
    pub partial: usize,
}

/** struct polyval_ctx - Context for computing a POLYVAL value */
#[repr(C)]
pub struct polyval_ctx {
    pub key: *const polyval_key,
    pub acc: polyval_elem,
    pub partial: usize,
}

extern "C" {
    pub fn ghash_preparekey(key: *mut ghash_key, raw_key: *const u8);
    pub fn polyval_preparekey(key: *mut polyval_key, raw_key: *const u8);
    pub fn ghash_update(ctx: *mut ghash_ctx, data: *const u8, len: usize);
    pub fn polyval_update(ctx: *mut polyval_ctx, data: *const u8, len: usize);
    pub fn ghash_final(ctx: *mut ghash_ctx, out: *mut u8);
    pub fn polyval_final(ctx: *mut polyval_ctx, out: *mut u8);
}

#[inline]
pub unsafe fn ghash_init(ctx: *mut ghash_ctx, key: *const ghash_key) {
    *ctx = core::mem::zeroed();
    (*ctx).key = key;
}

#[inline]
pub unsafe fn polyval_init(ctx: *mut polyval_ctx, key: *const polyval_key) {
    *ctx = core::mem::zeroed();
    (*ctx).key = key;
}

#[inline]
pub unsafe fn polyval_import_blkaligned(
    ctx: *mut polyval_ctx,
    key: *const polyval_key,
    acc: *const polyval_elem,
) {
    *ctx = core::mem::zeroed();
    (*ctx).key = key;
    (*ctx).acc = *acc;
}

#[inline]
pub unsafe fn polyval_export_blkaligned(ctx: *const polyval_ctx, acc: *mut polyval_elem) {
    *acc = (*ctx).acc;
}

#[inline]
pub unsafe fn ghash(
    key: *const ghash_key,
    data: *const u8,
    len: usize,
    out: *mut u8,
) {
    let mut ctx: ghash_ctx = core::mem::zeroed();
    ghash_init(&mut ctx, key);
    ghash_update(&mut ctx, data, len);
    ghash_final(&mut ctx, out);
}

#[inline]
pub unsafe fn polyval(
    key: *const polyval_key,
    data: *const u8,
    len: usize,
    out: *mut u8,
) {
    let mut ctx: polyval_ctx = core::mem::zeroed();
    polyval_init(&mut ctx, key);
    polyval_update(&mut ctx, data, len);
    polyval_final(&mut ctx, out);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
