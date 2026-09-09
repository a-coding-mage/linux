/* SPDX-License-Identifier: GPL-2.0 */
/*
 * AES-CCM authenticated encryption and decryption
 *
 * Copyright 2026 Google LLC
 */

// Dependency supplied by the AES header:
// use crate::crypto::aes::{aes_enckey, AES_BLOCK_SIZE};

/**
 * struct aes_ccm_key - A key prepared for AES-CCM encryption and decryption
 */
#[repr(C)]
pub struct aes_ccm_key {
    /* private: */
    pub aes: aes_enckey,
    pub authtag_len: usize, /* Length of authentication tags in bytes */
}

/**
 * struct aes_ccm_ctx - Context for incrementally en/decrypting a message
 */
#[repr(C, align(8))]
pub struct aes_ccm_ctx {
    /* private: */
    /*
     * Pointer to the key, which is assumed to live at least as long as this
     * struct.
     */
    pub key: *const aes_ccm_key,
    /*
     * The current CBC-MAC chaining value.  When not on a block boundary,
     * the partial block has been XOR'ed into this.  The number of partial
     * bytes is 'partial_len'.
     */
    pub mac: [u8; AES_BLOCK_SIZE],
    /* The current counter, a 128-bit big endian value */
    pub ctr: [u8; AES_BLOCK_SIZE],
    /* Buffered keystream for partial block updates */
    pub keystream: [u8; AES_BLOCK_SIZE],
    /* Encrypted counter of 0.  This gets XOR'ed with the tag at the end. */
    pub s0: [u8; AES_BLOCK_SIZE],
    /* Number of associated data bytes remaining to be provided */
    pub ad_remaining: u64,
    /* Number of en/decrypted data bytes remaining to be provided */
    pub data_remaining: u64,
    /* Current partial block length, 0 <= partial_len < AES_BLOCK_SIZE */
    pub partial_len: u32,
    /* True if associated data padding has been done */
    pub ad_padded: bool,
}

extern "C" {
    /** Prepare a key for AES-CCM encryption and decryption. */
    pub fn aes_ccm_preparekey(
        key: *mut aes_ccm_key,
        in_key: *const u8,
        key_len: usize,
        authtag_len: usize,
    ) -> i32;

    /** Encrypt a message with AES-CCM. */
    pub fn aes_ccm_encrypt(
        dst: *mut u8,
        src: *const u8,
        data_len: usize,
        authtag: *mut u8,
        ad: *const u8,
        ad_len: usize,
        nonce: *const u8,
        nonce_len: usize,
        key: *const aes_ccm_key,
    ) -> i32;

    /** Decrypt a message with AES-CCM. */
    pub fn aes_ccm_decrypt(
        dst: *mut u8,
        src: *const u8,
        data_len: usize,
        authtag: *const u8,
        ad: *const u8,
        ad_len: usize,
        nonce: *const u8,
        nonce_len: usize,
        key: *const aes_ccm_key,
    ) -> i32;

    /** Initialize context for incremental AES-CCM encryption or decryption. */
    pub fn aes_ccm_init(
        ctx: *mut aes_ccm_ctx,
        data_len: u64,
        ad_len: u64,
        nonce: *const u8,
        nonce_len: usize,
        key: *const aes_ccm_key,
    ) -> i32;

    /** Incrementally process AES-CCM associated data. */
    pub fn aes_ccm_auth_update(ctx: *mut aes_ccm_ctx, ad: *const u8, len: usize);

    /** Incrementally encrypt data with AES-CCM. */
    pub fn aes_ccm_encrypt_update(
        ctx: *mut aes_ccm_ctx,
        dst: *mut u8,
        src: *const u8,
        len: usize,
    );

    /** Incrementally decrypt data with AES-CCM. */
    pub fn aes_ccm_decrypt_update(
        ctx: *mut aes_ccm_ctx,
        dst: *mut u8,
        src: *const u8,
        len: usize,
    );

    /** Finish encrypting a message with AES-CCM. */
    pub fn aes_ccm_encrypt_final(ctx: *mut aes_ccm_ctx, authtag: *mut u8);

    /** Finish decrypting a message with AES-CCM. */
    pub fn aes_ccm_decrypt_final(ctx: *mut aes_ccm_ctx, authtag: *const u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
