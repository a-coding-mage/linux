/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

// Translated from chacha20poly1305.h.
// The Linux scatterlist type is supplied by the surrounding dependencies.

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(i32)]
pub enum chacha20poly1305_lengths {
    XCHACHA20POLY1305_NONCE_SIZE = 24,
    CHACHA20POLY1305_KEY_SIZE = 32,
    CHACHA20POLY1305_AUTHTAG_SIZE = 16,
}

extern "C" {
    pub fn chacha20poly1305_encrypt(
        dst: *mut u8,
        src: *const u8,
        src_len: usize,
        ad: *const u8,
        ad_len: usize,
        nonce: u64,
        key: *const u8,
    );

    #[must_use]
    pub fn chacha20poly1305_decrypt(
        dst: *mut u8,
        src: *const u8,
        src_len: usize,
        ad: *const u8,
        ad_len: usize,
        nonce: u64,
        key: *const u8,
    ) -> bool;

    pub fn xchacha20poly1305_encrypt(
        dst: *mut u8,
        src: *const u8,
        src_len: usize,
        ad: *const u8,
        ad_len: usize,
        nonce: *const u8,
        key: *const u8,
    );

    #[must_use]
    pub fn xchacha20poly1305_decrypt(
        dst: *mut u8,
        src: *const u8,
        src_len: usize,
        ad: *const u8,
        ad_len: usize,
        nonce: *const u8,
        key: *const u8,
    ) -> bool;

    #[must_use]
    pub fn chacha20poly1305_encrypt_sg_inplace(
        src: *mut scatterlist,
        src_len: usize,
        ad: *const u8,
        ad_len: usize,
        nonce: u64,
        key: *const u8,
    ) -> bool;

    #[must_use]
    pub fn chacha20poly1305_decrypt_sg_inplace(
        src: *mut scatterlist,
        src_len: usize,
        ad: *const u8,
        ad_len: usize,
        nonce: u64,
        key: *const u8,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
